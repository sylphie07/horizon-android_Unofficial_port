//! Mobile launcher state. Browser operations reuse the desktop backend unchanged.
use crate::browser::{self, Kind, Project, Query, Source, installed::Installed};
#[path = "skin_library.rs"]
pub mod skin_library;
#[path = "cape_library.rs"]
pub mod cape_library;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex, mpsc},
};
pub fn source_only_mod(project: &Project) -> bool {
    project.source == browser::SourceId::Horizon
        && project.loaders.iter().any(|loader| loader == "hnmod")
}

/// Temporary dev registration published as Minecraft Nintendo Switch by PrismarineJS.
/// Account tokens and client secrets are never bundled.
pub const MICROSOFT_CLIENT_ID: &str = "00000000441cc96b";

/// Real Vulkan rendering paths; these do not select a different driver or API.
#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RendererMode {
    Vulkan,
    OpenGl,
    #[default]
    VulkanCompatibility,
}
impl RendererMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Vulkan => "Vulkan / GPU batching",
            Self::OpenGl => "OpenGL ES / experimental",
            Self::VulkanCompatibility => "Vulkan / compatibility",
        }
    }
}

pub const WELCOME_NOTICE_REVISION: u32 = 1;

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub welcome_ack_revision: u32,
    pub profile: u32,
    pub proxy: Option<String>,
    pub name: String,
    pub server: String,
    #[serde(skip)]
    pub microsoft_client_id: String,
    pub renderer: RendererMode,
    pub render_scale: crate::display::RenderScale,
    pub orientation: Orientation,
    #[serde(flatten)]
    pub screen: crate::display::ScreenPreferences,
    pub input_host: InputHost,
    pub keypad_controls: bool,
    pub record_performance: bool,
    pub worker_limit: u32,
    pub cache_budget_mib: u32,
    #[serde(flatten)]
    pub device: crate::device_preferences::Settings,
    pub shader_source: crate::shader_catalog::Origin,
    pub mod_source: crate::shader_catalog::Origin,
}

pub use horizon_client::android_gamepad::InputMode as InputHost;

