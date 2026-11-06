use super::*;
use horizon_client::{osk::Output, ui_theme::color as base_color};

/// Sylphra's launcher-only palette: Minecraft Java-inspired greens over charcoal.
/// The in-game HUD continues using Horizon's original global theme.
mod color {
    use super::base_color;
    pub const BG: u32 = base_color::hex(0x111512, 255);
    pub const RAISED: u32 = base_color::hex(0x202722, 255);
    pub const HOVER: u32 = base_color::hex(0x303b33, 255);
    pub const BORDER: u32 = base_color::hex(0x414b43, 255);
    pub const TEXT: u32 = base_color::hex(0xf1f5f1, 255);
    pub const MUTED: u32 = base_color::hex(0xaeb8af, 255);
    pub const ACCENT: u32 = base_color::hex(0x36a832, 255);
    pub const ON_ACCENT: u32 = base_color::hex(0xffffff, 255);
}
use winit::{
    event::{KeyEvent, Touch},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Action {
    Page(Page),
    SkinStudio,
    ImportSkin,
    ImportCape,
    SkinsTab,
    CapesTab,
    SelectSkin(usize),
    SelectCape(usize),
    RenameCape(usize),
    DeleteCape(usize),
    ConfirmDeleteCape,
    NoCape,
    DefaultSkin,
    ToggleSkinModel,
    ToggleSkinLayer(u8),
    NextAppearanceProfile,
    NewAppearanceProfile,
    About,
    BackAbout,
    SylphraRepository,
    ContinueWelcome,
    Play,
    Search,
    ShaderSource(crate::shader_catalog::Origin),
    ModSource(crate::shader_catalog::Origin),
    Field(&'static str),
    FinishNameEdit,
    Install(usize),
    Import(horizon_client::content_files::Kind),
    OpenFolder(horizon_client::content_files::Kind),
    ToggleSection(usize),
    ToggleLocal(usize),
    DeleteLocal(usize),
    ConfirmDeleteLocal,
    ConfirmExit,
    Next,
    Previous,
    Profile,
    Logs,
    RecordPerformance,
    ExportPerformance,
    ExportDiagnostics,
    Account,
    SignIn,
    DeviceSignIn,
    CopyLoginCode,
    CancelLogin,
    ReviewApp,
    Repository,
    SignOut,
    LoginLink,
    CloseLogin,
    Renderer,
    RenderScale,
    ChooseRenderScale(crate::display::RenderScale),
    Orientation,
    InputHost,
    ChooseInputHost(crate::catalog::InputHost),
    ChooseOrientation(crate::catalog::Orientation),
    ChooseRenderer(crate::catalog::RendererMode),
    ChooseProfile(u32),
    Dismiss,
    Immersive,
    Cutout,
    Aspect,
    ChooseAspect(crate::display::Aspect),
    DisplayHz,
    ChooseDisplayHz(u16),
    CacheBudget,
    ChooseCacheBudget(u32),
    ToggleBundled(&'static str),
    WorldFiles,
    GameFiles,
    WorkerLimit,
    ChooseWorkerLimit(u32),
    PerformanceHints,
    GpuProfile,
}

pub(super) fn button(
    ui: &mut Ui,
    targets: &mut Vec<([f32; 4], Action)>,
    rect: [f32; 4],
    label: &str,
    action: Action,
    accent: bool,
) {
    let [x, y, w, h] = rect;
    ui.rounded(
        x,
        y,
        w,
        h,
        2.0,
        if accent { color::ACCENT } else { color::RAISED },
    );
    let padding = (w * 0.12).clamp(4.0, 12.0);
    let base = if w < 80.0 { 13.0 } else { 16.0 };
    let base = if h < 36.0 { 13.0 } else { base };
    let font = if matches!(action, Action::Page(_)) {
        (base * ((w - 2.0 * padding) / ui.measure(Face::Medium, base, label).max(1.0)).min(1.0))
            .max(9.0)
    } else {
        base
    };
    let label = ui.fit_text(Face::Medium, font, label, w - 2.0 * padding);
    let text_height = (font * 1.45).round();
    let text_y = y + ((h - text_height) / 2.0).max(0.0);
    ui.text(
        Face::Medium,
        font,
        x + padding,
        text_y,
        if accent {
            color::ON_ACCENT
        } else {
            color::TEXT
        },
        &label,
    );
    targets.push((rect, action));
}
/// Navigation treatment selected from the post-inset launcher viewport.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NavigationMode {
    Compact,
    Expanded,
    Sidebar,
}

/// Compute a stacked phone cape preview while reserving two complete list rows.
fn stacked_cape_preview_height(list_bottom: f32, preview_top: f32, minimum_list: f32) -> f32 {
    (list_bottom - preview_top - 8.0 - minimum_list).clamp(120.0, 260.0)
}

/// Keep a short-phone split preview near-square without exceeding the content.
fn compact_cape_preview_height(studio_bottom: f32, preview_top: f32, preview_width: f32) -> f32 {
    (studio_bottom - preview_top)
        .max(0.0)
        .min((preview_width * 1.15).max(72.0))
}

/// Shared geometry for the launcher shell and all page content.
#[derive(Clone, Copy)]
struct LauncherLayout {
    navigation_mode: NavigationMode,
    title_origin: [f32; 2],
    account_rect: [f32; 4],
    nav_rect: [f32; 4],
    content_top: f32,
    content_rect: [f32; 4],
}
impl LauncherLayout {
    fn new(width: f32, height: f32) -> Self {
        let navigation_mode = if width >= 900.0 && height >= 480.0 {
            NavigationMode::Sidebar
        } else if width >= 380.0 && height >= 500.0 {
            NavigationMode::Expanded
        } else {
            NavigationMode::Compact
        };
        let (title_origin, account_rect, nav_rect, content_top, content_rect) =
            match navigation_mode {
                NavigationMode::Compact => (
                    [32.0, 12.0],
                    [32.0, 47.0, (width - 64.0).max(0.0), 34.0],
                    [16.0, 82.0, (width - 32.0).max(0.0), 54.0],
                    144.0,
                    [20.0, 144.0, (width - 40.0).max(0.0), (height - 244.0).max(0.0)],
                ),
                NavigationMode::Expanded => (
                    [24.0, 14.0],
                    [(width - 152.0).max(24.0), 12.0, 136.0, 36.0],
                    [16.0, 58.0, (width - 32.0).max(0.0), 78.0],
                    146.0,
                    [20.0, 146.0, (width - 40.0).max(0.0), (height - 246.0).max(0.0)],
                ),
                NavigationMode::Sidebar => {
                    let rail = (width * 0.23).clamp(248.0, 360.0);
                    (
                        [rail + 24.0, 24.0],
                        [20.0, 20.0, rail - 40.0, 72.0],
                        [0.0, 112.0, rail, (height - 112.0).max(0.0)],
                        0.0,
                        [rail, 0.0, (width - rail).max(0.0), height],
                    )
                },
            };
        Self {
            navigation_mode,
            title_origin,
            account_rect,
            nav_rect,
            content_top,
            content_rect,
        }
    }

    fn tab_rect(&self, index: usize) -> [f32; 4] {
        match self.navigation_mode {
            NavigationMode::Compact => {
                let cell = self.nav_rect[2] / Page::ALL.len() as f32;
                [
                    self.nav_rect[0] + index as f32 * cell,
                    self.nav_rect[1] + 6.0,
                    (cell - 4.0).max(0.0),
                    42.0,
                ]
            }
            NavigationMode::Expanded => {
                let columns = 4;
                let cell = (self.nav_rect[2] - 16.0).max(0.0) / columns as f32;
                let column = index % columns;
                let row = index / columns;
                [
                    self.nav_rect[0] + 8.0 + column as f32 * cell,
                    self.nav_rect[1] + 6.0 + row as f32 * 38.0,
                    (cell - 4.0).max(0.0),
                    32.0,
                ]
            }
            NavigationMode::Sidebar => {
                let count = Page::ALL.len() as f32;
                let button_height = 50.0;
                let available = (self.nav_rect[3] - 28.0 - count * button_height).max(0.0);
                let gap = (available / (count - 1.0)).clamp(5.0, 22.0);
                [
                    self.nav_rect[0] + 12.0,
                    self.nav_rect[1] + 10.0 + index as f32 * (button_height + gap),
                    self.nav_rect[2] - 24.0,
                    button_height,
                ]
            }
        }
    }
}

#[derive(Clone, Copy)]
struct CatalogPageLayout {
    title_y: f32,
    source_y: f32,
    search_y: f32,
    folder_y: f32,
    hint_y: f32,
    list_top: f32,
}

impl CatalogPageLayout {
    fn for_launcher(layout: &LauncherLayout) -> Self {
        if layout.navigation_mode == NavigationMode::Sidebar {
            // Sidebar pages start at the top of the right-hand canvas. Keep
            // controls in a compact header and reserve a separate results area.
            Self {
                title_y: 24.0,
                source_y: 18.0,
                search_y: 68.0,
                folder_y: 116.0,
                hint_y: 164.0,
                list_top: 210.0,
            }
        } else {
            // Preserve the existing compact/expanded page geometry.
            Self {
                title_y: 142.0,
                source_y: 134.0,
                search_y: 174.0,
                folder_y: 222.0,
                hint_y: 270.0,
                list_top: layout.content_rect[1] + 152.0,
            }
        }
    }
}

fn toggle(
    ui: &mut Ui,
    targets: &mut Vec<([f32; 4], Action)>,
    rect: [f32; 4],
    on: bool,
    action: Action,
) {
    let [x, y, w, h] = rect;
    ui.text(
        Face::Regular,
        14.0,
        x,
        y + 11.0,
        color::TEXT,
        if on { "On" } else { "Off" },
    );
    ui.rounded(
        x + w - 52.0,
        y + 8.0,
        48.0,
        24.0,
        12.0,
        if on { color::TEXT } else { color::HOVER },
    );
    ui.rounded(
        x + w - if on { 26.0 } else { 50.0 },
        y + 10.0,
        20.0,
        20.0,
        10.0,
        if on { color::BG } else { color::TEXT },
    );
    targets.push(([x, y, w, h], action));
}
impl Launcher {
    /// The same spatial focus algorithm as the game's controller menus.
    pub(super) fn navigate(&mut self, key: &KeyEvent) -> bool {
        self.navigate_key(&key.logical_key)
    }
    pub(super) fn navigate_key(&mut self, key: &winit::keyboard::Key) -> bool {
        use horizon_client::gamepad::{self, Button};
        use winit::keyboard::{Key, NamedKey};
        if self.pending_focus.is_some() {
            return true;
        }
        if self.catalog.editing.is_some() || self.targets.is_empty() {
            return false;
        }
        let button = match key {
            Key::Named(NamedKey::ArrowUp) => Button::DPadUp,
            Key::Named(NamedKey::ArrowDown) => Button::DPadDown,
            Key::Named(NamedKey::ArrowLeft) => Button::DPadLeft,
            Key::Named(NamedKey::ArrowRight) => Button::DPadRight,
            Key::Named(NamedKey::Tab) => {
                self.focus = Some(self.focus.map_or(0, |i| (i + 1) % self.targets.len()));
                return true;
            }
            _ => return false,
        };
        if matches!(button, Button::DPadUp | Button::DPadDown)
            && let Some((_, action)) = self.focus.and_then(|i| self.targets.get(i))
            && let Some(index) = self.list_targets.iter().position(|(_, a)| a == action)
        {
            let rects: Vec<_> = self.list_targets.iter().map(|(r, _)| *r).collect();
            if let Some(next) = gamepad::focus_step_strict(rects[index], button, &rects)
                && let Some(view) = self.list_view
            {
                let (rect, action) = self.list_targets[next];
                let scroll = if let Some(chooser) = &mut self.chooser {
                    &mut chooser.scroll
                } else {
                    &mut self.catalog.scroll
                };
                *scroll = super::gesture::reveal(*scroll, rect, view);
                self.pending_focus = Some(action);
                self.focus = None;
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
                return true;
            }
        }
        self.focus = Some(
            if let Some(index) = self.focus.filter(|i| *i < self.targets.len()) {
                let rects: Vec<_> = self.targets.iter().map(|(rect, _)| *rect).collect();
                gamepad::focus_step_rect(rects[index], button, &rects).unwrap_or(index)
            } else {
                0
            },
        );
        true
    }

    pub(super) fn draw_pages(&mut self) -> Result<()> {
        self.skin_preview_rect = None;
        let (Some(r), Some(gui)) = (&mut self.renderer, &self.gui) else {
            return Ok(());
        };
        if r.suspended {
            return Ok(());
        }
        // Load the player preview from the page lifecycle, not only from the
        // Settings shortcut. The top navigation uses Action::Page(SkinStudio),
        // and document-import callbacks can also navigate here directly.
        if self.catalog.page == Page::SkinStudio
            && self.skin_preview.is_none()
            && self.skin_preview_error.is_none()
        {
            let root = self
                .home
                .join(".local/horizon-client")
                .join(horizon_core::VANILLA_VERSION);
            match horizon_client::mobs::preview::PlayerPreview::load(&root) {
                Ok(preview) => {
                    self.skin_preview = Some(preview);
                    self.catalog.status = "3D player model loaded.".into();
                }
                Err(error) => {
                    let diagnostic = format!(
                        "3D preview could not load from {}: {error}",
                        root.display()
                    );
                    log::warn!("{diagnostic}");
                    self.skin_preview_error = Some(diagnostic.clone());
                    self.catalog.status = diagnostic;
                }
            }
        }
        self.targets.clear();
        self.list_targets.clear();
        self.list_view = None;
        let mut b = HudBatch::new([r.config.width, r.config.height]);
        let mut ui = Ui::mobile(&gui.atlas, &mut b, (0.0, 0.0));
        // OEM navigation bars can remain visible even in fullscreen (keypad phones).
        // Inset the launcher layout only; keep surface and input coordinates intact.
        if let Some(window) = &self.window {
            use winit::platform::android::WindowExtAndroid;
            let rect = window.content_rect();
            let input = winit::platform::android::native_input_scale();
            if rect.bottom > 0 {
                ui.h = ui.h.min(rect.bottom as f32 * input[1] as f32 / ui.density);
            }
            if rect.right > 0 {
                ui.w = ui.w.min(rect.right as f32 * input[0] as f32 / ui.density);
            }
        }
        if let Some(bottom) = crate::native_ime::viewport_bottom() {
            ui.h = ui.h.min(bottom as f32 / ui.density);
        }
        let (w, h) = (ui.w, ui.h);
        let scale = ui.density / ui.batch.scale;
        ui.batch
            .fill(&gui.atlas, 0.0, 0.0, w * scale, h * scale, color::BG);
        let layout = LauncherLayout::new(w, h);
        if layout.navigation_mode != NavigationMode::Sidebar {
            let title_size = if w < 400.0 { 20.0 } else if w < 700.0 { 24.0 } else { 32.0 };
            let title_width = match layout.navigation_mode {
                NavigationMode::Compact => (w - 64.0).max(40.0),
                NavigationMode::Expanded => (layout.account_rect[0] - layout.title_origin[0] - 12.0).max(40.0),
                NavigationMode::Sidebar => 0.0,
            };
            let title = ui.fit_text(Face::Medium, title_size,
                &format!("Sylphra Launcher {}", crate::backend::VERSION), title_width);
            ui.text(Face::Medium, title_size, layout.title_origin[0], layout.title_origin[1], color::TEXT, &title);
        }

        if layout.navigation_mode == NavigationMode::Sidebar {
            // Dedicated dark rail: account at the top, navigation below.
            ui.batch.fill(&gui.atlas, 0.0, 0.0, layout.nav_rect[2] * scale,
                h * scale, base_color::hex(0x111315, 255));
            ui.border(0.0, 0.0, layout.nav_rect[2], h, 1.0, color::BORDER);
        }
        button(&mut ui, &mut self.targets, layout.account_rect,
            &self.login.status(&self.catalog.config.name), Action::Account,
            self.login.signed_in);
        ui.rounded(layout.nav_rect[0], layout.nav_rect[1], layout.nav_rect[2],
            layout.nav_rect[3], 0.0, if layout.navigation_mode == NavigationMode::Sidebar {
                base_color::hex(0x111315, 255)
            } else { color::RAISED });
        if layout.navigation_mode != NavigationMode::Sidebar {
            ui.border(layout.nav_rect[0], layout.nav_rect[1], layout.nav_rect[2],
                layout.nav_rect[3], 4.0, color::HOVER);
        }
        for (i, page) in Page::ALL.iter().enumerate() {
            let label = if layout.navigation_mode == NavigationMode::Sidebar {
                match page {
                    Page::Play => "Play",
                    Page::Installed => "Installations",
                    Page::SkinStudio => "Skins",
                    Page::News => "Patch Notes",
                    Page::Mods => "Mods",
                    Page::Packs => "Resource Packs",
                    Page::Shaders => "Shaders",
                    Page::Settings => "Profile",
                }
            } else if w < 700.0 {
                match page {
                    Page::Installed => "Files",
                    Page::SkinStudio => "Skins",
                    Page::Shaders => "FX",
                    Page::Settings => "Setup",
                    _ => page.name(),
                }
            } else {
                page.name()
            };
            let rect = layout.tab_rect(i);
            if layout.navigation_mode == NavigationMode::Sidebar {
                let [x, y, width, height] = rect;
                let selected = *page == self.catalog.page;
                ui.rounded(x, y, width, height, 3.0, if selected {
                    base_color::hex(0x1d3b1b, 255)
                } else {
                    base_color::hex(0x171a18, 255)
                });
                if selected {
                    ui.border(x, y, width, height, 3.0,
                        base_color::hex(0x59ed54, 255));
                }
                let icon = match page {
                    Page::Play => "item/grass_block",
                    Page::Installed => "item/crafting_table",
                    Page::SkinStudio => "item/player_head",
                    Page::News => "item/writable_book",
                    Page::Mods => "item/iron_pickaxe",
                    Page::Packs => "item/chest",
                    Page::Shaders => "item/glass_bottle",
                    Page::Settings => "item/player_head",
                };
                let icon_size = 30.0;
                ui.batch.sprite_scaled(ui.atlas, icon,
                    (x + 18.0) * scale,
                    (y + (height - icon_size) * 0.5) * scale,
                    icon_size * scale, icon_size * scale, u32::MAX);
                let font = if width < 220.0 { 14.0 } else { 17.0 };
                let fitted = ui.fit_text(Face::Medium, font, label,
                    (width - 72.0).max(40.0));
                ui.text(Face::Medium, font, x + 62.0,
                    y + (height - font * 1.45) * 0.5,
                    color::TEXT, &fitted);
                self.targets.push((rect, Action::Page(*page)));
            } else {
                button(&mut ui, &mut self.targets, rect, label, Action::Page(*page),
                    *page == self.catalog.page);
            }
        }
        let compact = w < 700.0;
        let page = self.catalog.page;
        let skin_studio_compact = page == Page::SkinStudio && h < 560.0;
        if page == Page::Play && layout.navigation_mode == NavigationMode::Sidebar {
            use horizon_client::official_install::InstallStage;
            let installed = horizon_client::official_install::ready(&self.home);
            let installing = self.worker.is_some();
            let progress = *self.install.lock().unwrap();
            let left = layout.content_rect[0];
            let right = left + layout.content_rect[2];
            let bottom = h;

            // Full-bleed canvas treatment: no opaque content card over the hero.
            ui.batch.fill(&gui.atlas, left * scale, 0.0,
                (right - left) * scale, h * scale, base_color::hex(0x241b1b, 255));
            // Dark edge shading keeps controls legible until a user background is configured.
            ui.rounded(left, 0.0, right-left, 76.0, 0.0, base_color::hex(0x151416, 210));
            ui.rounded(left, h-112.0, right-left, 112.0, 0.0, base_color::hex(0x111112, 220));

            let top_y = 22.0;
            button(&mut ui, &mut self.targets, [right-278.0, top_y, 112.0, 42.0],
                "FILES", Action::Page(Page::Installed), false);
            button(&mut ui, &mut self.targets, [right-154.0, top_y, 136.0, 42.0],
                "MULTIPLAYER", Action::Field("server"), true);

            if installing {
                let stage_index = InstallStage::ALL.iter().position(|s| *s == progress.stage).unwrap_or(0);
                let fraction = if progress.total > 0 { progress.done as f32 / progress.total as f32 } else { 0.0 };
                let overall = (stage_index as f32 + fraction.clamp(0.0, 1.0)) / InstallStage::ALL.len() as f32;
                let width = (right-left-80.0).clamp(120.0, 520.0);
                let x = left + (right-left-width)*0.5;
                let y = h*0.43;
                ui.text(Face::Medium, 25.0, x, y, color::TEXT, "Preparing Sylphra");
                ui.text(Face::Regular, 16.0, x, y+38.0, color::MUTED, progress.stage.label());
                ui.rounded(x, y+76.0, width, 6.0, 3.0, color::RAISED);
                ui.rounded(x, y+76.0, width*overall, 6.0, 3.0, color::ACCENT);
            } else {
                let logo = ui.fit_text(Face::Medium, if right-left < 600.0 { 32.0 } else { 46.0 },
                    "SYLPHRA", (right-left-64.0).max(80.0));
                let logo_width = ui.measure(Face::Medium, if right-left < 600.0 { 32.0 } else { 46.0 }, &logo);
                ui.text(Face::Medium, if right-left < 600.0 { 32.0 } else { 46.0 },
                    left+(right-left-logo_width)*0.5, h*0.25, color::TEXT, &logo);
                ui.text(Face::Regular, 16.0, left+24.0, h*0.25+54.0,
                    color::MUTED, "MINECRAFT · ANDROID EDITION");

                // Compact version selector; Horizon currently ships one engine version.
                ui.rounded(left+24.0, bottom-82.0, 250.0, 58.0, 7.0, color::RAISED);
                ui.border(left+24.0, bottom-82.0, 250.0, 58.0, 1.0, color::BORDER);
                ui.text(Face::Regular, 13.0, left+42.0, bottom-70.0, color::MUTED, "CURRENT VERSION");
                ui.text(Face::Medium, 17.0, left+42.0, bottom-43.0,
                    color::TEXT, &format!("Horizon {}", horizon_core::VANILLA_VERSION));

                let play_w = (right-left-48.0).clamp(180.0, 330.0);
                let play_x = right-play_w-24.0;
                let play_y = bottom-82.0;
                let play_label = if installed { "PLAY" } else if self.error.is_some() {
                    "RETRY INSTALLATION"
                } else { "INSTALL & PLAY" };
                button(&mut ui, &mut self.targets, [play_x, play_y, play_w, 58.0],
                    play_label, Action::Play, true);
                ui.border(play_x, play_y, play_w, 58.0, 2.0, base_color::hex(0x78ef4b, 255));
            }
        } else if page == Page::Play {
            use horizon_client::official_install::InstallStage;
            let installed = horizon_client::official_install::ready(&self.home);
            let installing = self.worker.is_some();
            let progress = *self.install.lock().unwrap();
            // Sylphra home: Java-launcher-inspired hero composition, with a
            // distinct layout and no changes to gameplay or shared Horizon HUD.
            let content_x = layout.content_rect[0] + 12.0;
            let content_right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
            let content_width = (layout.content_rect[2] - 24.0).max(0.0);
            let panel = [layout.content_rect[0], layout.content_top, layout.content_rect[2].max(0.0), (h - layout.content_top - 24.0).max(0.0)];
            ui.rounded(
                panel[0],
                panel[1],
                panel[2],
                panel[3],
                4.0,
                color::RAISED,
            );
            ui.border(
                panel[0],
                panel[1],
                panel[2],
                panel[3],
                1.0,
                color::HOVER,
            );
            let top = layout.content_top + 22.0;
            if layout.navigation_mode == NavigationMode::Sidebar {
                button(
                    &mut ui,
                    &mut self.targets,
                    [content_right - 124.0, top, 112.0, 38.0],
                    "FILES",
                    Action::Page(Page::Installed),
                    false,
                );
            }
            ui.rounded(layout.content_rect[0], top, 5.0, 48.0, 2.0, color::ACCENT);
            // Keep the hero readable on narrow phones and in landscape-sized windows.
            let title_size = if compact { 25.0 } else { 32.0 };
            let subtitle_size = if compact { 14.0 } else { 17.0 };
            ui.text(
                Face::Medium,
                title_size,
                content_x,
                top,
                color::TEXT,
                if installing {
                    if compact { "Preparing Sylphra" } else { "Preparing Sylphra for your game" }
                } else if installed {
                    "Ready to play"
                } else if compact {
                    "Your Java worlds"
                } else {
                    "Your Java worlds, on Android"
                },
            );
            ui.text(
                Face::Regular,
                subtitle_size,
                content_x,
                top + if compact { 44.0 } else { 56.0 },
                color::MUTED,
                &format!("Minecraft 26.3  /  Profile {}", self.catalog.config.profile),
            );
            if installing {
                let current = InstallStage::ALL
                    .iter()
                    .position(|s| *s == progress.stage)
                    .unwrap_or(0);
                let row_height = ((h - 360.0) / 6.0).clamp(25.0, 40.0);
                if h >= 620.0 {
                    for (index, stage) in InstallStage::ALL.iter().enumerate() {
                        let y = top + 90.0 + index as f32 * row_height;
                        let label = if index < current {
                            format!("{:02}  {}  - done", index + 1, stage.label())
                        } else {
                            format!("{:02}  {}", index + 1, stage.label())
                        };
                        ui.text(
                            Face::Regular,
                            16.0,
                            content_x,
                            y,
                            if index == current {
                                color::TEXT
                            } else {
                                color::MUTED
                            },
                            &label,
                        );
                    }
                } else {
                    ui.text(
                        Face::Regular,
                        18.0,
                        content_x,
                        top + 100.0,
                        color::TEXT,
                        progress.stage.label(),
                    );
                    ui.text(
                        Face::Regular,
                        15.0,
                        content_x,
                        top + 138.0,
                        color::MUTED,
                        &format!("Step {} of 6", current + 1),
                    );
                }
                let fraction = if progress.total > 0 {
                    progress.done as f32 / progress.total as f32
                } else {
                    0.0
                };
                let overall = (current as f32 + fraction.clamp(0.0, 1.0)) / 6.0;
                let bar_y = h - 152.0;
                ui.rounded(content_x, bar_y, content_width, 4.0, 2.0, color::RAISED);
                if overall > 0.0 {
                    ui.rounded(content_x, bar_y, content_width * overall, 4.0, 2.0, color::ACCENT);
                }
                let detail = match progress.stage {
                    InstallStage::Download => format!(
                        "{:.1} / {:.1} MiB  - verified downloads",
                        progress.done as f64 / 1048576.0,
                        progress.total as f64 / 1048576.0
                    ),
                    InstallStage::Controls => format!(
                        "{} / {} control files verified",
                        progress.done, progress.total
                    ),
                    _ => "Please keep Sylphra open. Downloaded files are kept for retry.".into(),
                };
                ui.text(
                    Face::Regular,
                    14.0,
                    content_x,
                    bar_y + 12.0,
                    color::MUTED,
                    &detail,
                );
            } else {
                let rows = if installed {
                    if compact {
                        ["01  Open a world or Java server", "02  Set up touch controls", "03  Browse packs and mods"]
                    } else {
                        [
                            "01  Open a world or join a Java server",
                            "02  Touch setup: Options > Controls",
                            "03  Find packs and .hmod in the tabs",
                        ]
                    }
                } else if compact {
                    ["01  Download game resources", "02  Verify files on this phone", "03  Play offline or sign in online"]
                } else {
                    [
                        "01  Download resources from Mojang",
                        "02  Install and verify on this phone",
                        "03  Play offline or sign in for servers",
                    ]
                };
                let row_size = if compact { 14.0 } else { 17.0 };
                let row_gap = if compact { 32.0 } else { 42.0 };
                let row_limit = if h < 440.0 { 2 } else { rows.len() };
                for (index, line) in rows.iter().take(row_limit).enumerate() {
                    ui.text(
                        Face::Regular,
                        row_size,
                        content_x,
                        top + if compact { 88.0 } else { 100.0 } + index as f32 * row_gap,
                        color::MUTED,
                        line,
                    );
                }
            }
            if installing {
                ui.text(
                    Face::Medium,
                    19.0,
                    content_x,
                    layout.content_rect[1] + layout.content_rect[3] + 10.0,
                    color::TEXT,
                    progress.stage.label(),
                );
            } else {
                let play_width = content_width.clamp(0.0, 320.0);
                let play_x = if layout.navigation_mode == NavigationMode::Sidebar {
                    content_right - play_width
                } else {
                    content_x
                };
                button(
                    &mut ui,
                    &mut self.targets,
                    [play_x, layout.content_rect[1] + layout.content_rect[3], play_width, 64.0],
                    if installed {
                        "Play"
                    } else if self.error.is_some() {
                        "Retry installation"
                    } else {
                        "Download Minecraft 26.3"
                    },
                    Action::Play,
                    true,
                );
            }
        } else if page == Page::News && crate::catalog::SHOW_DEVELOPMENT_NEWS {
            let left = layout.content_rect[0] + 12.0;
            let right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
            let top = layout.content_top + 58.0;
            let bottom = (h - 20.0).max(top + 1.0);
            let lines: Vec<&str> = include_str!("../../../../docs/SYLPHRA_DEVELOPMENT_NEWS.md")
                .lines()
                .collect();
            let line_height = 32.0;
            let maximum = (lines.len() as f32 * line_height - (bottom - top)).max(0.0);
            self.catalog.scroll = self.catalog.scroll.clamp(0.0, maximum);
            self.list_view = Some([left, top, right, bottom, maximum]);
            ui.text(Face::Medium, 25.0, left, layout.content_top + 12.0, color::TEXT, "Sylphra development");
            ui.text(Face::Regular, 14.0, left, layout.content_top + 40.0, color::MUTED,
                &format!("{} build  •  Experimental notes for testers and contributors", crate::catalog::BUILD_CHANNEL));
            for (index, raw) in lines.iter().enumerate() {
                if raw.trim().is_empty() { continue; }
                let y = top + index as f32 * line_height - self.catalog.scroll;
                if y + line_height < top || y > bottom { continue; }
                let heading = raw.starts_with('#');
                let label = raw.trim_start_matches('#').trim_start_matches("- ").trim();
                let label = ui.fit_text(Face::Regular, if heading { 17.0 } else { 14.0 }, label, (right - left).max(0.0));
                ui.text(
                    if heading { Face::Medium } else { Face::Regular },
                    if heading { 17.0 } else { 14.0 },
                    left + if heading { 0.0 } else { 12.0 },
                    y,
                    if heading { color::ACCENT } else { color::MUTED },
                    &label,
                );
            }
        } else if page.is_catalog() {
            let catalog_layout = CatalogPageLayout::for_launcher(&layout);
            let content_left = layout.content_rect[0] + 12.0;
            let content_right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
            let action_x = content_right - 118.0;
            let search_x = content_right - 158.0;
            ui.text(
                Face::Medium,
                18.0,
                content_left,
                catalog_layout.title_y,
                color::TEXT,
                if page == Page::Mods {
                    "Sylphra Mods"
                } else if page == Page::Shaders {
                    if w < 360.0 { "Shaders" } else { "Shader Packs" }
                } else {
                    "Resource Packs"
                },
            );
            button(
                &mut ui,
                &mut self.targets,
                [content_left, catalog_layout.search_y, search_x - content_left - 18.0, 44.0],
                &format!(
                    "{}{}",
                    if page == Page::Mods {
                        "Search / .hmod URL: "
                    } else {
                        "Search: "
                    },
                    self.catalog.query
                ),
                Action::Field("search"),
                false,
            );
            button(
                &mut ui,
                &mut self.targets,
                [search_x, catalog_layout.search_y, 158.0, 44.0],
                if page == Page::Mods && self.catalog.query.trim().starts_with("https://") {
                    "Import .hmod"
                } else {
                    "Search"
                },
                Action::Search,
                true,
            );
            if matches!(page, Page::Shaders | Page::Mods) {
                use crate::shader_catalog::Origin;
                let width = ((content_right - content_left - 121.0) / 2.0).clamp(58.0, 120.0);
                for (i, origin) in [Origin::Horizon, Origin::Modrinth].into_iter().enumerate() {
                    let rect = [content_right - (2 - i) as f32 * width, catalog_layout.source_y, width - 5.0, 32.0];
                    button(
                        &mut ui,
                        &mut self.targets,
                        rect,
                        origin.source().label(),
                        if page == Page::Mods {
                            Action::ModSource(origin)
                        } else {
                            Action::ShaderSource(origin)
                        },
                        false,
                    );
                    if (if page == Page::Mods {
                        self.catalog.config.mod_source
                    } else {
                        self.catalog.config.shader_source
                    }) == origin
                    {
                        ui.border(rect[0], rect[1], rect[2], rect[3], 1.0, color::TEXT);
                    }
                }
            }
            let hint = match page {
                Page::Mods
                    if self.catalog.config.mod_source
                        == crate::shader_catalog::Origin::Modrinth =>
                {
                    "Modrinth Java mods are browse-only; Fabric/Forge/NeoForge cannot run here"
                }
                Page::Mods => "Browse Sylphra mods or paste an HTTPS .hmod link",
                Page::Shaders
                    if self.catalog.config.shader_source
                        == crate::shader_catalog::Origin::Modrinth =>
                {
                    "Modrinth shader ZIPs / experimental compatibility; some packs may not work"
                }
                Page::Shaders => "Horizon native .hshader packs",
                _ => "Verified downloads / enable in Installed",
            };
            let hint = ui.fit_text(Face::Regular, 14.0, hint, content_right - content_left);
            ui.text(Face::Regular, 14.0, content_left, catalog_layout.hint_y, color::MUTED, &hint);
            let kind = match page {
                Page::Mods => horizon_client::content_files::Kind::Mods,
                Page::Shaders => horizon_client::content_files::Kind::Shaders,
                _ => horizon_client::content_files::Kind::Packs,
            };
            let file_button_width = ((content_right - content_left - 8.0) / 2.0).min(158.0);
            button(
                &mut ui,
                &mut self.targets,
                [content_left, catalog_layout.folder_y, file_button_width, 38.0],
                "Open folder",
                Action::OpenFolder(kind),
                false,
            );
            button(
                &mut ui,
                &mut self.targets,
                [content_right - file_button_width, catalog_layout.folder_y, file_button_width, 38.0],
                "Import file",
                Action::Import(kind),
                false,
            );
            let bundled: Vec<_> = if page == Page::Mods
                && self.catalog.config.mod_source == crate::shader_catalog::Origin::Horizon
            {
                let query = self.catalog.query.to_lowercase();
                horizon_client::mod_list::ANDROID_BUNDLED
                    .into_iter()
                    .filter(|(id, title, _)| {
                        id.contains(&query) || title.to_lowercase().contains(&query)
                    })
                    .collect()
            } else {
                Vec::new()
            };
            let catalog_rows: Vec<_> = self
                .catalog
                .results
                .iter()
                .enumerate()
                .filter(|(_, p)| {
                    !(page == Page::Mods
                        && p.source == crate::browser::SourceId::Horizon
                        && horizon_client::mod_list::ANDROID_BUNDLED
                            .iter()
                            .any(|(id, _, _)| *id == p.id))
                })
                .collect();
            let row_count = bundled.len() + catalog_rows.len();
            let top = catalog_layout.list_top;
            let bottom = layout.content_rect[1] + layout.content_rect[3] - 12.0;
            self.catalog.scroll = self
                .catalog
                .scroll
                .clamp(0.0, (row_count as f32 * 78.0 - (bottom - top)).max(0.0));
            self.list_view = Some([
                content_left,
                top,
                content_right,
                bottom,
                (row_count as f32 * 78.0 - (bottom - top)).max(0.0),
            ]);
            for (row, (id, title, description)) in bundled.iter().enumerate() {
                let y = top + row as f32 * 78.0 - self.catalog.scroll;
                if y < top - 1.0 || y + 68.0 > bottom {
                    continue;
                }
                let title = ui.fit_text(Face::Medium, 19.0, title, action_x - content_left - 32.0);
                let summary = ui.fit_text(Face::Regular, 14.0, description, action_x - content_left - 32.0);
                ui.text(Face::Medium, 19.0, content_left, y, color::TEXT, &title);
                ui.text(Face::Regular, 14.0, content_left, y + 29.0, color::MUTED, &summary);
                button(
                    &mut ui,
                    &mut self.targets,
                    [action_x, y, 118.0, 42.0],
                    if self.catalog.bundled_enabled.contains(*id) {
                        "Disable"
                    } else {
                        "Enable"
                    },
                    Action::ToggleBundled(id),
                    false,
                );
            }
            for (row, (i, p)) in catalog_rows.into_iter().enumerate() {
                let y = top + (row + bundled.len()) as f32 * 78.0 - self.catalog.scroll;
                let download = self.catalog.download_label(p);
                if download.is_none()
                    && !crate::catalog::source_only_mod(p)
                    && !(p.kind == crate::browser::Kind::Mod
                        && p.source == crate::browser::SourceId::Modrinth)
                {
                    self.list_targets
                        .push(([action_x, y, 118.0, 70.0], Action::Install(i)));
                }
                if y < top || y + 70.0 > bottom {
                    continue;
                }
                let title = ui.fit_text(Face::Medium, 19.0, &p.title, action_x - content_left - 32.0);
                let summary = ui.fit_text(Face::Regular, 14.0, &p.summary, action_x - content_left - 32.0);
                ui.text(Face::Medium, 19.0, content_left, y, color::TEXT, &title);
                ui.text(Face::Regular, 14.0, content_left, y + 29.0, color::MUTED, &summary);
                if let Some(state) = download {
                    ui.text(Face::Medium, 16.0, action_x, y + 12.0, color::TEXT, &state);
                } else if p.kind == crate::browser::Kind::Mod
                    && p.source == crate::browser::SourceId::Modrinth
                {
                    ui.text(
                        Face::Regular,
                        14.0,
                        action_x,
                        y + 12.0,
                        color::MUTED,
                        "Java mod",
                    );
                } else if crate::catalog::source_only_mod(p) {
                    ui.text(
                        Face::Regular,
                        14.0,
                        action_x,
                        y + 12.0,
                        color::MUTED,
                        ".hnmod: later",
                    );
                } else if p.kind == crate::browser::Kind::Mod && !cfg!(target_pointer_width = "64")
                {
                    ui.text(
                        Face::Regular,
                        14.0,
                        action_x,
                        y + 12.0,
                        color::MUTED,
                        "64-bit required",
                    );
                } else {
                    button(
                        &mut ui,
                        &mut self.targets,
                        [action_x, y, 118.0, 42.0],
                        self.catalog.install_button(p),
                        Action::Install(i),
                        false,
                    );
                }
            }
            button(
                &mut ui,
                &mut self.targets,
                [content_left, layout.content_rect[1] + layout.content_rect[3], 120.0, 40.0],
                "Previous",
                Action::Previous,
                false,
            );
            button(
                &mut ui,
                &mut self.targets,
                [content_left + 134.0, layout.content_rect[1] + layout.content_rect[3], 120.0, 40.0],
                "Next",
                Action::Next,
                false,
            );
        } else if page == Page::Installed {
            use horizon_client::content_files::Kind;
            let top = layout.content_rect[1] + 12.0;
            let bottom = layout.content_rect[1] + layout.content_rect[3];
            let row_height = if compact { 110.0 } else { 72.0 };
            // Derive horizontal controls and hitboxes from shared content bounds.
            let content_left = layout.content_rect[0] + 12.0;
            let content_right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
            let import_x = content_right - 133.0;
            let section_width = import_x - content_left - 13.0;
            let toggle_x = import_x - 135.0;
            let title_width = if compact {
                content_right - content_left
            } else {
                toggle_x - content_left - 32.0
            };
            let total = Kind::ALL
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    48.0 + if self.catalog.sections[i] {
                        self.catalog.local.iter().filter(|e| e.kind == *k).count() as f32
                            * row_height
                    } else {
                        0.0
                    }
                })
                .sum::<f32>();
            let maximum = (total - (bottom - top)).max(0.0);
            self.catalog.scroll = self.catalog.scroll.clamp(0.0, maximum);
            self.list_view = Some([content_left, top, content_right, bottom, maximum]);
            let mut offset = 0.0;
            for (section, kind) in Kind::ALL.into_iter().enumerate() {
                let count = self.catalog.local.iter().filter(|e| e.kind == kind).count();
                let y = top + offset - self.catalog.scroll;
                let label = format!(
                    "{} {} ({count})",
                    if self.catalog.sections[section] {
                        "-"
                    } else {
                        "+"
                    },
                    kind.label()
                );
                self.list_targets
                    .push(([content_left, y, section_width, 40.0], Action::ToggleSection(section)));
                self.list_targets
                    .push(([import_x, y, 133.0, 40.0], Action::Import(kind)));
                if y >= top && y + 40.0 <= bottom {
                    button(
                        &mut ui,
                        &mut self.targets,
                        [content_left, y, section_width, 40.0],
                        &label,
                        Action::ToggleSection(section),
                        false,
                    );
                    button(
                        &mut ui,
                        &mut self.targets,
                        [import_x, y, 133.0, 40.0],
                        "Import",
                        Action::Import(kind),
                        false,
                    );
                }
                offset += 48.0;
                if !self.catalog.sections[section] {
                    continue;
                }
                for (i, item) in self
                    .catalog
                    .local
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| e.kind == kind)
                {
                    let y = top + offset - self.catalog.scroll;
                    offset += row_height;
                    self.list_targets.push((
                        [
                            if compact { content_left } else { toggle_x },
                            y,
                            120.0,
                            row_height - 4.0,
                        ],
                        Action::ToggleLocal(i),
                    ));
                    self.list_targets.push((
                        [import_x, y, 133.0, row_height - 4.0],
                        Action::DeleteLocal(i),
                    ));
                    if y < top || y + row_height - 4.0 > bottom {
                        continue;
                    }
                    let title = ui.fit_text(
                        Face::Medium,
                        19.0,
                        &item.title,
                        title_width,
                    );
                    ui.text(Face::Medium, 19.0, content_left, y, color::TEXT, &title);
                    ui.text(
                        Face::Regular,
                        14.0,
                        content_left,
                        y + 28.0,
                        color::MUTED,
                        kind.label(),
                    );
                    if kind == Kind::Mods && !cfg!(target_pointer_width = "64") {
                        ui.text(
                            Face::Regular,
                            14.0,
                            if compact { content_left } else { toggle_x },
                            y + if compact { 62.0 } else { 12.0 },
                            color::MUTED,
                            "64-bit required",
                        );
                    } else {
                        toggle(
                            &mut ui,
                            &mut self.targets,
                            [
                                if compact { content_left } else { toggle_x },
                                y + if compact { 55.0 } else { 0.0 },
                                120.0,
                                40.0,
                            ],
                            item.enabled,
                            Action::ToggleLocal(i),
                        );
                    }
                    button(
                        &mut ui,
                        &mut self.targets,
                        [import_x, y + if compact { 55.0 } else { 0.0 }, 133.0, 40.0],
                        "Remove",
                        Action::DeleteLocal(i),
                        false,
                    );
                }
            }
        } else if page == Page::SkinStudio {
            let studio_compact = h < 560.0;
            let content_left = layout.content_rect[0] + 12.0;
            let content_right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
            let content_width = (content_right - content_left).max(0.0);
            let content_bottom = layout.content_rect[1] + layout.content_rect[3];
            let studio_bottom = (h - 12.0)
                .min(content_bottom - 10.0)
                .max(layout.content_rect[1] + 120.0)
                .min(content_bottom - 10.0);
            let desktop_wide_preview = w >= 640.0 && content_width >= 560.0;
            let base_title_y = if studio_compact { 128.0 } else { 156.0 };
            let base_default_y = base_title_y + 184.0;
            let stacked_cape_top = base_default_y + 42.0;
            let stacked_cape_bottom = content_bottom - 10.0;
            // If the normal phone stack cannot preserve a visible preview and
            // two complete list rows, use a compact split view instead.
            let compact_cape_split = self.cape_tab
                && !desktop_wide_preview
                && w < 640.0
                && content_width >= 280.0
                && stacked_cape_bottom - stacked_cape_top - 8.0 - 92.0 < 120.0;
            let wide_preview = desktop_wide_preview || compact_cape_split;
            let preview_width = if wide_preview { content_width * 0.42 } else { content_width };
            let column_gap = if wide_preview { 12.0 } else { 0.0 };
            let control_width = if wide_preview {
                (content_width - preview_width - column_gap).max(120.0)
            } else {
                content_width
            };
            let title_y = if compact_cape_split {
                layout.content_rect[1] + 2.0
            } else {
                base_title_y
            };
            let profile_y = title_y + if compact_cape_split { 28.0 } else { 30.0 };
            let profile_buttons_y = title_y + if compact_cape_split { 48.0 } else { 66.0 };
            let import_y = title_y + if compact_cape_split { 80.0 } else { 112.0 };
            let selected_y = title_y + if compact_cape_split { 114.0 } else { 160.0 };
            let default_y = title_y + if compact_cape_split { 134.0 } else { 184.0 };
            ui.text(Face::Medium, 24.0, content_left, title_y, color::TEXT, "Skin Studio");
            let tab_y = title_y - 4.0;
            let tab_area_width = if compact_cape_split { content_width } else { control_width };
            let tab_width = ((tab_area_width - 150.0) * 0.5).clamp(52.0, 84.0);
            let tab_left = content_left + tab_area_width - tab_width * 2.0 - 6.0;
            button(&mut ui, &mut self.targets, [tab_left, tab_y, tab_width, 32.0], "Skins", Action::SkinsTab, !self.cape_tab);
            button(&mut ui, &mut self.targets, [tab_left + tab_width + 6.0, tab_y, tab_width, 32.0], "Capes", Action::CapesTab, self.cape_tab);
            let profile = self.catalog.skin_library.profiles
                .iter()
                .find(|p| p.id == self.catalog.skin_library.active_profile)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "Default".into());
            let selected = if self.cape_tab {
                if self.catalog.cape_library_error.is_some() {
                    "Unavailable · read-only".into()
                } else {
                    self.catalog.cape_library
                        .active_cape(&self.catalog.skin_library.active_profile)
                        .map(|cape| cape.name.clone())
                        .unwrap_or_else(|| "None · account cape unchanged".into())
                }
            } else {
                self.catalog.skin_library.active_skin()
                    .map(|skin| skin.name.clone())
                    .unwrap_or_else(|| "Minecraft default".into())
            };
            let model = match self.catalog.skin_library.active_model().unwrap_or_default() {
                crate::catalog::skin_library::SkinModel::Classic => "Classic",
                crate::catalog::skin_library::SkinModel::Slim => "Slim",
            };
            ui.text(Face::Regular, 16.0, content_left, profile_y, color::MUTED, &format!("Appearance profile: {profile}"));
            if let Some(error) = &self.catalog.skin_library_error {
                ui.text(Face::Regular, 14.0, content_left, profile_y + 18.0, color::MUTED, &ui.fit_text(Face::Regular, 14.0, &format!("Read-only: {error}"), control_width));
            }
            let half = (control_width - 8.0).max(0.0) * 0.5;
            let profile_button_height = if compact_cape_split { 28.0 } else { 42.0 };
            let import_button_height = if compact_cape_split { 30.0 } else { 42.0 };
            button(&mut ui, &mut self.targets, [content_left, profile_buttons_y, half, profile_button_height], "Next", Action::NextAppearanceProfile, false);
            button(&mut ui, &mut self.targets, [content_left + 8.0 + half, profile_buttons_y, half, profile_button_height], "New", Action::NewAppearanceProfile, false);
            button(&mut ui, &mut self.targets, [content_left, import_y, half, import_button_height], if self.cape_tab { "Import cape" } else { "Import PNG" }, if self.cape_tab { Action::ImportCape } else { Action::ImportSkin }, true);
            if compact_cape_split {
                button(&mut ui, &mut self.targets, [content_left + 8.0 + half, import_y, half, import_button_height], "None", Action::NoCape, false);
            } else if !self.cape_tab {
                button(&mut ui, &mut self.targets, [content_left + 8.0 + half, import_y, half, import_button_height], model, Action::ToggleSkinModel, false);
            }
            let selected_text = ui.fit_text(Face::Regular, 15.0, &format!("Selected: {selected}"), control_width);
            ui.text(Face::Regular, 15.0, content_left, selected_y, color::MUTED, &selected_text);
            if !compact_cape_split {
                if self.cape_tab {
                    button(&mut ui, &mut self.targets, [content_left, default_y, control_width, 34.0], "None · keep account cape", Action::NoCape, false);
                } else {
                    button(&mut ui, &mut self.targets, [content_left, default_y, control_width, 34.0], "Use default skin", Action::DefaultSkin, false);
                }
            }
            let preview_top = if compact_cape_split {
                title_y + 52.0
            } else if wide_preview {
                layout.content_rect[1] + 4.0
            } else if self.cape_tab {
                default_y + 42.0
            } else {
                default_y + 118.0
            };
            let preview_height = if compact_cape_split {
                compact_cape_preview_height(studio_bottom, preview_top, preview_width)
            } else if wide_preview {
                (studio_bottom - (layout.content_rect[1] + 4.0)).max(180.0)
            } else if self.cape_tab {
                stacked_cape_preview_height(stacked_cape_bottom, preview_top, 92.0)
            } else {
                (h * 0.30).clamp(180.0, 260.0)
            };
            let preview_rect = if wide_preview {
                let left = content_left + control_width + column_gap;
                let top = preview_top;
                [left, top, content_right, top + preview_height]
            } else {
                let width = preview_width.min(300.0);
                let left = content_left + (content_width - width) * 0.5;
                [left, preview_top, left + width, preview_top + preview_height]
            };
            self.skin_preview_rect = Some(preview_rect);
            ui.text(Face::Regular, 14.0, preview_rect[0], preview_rect[1] - 18.0, color::MUTED, "3D preview · drag to rotate");
            ui.rounded(preview_rect[0], preview_rect[1], preview_rect[2] - preview_rect[0], preview_rect[3] - preview_rect[1], 8.0, color::RAISED);
            let mut skin_pixels = match self.catalog.skin_preview_rgba(&self.home) {
                Ok(pixels) => pixels.map(|pixels| pixels.to_vec()),
                Err(error) => {
                    self.catalog.status = format!("Skin preview unavailable: {error}");
                    None
                }
            };
            // The launcher's compact atlas is built from installer resources,
            // not the full in-game atlas. Load vanilla's default skin directly
            // when no custom skin is selected, so the mannequin always has a
            // valid texture to render against.
            if skin_pixels.is_none() {
                let default_skin = self
                    .home
                    .join(".local/horizon-client")
                    .join(horizon_core::VANILLA_VERSION)
                    .join("assets/minecraft/textures/entity/player/wide/steve.png");
                skin_pixels = std::fs::read(&default_skin).ok().and_then(|bytes| {
                    let (width, height, rgba) = horizon_client::hud::decode_bytes(&bytes).ok()?;
                    horizon_client::skins::process_legacy_skin(width, height, &rgba)
                });
            }
            // This dedicated 64x64 atlas slot is created by HudAtlas::slots,
            // unlike the vanilla player texture which is not part of the
            // launcher's installer-only atlas.
            const PLAYER_SKIN_TEXTURE: &str = "horizon/skin-preview";
            if skin_pixels.is_none() {
                // Keep the geometry visible even if a resource install is
                // incomplete; the status makes the placeholder explicit.
                skin_pixels = Some(vec![190, 150, 120, 255].repeat(64 * 64));
                self.catalog.status = "Default skin texture unavailable; showing a neutral preview.".into();
            }
            if let Some(preview) = &self.skin_preview {
                let skin_model = self.catalog.skin_library.active_model().unwrap_or_default()
                    == crate::catalog::skin_library::SkinModel::Slim;
                let mut selected_texture = None;
                if let Some(pixels) = skin_pixels {
                    if let Some(slot) = gui.atlas.sprites.get(PLAYER_SKIN_TEXTURE).copied() {
                        ui.batch.patches.push(horizon_client::font::Patch {
                            x: (slot.u0 * gui.atlas.width as f32).round() as u32,
                            y: (slot.v0 * gui.atlas.height as f32).round() as u32,
                            width: 64,
                            height: 64,
                            rgba: pixels,
                        });
                        selected_texture = Some((PLAYER_SKIN_TEXTURE, skin_model));
                    } else {
                        let message = format!("Skin preview atlas slot is missing: {PLAYER_SKIN_TEXTURE}");
                        if self.catalog.status != message {
                            log::warn!("{message}");
                            self.catalog.status = message;
                        }
                    }
                }
                let skin_texture_ready = selected_texture.is_some();
                const CAPE_TEXTURE: &str = "horizon/cape-preview";
                let mut selected_cape_texture = None;
                if let Some(cape) = self.catalog.cape_library.active_cape(
                        &self.catalog.skin_library.active_profile,
                    )
                {
                    let cape_path = self.home.join("capes").join(&cape.file);
                    let cape_pixels = std::fs::read(&cape_path)
                        .ok()
                        .and_then(|bytes| horizon_client::hud::decode_bytes(&bytes).ok())
                        .and_then(|(width, height, rgba)| {
                            horizon_client::skins::process_cape(width, height, &rgba)
                        });
                    if let Some(pixels) = cape_pixels {
                        if let Some(slot) = gui.atlas.sprites.get(CAPE_TEXTURE).copied() {
                            ui.batch.patches.push(horizon_client::font::Patch {
                                x: (slot.u0 * gui.atlas.width as f32).round() as u32,
                                y: (slot.v0 * gui.atlas.height as f32).round() as u32,
                                width: 64,
                                height: 64,
                                rgba: pixels,
                            });
                            selected_cape_texture = Some(CAPE_TEXTURE);
                        } else {
                            let message = format!("Cape preview atlas slot is missing: {CAPE_TEXTURE}");
                            if self.catalog.status != message {
                                log::warn!("{message}");
                                self.catalog.status = message;
                            }
                        }
                    } else {
                        self.catalog.status = format!("Could not load selected cape: {}", cape.name);
                    }
                }
                let atlas_patch_count = ui.batch.patches.len();
                // Launcher layout and touch coordinates are expressed in dp,
                // while HudBatch vertices use its scaled GUI coordinate space.
                // Match the conversion used by Ui::fill/text before handing the
                // preview rectangle to PlayerPreview::draw.
                let preview_scale = ui.density / ui.batch.scale;
                let preview_batch_rect = preview_rect.map(|value| value * preview_scale);
                let (vertices, quads) = preview.draw(
                    &mut *ui.batch,
                    &gui.atlas,
                    preview_batch_rect,
                    self.skin_preview_rotation,
                    self.skin_preview_hidden_parts,
                    selected_texture,
                    selected_cape_texture,
                );
                let message = if quads == 0 {
                    format!("3D preview geometry empty: {vertices} vertices / {quads} faces; skin texture ready: {skin_texture_ready}; atlas patches: {atlas_patch_count}.")
                } else {
                    format!("3D preview submitted: {vertices} vertices / {quads} faces; skin texture ready: {skin_texture_ready}; atlas patches: {atlas_patch_count}.")
                };
                if self.catalog.status != message {
                    if quads == 0 {
                        log::warn!("{message}");
                    } else {
                        log::info!("{message}");
                    }
                    self.catalog.status = message;
                }
            } else {
                let message = self.skin_preview_error.as_deref()
                    .unwrap_or("Loading player model…");
                let message = ui.fit_text(Face::Regular, 13.0, message, (preview_rect[2] - preview_rect[0] - 16.0).max(0.0));
                ui.text(Face::Regular, 13.0, preview_rect[0] + 8.0, preview_rect[1] + 52.0, color::MUTED, &message);
            }

            // Skin overlay toggles belong only to the Skins tab. Cape mode
            // reuses the mannequin, but must not show controls that cannot
            // affect a cape and should reclaim their space for the cape list.
            let top = if self.cape_tab {
                if compact_cape_split {
                    selected_y + 20.0
                } else if wide_preview {
                    default_y + 42.0
                } else {
                    preview_rect[3] + 8.0
                }
            } else {
                let layer_top = if wide_preview { default_y + 40.0 } else { preview_rect[3] + 5.0 };
                ui.text(Face::Regular, 12.0, content_left, layer_top, color::MUTED, "Visible skin layers");
                let layer_gap = 5.0;
                let layer_area_width = if wide_preview { control_width } else { content_width };
                let layer_width = ((layer_area_width - layer_gap * 2.0) / 3.0).max(0.0);
                let layers = [
                    (1u8 << 6, "Hat"),
                    (1u8 << 1, "Jacket"),
                    (1u8 << 2, "L sleeve"),
                    (1u8 << 3, "R sleeve"),
                    (1u8 << 4, "L pants"),
                    (1u8 << 5, "R pants"),
                ];
                for (index, (bit, label)) in layers.iter().enumerate() {
                    let column = index % 3;
                    let row = index / 3;
                    let x = content_left + column as f32 * (layer_width + layer_gap);
                    let y = layer_top + 18.0 + row as f32 * 32.0;
                    let visible = self.skin_preview_hidden_parts & *bit == 0;
                    button(
                        &mut ui,
                        &mut self.targets,
                        [x, y, layer_width, 28.0],
                        label,
                        Action::ToggleSkinLayer(*bit),
                        visible,
                    );
                }
                // Reserve both rows only while skin overlay controls are visible.
                layer_top + 86.0
            };
            let bottom = if wide_preview {
                studio_bottom
            } else {
                (layout.content_rect[1] + layout.content_rect[3] - 10.0).max(top + 1.0)
            };
            let row_height = 48.0;
            let row_gap = 4.0;
            let row_surface = row_height - row_gap;
            let cape_read_only = self.cape_tab && self.catalog.cape_library_error.is_some();
            let count = if cape_read_only {
                0
            } else if self.cape_tab {
                self.catalog.cape_library.capes.len()
            } else {
                self.catalog.skin_library.skins.len()
            };
            let maximum = (count as f32 * row_height - (bottom - top).max(0.0)).max(0.0);
            self.catalog.scroll = self.catalog.scroll.clamp(0.0, maximum);
            let list_right = if wide_preview {
                content_left + control_width
            } else {
                content_right
            };
            self.list_view = Some([content_left, top, list_right, bottom, maximum]);
            if count == 0 {
                let message = if cape_read_only {
                    format!(
                        "Cape library read-only: {}",
                        self.catalog.cape_library_error.as_deref().unwrap_or("metadata unavailable")
                    )
                } else if self.cape_tab {
                    "No capes imported yet. Choose Import cape.".into()
                } else {
                    "No imported skins yet. Choose Import PNG skin.".into()
                };
                let message = ui.fit_text(Face::Regular, 14.0, &message, (list_right - content_left).max(0.0));
                ui.text(Face::Regular, 14.0, content_left, top + 16.0, color::MUTED, &message);
            }
            if self.cape_tab {
                if !cape_read_only {
                    for (i, cape) in self.catalog.cape_library.capes.iter().enumerate() {
                    let y = top + i as f32 * row_height - self.catalog.scroll;
                    if y < top || y + row_surface > bottom { continue; }
                    let active = self.catalog.cape_library.active_cape(&self.catalog.skin_library.active_profile).is_some_and(|current| current.id == cape.id);
                    let row_width = (list_right - content_left).max(0.0);
                    let delete_width = if compact_cape_split { 38.0 } else { 42.0 };
                    let rename_width = if compact_cape_split { 44.0 } else { 50.0 };
                    let gap = if compact_cape_split { 3.0 } else { 4.0 };
                    let select_width = (row_width - delete_width - rename_width - gap * 2.0)
                        .max(if compact_cape_split { 64.0 } else { 72.0 });
                    let label = ui.fit_text(
                        Face::Regular,
                        14.0,
                        &format!("{}  ({}×{})", cape.name, cape.width, cape.height),
                        (select_width - 12.0).max(0.0),
                    );
                    button(
                        &mut ui,
                        &mut self.targets,
                        [content_left, y, select_width, row_surface],
                        &label,
                        Action::SelectCape(i),
                        active,
                    );
                    let rename_x = content_left + select_width + gap;
                    button(
                        &mut ui,
                        &mut self.targets,
                        [rename_x, y, rename_width, row_surface],
                        "Rename",
                        Action::RenameCape(i),
                        false,
                    );
                    button(
                        &mut ui,
                        &mut self.targets,
                        [rename_x + rename_width + gap, y, delete_width, row_surface],
                        "Del",
                        Action::DeleteCape(i),
                        false,
                    );
                    }
                }
            } else {
            for (i, skin) in self.catalog.skin_library.skins.iter().enumerate() {
                let y = top + i as f32 * row_height - self.catalog.scroll;
                if y < top || y + row_surface > bottom { continue; }
                let rect = [content_left, y, list_right - content_left, row_surface];
                self.list_targets.push((rect, Action::SelectSkin(i)));
                let active = self.catalog.skin_library.active_skin().is_some_and(|current| current.id == skin.id);
                let raw_label = format!("{}  ({}×{})", skin.name, skin.width, skin.height);
                let available_width = (control_width - 8.0).max(0.0);
                let natural_width = ui.measure(Face::Medium, 15.0, &raw_label) + 36.0;
                let row_width = natural_width.min(available_width).max(96.0);
                let rect = [content_left, y, row_width, row_surface];
                let label = ui.fit_text(Face::Regular, 15.0, &raw_label, (row_width - 24.0).max(0.0));
                button(&mut ui, &mut self.targets, rect, &label, Action::SelectSkin(i), active);
            }
            }
        } else if self.about_visible {
            let content_left = layout.content_rect[0] + 12.0;
            let content_right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
            let content_width = (content_right - content_left).max(0.0);
            let content_bottom = layout.content_rect[1] + layout.content_rect[3];
            ui.text(
                Face::Medium,
                24.0,
                content_left,
                layout.content_rect[1] + 12.0,
                color::TEXT,
                "About Sylphra",
            );
            let lines = [
                "Sylphra is an independent,",
                "unofficial Android launcher",
                "built using Horizon Android",
                "as its technical foundation.",
                "We are not affiliated with,",
                "sponsored by, or endorsed by",
                "the Horizon project or its",
                "original Android port author.",
                "We do not represent Horizon",
                "and do not know its creators'",
                "position on the Sylphra project.",
                "",
                "We deeply respect and thank",
                "the original Android port",
                "author. Horizon provided a",
                "valuable foundation and saved",
                "us substantial development work.",
                "",
                "Original project attribution",
                "and required license notices",
                "are retained. Each component",
                "remains governed by its own",
                "license. See LICENSE and",
                "THIRD_PARTY_NOTICES.",
                "",
                "If a rights holder raises a",
                "concrete rights or licensing",
                "concern, contact us. We will",
                "review it and take appropriate",
                "action, including removal",
                "where warranted.",
                "",
                "The original creators are",
                "welcome to review the project",
                "or collaborate. If they join,",
                "we will gladly credit them",
                "as collaborators.",
            ];
            let line_height = if compact { 32.0 } else { 30.0 };
            let top = layout.content_rect[1] + 50.0;
            let bottom = content_bottom - 72.0;
            let maximum = (lines.len() as f32 * line_height - (bottom - top)).max(0.0);
            self.catalog.scroll = self.catalog.scroll.clamp(0.0, maximum);
            self.list_view = Some([content_left, top, content_right, bottom, maximum]);
            for (i, line) in lines.iter().enumerate() {
                let y = top + i as f32 * line_height - self.catalog.scroll;
                if y + line_height < top || y > bottom {
                    continue;
                }
                let text = ui.fit_text(Face::Regular, 16.0, line, content_width);
                ui.text(Face::Regular, 16.0, content_left, y, color::TEXT, &text);
            }
            button(
                &mut ui,
                &mut self.targets,
                [content_left, content_bottom - 14.0, (content_width - 12.0) / 2.0, 42.0],
                "Back",
                Action::BackAbout,
                false,
            );
            button(
                &mut ui,
                &mut self.targets,
                [content_left + (content_width + 12.0) / 2.0, content_bottom - 14.0, (content_width - 12.0) / 2.0, 42.0],
                "Sylphra GitHub",
                Action::SylphraRepository,
                true,
            );
        } else {
            let c = &self.catalog.config;
            let settings_left = layout.content_rect[0] + 12.0;
            let settings_right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
            let settings_top = layout.content_rect[1] + 51.0;
            let settings_bottom = layout.content_rect[1] + layout.content_rect[3] - 30.0;
            // Preserve the current wide-screen column while deriving it from the
            // shared content rectangle; compact rows use the full content width.
            let settings_split = layout.content_rect[0] + layout.content_rect[2] * 0.36 - 5.6;
            ui.text(
                Face::Medium,
                if compact { 21.0 } else { 24.0 },
                settings_left,
                layout.content_rect[1] + if compact { 6.0 } else { 12.0 },
                color::TEXT,
                "Launch settings",
            );
            // Compact rows retain a stacked label/value layout. On short screens,
            // trim only the vertical padding; the list remains scrollable.
            let row_height = if compact {
                if h < 520.0 { 76.0 } else { 82.0 }
            } else {
                66.0
            };
            let settings_maximum =
                (21.0 * row_height - (settings_bottom - settings_top)).max(0.0);
            self.catalog.scroll = self.catalog.scroll.clamp(0.0, settings_maximum);
            self.list_view = Some([settings_left, settings_top, settings_right, settings_bottom, settings_maximum]);
            for (i, (title, value, action)) in [
                ("Rendering", c.renderer.label().to_owned(), Action::Renderer),
                (
                    "Client resolution",
                    if self.catalog.editing == Some("render_scale") {
                        self.catalog.render_scale_text.clone()
                    } else {
                        format!("{}%", c.render_scale.percent())
                    },
                    Action::RenderScale,
                ),
                (
                    "Chunk worker limit",
                    if c.worker_limit == 0 {
                        "No additional limit".into()
                    } else {
                        c.worker_limit.to_string()
                    },
                    Action::WorkerLimit,
                ),
                (
                    "Reusable RAM cache",
                    if c.cache_budget_mib == 0 {
                        "Engine default (80 MiB)".into()
                    } else {
                        format!("{} MiB", c.cache_budget_mib)
                    },
                    Action::CacheBudget,
                ),
                (
                    "Game files",
                    "Open in system Files".into(),
                    Action::GameFiles,
                ),
                (
                    "World files",
                    "Import / copy / back up in Files".into(),
                    Action::WorldFiles,
                ),
                ("Profile", format!("Profile {}", c.profile), Action::Profile),
                (
                    "Orientation",
                    c.orientation.label().to_owned(),
                    Action::Orientation,
                ),
                ("Input", c.input_host.label().to_owned(), Action::InputHost),
                (
                    "Immersive fullscreen",
                    if c.screen.immersive { "On" } else { "Off" }.into(),
                    Action::Immersive,
                ),
                (
                    "Avoid camera cutout",
                    if c.screen.avoid_cutout { "On" } else { "Off" }.into(),
                    Action::Cutout,
                ),
                (
                    "Aspect ratio",
                    c.screen.aspect.label().into(),
                    Action::Aspect,
                ),
                (
                    "Display refresh rate",
                    c.device.display_label(),
                    Action::DisplayHz,
                ),
                (
                    "Performance hints",
                    if c.device.performance_hints {
                        "On (ADPF)"
                    } else {
                        "Off"
                    }
                    .into(),
                    Action::PerformanceHints,
                ),
                (
                    "GPU profiling",
                    if c.device.gpu_profile {
                        "On / included in report"
                    } else {
                        "Off"
                    }
                    .into(),
                    Action::GpuProfile,
                ),
                (
                    "Performance recording",
                    if c.record_performance {
                        "On / up to 30 min"
                    } else {
                        "Off"
                    }
                    .into(),
                    Action::RecordPerformance,
                ),
                (
                    "Diagnostic report",
                    "Export one ZIP to share".into(),
                    Action::ExportDiagnostics,
                ),
                (
                    "Raw performance data",
                    "Advanced / JSONL in Reports".into(),
                    Action::ExportPerformance,
                ),
                (
                    "Server",
                    if c.server.is_empty() {
                        "Singleplayer / tap to enter server".into()
                    } else {
                        c.server.clone()
                    },
                    Action::Field("server"),
                ),
                (
                    "Skin Studio",
                    "Import and manage player skins".into(),
                    Action::SkinStudio,
                ),
                (
                    "About Sylphra",
                    "Project, attribution and licenses".into(),
                    Action::About,
                ),
            ]
            .into_iter()
            .enumerate()
            {
                let y = settings_top + i as f32 * row_height - self.catalog.scroll;
                self.list_targets.push((
                    [
                        if compact { settings_left } else { settings_split },
                        y,
                        if compact { settings_right - settings_left } else { settings_right - settings_split },
                        if compact { 70.0 } else { 46.0 },
                    ],
                    action,
                ));
                if y < settings_top - 10.0
                    || y + if compact { 70.0 } else { 46.0 } > settings_bottom
                {
                    continue;
                }
                ui.text(
                    Face::Regular,
                    16.0,
                    settings_left,
                    y + if compact { 0.0 } else { 13.0 },
                    color::MUTED,
                    title,
                );
                if matches!(
                    action,
                    Action::PerformanceHints | Action::Immersive | Action::Cutout
                ) {
                    toggle(
                        &mut ui,
                        &mut self.targets,
                        [
                            if compact { settings_left } else { settings_split },
                            y + if compact {
                                if h < 520.0 { 20.0 } else { 24.0 }
                            } else {
                                0.0
                            },
                            if compact { settings_right - settings_left } else { settings_right - settings_split },
                            if compact && h < 520.0 { 42.0 } else { 46.0 },
                        ],
                        match action {
                            Action::PerformanceHints => c.device.performance_hints,
                            Action::Immersive => c.screen.immersive,
                            Action::Cutout => c.screen.avoid_cutout,
                            _ => c.device.gpu_profile,
                        },
                        action,
                    );
                } else {
                    button(
                        &mut ui,
                        &mut self.targets,
                        [
                            if compact { settings_left } else { settings_split },
                            y + if compact {
                                if h < 520.0 { 20.0 } else { 24.0 }
                            } else {
                                0.0
                            },
                            if compact { settings_right - settings_left } else { settings_right - settings_split },
                            if compact && h < 520.0 { 42.0 } else { 46.0 },
                        ],
                        &format!(
                            "{} {}",
                            value,
                            if matches!(
                                action,
                                Action::Renderer
                                    | Action::Aspect
                                    | Action::DisplayHz
                                    | Action::RenderScale
                                    | Action::Profile
                                    | Action::Orientation
                                    | Action::InputHost
                            ) {
                                "v"
                            } else {
                                ""
                            }
                        ),
                        action,
                        false,
                    );
                }
                ui.rounded(
                    settings_left,
                    y + if compact {
                        if h < 520.0 { 70.0 } else { 78.0 }
                    } else {
                        62.0
                    },
                    settings_right - settings_left,
                    1.0,
                    0.0,
                    color::BORDER,
                );
            }
            button(
                &mut ui,
                &mut self.targets,
                [
                    settings_left + (settings_right - settings_left + 12.0) / 2.0,
                    settings_bottom + 16.0,
                    (settings_right - settings_left - 12.0) / 2.0,
                    42.0,
                ],
                "Microsoft / Xbox account",
                Action::Account,
                true,
            );
            button(
                &mut ui,
                &mut self.targets,
                [
                    settings_left,
                    settings_bottom + 16.0,
                    (settings_right - settings_left - 12.0) / 2.0,
                    42.0,
                ],
                "Diagnostics",
                Action::Logs,
                false,
            );
        }
        let footer_left = layout.content_rect[0] + 12.0;
        let footer_right = layout.content_rect[0] + layout.content_rect[2] - 12.0;
        let footer_text_width = (footer_right - footer_left - 146.0).max(40.0);
        if !skin_studio_compact {
            let status = self.error.as_deref().unwrap_or(&self.catalog.status);
            for (i, line) in status
                .chars()
                .collect::<Vec<_>>()
                .chunks(110)
                .take(2)
                .enumerate()
            {
                ui.text(
                    Face::Regular,
                    14.0,
                    footer_left,
                    h - 47.0 + i as f32 * 18.0,
                    color::MUTED,
                    &ui.fit_text(
                        Face::Regular,
                        14.0,
                        &line.iter().collect::<String>(),
                        footer_text_width,
                    ),
                );
            }
        }
        let wide_skin_preview = page == Page::SkinStudio
            && w >= 640.0
            && (layout.content_rect[2] - 24.0) >= 560.0;
        if !(page == Page::SkinStudio && (compact || wide_skin_preview)) {
            button(
                &mut ui,
                &mut self.targets,
                [footer_right - 110.0, h - 52.0, 110.0, 32.0],
                "Sylphra GitHub",
                Action::SylphraRepository,
                false,
            );
        }
        if self.login.visible {
            self.list_targets.clear();
            self.list_view = None;
            self.login.draw(
                &mut ui,
                &mut self.targets,
                &self.catalog.config.name,
                self.catalog.editing == Some("name"),
            );
        }
        if let Some(chooser) = &mut self.chooser {
            chooser.draw(&mut ui, &mut self.targets);
            self.list_targets = chooser.navigation(ui.w);
            self.list_view = Some(chooser.viewport(ui.w, ui.h));
        }
        if let Some([_, top, right, bottom, maximum]) = self.list_view
            && maximum > 0.0
            && bottom > top
        {
            let scroll = self
                .chooser
                .as_ref()
                .map_or(self.catalog.scroll, |c| c.scroll);
            let height = bottom - top;
            let thumb = (height * height / (height + maximum)).max(20.0).min(height);
            let y = top + (height - thumb) * (scroll / maximum).clamp(0.0, 1.0);
            let scrollbar_x = right - 12.0;
            ui.rounded(scrollbar_x, top, 4.0, height, 2.0, color::BORDER);
            ui.rounded(scrollbar_x, y, 4.0, thumb, 2.0, color::MUTED);
        }
        let caret_visible = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .is_ok_and(|t| t.as_millis() % 1000 < 500);
        if caret_visible && let Some(name) = self.catalog.editing {
            let value = match name {
                "name" => self.catalog.config.name.clone(),
                "server" => self.catalog.config.server.clone(),
                "render_scale" => self.catalog.render_scale_text.clone(),
                _ => format!("Search: {}", self.catalog.query),
            };
            for ([x, y, w, _], action) in &self.targets {
                if *action == Action::Field(name)
                    || (name == "render_scale" && *action == Action::RenderScale)
                {
                    let text = ui.fit_text(Face::Medium, 16.0, &value, w - 24.0);
                    let caret_x = x + 12.0 + ui.measure(Face::Medium, 16.0, &text);
                    ui.rounded(
                        caret_x.min(x + w - 10.0),
                        y + 12.0,
                        1.0,
                        18.0,
                        0.0,
                        color::TEXT,
                    );
                }
            }
        }
        if self.exit_confirmation {
            self.targets.clear();
            self.list_targets.clear();
            self.list_view = None;
            ui.rounded(0.0, 0.0, w, h, 0.0, horizon_client::hud::rgba(0, 0, 0, 160));
            let width = (w - 32.0).min(360.0);
            let x = (w - width) * 0.5;
            let y = (h - 160.0) * 0.5;
            ui.rounded(x, y, width, 160.0, 8.0, color::RAISED);
            ui.text(
                Face::Medium,
                22.0,
                x + 20.0,
                y + 24.0,
                color::TEXT,
                "Exit application?",
            );
            let bw = (width - 52.0) * 0.5;
            button(
                &mut ui,
                &mut self.targets,
                [x + 20.0, y + 92.0, bw, 44.0],
                "Cancel",
                Action::Dismiss,
                false,
            );
            button(
                &mut ui,
                &mut self.targets,
                [x + 32.0 + bw, y + 92.0, bw, 44.0],
                "Exit",
                Action::ConfirmExit,
                false,
            );
        }
        if self.welcome_visible {
            self.targets.clear();
            self.list_targets.clear();
            self.list_view = None;
            ui.rounded(0.0, 0.0, w, h, 0.0, horizon_client::hud::rgba(0, 0, 0, 190));
            let width = (w - 32.0).min(440.0);
            let height = (h - 24.0).min(330.0);
            let x = (w - width) * 0.5;
            let y = (h - height) * 0.5;
            ui.rounded(x, y, width, height, 8.0, color::RAISED);
            ui.text(Face::Medium, 24.0, x + 20.0, y + 24.0, color::TEXT, "Welcome to Sylphra");
            let notice = [
                "Independent, unofficial Android launcher",
                "based on Horizon Android.",
                "Not affiliated with or endorsed by",
                "the Horizon project creators.",
                "",
                "Open About Sylphra for full attribution,",
                "license information and project details.",
            ];
            for (i, line) in notice.iter().enumerate() {
                let text = ui.fit_text(Face::Regular, 16.0, line, width - 40.0);
                ui.text(
                    Face::Regular,
                    16.0,
                    x + 20.0,
                    y + 76.0 + i as f32 * 25.0,
                    color::TEXT,
                    &text,
                );
            }
            button(
                &mut ui,
                &mut self.targets,
                [x + 20.0, y + height - 64.0, width - 40.0, 44.0],
                "Continue",
                Action::ContinueWelcome,
                true,
            );
            self.pending_focus = Some(Action::ContinueWelcome);
        }
        if let Some(action) = self.pending_focus.take() {
            self.focus = self.targets.iter().position(|(_, a)| *a == action);
        }
        if let Some(index) = self.focus
            && let Some(([x, y, w, h], _)) = self.targets.get(index)
        {
            ui.border(*x - 2.0, *y - 2.0, *w + 4.0, *h + 4.0, 2.0, color::TEXT);
            self.cursor = (
                f64::from((x + w / 2.0) * ui.density),
                f64::from((y + h / 2.0) * ui.density),
            );
        }
        if self.mouse_down || self.drag.as_ref().is_some_and(|drag| !drag.moved) {
            let point = [
                self.cursor.0 as f32 / ui.density,
                self.cursor.1 as f32 / ui.density,
            ];
            if let Some(([x, y, w, h], _)) = self.targets.iter().rev().find(|(rect, _)| {
                point[0] >= rect[0]
                    && point[0] <= rect[0] + rect[2]
                    && point[1] >= rect[1]
                    && point[1] <= rect[1] + rect[3]
            }) {
                ui.rounded(
                    *x,
                    *y,
                    *w,
                    *h,
                    2.0,
                    horizon_client::hud::rgba(255, 255, 255, 48),
                );
            }
        }
        if !self.mouse_down
            && self.drag.is_none()
            && let Some((action, time)) = self.feedback
            && time.elapsed() < std::time::Duration::from_millis(160)
            && let Some(([x, y, w, h], _)) = self.targets.iter().find(|(_, a)| *a == action)
        {
            ui.rounded(
                *x,
                *y,
                *w,
                *h,
                2.0,
                horizon_client::hud::rgba(255, 255, 255, 48),
            );
        }
        crate::documents::draw_ui(&mut ui);
        ui.finish();

        if !r.render(&game::loading_frame(&b))? && !b.patches.is_empty() {
            // A suspended/lost surface skips the frame, including its glyph
            // uploads. Rebuild them next time instead of caching invisible text.
            horizon_client::ui_theme::reset_atlas();
        }
        Ok(())
    }
    pub(super) fn activate(&mut self, events: &ActiveEventLoop) -> Result<()> {
        if self.session_worker.is_some() && self.chooser.is_none() && !self.exit_confirmation {
            return Ok(());
        }
        let Some(r) = &self.renderer else {
            return Ok(());
        };
        let mut b = HudBatch::new([r.config.width, r.config.height]);
        let ui = Ui::mobile(&self.gui.as_ref().unwrap().atlas, &mut b, (0.0, 0.0));
        let p = self.focus.and_then(|index| self.targets.get(index)).map_or(
            [
                self.cursor.0 as f32 / ui.density,
                self.cursor.1 as f32 / ui.density,
            ],
            |([x, y, w, h], _)| [x + w / 2.0, y + h / 2.0],
        );
        let action = self
            .targets
            .iter()
            .find(|([x, y, w, h], _)| {
                (*x..*x + *w).contains(&p[0]) && (*y..*y + *h).contains(&p[1])
            })
            .map(|(_, a)| *a);
        if self.welcome_visible {
            if action == Some(Action::ContinueWelcome) {
                let previous = self.catalog.config.welcome_ack_revision;
                self.catalog.config.welcome_ack_revision =
                    crate::catalog::WELCOME_NOTICE_REVISION;
                match self.catalog.config.save(&self.home) {
                    Ok(()) => {
                        self.welcome_visible = false;
                        self.catalog.status = "Welcome to Sylphra.".into();
                        self.targets.clear();
                        self.list_targets.clear();
                        self.focus = None;
                    }
                    Err(error) => {
                        self.catalog.config.welcome_ack_revision = previous;
                        self.fail(error);
                    }
                }
            }
            return Ok(());
        }
        if self.catalog.busy() || crate::documents::busy() {
            return Ok(());
        }
        self.feedback = action.map(|action| (action, std::time::Instant::now()));
        self.focus = None;
        if action.is_some() && horizon_client::official_install::ready(&self.home) {
            let audio = self.audio.get_or_insert_with(|| {
                let mut audio = horizon_client::audio::Audio::new(
                    &self.home.join(".local/horizon-client/26.3"),
                );
                let settings = horizon_client::settings::Settings::load(
                    &self
                        .catalog
                        .config
                        .game_home(&self.home)
                        .join("options.txt"),
                );
                audio.master = settings.volumes[0];
                let mut volumes = settings.volumes;
                volumes[0] = 1.0;
                horizon_client::settings::set_source_volumes(volumes);
                audio
            });
            audio.play_button_click();
        }
        match action {
            Some(Action::ConfirmExit) => {
                events.exit();
            }
            Some(Action::Immersive) => {
                self.catalog.config.screen.immersive ^= true;
                self.catalog.config.save(&self.home)?;
                self.configure_screen();
            }
            Some(Action::Cutout) => {
                self.catalog.config.screen.avoid_cutout ^= true;
                self.catalog.config.save(&self.home)?;
                self.configure_screen();
            }
            Some(Action::Aspect) => self.open_chooser(chooser::Kind::Aspect),
            Some(Action::ChooseAspect(aspect)) => {
                self.catalog.config.screen.aspect = aspect;
                self.catalog.config.save(&self.home)?;
                self.configure_screen();
                self.chooser = None;
            }
            Some(Action::CacheBudget) => self.open_chooser(chooser::Kind::CacheBudget),
            Some(Action::ChooseCacheBudget(mib)) => {
                self.catalog.config.cache_budget_mib = mib.min(1024);
                self.catalog.config.save(&self.home)?;
                self.chooser = None;
                self.catalog.status = "Disposable cache budget saved. Applies on next Play; not a total RAM or VRAM limit.".into();
            }
            Some(Action::ToggleBundled(id)) => {
                let game = self.catalog.config.game_home(&self.home);
                let mut list = horizon_client::mod_list::ModList::load(&game);
                let enabled = list.enabled.contains(id) && !list.disabled.contains(id);
                if enabled {
                    list.enabled.remove(id);
                    list.disabled.insert(id.into());
                } else {
                    list.disabled.remove(id);
                    list.enabled.insert(id.into());
                }
                std::fs::create_dir_all(&game)?;
                std::fs::write(game.join(horizon_client::mod_list::FILE), list.serialize())?;
                self.catalog.refresh(&self.home);
                self.catalog.status = format!(
                    "Bundled {id} {}. Applies on Play; settings are in Options > Mods.",
                    if enabled { "disabled" } else { "enabled" }
                );
            }
            Some(Action::GameFiles) => {
                self.catalog.status = if crate::platform::open_documents(crate::document_store::ROOT) {
                    "Game files opened. Worlds, mods, packs, shaders and reports are available here."
                } else { "Open system Files > Horizon Android." }.into();
            }
            Some(Action::WorldFiles) => {
                let id = crate::document_store::folder_id(self.catalog.config.profile, "worlds");
                self.catalog.status = if crate::platform::open_documents(&id) {
                    "Copy a complete world folder to back up or import. Finish copying before Play; never replace an existing world silently."
                } else { "Open system Files > Horizon Android > Worlds. Copy complete world folders while the game is closed." }.into();
            }
            Some(Action::WorkerLimit) => self.open_chooser(chooser::Kind::WorkerLimit),
            Some(Action::ChooseWorkerLimit(limit)) => {
                self.catalog.config.worker_limit = limit.min(64);
                self.catalog.config.save(&self.home)?;
                self.chooser = None;
                self.catalog.status = "Chunk worker ceiling saved. Applies on the next Play; other engine threads are unchanged.".into();
            }
            Some(Action::DisplayHz) => self.open_chooser(chooser::Kind::DisplayHz),
            Some(Action::ChooseDisplayHz(hz)) => {
                self.catalog.config.device.display_hz = hz;
                self.catalog.config.device.sanitize();
                self.catalog.config.save(&self.home)?;
                crate::device_runtime::configure(self.catalog.config.device);
                self.chooser = None;
            }
            Some(Action::PerformanceHints) => {
                self.catalog.config.device.performance_hints ^= true;
                self.catalog.status="ADPF gives Android CPU frame-work hints on supported devices. It does not cap FPS or guarantee a speedup.".into();
                self.catalog.config.save(&self.home)?;
                crate::device_runtime::configure(self.catalog.config.device);
            }
            Some(Action::GpuProfile) => {
                self.catalog.config.device.gpu_profile ^= true;
                self.catalog.status="Detailed GPU profiling records diagnostics and has overhead. Reports remain on this device.".into();
                self.catalog.config.save(&self.home)?;
                crate::device_runtime::configure(self.catalog.config.device);
            }

            Some(Action::ModSource(origin)) => {
                self.catalog.config.mod_source = origin;
                self.catalog.config.save(&self.home)?;
                self.catalog.offset = 0;
                self.catalog.browse(&self.home);
            }
            Some(Action::ShaderSource(origin)) => {
                self.catalog.config.shader_source = origin;
                self.catalog.config.save(&self.home)?;
                self.catalog.offset = 0;
                self.catalog.browse(&self.home);
            }
            Some(Action::OpenFolder(kind)) => {
                crate::platform::hide();
                let folder = match kind {
                    horizon_client::content_files::Kind::Mods => "mods",
                    horizon_client::content_files::Kind::Packs => "packs",
                    horizon_client::content_files::Kind::Shaders => "shaders",
                };
                let id = crate::document_store::folder_id(self.catalog.config.profile, folder);
                self.catalog.status = if crate::platform::open_documents(&id) {
                    "Opened in Files. Return here after copying files.".into()
                } else {
                    "Open the system Files app and choose Horizon Android.".into()
                };
            }
            Some(Action::Import(kind)) => {
                crate::platform::hide();
                let home = self.catalog.config.game_home(&self.home);
                if let Err(e) = crate::documents::begin(kind, home) {
                    self.catalog.status = e;
                }
            }
            Some(Action::ToggleSection(i)) => {
                self.catalog.sections[i] = !self.catalog.sections[i];
                self.catalog.scroll = 0.0;
            }
            Some(Action::ToggleLocal(i)) => self.catalog.toggle_local(&self.home, i),
            Some(Action::DeleteLocal(i)) => {
                self.catalog.pending_delete = self.catalog.local.get(i).cloned();
                if let Some(item) = &self.catalog.pending_delete {
                    self.chooser = Some(super::chooser::Chooser::confirm_delete(item.file.clone()));
                }
            }
            Some(Action::ConfirmDeleteLocal) => {
                self.catalog.remove_local(&self.home);
                self.chooser = None;
            }
            Some(Action::About) => {
                self.about_visible = true;
                self.catalog.scroll = 0.0;
            }
            Some(Action::BackAbout) => {
                self.about_visible = false;
                self.catalog.scroll = 0.0;
            }
            Some(Action::SylphraRepository) => {
                self.catalog.status = if crate::platform::open_url("https://github.com/sylphie07/Sylphra-Android-Public") {
                    "Public Sylphra project page opened in browser."
                } else {
                    "No browser available to open the Sylphra repository."
                }
                .into();
            }
            Some(Action::ContinueWelcome) => {}
            Some(Action::Repository) => {
                self.catalog.status = if crate::platform::open_repository() {
                    "Repository opened in browser."
                } else {
                    "No browser available to open the repository."
                }
                .into();
            }
            Some(Action::Account) => {
                self.login.visible = true;
            }
            Some(Action::CloseLogin) => {
                self.login.hide();
                crate::platform::hide();
                self.catalog.editing = None;
            }
            Some(Action::DeviceSignIn) => {
                self.session_ready = false;
                self.prepared_session = None;
                self.login
                    .start_device(&self.catalog.config.microsoft_client_id);
            }
            Some(Action::CopyLoginCode) => self.login.copy_code(),
            Some(Action::CancelLogin) => {
                self.login.close();
                self.login.visible = true;
            }
            Some(Action::ReviewApp) => self.login.review_app(),
            Some(Action::SignIn) => {
                self.session_ready = false;
                self.prepared_session = None;
                self.login.start(&self.catalog.config.microsoft_client_id);
            }
            Some(Action::SignOut) => {
                self.session_ready = false;
                self.prepared_session = None;
                self.login.sign_out();
            }
            Some(Action::LoginLink) => self.login.open_link(),
            Some(Action::Page(page)) => {
                self.about_visible = false;
                self.catalog.editing = None;
                if crate::platform::active() {
                    crate::platform::hide();
                }
                self.catalog.page = page;
                if page == Page::SkinStudio && self.skin_preview.is_none() {
                    // Re-entering Skin Studio permits a fresh attempt after
                    // resources have been installed or repaired.
                    self.skin_preview_error = None;
                }
                self.catalog.scroll = 0.0;
                self.error = None;
                self.catalog.offset = 0;
                self.catalog.browse(&self.home);
            }
            Some(Action::SkinStudio) => {
                self.about_visible = false;
                self.catalog.editing = None;
                crate::platform::hide();
                self.catalog.page = Page::SkinStudio;
                self.catalog.scroll = 0.0;
                self.catalog.reload_skin_library(&self.home);
                if self.skin_preview.is_none() {
                    self.skin_preview_error = None;
                }
            }
            Some(Action::SkinsTab) => { self.cape_tab = false; self.catalog.scroll = 0.0; }
            Some(Action::CapesTab) => { self.cape_tab = true; self.catalog.reload_cape_library(&self.home); self.catalog.scroll = 0.0; }
            Some(Action::ImportCape) => {
                crate::platform::hide();
                if let Err(error) = crate::documents::begin_cape(self.home.clone()) { self.catalog.status = error; }
            }
            Some(Action::SelectCape(index)) => {
                if let Some(cape) = self.catalog.cape_library.capes.get(index) {
                    let id = cape.id.clone();
                    let name = cape.name.clone();
                    let profile = self.catalog.skin_library.active_profile.clone();
                    match self.catalog.select_cape_and_save(&profile, Some(&id), &self.home) {
                        Ok(()) => self.catalog.status = format!("Selected cape: {name}"),
                        Err(error) => self.catalog.status = error,
                    }
                }
            }
            Some(Action::RenameCape(index)) => {
                self.catalog.reload_cape_library(&self.home);
                if let Some(error) = &self.catalog.cape_library_error {
                    self.catalog.status = format!("Cape library is read-only: {error}");
                } else if let Some(cape) = self.catalog.cape_library.capes.get(index) {
                    self.catalog.cape_rename_id = Some(cape.id.clone());
                    self.catalog.cape_rename_text = cape.name.clone();
                    self.catalog.editing = Some("cape_name");
                    crate::platform::show(&self.catalog.cape_rename_text);
                }
            }
            Some(Action::DeleteCape(index)) => {
                self.catalog.reload_cape_library(&self.home);
                if let Some(error) = &self.catalog.cape_library_error {
                    self.catalog.status = format!("Cape library is read-only: {error}");
                } else if let Some(cape) = self.catalog.cape_library.capes.get(index) {
                    self.catalog.pending_cape_delete = Some(cape.id.clone());
                    self.chooser = Some(super::chooser::Chooser::confirm_delete_cape(cape.name.clone()));
                }
            }
            Some(Action::ConfirmDeleteCape) => {
                if let Some(id) = self.catalog.pending_cape_delete.take() {
                    match self.catalog.remove_cape(&id, &self.home) {
                        Ok(()) => {
                            self.catalog.status = "Cape removed from the library and all appearance profiles.".into();
                            self.catalog.reload_cape_library(&self.home);
                        }
                        Err(error) => {
                            self.catalog.status = format!("Could not completely remove cape: {error}");
                            self.catalog.reload_cape_library(&self.home);
                        }
                    }
                }
                self.chooser = None;
            }
            Some(Action::NoCape) => {
                let profile = self.catalog.skin_library.active_profile.clone();
                match self.catalog.select_cape_and_save(&profile, None, &self.home) {
                    Ok(()) => self.catalog.status = "Sylphra cape disabled; account cape remains unchanged.".into(),
                    Err(error) => self.catalog.status = error,
                }
            }
            Some(Action::ImportSkin) => {
                crate::platform::hide();
                if let Err(error) = crate::documents::begin_skin(self.home.clone()) {
                    self.catalog.status = error;
                }
            }
            Some(Action::SelectSkin(index)) => {
                if let Some(skin) = self.catalog.skin_library.skins.get(index) {
                    let id = skin.id.clone();
                    let name = skin.name.clone();
                    let profile = self.catalog.skin_library.active_profile.clone();
                    match self.catalog.skin_library.select_skin(&profile, Some(&id))
                        .and_then(|()| self.catalog.save_skin_library(&self.home))
                    {
                        Ok(()) => self.catalog.status = format!("Selected skin: {name}"),
                        Err(error) => self.catalog.status = error,
                    }
                }
            }
            Some(Action::DefaultSkin) => {
                let profile = self.catalog.skin_library.active_profile.clone();
                match self.catalog.skin_library.select_skin(&profile, None)
                    .and_then(|()| self.catalog.save_skin_library(&self.home))
                {
                    Ok(()) => self.catalog.status = "Minecraft default skin selected.".into(),
                    Err(error) => self.catalog.status = error,
                }
            }
            Some(Action::ToggleSkinLayer(bit)) => {
                self.skin_preview_hidden_parts ^= bit;
                let part = match bit {
                    64 => "hat",
                    2 => "jacket",
                    4 => "left sleeve",
                    8 => "right sleeve",
                    16 => "left trousers",
                    32 => "right trousers",
                    _ => "skin layer",
                };
                self.catalog.status = format!(
                    "{} {part} in the 3D preview.",
                    if self.skin_preview_hidden_parts & bit == 0 { "Showing" } else { "Hiding" }
                );
            }
            Some(Action::ToggleSkinModel) => {
                let model = match self.catalog.skin_library.active_model().unwrap_or_default() {
                    crate::catalog::skin_library::SkinModel::Classic => crate::catalog::skin_library::SkinModel::Slim,
                    crate::catalog::skin_library::SkinModel::Slim => crate::catalog::skin_library::SkinModel::Classic,
                };
                match self.catalog.skin_library.set_active_model(model)
                    .and_then(|()| self.catalog.save_skin_library(&self.home))
                {
                    Ok(()) => self.catalog.status = format!("Skin model set to {:?}.", model),
                    Err(error) => self.catalog.status = error,
                }
            }
            Some(Action::NextAppearanceProfile) => {
                let profiles = &self.catalog.skin_library.profiles;
                if !profiles.is_empty() {
                    let current = profiles.iter().position(|p| p.id == self.catalog.skin_library.active_profile).unwrap_or(0);
                    let next = profiles[(current + 1) % profiles.len()].id.clone();
                    match self.catalog.skin_library.set_active_profile(&next)
                        .and_then(|()| self.catalog.save_skin_library(&self.home))
                    {
                        Ok(()) => self.catalog.status = format!("Active appearance profile: {next}"),
                        Err(error) => self.catalog.status = error,
                    }
                }
            }
            Some(Action::NewAppearanceProfile) => {
                let mut index = self.catalog.skin_library.profiles.len() + 1;
                while self.catalog.skin_library.profiles.iter().any(|p| p.id == format!("appearance-{index}")) {
                    index += 1;
                }
                let id = format!("appearance-{index}");
                let name = format!("Appearance {index}");
                match self.catalog.skin_library.add_profile(id.clone(), name.clone())
                    .and_then(|()| self.catalog.skin_library.set_active_profile(&id))
                    .and_then(|()| self.catalog.save_skin_library(&self.home))
                {
                    Ok(()) => {
                        self.catalog.scroll = 0.0;
                        self.catalog.status = format!("Created appearance profile: {name}");
                    }
                    Err(error) => self.catalog.status = error,
                }
            }
            Some(Action::Play) => self.action(events)?,
            Some(Action::Search) => {
                crate::platform::hide();
                self.catalog.offset = 0;
                self.catalog.search(&self.home);
            }
            Some(Action::FinishNameEdit) => {
                self.catalog.editing = None;
                crate::platform::hide();
                self.catalog.config.save(&self.home)?;
            }
            Some(Action::Field(name)) => {
                if name == "render_scale" {
                    self.catalog.render_scale_text =
                        self.catalog.config.render_scale.percent().to_string();
                    self.chooser = None;
                }
                self.catalog.editing = Some(name);
                let initial = match name {
                    "name" => &self.catalog.config.name,
                    "server" => &self.catalog.config.server,
                    "render_scale" => &self.catalog.render_scale_text,
                    "client_id" => &self.catalog.config.microsoft_client_id,
                    _ => &self.catalog.query,
                };
                crate::platform::show(initial);
            }
            Some(Action::Install(i)) => self.catalog.install(&self.home, i),
            Some(Action::Next) => {
                self.catalog.offset += 12;
                self.catalog.search(&self.home);
            }
            Some(Action::Previous) => {
                self.catalog.offset = self.catalog.offset.saturating_sub(12);
                self.catalog.search(&self.home);
            }
            Some(Action::Profile) => self.open_chooser(chooser::Kind::Profile),
            Some(Action::Renderer) => self.open_chooser(chooser::Kind::Renderer),
            Some(Action::RenderScale) => self.open_chooser(chooser::Kind::RenderScale),
            Some(Action::ChooseRenderScale(scale)) => {
                self.catalog.config.render_scale = scale;
                self.catalog.config.save(&self.home)?;
                self.chooser = None;
                crate::display::set_scale(scale);
            }
            Some(Action::Orientation) => self.open_chooser(chooser::Kind::Orientation),
            Some(Action::InputHost) => self.open_chooser(chooser::Kind::InputHost),
            Some(Action::ChooseInputHost(mode)) => {
                self.catalog.config.input_host = mode;
                self.catalog.config.save(&self.home)?;
                self.chooser = None;
                crate::configure_input_mode(mode);
            }
            Some(Action::ChooseOrientation(mode)) => {
                self.catalog.config.orientation = mode;
                self.catalog.config.save(&self.home)?;
                self.chooser = None;
                crate::platform::set_orientation(mode);
            }
            Some(Action::ChooseRenderer(mode)) => {
                self.catalog.config.renderer = mode;
                self.catalog.config.save(&self.home)?;
                self.chooser = None;
            }
            Some(Action::ChooseProfile(profile)) => {
                std::fs::create_dir_all(if profile == 0 {
                    self.home.clone()
                } else {
                    self.home
                        .join("profiles")
                        .join(format!("profile-{profile}"))
                })?;
                self.catalog.config.profile = profile;
                self.catalog.config.save(&self.home)?;
                self.catalog.refresh(&self.home);
                self.chooser = None;
            }
            Some(Action::Dismiss) => {
                self.exit_confirmation = false;
                self.chooser = None;
                // A cancelled cape-delete confirmation must never leave a stale
                // destructive target waiting for a later chooser action.
                self.catalog.pending_cape_delete = None;
                if self.session_worker.is_none() {
                    self.session_ready = false;
                    self.prepared_session = None;
                }
            }
            Some(Action::Logs) => {
                self.catalog.status = crate::diagnostics::summary(&self.home);
                if let Ok(error) = std::fs::read_to_string(self.home.join("android-error.txt")) {
                    self.catalog.status = format!(
                        "Last handled error: {}. {}",
                        crate::diagnostics::redact(&error),
                        self.catalog.status
                    );
                }
            }
            Some(Action::ExportDiagnostics) if self.diagnostic_export.is_none() => {
                let home = self.home.clone();
                let (sender, receiver) = mpsc::channel();
                self.diagnostic_export = Some(receiver);
                self.catalog.status =
                    "Preparing local diagnostic ZIP. No upload; review before sharing.".into();
                std::thread::spawn(move || {
                    let result = crate::diagnostics::export(&home).map(|p| {
                        p.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned()
                    });
                    let _ = sender.send(result);
                });
            }
            Some(Action::ExportDiagnostics) => {}
            Some(Action::RecordPerformance) => {
                self.catalog.config.record_performance = !self.catalog.config.record_performance;
                if !self.catalog.config.record_performance {
                    crate::performance::stop();
                }
                self.catalog.config.save(&self.home)?;
                self.catalog.status = if self.catalog.config.record_performance {
                    "Recording starts on Play. Detailed timings add overhead. Return here to save the report."
                } else { "Performance recording off." }.into();
            }
            Some(Action::ExportPerformance) => {
                self.catalog.status = match crate::performance::export(&self.home) {
                    Ok(()) => "Reports opened in Files. Copy or share a report from there.".into(),
                    Err(e) => e,
                };
            }
            None => {}
        }
        Ok(())
    }
    pub(super) fn ui_density(&self) -> f32 {
        let (Some(r), Some(gui)) = (&self.renderer, &self.gui) else {
            return 1.0;
        };
        let mut batch = HudBatch::new([r.config.width, r.config.height]);
        Ui::mobile(&gui.atlas, &mut batch, (0.0, 0.0)).density
    }

    pub(super) fn touch_event(&mut self, events: &ActiveEventLoop, t: Touch) -> Result<()> {
        self.focus = None;
        self.pending_focus = None;
        if let Some(window) = &self.window {
            window.request_redraw();
        }
        self.cursor = (t.location.x, t.location.y);
        let density = self.ui_density().max(0.01) as f64;
        match t.phase {
            TouchPhase::Started => {
                let x = t.location.x / density;
                let y = t.location.y / density;
                let in_preview = self.skin_preview_rect.is_some_and(|[left, top, right, bottom]| {
                    x >= left as f64 && x <= right as f64
                        && y >= top as f64 && y <= bottom as f64
                }) && self.skin_preview.is_some();
                if in_preview {
                    self.skin_preview_touch = Some((t.id, t.location.x, t.location.y));
                    self.drag = None;
                } else if self.skin_preview_touch.is_none() {
                    self.drag = Some(super::gesture::Drag::new(t.location.x, t.location.y));
                }
            }
            TouchPhase::Moved => {
                if let Some((touch_id, previous_x, previous_y)) = self.skin_preview_touch {
                    if touch_id == t.id {
                        let dx = ((t.location.x - previous_x) / density) as f32;
                        let dy = ((t.location.y - previous_y) / density) as f32;
                        self.skin_preview_rotation[0] =
                            (self.skin_preview_rotation[0] + dx * 0.8).rem_euclid(360.0);
                        self.skin_preview_rotation[1] =
                            (self.skin_preview_rotation[1] + dy * 0.55).clamp(-35.0, 35.0);
                        self.skin_preview_touch = Some((touch_id, t.location.x, t.location.y));
                    }
                } else if let Some(drag) = &mut self.drag {
                    let delta = drag.update(t.location.x, t.location.y, density);
                    if drag.moved
                        && self.list_view.is_some_and(|view| {
                            super::gesture::contains_origin(
                                view,
                                drag.origin_x() / density,
                                drag.origin_y() / density,
                            )
                        })
                    {
                        if let Some(chooser) = &mut self.chooser {
                            chooser.scroll = (chooser.scroll + delta).max(0.0);
                        } else {
                            self.catalog.scroll = (self.catalog.scroll + delta).max(0.0);
                        }
                    }
                }
            }
            TouchPhase::Ended => {
                if self.skin_preview_touch.is_some_and(|(touch_id, _, _)| touch_id == t.id) {
                    self.skin_preview_touch = None;
                } else if self.drag.take().is_some_and(|drag| !drag.moved) {
                    self.activate(events)?;
                }
            }
            TouchPhase::Cancelled => {
                if self.skin_preview_touch.is_some_and(|(touch_id, _, _)| touch_id == t.id) {
                    self.skin_preview_touch = None;
                }
                self.drag = None;
            }
        }
        Ok(())
    }
    pub(super) fn type_key(&mut self, key: KeyEvent) {
        if key.physical_key == PhysicalKey::Code(KeyCode::Escape)
            || key.logical_key
                == winit::keyboard::Key::Named(winit::keyboard::NamedKey::BrowserBack)
        {
            if !key.repeat {
                self.request_back();
            }
            return;
        }
        if self.catalog.editing.is_none() {
            return;
        }
        let out = match key.physical_key {
            PhysicalKey::Code(KeyCode::Backspace) => Some(Output::Backspace),
            PhysicalKey::Code(KeyCode::Enter) => {
                crate::platform::hide();
                Some(Output::Enter)
            }
            _ => key.text.map(|s| Output::Text(s.into())),
        };
        if let Some(out) = out {
            self.catalog.edit(out, &self.home);
        }
    }
}


