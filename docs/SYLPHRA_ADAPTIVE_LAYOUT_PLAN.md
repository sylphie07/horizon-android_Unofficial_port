# Sylphra adaptive launcher layout — implementation plan

## Purpose
Complete the Java-launcher-inspired navigation redesign without breaking the current Android touch, mouse, keyboard, or gamepad interaction.

## Preflight snapshot (2026-11-02 session)
- Repository: `sylphie07/horizon-android_Unofficial_port`
- Authorized branch: `android` only.
- HEAD before this documentation update: `22847b254ba4ea9d02de0c334480075d83cbd935`.
- Only branch returned by the branch search: `android`.
- Read `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `catalog.rs`, `launcher_pages.rs`, `launcher.rs`, and `ui_theme.rs`.
- `docs/PROJECT_CONTEXT.md` is absent at the checked path (404); continuity and development log are the active handoff records.

## Renderer findings
- `Launcher::draw_pages()` constructs one `Ui::mobile` and clamps its logical width/height for Android insets and the IME viewport.
- The header, account control, seven-tab row, and page content are laid out with direct coordinates in `launcher_pages.rs`.
- The tab buttons append rectangles to `self.targets`; list controls also populate `self.list_targets` and `self.list_view`.
- Gamepad focus and touch activation consume those stored rectangles. Moving only the drawn buttons would desynchronize visible controls from hit targets.
- Page-specific content is not one shared panel: Play, catalog pages, Installed, Skin Studio, Settings, dialogs, and scrollbars each use their own absolute coordinates. Several controls are anchored to the right or bottom edge.
- `ui_theme.rs` documents the shared UI layer and is not an appropriate place for Sylphra-only navigation styling. Keep the work scoped to `launcher_pages.rs` unless a narrowly justified helper is needed.

## Safe implementation sequence
1. Introduce a launcher-only layout model that computes viewport class, header bounds, navigation bounds, content origin/width/height, and bottom action bounds from the already inset `Ui` dimensions.
2. First migrate existing horizontal layout to consume that model with no visual or interaction change. Ensure every drawn control and every target/list viewport uses the same geometry.
3. Add a spacious phone/tablet navigation treatment while retaining a compact phone fallback.
4. Add the wide-screen Java-style sidebar only after page content can consume a content rectangle rather than assuming the full viewport. Migrate page families incrementally and keep right/bottom anchored controls inside the content rectangle.
5. Audit dialogs, scrollbars, list reveal, keyboard/controller focus, and touch drag origins at each stage.
6. Run formatting, source checks and the four-ABI native matrix. Only after they pass may the isolated test APK workflow be considered; visual behaviour still requires the user's Android device test.

## Current status
- Completed: navigation priority, compact labels, framed horizontal navigation surface.
- Investigation complete: coordinate and hit-test dependencies mapped at the top-level and across page families.
- Not implemented: adaptive layout model, expanded phone navigation, wide-screen sidebar.
- No source files changed during this audit. No formatter, compiler, unit test, CI, APK build, install, or device test was run.
- Future background customization (static image, GIF, video) is a separate later feature and is intentionally out of scope for this navigation phase.

## Guardrails
- Work only on branch `android`; never modify `main` or merge PRs.
- Preserve page actions, game/engine behaviour, authentication, profiles, worlds, storage, catalog source IDs, and shared theme tokens.
- Keep Sylphra-specific visual styling in `crates/horizon-client/src/android/launcher_pages.rs`.
- Record proposals, applied changes, and verified results separately.

## Implementation progress update — 2026-11-03

The original preflight status above is historical. Current progress:
- **Applied:** shared launcher geometry model and migration stages 1–6: shell/header/navigation bounds, Installed viewport and horizontal anchors, catalog geometry, Skin Studio geometry, and Settings geometry.
- **Verified by source inspection:** current page drawing and corresponding list/action targets consume the shared content rectangle in the migrated page families.
- **CI:** source run #172 (`41580868881`) passed on code commit `1a2d0764d7e5a74b254e9a3464c0ff9afd8dcbeb`.
- **Not yet done:** About and Play page migration; footer/status and scrollbar content bounds; x-aware list drag regions; final focus/controller/modal audit; expanded phone navigation and wide-screen sidebar.
- **Not verified:** current geometry stages have no new native four-ABI run, APK or physical-device visual/input validation. Do not activate adaptive navigation until these gates are complete.
- Background image/GIF/video customization remains deferred until navigation redesign is finished.