/// Android ActivityInfo orientations, independent of the device's natural axis.
#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Orientation {
    #[default]
    Landscape,
    Portrait,
    ReverseLandscape,
    ReversePortrait,
    Auto,
}
impl Orientation {
    pub const ALL: [Self; 5] = [
        Self::Landscape,
        Self::Portrait,
        Self::ReverseLandscape,
        Self::ReversePortrait,
        Self::Auto,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Landscape => "Landscape (sensor)",
            Self::Portrait => "Portrait (sensor)",
            Self::ReverseLandscape => "Landscape (reversed)",
            Self::ReversePortrait => "Portrait (reversed)",
            Self::Auto => "Automatic rotation",
        }
    }
    pub fn android_value(self) -> i32 {
        match self {
            Self::Landscape => 6,
            Self::Portrait => 7,
            Self::ReverseLandscape => 8,
            Self::ReversePortrait => 9,
            Self::Auto => 10,
        }
    }
}
impl Config {
    pub fn load(home: &Path) -> Self {
        let value: serde_json::Value = std::fs::read(home.join("android-launcher.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default();
        let mut config: Self = serde_json::from_value(value.clone()).unwrap_or_default();
        if cfg!(target_pointer_width = "32") && value.get("renderer").is_none() {
            config.renderer = RendererMode::OpenGl;
        }
        // Ignore saved app registrations while this dev build uses the fixed ID.
        config.microsoft_client_id =
            crate::account::client_id().unwrap_or_else(|| MICROSOFT_CLIENT_ID.into());
        if value.get("renderer").is_none() && value["gpu_batching"].as_bool() == Some(true) {
            config.renderer = RendererMode::Vulkan;
        }
        let legacy = std::fs::read(config.game_home(home).join("horizon-touch.json"))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
            .unwrap_or_default();
        config.device.migrate_missing(&value, &legacy);
        config
    }
    pub fn save(&self, home: &Path) -> Result<(), String> {
        let part = home.join("android-launcher.partial");
        std::fs::write(
            &part,
            serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        std::fs::rename(part, home.join("android-launcher.json")).map_err(|e| e.to_string())
    }
    pub fn profile_home(&self, home: &Path) -> PathBuf {
        if self.profile == 0 {
            home.into()
        } else {
            home.join("profiles")
                .join(format!("profile-{}", self.profile))
        }
    }
    pub fn game_home(&self, home: &Path) -> PathBuf {
        // Legacy modpack selections are ignored; their files remain untouched.
        self.profile_home(home)
    }
}
/// Build channel injected by the Android build workflow. Stable builds omit
/// development-only navigation and notes.
pub const BUILD_CHANNEL: &str = env!("SYLPHRA_BUILD_CHANNEL", "stable");
pub const SHOW_DEVELOPMENT_NEWS: bool = cfg!(sylphra_development);

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Play,
    News,
    Mods,
    Packs,
    Shaders,
    Installed,
    Settings,
    SkinStudio,
}
impl Page {
    pub const ALL: &'static [Self] = &[
        Self::Play,
        #[cfg(sylphra_development)]
        Self::News,
        Self::Installed,
        Self::SkinStudio,
        Self::Mods,
        Self::Packs,
        Self::Shaders,
        Self::Settings,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Play => "Play",
            Self::News => "News",
            Self::Mods => "Mods",
            Self::Packs => "Packs",
            Self::Shaders => "Shaders",
            Self::Installed => "Installations",
            Self::Settings => "Settings",
            Self::SkinStudio => "Skins",
        }
    }
    pub fn is_catalog(self) -> bool {
        matches!(self, Self::Packs | Self::Mods | Self::Shaders)
    }
}
enum Reply {
    Done(String, horizon_client::content_files::Kind),
}
pub struct Catalog {
    pub local: Vec<LocalItem>,
    pub bundled_enabled: std::collections::BTreeSet<String>,
    pub sections: [bool; 3],
    pub pending_delete: Option<LocalItem>,
    downloaded: Vec<(browser::SourceId, String)>,
    downloading: Option<(browser::SourceId, String)>,
    progress: Arc<Mutex<f32>>,
    failed: Option<(browser::SourceId, String)>,
    pub page: Page,
    pub query: String,
    pub results: Vec<Project>,
    pub status: String,
    pub offset: usize,
    pub scroll: f32,
    pub config: Config,
    pub skin_library: skin_library::SkinLibrary,
    pub skin_library_error: Option<String>,
    pub cape_library: cape_library::CapeLibrary,
    pub cape_library_error: Option<String>,
    pub pending_cape_delete: Option<String>,
    pub cape_rename_id: Option<String>,
    pub cape_rename_text: String,
    skin_preview_cache: Option<(String, Vec<u8>)>,
    cape_preview_cache: Option<(String, Result<Vec<u8>, String>)>,
    pub editing: Option<&'static str>,
    pub render_scale_text: String,
    worker: Option<mpsc::Receiver<Result<Reply, String>>>,
    search_worker: Option<mpsc::Receiver<Result<Vec<Project>, String>>>,
    searches: crate::search_queue::Queue<(Page, browser::SourceId, String, usize), Vec<Project>>,
}
#[derive(Clone)]
pub struct LocalItem {
    pub kind: horizon_client::content_files::Kind,
    pub file: String,
    pub title: String,
    pub enabled: bool,
}
impl Catalog {
    pub fn new(home: &Path) -> Self {
        let config = Config::load(home);
        let (skin_library, skin_library_error) = match skin_library::SkinLibrary::load(home) {
            Ok(library) => (library, None),
            Err(error) => (skin_library::SkinLibrary::default(), Some(error)),
        };
        let (cape_library, cape_library_error) = match cape_library::CapeLibrary::load(home) {
            Ok(library) => (library, None),
            Err(error) => (cape_library::CapeLibrary::default(), Some(error)),
        };
        let initial_status = skin_library_error
            .as_ref()
            .map(|error| format!("Skin library unavailable: {error}"))
            .or_else(|| cape_library_error.as_ref().map(|error| format!("Cape library unavailable: {error}")))
            .unwrap_or_default();
        let mut result = Self {
            local: vec![],
            bundled_enabled: Default::default(),
            sections: [false, false, false],
            pending_delete: None,
            downloaded: vec![],
            downloading: None,
            progress: Arc::new(Mutex::new(0.0)),
            failed: None,
            page: Page::Play,
            query: String::new(),
            results: vec![],
            status: initial_status,
            offset: 0,
            scroll: 0.0,
            config,
            skin_library,
            skin_library_error,
            cape_library,
            cape_library_error,
            pending_cape_delete: None,
            cape_rename_id: None,
            cape_rename_text: String::new(),
            skin_preview_cache: None,
            cape_preview_cache: None,
            editing: None,
            render_scale_text: String::new(),
            worker: None,
            search_worker: None,
            searches: crate::search_queue::Queue::new(),
        };
        result.refresh(home);
        result
    }
    pub fn show_installed(&mut self, kind: Option<horizon_client::content_files::Kind>) {
        self.searches.cancel_interest();
        self.page = Page::Installed;
        self.scroll = 0.0;
        self.editing = None;
        self.sections = [false; 3];
        if let Some(i) = horizon_client::content_files::Kind::ALL
            .iter()
            .position(|k| Some(*k) == kind)
        {
            self.sections[i] = true;
        }
    }
    pub fn skin_preview_rgba(&mut self, home: &Path) -> Result<Option<&[u8]>, String> {
        if self.skin_library_error.is_some() {
            return Err("Skin library is read-only".into());
        }
        let Some(record) = self.skin_library.active_skin().cloned() else {
            self.skin_preview_cache = None;
            return Ok(None);
        };
        let cached = self
            .skin_preview_cache
            .as_ref()
            .is_some_and(|(id, _)| id == &record.id);
        if !cached {
            let path = home.join("skins").join(&record.file);
            let metadata = std::fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err("Selected skin is not a regular app-private PNG file".into());
            }
            if metadata.len() > skin_library::MAX_SKIN_BYTES as u64 {
                return Err("Selected skin exceeds the 1 MiB preview limit".into());
            }
            let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
            let (width, height, rgba) =
                horizon_client::hud::decode_bytes(&bytes).map_err(|e| e.to_string())?;
            let rgba = horizon_client::skins::process_legacy_skin(width, height, &rgba)
                .ok_or("Selected skin is not 64×64 or legacy 64×32")?;
            self.skin_preview_cache = Some((record.id, rgba));
        }
        Ok(self.skin_preview_cache.as_ref().map(|(_, rgba)| rgba.as_slice()))
    }
    pub fn reload_skin_library(&mut self, home: &Path) {
        match skin_library::SkinLibrary::load(home) {
            Ok(library) => {
                self.skin_library = library;
                self.skin_library_error = None;
            }
            Err(error) => {
                self.skin_library_error = Some(error.clone());
                self.status = format!("Skin library unavailable: {error}");
            }
        }
    }
    pub fn save_skin_library(&mut self, home: &Path) -> Result<(), String> {
        if let Some(error) = &self.skin_library_error {
            return Err(format!("Skin library is read-only until its metadata is repaired: {error}"));
        }
        self.skin_library.save(home)?;
        self.skin_library_error = None;
        Ok(())
    }
    pub fn reload_cape_library(&mut self, home: &Path) {
        self.cape_preview_cache = None;
        match cape_library::CapeLibrary::load(home) {
            Ok(library) => {
                self.cape_library = library;
                self.cape_library_error = None;
            }
            Err(error) => {
                self.cape_library_error = Some(error.clone());
                self.status = format!("Cape library unavailable: {error}");
            }
        }
    }
    pub fn save_cape_library(&mut self, home: &Path) -> Result<(), String> {
        if let Some(error) = &self.cape_library_error {
            return Err(format!("Cape library is read-only until its metadata is repaired: {error}"));
        }
        self.cape_library.save(home)?;
        self.cape_library_error = None;
        Ok(())
    }

    /// Reload from disk before every selection or destructive mutation. This
    /// catches metadata that became corrupt after Catalog was constructed and
    /// prevents stale in-memory state from replacing the on-disk manifest.
    fn prepare_cape_library_write(&mut self, home: &Path) -> Result<(), String> {
        self.reload_cape_library(home);
        if let Some(error) = &self.cape_library_error {
            return Err(format!(
                "Cape library is read-only until its metadata is repaired: {error}"
            ));
        }
        Ok(())
    }

    pub fn select_cape_and_save(
        &mut self,
        profile_id: &str,
        cape_id: Option<&str>,
        home: &Path,
    ) -> Result<(), String> {
        self.prepare_cape_library_write(home)?;
        self.cape_library
            .select_cape_and_save(profile_id, cape_id, home)
    }

    pub fn rename_cape(
        &mut self,
        cape_id: &str,
        name: &str,
        home: &Path,
    ) -> Result<String, String> {
        self.prepare_cape_library_write(home)?;
        self.cape_library.rename(cape_id, name, home)?;
        Ok(self
            .cape_library
            .capes
            .iter()
            .find(|cape| cape.id == cape_id)
            .map(|cape| cape.name.clone())
            .unwrap_or_else(|| "Imported cape".into()))
    }

    pub fn remove_cape(&mut self, cape_id: &str, home: &Path) -> Result<(), String> {
        self.prepare_cape_library_write(home)?;
        self.cape_library.remove(cape_id, home)
    }
    /// Decode the selected cape once and reuse its RGBA texture until the
    /// library is reloaded or the active profile selection changes.
    pub fn cape_preview_rgba(&mut self, home: &Path) -> Result<Option<&[u8]>, String> {
        if let Some(error) = &self.cape_library_error {
            return Err(format!("Cape library is unavailable: {error}"));
        }
        let Some(record) = self
            .cape_library
            .active_cape(&self.skin_library.active_profile)
            .cloned()
        else {
            self.cape_preview_cache = None;
            return Ok(None);
        };
        let cached = self
            .cape_preview_cache
            .as_ref()
            .is_some_and(|(id, _)| id == &record.id);
        if !cached {
            let decoded = (|| {
                let (_, bytes) = self.cape_library.read_validated_file(home, &record.id)?;
                let (width, height, rgba) =
                    horizon_client::hud::decode_bytes(&bytes).map_err(|error| error.to_string())?;
                horizon_client::skins::process_cape(width, height, &rgba)
                    .ok_or_else(|| "Selected cape is not 64×32 or 22×17".to_owned())
            })();
            self.cape_preview_cache = Some((record.id, decoded));
        }
        match self.cape_preview_cache.as_ref() {
            Some((_, Ok(rgba))) => Ok(Some(rgba.as_slice())),
            Some((_, Err(error))) => Err(error.clone()),
            None => Ok(None),
        }
    }

    /// Resolve the active profile's cape to a validated app-private file path.
    /// The game receives this path only for the current launch.
    pub fn active_cape_file(&self, home: &Path) -> Result<Option<PathBuf>, String> {
        if let Some(error) = &self.cape_library_error {
            return Err(format!("Cape library is unavailable: {error}"));
        }
        let Some(record) = self
            .cape_library
            .active_cape(&self.skin_library.active_profile)
        else {
            return Ok(None);
        };
        self.cape_library
            .read_validated_file(home, &record.id)
            .map(|(path, _)| Some(path))
    }
    pub fn refresh(&mut self, home: &Path) {
        self.reload_skin_library(home);
        self.reload_cape_library(home);
        let game = self.config.game_home(home);
        let installed = Installed::load(&game);
        self.downloaded = installed
            .entries
            .iter()
            .filter(|e| game.join(&e.file).is_file())
            .map(|e| (e.source, e.project.clone()))
            .collect();
        let on = horizon_client::mod_list::ModList::load(&game);
        self.bundled_enabled = on.enabled.difference(&on.disabled).cloned().collect();
        let settings = horizon_client::settings::Settings::load(&game.join("options.txt"));
        self.local.clear();
        for kind in horizon_client::content_files::Kind::ALL {
            let dir = game.join(kind.folder());
            if std::fs::symlink_metadata(&dir).is_ok_and(|m| m.file_type().is_symlink()) {
                continue;
            }
            for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
                if !entry.file_type().is_ok_and(|t| t.is_file()) {
                    continue;
                }
                let name = entry.file_name().to_string_lossy().into_owned();
                if !horizon_client::content_files::plain_name(&name) || !kind.accepts(&name) {
                    continue;
                }
                let enabled = match kind {
                    horizon_client::content_files::Kind::Packs => {
                        settings.resource_packs.contains(&format!("file/{name}"))
                    }
                    horizon_client::content_files::Kind::Shaders => {
                        settings.horizon.shader_pack == name
                    }
                    horizon_client::content_files::Kind::Mods => name
                        .strip_suffix(".hmod")
                        .is_some_and(|id| on.enabled.contains(id)),
                };
                self.local.push(LocalItem {
                    kind,
                    file: name.clone(),
                    title: name,
                    enabled,
                });
            }
        }
        self.local.sort_by_key(|e| e.title.to_lowercase());
    }
    pub fn busy(&self) -> bool {
        self.worker.is_some()
    }
    pub fn download_label(&self, project: &Project) -> Option<String> {
        if self
            .downloading
            .as_ref()
            .is_some_and(|(source, id)| *source == project.source && *id == project.id)
        {
            return Some(format!(
                "{:.0}%",
                *self.progress.lock().unwrap_or_else(|e| e.into_inner()) * 100.0
            ));
        }
        if self
            .downloaded
            .iter()
            .any(|(source, id)| *source == project.source && *id == project.id)
            || (project.kind == Kind::Mod
                && self.local.iter().any(|item| {
                    item.kind == horizon_client::content_files::Kind::Mods
                        && item.file == format!("{}.hmod", project.id)
                }))
        {
            return Some("Installed".into());
        }
        None
    }
    pub fn install_button(&self, project: &Project) -> &'static str {
        if self
            .failed
            .as_ref()
            .is_some_and(|(source, id)| *source == project.source && *id == project.id)
        {
            "Retry"
        } else {
            "Install"
        }
    }
    fn job(&mut self, status: &str, job: impl FnOnce() -> Result<Reply, String> + Send + 'static) {
        if self.busy() {
            return;
        }
        self.status = status.into();
        let (tx, rx) = mpsc::channel();
        self.worker = Some(rx);
        std::thread::spawn(move || {
            let _ = tx.send(job());
        });
    }
    pub fn poll(&mut self, home: &Path) {
        let reply = self.worker.as_ref().and_then(|rx| match rx.try_recv() {
            Ok(v) => Some(v),
            Err(mpsc::TryRecvError::Disconnected) => Some(Err("Launcher worker stopped".into())),
            Err(_) => None,
        });
        if let Some(reply) = reply {
            self.worker = None;
            let downloading = self.downloading.take();
            if reply.is_err() {
                self.failed = downloading;
            }
            match reply {
                Ok(Reply::Done(s, kind)) => {
                    self.status = s;
                    self.show_installed(Some(kind));
                }
                Err(e) => self.status = e,
            }
            self.refresh(home);
        }
        let result = self
            .search_worker
            .as_ref()
            .and_then(|rx| match rx.try_recv() {
                Ok(v) => Some(v),
                Err(mpsc::TryRecvError::Disconnected) => Some(Err("Search worker stopped".into())),
                Err(_) => None,
            });
        if let Some(result) = result {
            self.search_worker = None;
            let visible = self.searches.finish(result.as_ref().ok().cloned());
            if visible && self.page.is_catalog() && !self.busy() {
                match result {
                    Ok(items) => self.show_results(items),
                    Err(error) => self.status = error,
                }
            }
        }
        self.start_search();
    }
    pub fn search(&mut self, home: &Path) {
        self.request_search(home, true);
    }
    pub fn browse(&mut self, home: &Path) {
        self.searches.cancel_interest();
        if self.page.is_catalog() {
            self.request_search(home, false);
        }
    }
    fn request_search(&mut self, home: &Path, refresh: bool) {
        if !self.page.is_catalog() {
            return;
        }
        if self.page == Page::Mods && self.query.trim().starts_with("https://") {
            self.searches.cancel_interest();
            if refresh {
                self.import_native(home);
            } else {
                self.results.clear();
                self.status = "Press Import .hmod to download this HTTPS link".into();
            }
            return;
        }
        self.scroll = 0.0;
        let source = match self.page {
            Page::Packs => browser::SourceId::Modrinth,
            Page::Shaders => self.config.shader_source.source(),
            Page::Mods => self.config.mod_source.source(),
            _ => browser::SourceId::Horizon,
        };
        let key = (self.page, source, self.query.trim().to_owned(), self.offset);
        if let Some(items) = self.searches.request(key, refresh) {
            self.show_results(items);
        } else {
            self.results.clear();
            self.status = "Searching... You can switch tabs while results load.".into();
        }
        self.start_search();
    }
    fn show_results(&mut self, items: Vec<Project>) {
        self.status = if self.page == Page::Shaders
            && items.is_empty()
            && self.config.shader_source == crate::shader_catalog::Origin::Horizon
        {
            "No Horizon shader packs match this search. Try another query or import a .hshader."
                .into()
        } else if self.page == Page::Mods
            && self.config.mod_source == crate::shader_catalog::Origin::Modrinth
        {
            format!(
                "{} Java mods - browsing only; this Rust client cannot run Java loaders",
                items.len()
            )
        } else if self.page == Page::Mods {
            let native = items.iter().filter(|p| source_only_mod(p)).count();
            format!(
                "{} .hmod / {} source-only .hnmod (not supported yet)",
                items.len() - native,
                native
            )
        } else {
            format!(
                "{} results - select Install to check compatibility",
                items.len()
            )
        };
        self.results = items;
    }
    fn start_search(&mut self) {
        if self.busy() || self.search_worker.is_some() {
            return;
        }
        let Some((page, source, text, offset)) = self.searches.take_next() else {
            return;
        };
        let (tx, rx) = mpsc::channel();
        self.search_worker = Some(rx);
        std::thread::spawn(move || {
            let kind = match page {
                Page::Mods => Kind::Mod,
                Page::Shaders => Kind::Shader,
                _ => Kind::ResourcePack,
            };
            let mut query = Query::new(&text, kind);
            query.offset = offset;
            query.limit = 12;
            let result = if page == Page::Shaders && source == browser::SourceId::Modrinth {
                query.everything = true;
                browser::modrinth::search_shader_packs(&query)
            } else if page == Page::Packs
                || (page == Page::Mods && source == browser::SourceId::Modrinth)
            {
                query.everything = true;
                browser::modrinth::Modrinth.search(&query)
            } else {
                browser::horizon::Registry.search(&query)
            };
            let _ = tx.send(result);
        });
    }
    pub fn import_native(&mut self, home: &Path) {
        let url = self.query.trim().to_owned();
        let game = self.config.game_home(home);
        self.job("Downloading sandboxed .hmod over HTTPS...",move||{
            use std::io::Read;
            let mut bytes=Vec::new();
            ureq::AgentBuilder::new().https_only(true).timeout_connect(std::time::Duration::from_secs(20)).timeout_read(std::time::Duration::from_secs(90)).build().get(&url).call().map_err(|e|e.to_string())?.into_reader().take(32*1024*1024+1).read_to_end(&mut bytes).map_err(|e|e.to_string())?;
            if bytes.len()>32*1024*1024 {return Err("Horizon mod exceeds 32 MiB".into());}
            let dir=game.join("horizon-mods");std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
            let stage=dir.join("import.partial");std::fs::write(&stage,bytes).map_err(|e|e.to_string())?;
            let result=(||{
                let package=horizon_mods::Package::read(&stage)?;
                let destination=dir.join(format!("{}.hmod",package.manifest.id));
                if destination.exists(){std::fs::copy(&destination,destination.with_extension("hmod.previous")).map_err(|e|e.to_string())?;}
                std::fs::rename(&stage,destination).map_err(|e|e.to_string())?;
                Ok(Reply::Done(format!("Installed {}. Enable it in Installed after reviewing its capabilities in game Mods.",package.manifest.name), horizon_client::content_files::Kind::Mods))
            })();
            let _=std::fs::remove_file(stage);result
        });
    }
    pub fn install(&mut self, home: &Path, index: usize) {
        let Some(project) = self.results.get(index).cloned() else {
            return;
        };
        if source_only_mod(&project) {
            self.status = "This .hnmod needs a separately compiled client. Android support is planned; sandboxed .hmod mods can be installed now.".into();
            return;
        }
        if self.busy() || self.download_label(&project).is_some() {
            return;
        }
        let game = self.config.game_home(home);
        if project.kind != Kind::ResourcePack
            && !(project.kind == Kind::Shader
                && matches!(
                    project.source,
                    browser::SourceId::Horizon | browser::SourceId::Modrinth
                ))
            && !(cfg!(target_pointer_width = "64")
                && project.kind == Kind::Mod
                && project.source == browser::SourceId::Horizon)
        {
            return;
        }
        self.downloading = Some((project.source, project.id.clone()));
        self.failed = None;
        *self.progress.lock().unwrap_or_else(|e| e.into_inner()) = 0.0;
        let progress = self.progress.clone();
        let status =
            if project.kind == Kind::Shader && project.source == browser::SourceId::Modrinth {
                "Downloading experimental shader ZIP..."
            } else {
                "Checking compatibility and downloading..."
            };
        self.job(status, move || {
            std::fs::create_dir_all(&game).map_err(|e| e.to_string())?;
            let source = browser::source(project.source).ok_or("Unsupported download source")?;
            let count = if project.kind == Kind::Shader
                && project.source == browser::SourceId::Modrinth
            {
                crate::shader_catalog::install(source.as_ref(), &project, &game, &|_name, p| {
                    if p.is_finite() {
                        *progress.lock().unwrap_or_else(|e| e.into_inner()) = p.clamp(0.0, 1.0);
                    }
                })?
            } else {
                browser::installed::install(source.as_ref(), &project, true, &game, &|_name, p| {
                    if p.is_finite() {
                        *progress.lock().unwrap_or_else(|e| e.into_inner()) = p.clamp(0.0, 1.0);
                    }
                })?
                .len()
            };
            Ok(Reply::Done(
                format!("Installed {count} item(s). Open Play to load this profile."),
                match project.kind {
                    Kind::Mod => horizon_client::content_files::Kind::Mods,
                    Kind::Shader => horizon_client::content_files::Kind::Shaders,
                    _ => horizon_client::content_files::Kind::Packs,
                },
            ))
        });
    }
    pub fn toggle_local(&mut self, home: &Path, index: usize) {
        let Some(item) = self.local.get(index).cloned() else {
            return;
        };
        let game = self.config.game_home(home);
        let path = game.join("options.txt");
        let mut settings = horizon_client::settings::Settings::load(&path);
        match item.kind {
            horizon_client::content_files::Kind::Mods => {
                if !cfg!(target_pointer_width = "64") {
                    self.status = "Sandboxed code mods require a 64-bit APK".into();
                    return;
                }
                let mut on = horizon_client::mod_list::ModList::load(&game);
                if let Some(id) = item.file.strip_suffix(".hmod") {
                    if item.enabled {
                        on.enabled.remove(id);
                    } else {
                        on.enabled.insert(id.to_owned());
                    }
                }
                on.save(&game);
            }
            horizon_client::content_files::Kind::Packs => {
                let id = format!("file/{}", item.file);
                if item.enabled {
                    settings.resource_packs.retain(|p| p != &id);
                } else {
                    settings.resource_packs.push(id);
                }
                settings.save(&path);
            }
            horizon_client::content_files::Kind::Shaders => {
                settings.horizon.shader_pack = if item.enabled {
                    String::new()
                } else {
                    item.file
                };
                settings.save(&path);
            }
        }
        self.status = "Selection saved for the next game launch".into();
        self.refresh(home);
    }
    pub fn remove_local(&mut self, home: &Path) {
        let Some(item) = self.pending_delete.take() else {
            return;
        };
        let game = self.config.game_home(home);
        let result = (|| -> Result<(), String> {
            if !horizon_client::content_files::plain_name(&item.file) {
                return Err("Invalid content filename".into());
            }
            let dir = game.join(item.kind.folder());
            if std::fs::symlink_metadata(&dir)
                .map_err(|e| e.to_string())?
                .file_type()
                .is_symlink()
            {
                return Err("Refusing a redirected content directory".into());
            }
            let target = dir.join(&item.file);
            let meta = std::fs::symlink_metadata(&target).map_err(|e| e.to_string())?;
            if !meta.is_file() || meta.file_type().is_symlink() {
                return Err(
                    "Only an explicitly selected regular add-on file can be removed".into(),
                );
            }
            std::fs::remove_file(target).map_err(|e| e.to_string())?;
            let relative = format!("{}/{}", item.kind.folder(), item.file);
            let mut installed = Installed::load(&game);
            installed.entries.retain(|e| e.file != relative);
            installed.save(&game)?;
            let path = game.join("options.txt");
            let mut settings = horizon_client::settings::Settings::load(&path);
            if item.kind == horizon_client::content_files::Kind::Packs {
                settings
                    .resource_packs
                    .retain(|p| p != &format!("file/{}", item.file));
            }
            if item.kind == horizon_client::content_files::Kind::Shaders
                && settings.horizon.shader_pack == item.file
            {
                settings.horizon.shader_pack.clear();
            }
            settings.save(&path);
            if item.kind == horizon_client::content_files::Kind::Mods {
                let mut on = horizon_client::mod_list::ModList::load(&game);
                if let Some(id) = item.file.strip_suffix(".hmod") {
                    on.enabled.remove(id);
                }
                on.save(&game);
            }
            Ok(())
        })();
        self.status = match result {
            Ok(()) => format!("Removed {}", item.title),
            Err(e) => e,
        };
        self.refresh(home);
    }

