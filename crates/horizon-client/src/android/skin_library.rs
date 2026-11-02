//! Sylphra-owned custom skin metadata and private PNG storage.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::{Cursor, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const FORMAT_VERSION: u32 = 1;
pub const MAX_SKIN_BYTES: usize = 1024 * 1024;
const MANIFEST: &str = "skin-library.json";
const SKIN_DIR: &str = "skins";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkinModel {
    #[default]
    Classic,
    Slim,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkinRecord {
    pub id: String,
    pub name: String,
    pub file: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceProfile {
    pub id: String,
    pub name: String,
    pub selected_skin: Option<String>,
    #[serde(default)]
    pub model: SkinModel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkinLibrary {
    pub format_version: u32,
    pub active_profile: String,
    pub profiles: Vec<AppearanceProfile>,
    pub skins: Vec<SkinRecord>,
}

impl Default for SkinLibrary {
    fn default() -> Self {
        Self {
            format_version: FORMAT_VERSION,
            active_profile: "default".into(),
            profiles: vec![AppearanceProfile {
                id: "default".into(),
                name: "Default".into(),
                selected_skin: None,
                model: SkinModel::Classic,
            }],
            skins: Vec::new(),
        }
    }
}

impl SkinLibrary {
    /// Missing metadata is a clean first-run state. Malformed or newer metadata
    /// is an error and is never silently replaced.
    pub fn load(home: &Path) -> Result<Self, String> {
        let path = home.join(MANIFEST);
        match fs::symlink_metadata(&path) {
            Ok(meta) if meta.file_type().is_symlink() || !meta.is_file() => {
                return Err("Skin library metadata is not a regular file".into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(error) => return Err(error.to_string()),
        }
        let bytes = fs::read(&path).map_err(|e| e.to_string())?;
        let library: Self = serde_json::from_slice(&bytes)
            .map_err(|e| format!("Skin library metadata is invalid: {e}"))?;
        library.validate()?;
        Ok(library)
    }

    pub fn save(&self, home: &Path) -> Result<(), String> {
        self.validate()?;
        ensure_home(home)?;
        let path = home.join(MANIFEST);
        if fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("Refusing to replace redirected skin metadata".into());
        }
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let temporary = home.join(format!(".skin-library-{stamp}.partial"));
        let bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        let result = (|| -> Result<(), String> {
            file.write_all(&bytes).map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            drop(file);
            fs::rename(&temporary, &path).map_err(|e| e.to_string())?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }
        result
    }

    /// Import a validated PNG into Sylphra's app-private directory and select
    /// it for the active appearance profile. The original game files are never
    /// touched.
    pub fn import_png(
        &mut self,
        home: &Path,
        display_name: &str,
        bytes: &[u8],
        model: SkinModel,
    ) -> Result<SkinRecord, String> {
        let (width, height) = validate_png(bytes)?;
        ensure_home(home)?;
        let directory = home.join(SKIN_DIR);
        if let Ok(meta) = fs::symlink_metadata(&directory)
            && meta.file_type().is_symlink()
        {
            return Err("Refusing a redirected skin directory".into());
        }
        fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        if fs::symlink_metadata(&directory)
            .map_err(|e| e.to_string())?
            .file_type()
            .is_symlink()
        {
            return Err("Refusing a redirected skin directory".into());
        }

        let digest = Sha256::digest(bytes);
        let id = format!("{digest:x}");
        let file_name = format!("{id}.png");
        let target = directory.join(&file_name);
        let mut created = false;
        match OpenOptions::new().write(true).create_new(true).open(&target) {
            Ok(mut file) => {
                created = true;
                if let Err(error) = file.write_all(bytes).and_then(|_| file.sync_all()) {
                    let _ = fs::remove_file(&target);
                    return Err(error.to_string());
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                let meta = fs::symlink_metadata(&target).map_err(|e| e.to_string())?;
                if meta.file_type().is_symlink() || !meta.is_file() {
                    return Err("Skin file is not a regular file".into());
                }
                if fs::read(&target).map_err(|e| e.to_string())? != bytes {
                    return Err("Skin file hash collision detected".into());
                }
            }
            Err(error) => return Err(error.to_string()),
        }

        let record = SkinRecord {
            id: id.clone(),
            name: clean_name(display_name),
            file: file_name,
            width,
            height,
        };
        let previous = self.clone();
        if let Some(existing) = self.skins.iter_mut().find(|skin| skin.id == id) {
            existing.name = record.name.clone();
            existing.width = record.width;
            existing.height = record.height;
        } else {
            self.skins.push(record.clone());
        }
        let profile = self.active_profile_mut()?;
        profile.selected_skin = Some(id);
        profile.model = model;
        if let Err(error) = self.save(home) {
            *self = previous;
            if created {
                let _ = fs::remove_file(target);
            }
            return Err(error);
        }
        Ok(record)
    }

    pub fn select_skin(&mut self, profile_id: &str, skin_id: Option<&str>) -> Result<(), String> {
        if let Some(id) = skin_id
            && !self.skins.iter().any(|skin| skin.id == id)
        {
            return Err("Selected skin does not exist in the library".into());
        }
        let profile = self
            .profiles
            .iter_mut()
            .find(|profile| profile.id == profile_id)
            .ok_or("Appearance profile does not exist")?;
        profile.selected_skin = skin_id.map(str::to_owned);
        Ok(())
    }

    pub fn set_active_profile(&mut self, profile_id: &str) -> Result<(), String> {
        if !self.profiles.iter().any(|profile| profile.id == profile_id) {
            return Err("Appearance profile does not exist".into());
        }
        self.active_profile = profile_id.to_owned();
        Ok(())
    }

    pub fn add_profile(&mut self, id: String, name: String) -> Result<(), String> {
        if !valid_component(&id)
            || self.profiles.iter().any(|profile| profile.id == id)
            || name.trim().is_empty()
            || name.chars().count() > 48
        {
            return Err("Invalid or duplicate appearance profile".into());
        }
        self.profiles.push(AppearanceProfile {
            id,
            name: name.trim().to_owned(),
            selected_skin: None,
            model: SkinModel::Classic,
        });
        Ok(())
    }

    pub fn active_skin(&self) -> Option<&SkinRecord> {
        let profile = self.profiles.iter().find(|p| p.id == self.active_profile)?;
        let id = profile.selected_skin.as_deref()?;
        self.skins.iter().find(|skin| skin.id == id)
    }

    pub fn active_model(&self) -> Option<SkinModel> {
        self.profiles
            .iter()
            .find(|profile| profile.id == self.active_profile)
            .map(|profile| profile.model)
    }

    pub fn set_active_model(&mut self, model: SkinModel) -> Result<(), String> {
        self.active_profile_mut()?.model = model;
        Ok(())
    }

    fn active_profile_mut(&mut self) -> Result<&mut AppearanceProfile, String> {
        self.profiles
            .iter_mut()
            .find(|profile| profile.id == self.active_profile)
            .ok_or_else(|| "Active appearance profile is missing".into())
    }

    fn validate(&self) -> Result<(), String> {
        if self.format_version != FORMAT_VERSION {
            return Err(format!(
                "Unsupported skin library version {}",
                self.format_version
            ));
        }
        if self.profiles.is_empty()
            || !self.profiles.iter().any(|p| p.id == self.active_profile)
        {
            return Err("Skin library has no valid active appearance profile".into());
        }
        for (index, profile) in self.profiles.iter().enumerate() {
            if !valid_component(&profile.id)
                || profile.name.trim().is_empty()
                || profile.name.chars().count() > 48
                || self.profiles[..index].iter().any(|p| p.id == profile.id)
            {
                return Err("Skin library contains an invalid appearance profile".into());
            }
        }
        for (index, skin) in self.skins.iter().enumerate() {
            if !valid_hash(&skin.id)
                || skin.file != format!("{}.png", skin.id)
                || !matches!((skin.width, skin.height), (64, 64) | (64, 32))
                || skin.name.trim().is_empty()
                || skin.name.chars().count() > 48
                || self.skins[..index].iter().any(|s| s.id == skin.id)
            {
                return Err("Skin library contains an invalid skin record".into());
            }
        }
        if self.profiles.iter().any(|profile| {
            profile
                .selected_skin
                .as_ref()
                .is_some_and(|id| !self.skins.iter().any(|skin| &skin.id == id))
        }) {
            return Err("Appearance profile references a missing skin".into());
        }
        Ok(())
    }
}

fn ensure_home(home: &Path) -> Result<(), String> {
    fs::create_dir_all(home).map_err(|e| e.to_string())?;
    let meta = fs::symlink_metadata(home).map_err(|e| e.to_string())?;
    if meta.file_type().is_symlink() || !meta.is_dir() {
        return Err("Sylphra data directory is not a regular directory".into());
    }
    Ok(())
}

fn valid_component(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn valid_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

fn clean_name(value: &str) -> String {
    let stem = Path::new(value)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported skin");
    let mut name: String = stem
        .chars()
        .filter(|c| !c.is_control() && !matches!(c, '/' | '\\' | ':'))
        .take(48)
        .collect();
    name = name.trim().to_owned();
    if name.is_empty() {
        "Imported skin".into()
    } else {
        name
    }
}

fn validate_png(bytes: &[u8]) -> Result<(u32, u32), String> {
    if bytes.is_empty() || bytes.len() > MAX_SKIN_BYTES {
        return Err("Skin PNG must be between 1 byte and 1 MiB".into());
    }
    let decoder = png::Decoder::new(Cursor::new(bytes));
    let mut reader = decoder
        .read_info()
        .map_err(|e| format!("Invalid PNG: {e}"))?;
    let (width, height) = (reader.info().width, reader.info().height);
    if !matches!((width, height), (64, 64) | (64, 32)) {
        return Err("Skin PNG must be 64×64 or legacy 64×32".into());
    }
    let output_size = reader.output_buffer_size();
    if output_size > 64 * 64 * 4 {
        return Err("Skin PNG expands beyond the supported image size".into());
    }
    let mut output = vec![0; output_size];
    reader
        .next_frame(&mut output)
        .map_err(|e| format!("Invalid PNG image data: {e}"))?;
    Ok((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn home() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("sylphra-skins-{}-{stamp}", std::process::id()))
    }

    fn png(width: u32, height: u32) -> Vec<u8> {
        png_with(width, height, [255, 255, 255, 255])
    }

    fn png_with(width: u32, height: u32, pixel: [u8; 4]) -> Vec<u8> {
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(Cursor::new(&mut bytes), width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            let pixels: Vec<u8> = (0..width * height).flat_map(|_| pixel).collect();
            writer.write_image_data(&pixels).unwrap();
        }
        bytes
    }

    #[test]
    fn missing_manifest_is_a_backward_compatible_empty_library() {
        let path = home();
        let library = SkinLibrary::load(&path).unwrap();
        assert_eq!(library.format_version, FORMAT_VERSION);
        assert_eq!(library.active_profile, "default");
        assert!(library.skins.is_empty());
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn imports_modern_and_legacy_png_and_selects_active_profile() {
        for (width, height) in [(64, 64), (64, 32)] {
            let path = home();
            let mut library = SkinLibrary::load(&path).unwrap();
            let record = library
                .import_png(&path, "My Skin.png", &png(width, height), SkinModel::Slim)
                .unwrap();
            assert_eq!((record.width, record.height), (width, height));
            assert_eq!(library.active_model(), Some(SkinModel::Slim));
            assert_eq!(library.active_skin(), Some(&record));
            let loaded = SkinLibrary::load(&path).unwrap();
            assert_eq!(loaded.active_skin(), Some(&record));
            let _ = fs::remove_dir_all(path);
        }
    }

    #[test]
    fn rejects_bad_dimensions_malformed_and_oversized_png() {
        let path = home();
        let mut library = SkinLibrary::default();
        assert!(library.import_png(&path, "bad.png", &png(32, 32), SkinModel::Classic).is_err());
        assert!(library.import_png(&path, "bad.png", b"not png", SkinModel::Classic).is_err());
        assert!(
            library
                .import_png(&path, "large.png", &vec![0; MAX_SKIN_BYTES + 1], SkinModel::Classic)
                .is_err()
        );
        assert!(!path.exists());
    }

    #[test]
    fn profiles_keep_independent_skin_selections() {
        let path = home();
        let mut library = SkinLibrary::load(&path).unwrap();
        let first = library
            .import_png(&path, "one.png", &png(64, 64), SkinModel::Classic)
            .unwrap();
        let second = library
            .import_png(
                &path,
                "two.png",
                &png_with(64, 64, [0, 0, 0, 255]),
                SkinModel::Slim,
            )
            .unwrap();
        library.add_profile("builder".into(), "Builder".into()).unwrap();
        library.select_skin("builder", Some(&second.id)).unwrap();
        library.set_active_model(SkinModel::Slim).unwrap();
        library.select_skin("default", Some(&first.id)).unwrap();
        library.set_active_model(SkinModel::Classic).unwrap();
        library.set_active_profile("builder").unwrap();
        library.save(&path).unwrap();
        let loaded = SkinLibrary::load(&path).unwrap();
        assert_eq!(loaded.active_skin().unwrap().id, second.id);
        assert_eq!(loaded.active_model(), Some(SkinModel::Slim));
        let default = loaded.profiles.iter().find(|p| p.id == "default").unwrap();
        assert_eq!(default.selected_skin.as_deref(), Some(first.id.as_str()));
        assert_eq!(default.model, SkinModel::Classic);
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn corrupt_or_newer_manifest_is_not_overwritten() {
        let path = home();
        fs::create_dir_all(&path).unwrap();
        let manifest = path.join(MANIFEST);
        fs::write(&manifest, b"{broken").unwrap();
        assert!(SkinLibrary::load(&path).is_err());
        assert_eq!(fs::read(&manifest).unwrap(), b"{broken");
        let _ = fs::remove_dir_all(path);
    }
}
