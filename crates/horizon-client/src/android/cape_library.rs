//! Sylphra-owned textile cape metadata and private PNG storage.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::{Cursor, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const FORMAT_VERSION: u32 = 1;
pub const MAX_CAPE_BYTES: usize = 1024 * 1024;
const MANIFEST: &str = "cape-library.json";
const CAPE_DIR: &str = "capes";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapeRecord {
    pub id: String,
    pub name: String,
    pub file: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapeSelection {
    pub profile: String,
    pub cape: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapeLibrary {
    pub format_version: u32,
    pub capes: Vec<CapeRecord>,
    #[serde(default)]
    pub selections: Vec<CapeSelection>,
}

impl Default for CapeLibrary {
    fn default() -> Self {
        Self {
            format_version: FORMAT_VERSION,
            capes: Vec::new(),
            selections: Vec::new(),
        }
    }
}

impl CapeLibrary {
    /// A missing manifest is a clean first-run state. Invalid or newer
    /// manifests are never silently replaced.
    pub fn load(home: &Path) -> Result<Self, String> {
        let path = home.join(MANIFEST);
        match fs::symlink_metadata(&path) {
            Ok(meta) if meta.file_type().is_symlink() || !meta.is_file() => {
                return Err("Cape library metadata is not a regular file".into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(error) => return Err(error.to_string()),
        }
        let bytes = fs::read(&path).map_err(|e| e.to_string())?;
        let library: Self = serde_json::from_slice(&bytes)
            .map_err(|e| format!("Cape library metadata is invalid: {e}"))?;
        library.validate()?;
        Ok(library)
    }

    pub fn save(&self, home: &Path) -> Result<(), String> {
        self.validate()?;
        ensure_home(home)?;
        let path = home.join(MANIFEST);
        match fs::symlink_metadata(&path) {
            Ok(meta) if meta.file_type().is_symlink() || !meta.is_file() => {
                return Err("Refusing to replace non-regular cape metadata".into());
            }
            Ok(_) => {
                // Validate the on-disk manifest before replacing it. A caller
                // holding a default or stale in-memory library must never turn
                // corrupt/unknown metadata into an empty library on save.
                let existing_bytes = fs::read(&path).map_err(|error| error.to_string())?;
                let existing: Self = serde_json::from_slice(&existing_bytes).map_err(|error| {
                    format!("Existing cape library metadata is invalid; refusing to overwrite: {error}")
                })?;
                existing.validate().map_err(|error| {
                    format!("Existing cape library metadata is invalid; refusing to overwrite: {error}")
                })?;
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let temporary = home.join(format!(".cape-library-{stamp}.partial"));
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

    pub fn import_png(
        &mut self,
        home: &Path,
        display_name: &str,
        bytes: &[u8],
    ) -> Result<CapeRecord, String> {
        let (width, height) = validate_png(bytes)?;
        ensure_home(home)?;
        let directory = home.join(CAPE_DIR);
        if let Ok(meta) = fs::symlink_metadata(&directory)
            && meta.file_type().is_symlink()
        {
            return Err("Refusing a redirected cape directory".into());
        }
        fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        if fs::symlink_metadata(&directory)
            .map_err(|e| e.to_string())?
            .file_type()
            .is_symlink()
        {
            return Err("Refusing a redirected cape directory".into());
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
                    return Err("Cape file is not a regular file".into());
                }
                if fs::read(&target).map_err(|e| e.to_string())? != bytes {
                    return Err("Cape file hash collision detected".into());
                }
            }
            Err(error) => return Err(error.to_string()),
        }

        let record = CapeRecord {
            id: id.clone(),
            name: clean_name(display_name),
            file: file_name,
            width,
            height,
        };
        let previous = self.clone();
        if let Some(existing) = self.capes.iter_mut().find(|cape| cape.id == id) {
            existing.name = record.name.clone();
            existing.width = record.width;
            existing.height = record.height;
        } else {
            self.capes.push(record.clone());
        }
        if let Err(error) = self.save(home) {
            *self = previous;
            if created {
                let _ = fs::remove_file(target);
            }
            return Err(error);
        }
        Ok(record)
    }

    pub fn select_cape(
        &mut self,
        profile_id: &str,
        cape_id: Option<&str>,
    ) -> Result<(), String> {
        if let Some(id) = cape_id
            && !self.capes.iter().any(|cape| cape.id == id)
        {
            return Err("Selected cape does not exist in the library".into());
        }
        self.selections.retain(|selection| selection.profile != profile_id);
        if let Some(id) = cape_id {
            self.selections.push(CapeSelection {
                profile: profile_id.to_owned(),
                cape: id.to_owned(),
            });
        }
        Ok(())
    }

    pub fn active_cape(&self, profile_id: &str) -> Option<&CapeRecord> {
        let id = self
            .selections
            .iter()
            .find(|selection| selection.profile == profile_id)?
            .cape
            .as_str();
        self.capes.iter().find(|cape| cape.id == id)
    }

    /// Read a cape only after verifying its app-private path, content hash,
    /// PNG structure, and dimensions against the manifest record.
    pub fn read_validated_file(
        &self,
        home: &Path,
        cape_id: &str,
    ) -> Result<(PathBuf, Vec<u8>), String> {
        let record = self
            .capes
            .iter()
            .find(|cape| cape.id == cape_id)
            .ok_or("Cape does not exist in the library")?;
        let directory = home.join(CAPE_DIR);
        let directory_metadata =
            fs::symlink_metadata(&directory).map_err(|error| error.to_string())?;
        if directory_metadata.file_type().is_symlink() || !directory_metadata.is_dir() {
            return Err("Cape storage directory is not a regular app-private directory".into());
        }
        let path = directory.join(&record.file);
        let metadata = fs::symlink_metadata(&path).map_err(|error| error.to_string())?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err("Cape file is not a regular app-private PNG file".into());
        }
        if metadata.len() == 0 || metadata.len() > MAX_CAPE_BYTES as u64 {
            return Err("Cape file is empty or exceeds the 1 MiB limit".into());
        }
        let bytes = fs::read(&path).map_err(|error| error.to_string())?;
        if bytes.is_empty() || bytes.len() > MAX_CAPE_BYTES {
            return Err("Cape file is empty or exceeds the 1 MiB limit".into());
        }
        if format!("{:x}", Sha256::digest(&bytes)) != record.id {
            return Err("Cape file content does not match its library hash".into());
        }
        let dimensions = validate_png(&bytes)?;
        if dimensions != (record.width, record.height) {
            return Err("Cape file dimensions do not match its library record".into());
        }
        Ok((path, bytes))
    }

    /// Persist a profile selection atomically from the caller's perspective.
    /// If the manifest cannot be saved, restore the previous in-memory state
    /// so a failed UI action cannot affect the next game launch.
    pub fn select_cape_and_save(
        &mut self,
        profile_id: &str,
        cape_id: Option<&str>,
        home: &Path,
    ) -> Result<(), String> {
        let previous = self.clone();
        self.select_cape(profile_id, cape_id)?;
        if let Err(error) = self.save(home) {
            *self = previous;
            return Err(error);
        }
        Ok(())
    }

    pub fn rename(&mut self, cape_id: &str, name: &str, home: &Path) -> Result<(), String> {
        let name = clean_name(name);
        let previous = self.clone();
        let cape = self
            .capes
            .iter_mut()
            .find(|cape| cape.id == cape_id)
            .ok_or("Cape does not exist")?;
        cape.name = name;
        if let Err(error) = self.save(home) {
            *self = previous;
            return Err(error);
        }
        Ok(())
    }

    /// Remove metadata and all profile selections first. The caller must ask
    /// the user for confirmation before invoking this destructive operation.
    /// The PNG is removed only after the manifest update succeeds.
    pub fn remove(&mut self, cape_id: &str, home: &Path) -> Result<(), String> {
        let previous = self.clone();
        let record = self
            .capes
            .iter()
            .find(|cape| cape.id == cape_id)
            .cloned()
            .ok_or("Cape does not exist")?;
        self.capes.retain(|cape| cape.id != cape_id);
        self.selections.retain(|selection| selection.cape != cape_id);
        if let Err(error) = self.save(home) {
            *self = previous;
            return Err(error);
        }
        let path = home.join(CAPE_DIR).join(record.file);
        match fs::symlink_metadata(&path) {
            Ok(meta) if meta.file_type().is_symlink() || !meta.is_file() => {
                return Err("Cape file is not a regular file; metadata was removed".into());
            }
            Ok(_) => fs::remove_file(path).map_err(|e| e.to_string())?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        if self.format_version != FORMAT_VERSION {
            return Err(format!(
                "Unsupported cape library version {}",
                self.format_version
            ));
        }
        for (index, cape) in self.capes.iter().enumerate() {
            if !valid_hash(&cape.id)
                || cape.file != format!("{}.png", cape.id)
                || !matches!((cape.width, cape.height), (64, 32) | (22, 17))
                || cape.name.trim().is_empty()
                || cape.name.chars().count() > 48
                || self.capes[..index].iter().any(|other| other.id == cape.id)
            {
                return Err("Cape library contains an invalid cape record".into());
            }
        }
        for (index, selection) in self.selections.iter().enumerate() {
            if !valid_component(&selection.profile)
                || !valid_hash(&selection.cape)
                || !self.capes.iter().any(|cape| cape.id == selection.cape)
                || self.selections[..index]
                    .iter()
                    .any(|other| other.profile == selection.profile)
            {
                return Err("Cape library contains an invalid profile selection".into());
            }
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
    let value = value.trim();
    // Strip the imported file extension, but preserve dots in user-assigned
    // names such as "Cape.v2".
    let stem = value
        .rsplit_once('.')
        .filter(|(_, extension)| extension.eq_ignore_ascii_case("png"))
        .map(|(stem, _)| stem)
        .unwrap_or(value);
    let mut name: String = stem
        .chars()
        .filter(|c| !c.is_control() && !matches!(c, '/' | '\\' | ':'))
        .take(48)
        .collect();
    name = name.trim().to_owned();
    if name.is_empty() {
        "Imported cape".into()
    } else {
        name
    }
}

fn validate_png(bytes: &[u8]) -> Result<(u32, u32), String> {
    if bytes.is_empty() || bytes.len() > MAX_CAPE_BYTES {
        return Err("Cape PNG must be between 1 byte and 1 MiB".into());
    }
    let decoder = png::Decoder::new(Cursor::new(bytes));
    let mut reader = decoder
        .read_info()
        .map_err(|e| format!("Invalid PNG: {e}"))?;
    let (width, height) = (reader.info().width, reader.info().height);
    if !matches!((width, height), (64, 32) | (22, 17)) {
        return Err("Cape PNG must be 64×32 or 22×17".into());
    }
    let Some(output_size) = reader.output_buffer_size() else {
        return Err("Unable to determine decoded cape image size".into());
    };
    if output_size > 64 * 32 * 4 {
        return Err("Cape PNG expands beyond the supported image size".into());
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
    use std::{io::Cursor, path::PathBuf};

    fn home() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("sylphra-capes-{}-{stamp}", std::process::id()))
    }

    fn png(width: u32, height: u32, pixel: [u8; 4]) -> Vec<u8> {
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
    fn imports_supported_formats_and_deduplicates_by_hash() {
        for (width, height) in [(64, 32), (22, 17)] {
            let path = home();
            let mut library = CapeLibrary::load(&path).unwrap();
            let bytes = png(width, height, [255, 0, 0, 255]);
            let first = library.import_png(&path, "Cape.png", &bytes).unwrap();
            let second = library.import_png(&path, "Cape copy.png", &bytes).unwrap();
            assert_eq!(first.id, second.id);
            assert_eq!((first.width, first.height), (width, height));
            let loaded = CapeLibrary::load(&path).unwrap();
            assert_eq!(loaded.capes.len(), 1);
            let _ = fs::remove_dir_all(path);
        }
    }

    #[test]
    fn rejects_bad_png_dimensions_and_oversized_input() {
        let path = home();
        let mut library = CapeLibrary::default();
        assert!(library.import_png(&path, "bad.png", &png(64, 64, [0, 0, 0, 255])).is_err());
        assert!(library.import_png(&path, "bad.png", b"not png").is_err());
        assert!(library.import_png(&path, "large.png", &vec![0; MAX_CAPE_BYTES + 1]).is_err());
        assert!(!path.exists());
    }

    #[test]
    fn profile_selections_are_independent_and_none_clears_only_one() {
        let path = home();
        let mut library = CapeLibrary::load(&path).unwrap();
        let first = library.import_png(&path, "one.png", &png(64, 32, [255, 0, 0, 255])).unwrap();
        let second = library.import_png(&path, "two.png", &png(64, 32, [0, 0, 255, 255])).unwrap();
        library.select_cape("default", Some(&first.id)).unwrap();
        library.select_cape("builder", Some(&second.id)).unwrap();
        library.select_cape("default", None).unwrap();
        assert!(library.active_cape("default").is_none());
        assert_eq!(library.active_cape("builder").unwrap().id, second.id);
        library.save(&path).unwrap();
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn failed_selection_save_restores_previous_in_memory_selection() {
        let path = home();
        let mut library = CapeLibrary::load(&path).unwrap();
        let cape = library
            .import_png(&path, "one.png", &png(64, 32, [255, 0, 0, 255]))
            .unwrap();
        library.select_cape("default", Some(&cape.id)).unwrap();
        library.save(&path).unwrap();

        let manifest = path.join(MANIFEST);
        fs::remove_file(&manifest).unwrap();
        fs::create_dir(&manifest).unwrap();
        assert!(library
            .select_cape_and_save("default", None, &path)
            .is_err());
        assert_eq!(library.active_cape("default").unwrap().id, cape.id);

        assert!(library
            .select_cape_and_save("builder", Some(&cape.id), &path)
            .is_err());
        assert!(library.active_cape("builder").is_none());
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn remove_clears_profile_references_after_manifest_save() {
        let path = home();
        let mut library = CapeLibrary::load(&path).unwrap();
        let cape = library.import_png(&path, "one.png", &png(64, 32, [255, 0, 0, 255])).unwrap();
        library.select_cape("default", Some(&cape.id)).unwrap();
        library.save(&path).unwrap();
        library.remove(&cape.id, &path).unwrap();
        assert!(library.active_cape("default").is_none());
        assert!(!path.join("capes").join(cape.file).exists());
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn rename_preserves_dotted_names_and_selection_persists_per_profile() {
        let path = home();
        let mut library = CapeLibrary::load(&path).unwrap();
        let cape = library
            .import_png(&path, "Cape.png", &png(64, 32, [255, 0, 0, 255]))
            .unwrap();
        library.select_cape("default", Some(&cape.id)).unwrap();
        library.select_cape("builder", Some(&cape.id)).unwrap();

        library.rename(&cape.id, "Cape.v2", &path).unwrap();
        assert_eq!(library.capes[0].name, "Cape.v2");
        let mut loaded = CapeLibrary::load(&path).unwrap();
        assert_eq!(loaded.active_cape("default").unwrap().name, "Cape.v2");
        assert_eq!(loaded.active_cape("builder").unwrap().id, cape.id);

        loaded.select_cape("default", None).unwrap();
        loaded.save(&path).unwrap();
        let loaded = CapeLibrary::load(&path).unwrap();
        assert!(loaded.active_cape("default").is_none());
        assert_eq!(loaded.active_cape("builder").unwrap().id, cape.id);
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn remove_clears_only_references_to_removed_cape() {
        let path = home();
        let mut library = CapeLibrary::load(&path).unwrap();
        let first = library
            .import_png(&path, "one.png", &png(64, 32, [255, 0, 0, 255]))
            .unwrap();
        let second = library
            .import_png(&path, "two.png", &png(64, 32, [0, 0, 255, 255]))
            .unwrap();
        library.select_cape("default", Some(&first.id)).unwrap();
        library.select_cape("builder", Some(&second.id)).unwrap();
        library.save(&path).unwrap();

        library.remove(&first.id, &path).unwrap();
        let loaded = CapeLibrary::load(&path).unwrap();
        assert!(loaded.active_cape("default").is_none());
        assert_eq!(loaded.active_cape("builder").unwrap().id, second.id);
        assert!(!path.join("capes").join(first.file).exists());
        assert!(path.join("capes").join(second.file).is_file());
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn validated_file_reader_rejects_tampered_content() {
        let path = home();
        let mut library = CapeLibrary::load(&path).unwrap();
        let cape = library
            .import_png(&path, "cape.png", &png(64, 32, [255, 0, 0, 255]))
            .unwrap();

        let (resolved, bytes) = library.read_validated_file(&path, &cape.id).unwrap();
        assert_eq!(resolved, path.join(CAPE_DIR).join(&cape.file));
        assert_eq!(bytes.len(), fs::metadata(&resolved).unwrap().len() as usize);

        fs::write(&resolved, png(64, 32, [0, 0, 255, 255])).unwrap();
        assert!(library
            .read_validated_file(&path, &cape.id)
            .unwrap_err()
            .contains("hash"));
        let _ = fs::remove_dir_all(path);
    }

    #[cfg(unix)]
    #[test]
    fn validated_file_reader_rejects_symlinked_cape_directory() {
        let path = home();
        let mut library = CapeLibrary::load(&path).unwrap();
        let cape = library
            .import_png(&path, "cape.png", &png(64, 32, [255, 0, 0, 255]))
            .unwrap();
        let real_directory = path.join("capes-real");
        let redirected_directory = path.join(CAPE_DIR);
        fs::rename(&redirected_directory, &real_directory).unwrap();
        let external_directory = path.join("capes-external");
        fs::create_dir_all(&external_directory).unwrap();
        fs::copy(
            real_directory.join(&cape.file),
            external_directory.join(&cape.file),
        )
        .unwrap();
        std::os::unix::fs::symlink(&external_directory, &redirected_directory).unwrap();

        assert!(library
            .read_validated_file(&path, &cape.id)
            .unwrap_err()
            .contains("directory"));
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn missing_manifest_is_backward_compatible_and_corrupt_manifest_is_preserved() {
        let path = home();
        assert!(CapeLibrary::load(&path).unwrap().capes.is_empty());
        fs::create_dir_all(&path).unwrap();
        let manifest = path.join(MANIFEST);
        fs::write(&manifest, b"{broken").unwrap();
        assert!(CapeLibrary::load(&path).is_err());
        assert!(CapeLibrary::default().save(&path).is_err());
        assert_eq!(fs::read(&manifest).unwrap(), b"{broken");
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn save_refuses_unknown_manifest_versions_without_replacing_them() {
        let path = home();
        fs::create_dir_all(&path).unwrap();
        let manifest = path.join(MANIFEST);
        let original = br#"{"format_version":99,"capes":[],"selections":[]}"#;
        fs::write(&manifest, original).unwrap();

        assert!(CapeLibrary::default().save(&path).is_err());
        assert_eq!(fs::read(&manifest).unwrap(), original);
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn import_rolls_back_new_png_when_existing_manifest_is_corrupt() {
        let path = home();
        fs::create_dir_all(&path).unwrap();
        let manifest = path.join(MANIFEST);
        let original = b"{broken";
        fs::write(&manifest, original).unwrap();
        let mut library = CapeLibrary::default();

        assert!(library
            .import_png(&path, "cape.png", &png(64, 32, [255, 0, 0, 255]))
            .is_err());
        assert!(library.capes.is_empty());
        assert_eq!(fs::read(&manifest).unwrap(), original);
        assert_eq!(fs::read_dir(path.join(CAPE_DIR)).unwrap().count(), 0);
        let _ = fs::remove_dir_all(path);
    }
}