#[cfg(test)]
mod launcher_layout_tests {
    use super::*;

    #[test]
    fn expanded_phone_navigation_uses_two_rows_above_content() {
        let layout = LauncherLayout::new(393.0, 820.0);
        assert_eq!(layout.navigation_mode, NavigationMode::Expanded);
        assert!(layout.tab_rect(4)[1] > layout.tab_rect(0)[1]);
        assert!(layout.content_rect[1] >= layout.nav_rect[1] + layout.nav_rect[3]);
    }

    #[test]
    fn wide_layout_places_sidebar_beside_the_content_area() {
        let layout = LauncherLayout::new(1280.0, 800.0);
        assert_eq!(layout.navigation_mode, NavigationMode::Sidebar);
        assert!(layout.nav_rect[0] + layout.nav_rect[2] < layout.content_rect[0]);
        assert!(layout.tab_rect(Page::ALL.len() - 1)[1] > layout.tab_rect(0)[1]);
    }

    #[test]
    fn narrow_or_short_viewports_keep_compact_navigation() {
        assert_eq!(
            LauncherLayout::new(360.0, 800.0).navigation_mode,
            NavigationMode::Compact
        );
        assert_eq!(
            LauncherLayout::new(393.0, 460.0).navigation_mode,
            NavigationMode::Compact
        );
    }