pub fn edit(&mut self, out: horizon_client::osk::Output, home: &Path) {
        use horizon_client::osk::Output;
        if self.editing == Some("cape_name") {
            match out {
                Output::Text(value) => {
                    for character in value.chars() {
                        if !character.is_control()
                            && !matches!(character, '/' | '\\\\' | ':')
                            && self.cape_rename_text.chars().count() < 48
                        {
                            self.cape_rename_text.push(character);
                        }
                    }
                }
                Output::Backspace => {
                    self.cape_rename_text.pop();
                }
                Output::Enter | Output::Close => {
                    self.editing = None;
                    if let Some(id) = self.cape_rename_id.take() {
                        let requested_name = self.cape_rename_text.clone();
                        match self.rename_cape(&id, &requested_name, home) {
                            Ok(name) => self.status = format!("Cape renamed to: {name}"),
                            Err(error) => self.status = error,
                        }
                    }
                }
            }
            return;
        }
        let field = match self.editing {
            Some("name") => &mut self.config.name,
            Some("server") => &mut self.config.server,
            Some("render_scale") => &mut self.render_scale_text,
            Some("client_id") => &mut self.config.microsoft_client_id,
            _ => &mut self.query,
        };
        match out {
            Output::Text(s) => field.push_str(&s),
            Output::Backspace => {
                field.pop();
            }
            Output::Enter | Output::Close => {
                if self.editing == Some("render_scale") {
                    match crate::display::RenderScale::parse(&self.render_scale_text) {
                        Ok(scale) => {
                            self.config.render_scale = scale;
                            crate::display::set_scale(scale);
                        }
                        Err(error) => {
                            self.status = error.into();
                            return;
                        }
                    }
                }
                self.editing = None;
                let _ = self.config.save(home);
                if self.page.is_catalog() {
                    self.offset = 0;
                    self.search(home);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Catalog, Config};
    use std::{fs, time::{SystemTime, UNIX_EPOCH}};

    #[test]
    fn cape_mutations_refuse_corrupt_manifest_without_replacing_it() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let home = std::env::temp_dir().join(format!(
            "sylphra-catalog-cape-{}-{stamp}",
            std::process::id()
        ));
        fs::create_dir_all(&home).unwrap();
        let manifest = home.join("cape-library.json");
        let original = b"{broken";
        fs::write(&manifest, original).unwrap();

        let mut catalog = Catalog::new(&home);
        assert!(catalog
            .select_cape_and_save("default", None, &home)
            .unwrap_err()
            .contains("read-only"));
        let id = "0".repeat(64);
        assert!(catalog.rename_cape(&id, "Renamed", &home).is_err());
        assert!(catalog.remove_cape(&id, &home).is_err());
        assert_eq!(fs::read(&manifest).unwrap(), original);

        let _ = fs::remove_dir_all(home);
    }

    #[test]
    fn old_config_defaults_welcome_acknowledgement_to_zero() {
        let config: Config = serde_json::from_str(r#"{"name":"Existing player"}"#).unwrap();
        assert_eq!(config.welcome_ack_revision, 0);
        assert_eq!(config.name, "Existing player");
    }

    #[test]
    fn welcome_acknowledgement_round_trips() {
        let config = Config {
            welcome_ack_revision: super::WELCOME_NOTICE_REVISION,
            ..Default::default()
        };
        let encoded = serde_json::to_string(&config).unwrap();
        let decoded: Config = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.welcome_ack_revision, super::WELCOME_NOTICE_REVISION);
    }
}

