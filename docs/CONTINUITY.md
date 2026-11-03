# Sylphra Android — continuity

## Repository state
- Repository: `sylphie07/horizon-android_Unofficial_port`
- Authorized development branch: `android` only.
- Do not modify `main`, merge PRs, publish a Release, or publish to an app store unless the user explicitly authorizes it.
- User workflow: Android/Termux/GitHub Actions; no desktop PC assumed.
- Keep implementation and verification claims separate. A commit or source inspection is not a passing build or device test.

## Current project direction
- Sylphra is the Android-focused branded fork of Horizon.
- Preserve upstream attribution and the original Horizon repository/provider references.
- Keep Sylphra-specific launcher styling inside `crates/horizon-client/src/android/launcher_pages.rs`; do not recolor shared theme tokens or alter game overlays globally.
- Do not blindly rename Horizon internals, engine/protocol identifiers, storage paths, profile/world data, authentication, or catalog source IDs.
- Skin Studio and local skin runtime work must preserve account identity, UUIDs, worlds, server protocol, and remote-player textures.

## Latest applied work
- Play page responsive pass: code commit `4fba2f5335730e049ddecd561a6d10915b2dc948`.
- Settings compact-layout pass: code commit `d19f7c8fcf71676777b618261d965f6770f4c10c`.
- Main navigation includes **Play**, **Installations**, **Skins**, **Mods**, **Packs**, **Shaders**, and **Settings**, in that priority order. Compact phone labels include **Files**, **Skins**, **FX**, and **Setup** where applicable. Latest navigation commits: `ccb89774e6db8192c2420ed2d5525598fa7ca5f7` and `ddb17228df9c194bd7317be96bcf442c09490654`.
- Detailed chronological record: `docs/DEVELOPMENT_LOG.md`.
- The Skins navigation entry only exposes existing Skin Studio functionality; it does not add new skin runtime or multiplayer-sharing behaviour.
- Navigation now has a distinct raised/bordered surface behind the existing seven tabs; page hitboxes and content coordinates were deliberately preserved. Source commit: `995cb7f6967435a88d5c3edd50cdce055b6720ef`.
- The renderer still uses a horizontal tab row. A Java-style wide-screen sidebar and a more spacious phone navigation pattern are not implemented yet; those require a coordinated content-layout and hit-testing refactor.

## Adaptive navigation layout
- Audit plan: `docs/SYLPHRA_ADAPTIVE_LAYOUT_PLAN.md`.
- Current horizontal navigation and page layouts use direct coordinates; navigation targets, list targets, scrolling viewports, and gamepad focus share those rectangles. A sidebar-only move would break alignment or overlap page content.
- Next implementation stage: introduce a launcher-only geometry model and migrate existing layout without visual change before enabling expanded phone navigation or a wide-screen sidebar.
- No source code changed during the 2026-11-02 audit. No formatter, compile, tests, CI, APK, or device validation was performed for that audit.
- Background customization (image/GIF/video) is deferred until navigation redesign is complete.

## Verification state
- Latest source commits for the navigation priority pass: `ccb89774e6db8192c2420ed2d5525598fa7ca5f7` (page order) and `ddb17228df9c194bd7317be96bcf442c09490654` (compact Shaders label).
- Documentation commits for this pass: `320a8cc1ecda223f575512054cf622f20da212af` (development log) and the current continuity update follows.
- Source files were fetched again from branch `android`; the seven-page order and compact `FX` label are present.
- No CI status checks were attached to the current documentation HEAD when queried. No formatter, Rust compile, unit test, APK build, install, or device verification is claimed for this navigation pass.
- Required next validation: manually run **Android source checks** on branch `android` with `native_checks=true`; inspect `source`, `armeabi-v7a`, `arm64-v8a`, `x86`, and `x86_64` jobs.
- The earlier run `41523801686` is historical and does not validate these latest navigation commits.
 
## APK workflow
- Test-only workflow: `.github/workflows/sylphra-test-apk.yml`.
- Manual dispatch on branch `android` only, arm64-v8a only.
- Isolated application ID: `dev.sylphra.android.test`; label: `Sylphra Launcher`.
- Artifact: `sylphra-launcher-android-public.apk`, seven-day retention.
- No GitHub Release or store publication.
- Do not dispatch the APK build until source and native checks for the latest code are green.

## Session closeout procedure
1. Recheck branch HEAD and workflow results.
2. Update `docs/DEVELOPMENT_LOG.md` with exact commits, changed files, actual tests/results, errors and corrections.
3. Update this file with exact current state and pending steps.
4. Never mark an unrun formatter, test, build, CI job, APK install, or device behaviour as verified.


## Shared launcher geometry — stage 1
- Implemented a launcher-only `LauncherLayout` in `launcher_pages.rs` for title, account button, navigation surface, tab bounds, and Play hero content top.
- Header, account, navigation surface and border, tab buttons, and Play hero panel now use the shared geometry. Existing horizontal coordinates are represented; adaptive sidebar is not yet active.
- Source commits: `72b99bcd8920e8a7cf221ab05e0e3f91526aa1c8` and `36891d928fcf82917849a1a29a8e650793f237b0`.
- Static source fetch confirmed the geometry consumers. No formatter, compile, tests, CI, APK, install, or device test has run for this stage.
- Next: migrate page families and their hitboxes/list viewports/anchors to shared content bounds before enabling a sidebar.


## Shared Installed-page viewport geometry — stage 2 (2026-11-03)
- Added `content_rect` to the launcher-local `LauncherLayout`, based on the post-inset UI dimensions.
- The Installed page now derives its scroll viewport top/bottom from that rectangle. Scroll clamping, `list_view`, visible-row checks, and list hitbox Y positions still share the same bounds.
- Current visual coordinates and horizontal anchors are intentionally unchanged. No sidebar or expanded navigation is active.
- Code commits: `67bbd8015b558441f456b654a796472e6f45aa85` and `29134294339fdefd810df7503df268f1aa9ddea2`.
- Static fetch verified the geometry fields and Installed viewport consumers. No formatter, compile, unit tests, CI, APK, install, or device test has run for this stage.
- Next: migrate Installed horizontal anchors with drawing and hitboxes together, then migrate catalog page geometry. Do not enable the sidebar before page-family input and scroll migration plus successful CI.
- Latest development log entry is in `docs/DEVELOPMENT_LOG.md`; documentation commit follows this continuity update.


## Installed page horizontal anchors — stage 3 (2026-11-03)
- Migrated Installed-page horizontal layout values to `LauncherLayout.content_rect`: left/right content edges, import/remove column, section width, enable/disable column and item title width.
- Both visible controls and `list_targets` derive their rectangles from the same values. Existing dimensions are preserved at the current viewport width.
- Code commit: `a6d661a987459ae9114c123490098c29ecf26f30`.
- Static source fetch confirms old hard-coded horizontal anchors are absent from the Installed page block and shared geometry is used. No formatter, compile, tests, CI, APK or device verification has run.
- Next: migrate catalog page content bounds and ensure row drawing, action targets, pagination and scroll viewport remain aligned; then migrate Skin Studio and Settings before enabling adaptive navigation.
- Development log entry: `docs/DEVELOPMENT_LOG.md` updated in commit `92fdcfcd01054e784277147d70ea7803bd62c53c`.
