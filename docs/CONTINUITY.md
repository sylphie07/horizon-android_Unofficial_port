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


## Catalog page shared content geometry — stage 4 (2026-11-03)
- Catalog title, search field/button, source selectors, hints, folder/import buttons, result text/actions and pagination now derive horizontal positions from `LauncherLayout.content_rect`.
- Catalog result action hitboxes use the same derived action column as visible controls. Scroll viewport top/bottom and pagination Y also derive from the shared rectangle.
- Derived offsets were corrected to preserve the previous numerical spacing; this stage centralizes geometry rather than changing the visual layout.
- Code commits: `62de6b668537b6face24c9c7f7fe8e9b11ac17a0`, `d8ed8bbb89d50e4fe8ff51953c4525931f5b6767`, `ebf780dc119cee3aa23ffd35352668f4e21b7655`.
- Static source inspection confirms old catalog hard-coded position/width anchors were replaced. No formatter, compiler, tests, CI, APK or device test has run.
- Next: migrate Skin Studio and Settings, then audit dialogs, scrollbars, focus and touch gestures before implementing wide-screen sidebar/adaptive phone navigation.
- Development log updated in commit `12cb1b6c9187cfda04a2dede50cb6e8de2212397`.


## Skin Studio shared content geometry — stage 5 (2026-11-03)
- Migrated Skin Studio control-column positions/widths, profile and selection labels, profile/import/model/default buttons, preview panel edges, and skin-list row rectangles to `LauncherLayout.content_rect`.
- Skin-list row drawing and `list_targets` share the same rectangle. Its lower viewport edge is now derived from the shared content bottom while preserving the prior compact/non-compact endpoints.
- Code commits: `bb356f49be3c57328c0c03c449bfc5863cfda14a` and `24aba85afd45d518743ef74264cb7d3d5f1c4129`.
- Static source fetch confirmed the migrated geometry and matching skin row hitboxes. No formatter, compiler, tests, CI, APK or device test has run.
- Next: migrate Settings page content/list targets/viewport; then audit About, dialogs, scrollbars and input gestures/focus before activating adaptive navigation.
- Development log updated in commit `a4878abb98e38284b48ae7e6ab378eb007fcda1b`.


## Settings shared geometry — stage 6 (2026-11-03)
- Migrated Settings title, row positions, list viewport/scroll clamp, row hitboxes, toggle/button rectangles, dividers and bottom Diagnostics/Account actions to `LauncherLayout.content_rect`.
- The compact and wide-screen formulas preserve the previous coordinates and dimensions at the existing layout sizes; drawing and input targets use the same derived bounds.
- Code commit: `1a2d0764d7e5a74b254e9a3464c0ff9afd8dcbeb`.
- Android source checks #172, run `41580868881`, completed successfully on that code commit. Diff check, public-tree history, Android Python unit tests, diagnostics, performance, content and workflow-export consistency all passed.
- The workflow has no native ABI jobs on push; Rust formatting/compile, native matrix, APK and device validation remain unverified for this stage.
- Next: migrate About content/list/actions and Play hero content to the shared rectangle; then align footer/status and scrollbar bounds, audit touch drag origins/gamepad focus and modal behaviour before enabling sidebar/adaptive navigation.


## About, Play and footer shared geometry — stages 7–8 (2026-11-03)
- Migrated About title, body width, scroll viewport and Back/Sylphra GitHub actions to `LauncherLayout.content_rect`.
- Migrated Play hero panel, accent rail, content origin, progress bar, progress text and Play button width/position to shared content bounds.
- Migrated global status text and Horizon GitHub button to the content rectangle so the footer can follow a future sidebar layout.
- Code commits: `9c04410b6c17e7afe3860cd2889b7ff124573dee` and scope correction `b1980e9c8803ca5bb3caeda942158e05aa2a884a`.
- Static review caught a footer-bound variable declared inside a conditional but used after it; moved the bounds into the enclosing draw scope in `b1980e9`. No compiler failure is claimed; the initial run #174 was cancelled by overlapping push delivery. Run #175 (`41581418514`) passed source checks on the preceding `9c04410` commit, not the corrected HEAD.
- Current branch HEAD `b1980e9` requires a fresh source run. Native Rust/ABI, APK and device validation remain unverified.
- Next: make list viewport hit regions X-aware, then audit modal/focus/scrollbar/touch behaviour before enabling the phone and sidebar navigation modes.


## Responsive navigation and bounded scrolling — stages 9–10 (2026-11-03)