    #[test]
    fn tall_phone_cape_preview_reserves_two_library_rows() {
        let shell = LauncherLayout::new(393.0, 820.0);
        let content_bottom = shell.content_rect[1] + shell.content_rect[3];
        let studio_bottom = content_bottom - 10.0;
        let preview_top = 340.0 + 42.0;
        let preview_height = stacked_cape_preview_height(studio_bottom, preview_top, 92.0);
        let list_top = preview_top + preview_height + 8.0;
        assert_eq!(preview_height, 228.0);
        assert_eq!(studio_bottom - list_top, 92.0);
    }

    #[test]
    fn short_phone_cape_split_keeps_preview_and_controls_inside_viewport() {
        for (width, height) in [(393.0, 460.0), (360.0, 460.0), (393.0, 600.0)] {
            let shell = LauncherLayout::new(width, height);
            let content_bottom = shell.content_rect[1] + shell.content_rect[3];
            let studio_bottom = content_bottom - 10.0;
            let content_width = shell.content_rect[2] - 24.0;
            let preview_width = content_width * 0.42;
            let control_width = content_width - preview_width - 12.0;
            let title_y = shell.content_rect[1] + 2.0;
            let preview_top = title_y + 52.0;
            let preview_height = compact_cape_preview_height(studio_bottom, preview_top, preview_width);
            let list_top = title_y + 114.0 + 20.0;
            let delete_width = 38.0;
            let rename_width = 44.0;
            let gap = 3.0;
            let select_width = (control_width - delete_width - rename_width - gap * 2.0).max(64.0);

            assert!(preview_top >= shell.content_rect[1]);
            assert!(preview_top + preview_height <= studio_bottom + 0.1);
            assert!(studio_bottom - list_top >= 44.0);
            assert!(select_width + rename_width + delete_width + gap * 2.0 <= control_width + 0.1);
        }
    }

    #[test]
    fn sidebar_catalog_controls_do_not_overlap_results() {
        let shell = LauncherLayout::new(1536.0, 691.0);
        let page = CatalogPageLayout::for_launcher(&shell);
        assert_eq!(shell.navigation_mode, NavigationMode::Sidebar);
        assert!(page.source_y + 32.0 <= page.search_y);
        assert!(page.search_y + 44.0 <= page.folder_y);
        assert!(page.folder_y + 38.0 <= page.hint_y);
        assert!(page.hint_y + 18.0 < page.list_top);
        assert!(page.list_top < shell.content_rect[3]);
    }

    #[test]
    fn compact_catalog_geometry_preserves_existing_offsets() {
        let shell = LauncherLayout::new(393.0, 820.0);
        let page = CatalogPageLayout::for_launcher(&shell);
        assert_eq!(shell.navigation_mode, NavigationMode::Expanded);
        assert_eq!(page.title_y, 142.0);
        assert_eq!(page.search_y, 174.0);
        assert_eq!(page.folder_y, 222.0);
        assert_eq!(page.hint_y, 270.0);
        assert_eq!(page.list_top, shell.content_rect[1] + 152.0);
    }
}