- List viewports now carry left/top/right/bottom/maximum bounds. Touch drags and mouse-wheel scrolling only scroll when the gesture/pointer starts inside the active list viewport. Scrollbar tracks follow the viewport's right edge.
- Chooser viewport bounds now include its horizontal range. Gamepad focus reveal uses the updated viewport shape and keeps its vertical reveal/clamp behaviour.
- Added geometry modes in `LauncherLayout`: compact single-row fallback for narrow/short viewports; two-row phone/tablet navigation for width >= 380 and height >= 500; Java-style left sidebar for width >= 900 and height >= 480.
- The seven destinations keep their existing order and actions. The exact same tab rectangles are used to draw buttons and register input targets. Expanded phone mode places the account control in the header row and uses a 4+3 tab grid. Sidebar mode shifts the page content and footer into the right-hand content rectangle.
- Code commits: bounded viewport implementation `45493c0a6fad782ff93c225ea9a8dde5542ad728`; responsive navigation and wheel bounds `a784adcc24727cc45160e2b0a97e2c9c702f98c9`; sidebar spacing/test assertion correction `f629757e008003561d14704c4aae25ae123c5be3`.
- Source CI #185 (`41583666042`) passed on `f629757e008003561d14704c4aae25ae123c5be3`: diff check, public-tree history, Android Python unit tests, diagnostics, performance, content and workflow-export consistency.
- **Not verified:** Rust formatting/compilation, four-ABI native checks, APK build/install, or visual/touch behaviour on a physical device. The native matrix requires manually dispatching Android source checks with `native_checks=true`.
- Next: synchronize this Horizon tip into Sylphra's authorized foundation branch, then obtain the native matrix and physical-device review before declaring the design phase fully verified. Background image/GIF/video customization remains deferred.


## Native validation completed — responsive navigation (2026-11-03)
- Manual Android source checks run #188, ID `41584912109`, completed with overall conclusion `success` on branch `android`, HEAD `b22e97b2ca0b3e16537e57efafc04a513abe4851`.
- Verified jobs: `source`, `native (arm64-v8a)`, `native (armeabi-v7a)`, `native (x86)`, and `native (x86_64)`; every job and its configured steps completed successfully, including Android SDK/NDK setup, Rust Android code checks, source tests, diagnostics, performance, content and workflow-export consistency.
- This run validates native Rust code checks for all four configured ABIs. It explicitly does **not** build an APK. No installation, manifest inspection, visual/touch/gamepad/orientation device test, or release signing verification is claimed.
- Test-only APK workflow remains `.github/workflows/sylphra-test-apk.yml`, manually dispatched on `android`; it checks out this branch, builds arm64 with isolated ID `dev.sylphra.android.test`, label `Sylphra Launcher`, and uploads a seven-day artifact without creating a GitHub Release.
- Next: run the isolated test APK workflow, audit the produced artifact and package metadata, then request physical-device acceptance. Keep background image/GIF/video customization deferred until the base design is accepted.


## Current work: Skin Studio (2026-11-03)

- Active code branch: `android` in `sylphie07/horizon-android_Unofficial_port`.
- Current applied code tip before documentation update: `3e7f11027fccd6f60548e9cab31b7422466c3d05`.
- Responsive preview layout and dynamic model scale are applied (commits `170326f09ca0e19641925fb256a9c9ce25fad1e6`, `7218728c11abbb39b977c7e1149a830a382deae5`).
- Cumulative touch rotation is now applied (commits `3404878f09e19f66e11d06a592c4bfacfc9d6f1b`, `273003a9b2141b9a89d6e8b26af18081c7af33cd`, `3e7f11027fccd6f60548e9cab31b7422466c3d05`). A touch beginning inside the preview rotates the mannequin; a touch beginning outside it retains list scrolling. Yaw accumulates and pitch is bounded.
- Not yet verified: rustfmt, Rust compilation, unit tests, four-ABI native CI, test APK, or real-device layout/touch behavior. Wait for CI before requesting a new APK.
- Real skin-layer toggles remain unimplemented. Investigate the baked player model's child-part tree and implement preview-only visibility for hat/jacket/sleeves/trousers; avoid decorative controls and avoid changing normal gameplay rendering.
- After code and native checks pass, synchronize the verified Horizon gitlink into Sylphra's authorized branch `sylphra/horizon-foundation`. Then build the isolated test APK and request device acceptance. Do not publish a Release.
- Keep the optional launcher background image/GIF/video feature deferred until the Skin Studio base design is accepted.
