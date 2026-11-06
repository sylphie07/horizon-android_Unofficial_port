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
- Responsive preview layout and dynamic model scale are applied (commits `170326f09ca0e19641925fb256a9c9ce25fad1e6`, `7218728c11abbb39b977c7e1149a830a382deae5`).
- Cumulative touch rotation is applied. Touches beginning inside the 3D preview rotate the mannequin; touches beginning outside retain list scrolling. Yaw accumulates and pitch is clamped. See code commits `3404878f09e19f66e11d06a592c4bfacfc9d6f1b`, `273003a9b2141b9a89d6e8b26af18081c7af33cd`, `3e7f11027fccd6f60548e9cab31b7422466c3d05`.
- Functional layer controls are now applied: Hat, Jacket, Left sleeve, Right sleeve, Left pants and Right pants. They toggle the existing vanilla player model-part visibility bits on the preview mob clone only. Gameplay rendering is not changed. Commits: `ea74fc3299088746d13f458a900d3e6802a80e7b`, `28dfe575cbab230afb54834571cb6179a4a4080f`, `3db7cd5ce8f1c30ab58078d888c45b427a4f8820`, with follow-up correctness fixes `c65d5f3fe6b20c4928e8e01a720e325c51e9212e`, `1d1baca8f6af2007dc498e105b3a71e13132f724`, `b1b8f0918902e5e525a9f1f89f8f5ad01d7e2d43`.
- Technical basis: `MobExtra.hidden_model_parts` and `mobs/anim.rs::player_model_visibility` already implement the vanilla bit mapping for jacket, sleeves, pants and hat. The new controls reuse that behavior without touching the in-game player.
- Not yet verified: rustfmt, Rust compilation, unit tests, four-ABI native CI, APK build or physical-device layout/touch/layer behavior. Multiple push CI runs were cancelled due to successive commits; inspect a fresh run on the final documented tip before proceeding.
- Next: run/inspect source CI on the final tip, then manually dispatch native checks with `native_checks=true`. Fix any failures before building the isolated test APK. Do not publish a Release.
- After successful validation, synchronize the verified Horizon gitlink into Sylphra's authorized branch `sylphra/horizon-foundation`; request device acceptance before declaring Skin Studio complete.
- Keep optional launcher background image/GIF/video customization deferred until the base design is accepted.


## Current task — Skin Studio overlap correction (2026-11-03)
- Active branch: `android`; code commit `2325973ad65b53cfcb671ca10a703024b0545087`.
- Six layer controls moved to a dedicated strip under the preview. Skin list starts below that strip, fills remaining content height, uses compact rows and fitted labels, and only draws/adds hit targets for rows fully inside its viewport.
- The screenshot's original failure was confirmed in geometry: layer buttons were drawn at y≈default_y+56..+83 and +86..+113, while the list began at preview bottom +16; on wide layout those ranges intersected. New ordering removes that intersection.
- User also reports 3D player preview remains unavailable after several game launches. Current code tries `PlayerPreview::load(home/.local/horizon-client/VANILLA_VERSION)` only on entering Skin Studio. Verify `EntityModelDocument::load` expected root and actual installer output before changing lifecycle; avoid a per-frame disk-load loop.
- No APK or physical-device validation yet. Do not sync to Sylphra until source/native CI passes.


## Continuity — Skin Studio preview diagnostics (2026-11-03)
- Latest code commit at this stage: 98dbcb1bc8e5c3a9e845a748f19a4cb4f67b4d59 (preview diagnostic UI); preceding loader change: e61382bd0e6816b2b1ec0fefb332a305e7c1ddf9.
- The preview loader now returns the underlying entity-models.json read/parse error or missing player-model-key error, and the UI logs/displays it. It does not claim successful rendering. Failed loads are retryable on a later Skin Studio visit; successful models are cached.
- Skin list overlap fix is included on this branch: separate six-button layer strip and compact clipped list below it.
- Source workflow #214 (41698244902) was queued for the latest code commit; verify its final result before proceeding. Native ABI matrix, APK and device test remain unverified.
- Required next action: obtain the exact diagnostic from a test run and fix the root cause; do not introduce blind launch retries. Integrate the fork tip into Sylphra only after source/native checks and document the exact gitlink.


## Verification update — source CI green (2026-11-03)
- The latest documented branch tip `708f42cfcffc74804f680ad33291644550a13d24` completed Android source checks successfully in run #216 (`41698337604`). Every step in the source job passed, including diff check, public-tree history check, Android Python tests, diagnostics, performance, content and workflow-export parity. Native job was skipped because the workflow was not manually dispatched with `native_checks=true`.
- Runs #213 and #214 were cancelled by concurrency; they are not evidence of code failure. Run #216 checked out the latest branch tip, including both preview diagnostic code commits, so source-level validation applies to the documented tree.
- The preview is still not device-verified. Diagnostic code now reports exact file/model-key errors, but the actual cause and successful 3D rendering remain unconfirmed.
- Next required action: manually dispatch Android source checks on branch `android` with `native_checks=true`; after all four ABI checks pass, dispatch the isolated test APK workflow, audit its artifact, then request user device testing. Do not sync the Horizon submodule into Sylphra yet.


## Native CI failure and correction (2026-11-03)
- Manual native matrix run #219 (`41715591765`) failed on all four ABIs at the same Rust type mismatch. The source job passed. Root cause: `mobs::preview::player_type` now returns `crate::Result<Arc<MobType>>` to preserve diagnostics, while the existing gameplay camera wrapper in `game/perspective.rs` intentionally exposes `Option<Arc<MobType>>`.
- Applied compatibility-boundary fix in commit `88850c1afa60c386a63af38ec37caf78686834f9`: convert the fallible preview loader result with `.ok()` only in the gameplay wrapper. Skin Studio continues to receive and display the detailed error; existing gameplay callers retain their Option contract.
- No APK/device verification. Next: confirm source CI, rerun manual native matrix, then proceed to isolated test APK only if all ABIs pass.


## Native matrix passed (2026-11-03)
- Run #223 (`41717605149`) completed successfully on branch android, commit `9abc241e5f73b2ef9d12bef96018beb7ce76cc69`.
- Confirmed source job plus all four native jobs passed: x86/i686, arm64-v8a/aarch64, x86_64, and armeabi-v7a/armv7. This validates native compilation for the matrix on that commit, including the compatibility fix mapping the detailed preview Result to the gameplay Option boundary.
- Run #220 was cancelled by concurrency and is not the positive run. Run #223 is the authoritative successful verification.
- Next: inspect and dispatch the isolated test APK workflow on branch android, then audit the artifact before asking the user to install/test it. No device-level 3D preview verification yet; no Sylphra submodule update or public release.


## Test APK manifest mismatch — current continuation state (2026-11-03)
- The isolated APK workflow run #9 (`41720733718`) failed before APK compilation, at `Prepare pinned Minecraft-derived engine tables`. The live Mojang catalog served 26.3 JSON SHA-1 `80608ff6753705b6a82e2cb86a5937d4d233abd4`, which the engine correctly rejected as unrecognized.
- Applied correction in `tools/android_prepare.py`, commit `8cbd2ead693b8e03bcedad3e6ef251fbab39d88a`: download the immutable manifest package URL derived from `research-lock.json:minecraft.manifest_sha1` and verify against that pinned SHA-1 (`11f1aefe473eefba0af62e3286c33b692807b2eb`) instead of following the mutable live catalog entry.
- Development log records the exact failed run and correction in commit `1906443271fe3a430e655fc7ed3938f2a23fcf3c`.
- Current status: correction committed; not yet rerun or CI-verified. Branch remains `android`; no Sylphra submodule sync, APK artifact, install, or public release has occurred.
- Next: run the Android source checks to validate Python tests/workflow consistency, then manually dispatch `Sylphra Launcher test APK` on branch `android`. Inspect artifact/signature/manifest before asking for device validation.


## Skin Studio — preview enlargement and diagnostic persistence (2026-11-03)
- User installed the preceding test APK and confirmed the controls/list are better arranged, but the 3D preview remains too small and the generic model-not-loaded message persists. Screenshot: 1536×691; the prior wide preview height was fixed at 194 units and the renderer capped mannequin scale at 112 units.
- Applied code on authorized branch `android`: responsive preview height (wide: 42% of viewport, 260–340; stacked: 30%, 180–260), larger mannequin transform (up to 220 units, 72% of viewport height), and dedicated `skin_preview_error` state so the full load failure is not overwritten by ordinary catalog status updates.
- Code commits: `81d6dc29457b127de3af02b0d0fee8607091c4fb`, `e7a64827dfa56ce795eb732275884104614d5f55`, `6c4ac1d99932a5d6587d27d44b3a22cf1baa8c17`, `91aa7a24dac5cae75cdb455b88fe66881b435a1e`.
- Documentation log entry committed as `5e8257aba72563761e4ba21203489bca4ff9296b`.
- **Not yet verified:** formatting, Rust compile, tests, source CI, four-ABI native CI, APK build, or device behavior for these changes. The underlying player-model resource failure is still unknown.
- Next: run source checks and native matrix, build/audit the isolated arm64 test APK, then read the persistent diagnostic from the device and fix the actual resource path/load issue. No Sylphra submodule sync or Release publication.

- Follow-up scale-fit correction committed as `2534eb6f03f5487f470703b6850b03c83edf46ba`: model scale now uses 42% of preview height (rather than 72%) and 46% of preview width, with a 220-unit ceiling. This keeps the full 1.8-unit player model inside the taller viewport instead of enlarging it beyond the clip bounds. Still uncompiled/unverified.


## Skin Studio — full-height preview and first-install model validation (2026-11-03)
- User supplied a layout reference: the 3D player should fill the entire right-hand content column; skin selector rows should shrink to the displayed name; the skin list should use the remaining left-column vertical space with scrolling.
- Applied on branch `android`: `launcher_pages.rs` now allocates 42% of wide content width to a full-height preview, keeps controls and layer toggles in the left column, sizes skin rows to measured text width (bounded by the column), and retains the scroll viewport below the layers.
- Applied on branch `android`: `official_install.rs` preflights `PlayerPreview::load(&stage)` during first official resource installation. Successful preflight writes `skin-preview-ready.json`; failure logs the exact cause but does not block game installation. This reuses the existing pinned engine resource bundle rather than downloading a duplicate model.
- Code commits: `a51b97c80e15bcafa97be9090076126403073644`, `ce95e3ec38fadacdab2f83a1b17ee79b0cd7ea2e`, `a71b0bcaead6f0642a8ac86bd49885add156aab7`.
- Development log update: `b131ab6aee93a48a99ac42f9b41a6ca1baa227ee`.
- **Not yet verified:** formatter, compile, unit tests, source/native CI, APK build, APK install, and on-device layout/model behavior. The new installer preflight has not yet confirmed the model exists in the staged APK resources.
- Next: source checks + all four native ABIs; then isolated arm64 test APK and device check. No Sylphra submodule sync or Release publication.

- Final geometry follow-up: commit `c88c2a7601c5016577d7bdf73bde3d161b7d1e02` extends the wide preview and skin-list viewport to `h - 12`, instead of the shorter generic content rectangle, to reclaim the unused lower area shown in the reference screenshot. Not yet compiled or device-verified.


## Sylphra launcher visual identity and build-channel news (2026-11-03)
- User supplied Minecraft Java Launcher screenshot as visual reference. Applied a Sylphra-specific charcoal/green palette in `launcher_pages.rs`; it does not recolor shared Horizon UI theme or in-game HUD.
- Wide sidebar now places account/profile above navigation. Play home has a Java-inspired hero composition, a Files shortcut and a prominent right-aligned green Play button. Sylphra's own layout and existing game launch action are preserved; no fake Multiplayer shortcut was added because there is no separate launcher multiplayer action in the current code.
- Added `Page::News` gated by compile-time `SYLPHRA_BUILD_CHANNEL`: stable/release excludes News from navigation; beta/development includes it. `crates/horizon-client/build.rs` validates the allowed channel names and defaults to stable. The isolated test APK workflow sets `development`; a beta APK workflow is not yet present.
- News content comes from `docs/SYLPHRA_DEVELOPMENT_NEWS.md` and is scrollable. It distinguishes PROPOSED, IN PROGRESS, APPLIED — UNVERIFIED and VERIFIED; includes tester/contributor guidance.
- Code commits: `ec75e42585a62b730b24bd86d9af66addf1e0f0e`, `df7b9d5f8b3789a3a9a088c903968e32a8cd0cb4`, `51364b6eaabc3b103a1edddb4161edd182ba488a`, `427a9803d5864318aa013fdcd67e8f35fcbc4179`, `351aea1d323dd6cc85c7adf39fadba310b8dbbcd`, `e3762924902159ee1d1f1675983a0c30cf1133b5`, `e8c9c5e424cf74d9671b1c25177f5002c3bfeb95`.
- Development log entry committed as `33f6809fa5cfbae29632f59f5b067fe0a103484c`.
- **Not yet verified:** rustfmt, compilation, tests, source CI, four-ABI native checks, APK build/install, or device appearance/interaction. No beta workflow was created. Do not sync the Sylphra submodule or publish a Release before validation and explicit authorization.
- Next: run Android source checks and native checks for arm64-v8a, armeabi-v7a, x86 and x86_64; then build the isolated development APK and review UI/device behaviour. Also validate the player-model first-install preflight from the prior Skin Studio task in the same APK.


## Repository link and independence notice (2026-11-03)
- Footer GitHub button now explicitly reads `Sylphra GitHub` and invokes the existing `SylphraRepository` action to open `https://github.com/sylphie07/Sylphra-Android`; it no longer routes visitors to Horizon's upstream repository.
- About Sylphra now states that this is an independent unofficial project using Horizon Android as a technical foundation; it is not affiliated with, sponsored by or endorsed by Horizon or the original Android port author; Sylphra does not speak for them and does not know their position on the project. It also explicitly thanks and respects the original Android port author and acknowledges Horizon's contribution.
- Commit: `9f1b3e5d39728f0b3bd1e848303bb7078bd4e744`; development log update: `f0969b9cb87c5feed7d2aa3d1cbc6072d0e6fc79`.
- Static source inspection confirms the footer no longer creates the legacy generic GitHub action. The legacy handler remains but has no call site. No compile, CI, APK or device verification yet.
- Next: include this in source/native CI and the next isolated development APK; tap-test the footer link and review About text on device. Preserve upstream attribution and third-party licenses.


## Public repository destination correction (2026-11-03)
- Corrected the Sylphra GitHub button target from private `sylphie07/Sylphra-Android` to public `sylphie07/Sylphra-Android-Public` after verifying repository visibility through GitHub metadata.
- Public target: `https://github.com/sylphie07/Sylphra-Android-Public`. Private implementation repository remains private and is not linked from the launcher.
- Code commit: `7ce3d959b4461107ea37d4980fc496a87c0f5022`. Not yet compiled or device-tested.


## Validation checkpoint (2026-11-03)
- Android source checks #261 / run `41738307509` passed on commit `cc5ea8e5c5a599dce392ec3065eb6abd25f516d6`. All source, tests, diagnostics, performance/content checks and workflow-export comparison succeeded.
- Native ABI matrix remains pending; dispatch `.github/workflows/android-source.yml` on branch `android` with `native_checks=true`. The workflow dispatch action is not available through the connected GitHub tools, so the user must start it from GitHub Actions.
- After four ABI jobs pass, dispatch `.github/workflows/sylphra-test-apk.yml` (development APK only, not a Release). Then verify UI and public repository button on device.


## Native CI correction checkpoint (2026-11-03)
- Source run #264 source job passed, but all native ABI jobs failed because the channel env was injected by `horizon-client/build.rs` rather than `horizon-android/build.rs`; const string comparison also failed on the toolchain.
- Fixed in `catalog.rs` and `crates/horizon-android/build.rs`: channel now injected by the actual Android crate; beta/development use a Cargo cfg; stable defaults safely.
- Commits: `0b0bd77bdd0578b9d630a0a1e24ea9cd4e12c33b`, `d65b792f7fe229f4cca5e46bf8d12bd0016d3efb`. Not yet CI-verified. Rerun Android source checks with `native_checks=true`; do not build APK until the native matrix passes.

## Native Android validation passed — APK build handoff (2026-11-03)
- Manual Android source/native workflow run #269 (41764300816) completed successfully on branch `android`, commit `1eed09d156fb0f41b37cd05acf400ecfd6187273`.
- All five jobs passed: source checks plus native Rust checks for `arm64-v8a`, `armeabi-v7a`, `x86_64`, and `x86`. The native steps completed `Check Android Rust code without building an APK`.
- This verifies the build-channel correction and Android Rust compilation checks. It does **not** build an APK or verify launcher visuals/interactions on a physical device.
- The isolated development APK workflow `.github/workflows/sylphra-test-apk.yml` is present and configured for `dev.sylphra.android.test`, `arm64-v8a`, `SYLPHRA_BUILD_CHANNEL=development`; it uploads a seven-day Actions artifact and does not publish a Release.
- No APK workflow run was found after the native CI pass. Next: manually dispatch the APK workflow on branch `android`, inspect build/audit/artifact results, then install and device-test the resulting artifact. Keep Release publication and submodule sync disabled.

## Skin Studio device feedback and first layout correction (2026-11-04)
- User tested the launcher and confirmed the game starts, but the custom skin is not visible in Skin Studio's 3D preview. Screenshots show the six layer buttons overlapping the preview column; one capture still shows the old 'Player model is not loaded yet' placeholder, while the later capture shows an empty preview panel. This is not considered fixed.
- Inspected `launcher_pages.rs`: `wide_preview` creates separate control and preview columns, but layer button widths were incorrectly calculated from the full content width. Applied correction in commit `312a7b7968ad62ffd048803c1b7f108aaea3778b`: layer controls now size against the left control column, and the scroll viewport is bounded to that column.
- APK workflow run #? is not claimed here; the previously inspected run #? is not relevant to this change. The current correction has not yet passed source CI, native ABI checks, APK build, or device testing.
- Skin rendering investigation: `PlayerPreview::draw` uses the custom atlas sprite `horizon/skin0`; `skin_preview_rgba` decodes the selected private PNG and processes legacy dimensions. The screenshot alone does not establish whether the remaining blank model is caused by missing model geometry, missing atlas sprite, or rendering/depth submission. Next: add focused diagnostics/tests around generated preview vertices and atlas sprite lookup; inspect model asset loading and ensure default/custom texture fallback is visible. Then rerun source/native CI and build a new isolated development APK.
- Do not mark Skin Studio complete until the 3D player model and selected skin are visibly rendered and the layout is verified on-device. Do not publish a Release or sync the submodule.
## Skin texture atlas correction (2026-11-04)
- Further source inspection found a likely direct cause of the invisible custom skin: the preview patched and assigned `horizon/skin0`, while the player model's vanilla texture identifier is `entity/player/wide/steve`. If the custom identifier is absent from the runtime atlas, `PlayerPreview::draw` cannot resolve it and the selected skin is silently not used.
- Applied code commit `1812890524738441941e5696dd67ee617c4171c3` in `crates/horizon-client/src/android/launcher_pages.rs`: custom 64×64 pixels now patch the existing `entity/player/wide/steve` atlas slot, and the same identifier is passed to preview rendering. Missing slot now produces a warning and visible status instead of silently falling back.
- This is a source-based correction, not yet CI-verified or device-verified. It is not yet proven to resolve the blank model: geometry generation, atlas upload, and depth submission remain to be checked if the preview stays empty.
- Latest layout correction `312a7b7968ad62ffd048803c1b7f108aaea3778b` had source checks pass in run `41772593884`; native checks were skipped. The previous APK run `41765962850` was built from older commit `4c00f89f9c95b1df6576b0f30ff1561dbc2ea5dd`, before the Skin Studio layout/texture corrections.
- Next: inspect the new source CI result; manually run native checks with `native_checks=true`; then build a fresh isolated development APK and test on device. If blank preview persists, add counters/diagnostics for render_with vertex count, culled quad count, atlas slot dimensions, and texture patch upload. No Release or submodule sync.
## Skin Studio blank-model render diagnosis (2026-11-04)
- Latest source workflow run #277 (41773866734) passed the `source` job on commit `0255865a66f4e650995fd3e117afe34d92a4543f`; `native` was skipped. This validates source/unit checks only, not four-ABI compilation.
- Deeper inspection of `mobs/preview.rs` found the GUI transform used `Mat4::from_scale(Vec3(model_scale, model_scale, -model_scale))`, reflecting the model on Z. `player_preview_quads` subsequently culls faces using the sign of the transformed normal (`if n.z >= 0.0 { continue; }`). The reflection reverses winding, which can cause the face-culling stage to discard the entire player model. This is a strong source-level explanation for the blank preview.
- Applied commit `456c2cb213476a558e7412e3d5cdd48be62b33ba` in `crates/horizon-client/src/mobs/preview.rs`: use a positive uniform scale so the preview transform preserves orientation and winding; documented why in code. This change is not yet CI-verified or device-verified.
- Skin texture correction commit `1812890524738441941e5696dd67ee617c4171c3` passed the source job in run #277; its atlas lookup now uses `entity/player/wide/steve`. The source workflow's native job was skipped because it was dispatched without `native_checks=true`.
- Next: inspect source CI for the positive-scale fix; dispatch `android-source.yml` manually with `native_checks=true` to validate all four ABIs; if successful, build a new isolated development APK and device-test preview geometry, selected skin texture, rotation, and layer toggles. If still blank, instrument vertex/quad counts before altering more transforms. No Release or submodule sync.
## Native CI failure — list viewport scope correction (2026-11-04)
- Inspected failed manual native workflow run #281 (`41775392689`) on documentation commit `f4d6350cb5808fa6c06f72db10227b2229dc1107`. Source job passed; all four native ABI jobs failed at the same Rust compile error in `launcher_pages.rs` lines 966–967: `wide_preview` and `control_width` were referenced in the Installed page branch, where they are out of scope.
- Root cause: the previous layout patch placed the `list_right` conditional at the first matching `self.list_view` occurrence (Installed page), not at the Skin Studio occurrence. This was an implementation placement error, not a problem with Rust targets/toolchains.
- Corrected in commit `2a5342e2e186268021f0bc3054770af5bf1699b7`: restored the Installed page viewport to `content_right`; moved the `wide_preview`/`control_width`-bounded viewport into Skin Studio and constrained each skin row rectangle to the same `list_right` boundary.
- Existing model preview transform correction (`456c2cb213476a558e7412e3d5cdd48be62b33ba`) remains in the branch but has not been natively compiled or device-tested because the unrelated scope error stopped compilation.
- Next: rerun source checks and native checks (`native_checks=true`) on latest `android` HEAD. If all four ABI checks pass, build isolated development APK and test. Do not publish Release or sync submodule.
## Native CI passed after Skin Studio fixes — APK handoff (2026-11-04)
- Manual Android source/native workflow run #285 (`41777706392`) completed successfully on branch `android`, commit `a07f1151df91605a1ed97c4e2c3c16bced5e4fe5`.
- Independently inspected jobs: source checks passed and all four native Rust ABI jobs passed (`x86`, `armeabi-v7a`, `x86_64`, `arm64-v8a`).
- This validates compilation of the Skin Studio layout scope correction, atlas texture slot correction, and orientation-preserving preview transform. It does not verify runtime rendering or device layout.
- No newer run of `.github/workflows/sylphra-test-apk.yml` exists; prior successful run #10 (`41765962850`) is stale and built an earlier commit. The current APK workflow checks out `android`, builds only `arm64-v8a`, uses isolated application ID `dev.sylphra.android.test`, development channel, audits the artifact, and uploads a seven-day Actions artifact without publishing a Release.
- Next: manually dispatch the isolated APK workflow on branch `android`; inspect build and audit jobs/artifact; install the test APK and verify preview model visibility, imported skin texture, layer controls, and list bounds. No Release publication or submodule sync.

## Isolated development APK built — device review required (2026-11-04)
- Current branch HEAD: `39037d3ebdd0d2e62d6a3b248e5a80cb95493995` on authorized branch `android`.
- Android source workflow #287 (run `41778935887`) completed successfully on this exact HEAD.
- Isolated test APK workflow #11 (run `41779254040`) completed successfully on this exact HEAD. All steps passed, including pinned resource preparation, arm64 APK build, release-candidate audit and artifact upload.
- Artifact: `sylphra-launcher-arm64-test`, 36,454,569 bytes, SHA-256 `0c286d50ab8a61f90427c1dfea1927166ca6af0e4c9ffac23616d9038e86aaca`, expires 2026-11-11. This is an Actions test artifact only; no GitHub Release was published.
- The code includes the responsive launcher shell (compact phone, expanded phone/tablet and wide sidebar), Sylphra charcoal/green visual theme, Java-inspired Play page, and Skin Studio 3D preview with six independently toggled outer skin parts. These are source/build facts, not physical-device acceptance.
- Next required user action: install the artifact from run #11 and inspect the actual 3D model, imported skin texture, Classic/Slim model, all six layer toggles, preview rotation, list clipping/scrolling, portrait/landscape layout, and general navigation. Report any visual defect with a screenshot. No Release publication or Sylphra submodule sync.


## Skin Studio preview atlas root-cause correction (2026-11-04)
- User confirmed the latest test APK still showed the skin list but no 3D mannequin, even after entering and leaving a world. Entering gameplay does not populate the launcher preview atlas and is not a valid workaround.
- Re-inspected the complete path. The launcher constructs `HudAtlas::build(&self.home.join("installer"))` in `crates/horizon-client/src/android/launcher.rs`; this is an installer-only GUI atlas, not the full game atlas. `HudAtlas::slots` guarantees 64x64 runtime slots named `horizon/skin0...`, but the preview code had been changed to look up `entity/player/wide/steve`, which is not guaranteed in the installer atlas. When lookup fails, `selected_texture` stays `None`; `render::render_visible` then cannot resolve the player's texture UV and submits no body vertices. This explains why model loading can succeed while the preview remains visually blank.
- Applied code commits on authorized branch `android` only: `27128e03ad1c128dd1f7ee25c4baadec14137619` adds a dedicated 64x64 `horizon/skin-preview` atlas slot in `crates/horizon-client-render/src/hud.rs`; `9b97941732f0f520d82b845ecc0814f2a488d36e` changes `PlayerPreview::draw` to return generated vertex/quad counts in `crates/horizon-client/src/mobs/preview.rs`; `ea9fe2f2b0f975e7bff875e131fa5a95e0fa877f` patches selected skin pixels into the guaranteed dedicated slot and loads the installed default Steve PNG when no custom skin is selected, with a neutral fallback and visible diagnostic if it is unavailable, in `crates/horizon-client/src/android/launcher_pages.rs`.
- The launcher now reports when geometry produces zero vertices/faces, so a future device test distinguishes resource/atlas failure from model-geometry/culling failure. This is an implementation hypothesis supported by the inspected atlas construction and renderer lookup path; physical-device confirmation is still required.
- Android source workflow #292 (run `41794149797`) was queued/running for final code commit `ea9fe2f2b0f975e7bff875e131fa5a95e0fa877f` at time of this note. Earlier run #290 was cancelled by newer pushes; do not treat it as final verification. No native four-ABI check or new APK has yet been run for these commits.
- Next: verify source workflow #292; then run Android native checks for all four ABIs. Only after they pass, build a new isolated development APK and ask user to verify model, imported skin, Classic/Slim, and six layer controls on device. Do not publish a Release, merge PRs, touch master, or sync the Sylphra submodule.


## Skin Studio preview lifecycle correction (2026-11-04)
- User confirmed that the model still did not appear after entering and leaving a world; the old “Open Play once, then return here” text remained visible.
- Re-inspection of the current `android` branch identified a concrete navigation gap: the top navigation tabs dispatch `Action::Page(Page::SkinStudio)`, while the original `PlayerPreview::load` call existed only in the Settings shortcut `Action::SkinStudio`. Therefore opening Skin Studio through the main tab could render the placeholder without ever attempting model loading. This explains why playing once did not fix it.
- Applied on `android`: `launcher_pages.rs` now lazily loads `PlayerPreview` when Skin Studio is actually drawn, regardless of which navigation route entered the page. Failed loads preserve and display the exact resource path/error; the obsolete “Open Play once” message was replaced with a neutral loading fallback.
- Applied on `android`: entering Skin Studio through either page navigation clears a prior failed-load diagnostic to permit one fresh retry. Returning from the game clears the preview error alongside the preview instance, so the next visit retries. Document-import navigation into Skin Studio also clears the prior error.
- Commits: `4a7c84ccfbdc4eb13108d906c921135893aaec4d` (page lifecycle loading) and `2dbc4daff58f6fdea53d7910e97410992a43e30e` (retry after game return / document import).
- Verification status: code edits applied; no source-check workflow or APK build has yet been run for these two commits. Do not claim the device-side model rendering is fixed until CI passes, a fresh APK is installed, and the user confirms the preview.
- If the next build shows a concrete `entity-models.json` read/parse/key error, investigate the installed resource ZIP contents and installer output. The existing installer preflight at `official_install.rs` lines 495–506 already logs whether `PlayerPreview::load(&stage)` succeeds during resource installation.


## Skin Studio — two-sided preview geometry and layer controls (2026-11-04)
- Repository preflight: continued on authorized branch `android`; starting HEAD was `4d28bf60054a663a27f8faa2e04037957914100c`. The preceding test APK workflow #12 (run `41796368621`) succeeded on that commit, but the user confirmed the preview area was still blank.
- Diagnosis from source: `PlayerPreview::draw` did generate the mesh and `player_preview_quads` discarded every face whose transformed normal had `n.z >= 0`. The exported player model does not guarantee a single winding convention for all faces, so this one-sided filter could discard the mannequin. The GUI preview uses a depth-tested pipeline, which can correctly resolve front/rear visibility without that winding-dependent rejection.
- Applied in `crates/horizon-client/src/mobs/preview.rs`: retain both face orientations, orient normals toward the viewer for UI lighting, and let the preview depth buffer hide rear faces. Commit: `0e2e0eb7e92349b2aeaaba2cd6bbb89f026a5c25`.
- Applied in `crates/horizon-client/src/android/launcher_pages.rs`: arrange the six skin-layer toggles in two rows of three, with the skin-list viewport moved down to preserve spacing. Commit: `47826c3237f825cd126417564bc681ffa3559296`.
- Verification boundary: source inspection confirmed both edits were applied to branch `android`. No compiler, Rust tests, CI run, new APK, or device verification has yet tested these commits. The user must not be told the model is fixed until a fresh build is installed and visually checked.
- Next: run source checks for the new HEAD; if available, run the native ABI matrix and test-APK workflow. Inspect whether preview geometry is now visible; if still blank, use the vertex/quad counters and on-screen diagnostic to isolate atlas upload versus depth pipeline. Then tune preview sizing and clipped status text from a fresh device capture.


## Skin Studio follow-up after device screenshot (2026-11-04)
- User tested the APK built from `3e2192ed2b85ca62e5d07d71ac0d46da9f0a7639`. Screenshot confirms the two-row layer layout is improved and the status still says the model loaded, but the preview rectangle remains completely blank.
- The current page only changes its status to a zero-face error when `PlayerPreview::draw` returns no quads. Since the screenshot still displayed the earlier generic loaded status, the next diagnostic needs to expose the actual submitted vertex/face counts and whether the dedicated skin texture was selected / queued for atlas upload.
- Applied diagnostic instrumentation in `crates/horizon-client/src/android/launcher_pages.rs`: status and log now report preview vertices, faces, selected texture readiness, and batch atlas patch count. Commit: `f1cf21940eed9a9914fc163afde79633ba4550cb`.
- The prior test APK run `41950724317` succeeded on `3e2192e` (artifact `sylphra-launcher-arm64-test`, SHA-256 `1845327a8114e522c436cb2850c413e90f691fb7cdd99695fe89e0a2d1e6cd29`); that APK did not yet contain this new instrumentation. New source checks and APK are pending for the diagnostic commit.
- Do not assume the atlas or depth pipeline is fixed: use the next visible diagnostic to determine whether the mesh is empty, texture patch is missing, or non-empty geometry is submitted but not displayed. No main branch, Release, or other branch was changed.


## Diagnostic build handoff status (2026-11-04)
- Diagnostic code commit: `f1cf21940eed9a9914fc163afde79633ba4550cb`; documentation commits: `57fb3d5b4999c4356ef168e7adcb42e56c0508bf` and `389517f44a0994470e7ca37f505abf734b63be6f` (current HEAD at handoff).
- Source checks run `41955511550` for the diagnostic code commit was cancelled when the documentation-only follow-up push superseded it; the source run for the intermediate docs commit `41955516794` was also cancelled. Therefore the diagnostic code has not yet passed source CI.
- The test APK workflow `.github/workflows/sylphra-test-apk.yml` is `workflow_dispatch` only. The available GitHub connection exposes GET/read operations but no workflow-dispatch action, so no new APK can be triggered from this session. The previous successful artifact from run `41950724317` is for old commit `3e2192e` and does not include the new diagnostic.
- Required handoff: manually run **Actions → Sylphra Launcher test APK → Run workflow**, selecting branch `android`. After installation, read the visible Skin Studio status for vertex/face counts, skin-texture readiness and atlas patch count. Then continue from that evidence; do not claim a rendering fix yet.


## Diagnostic APK verification — 2026-11-05
- Confirmed current branch `android` HEAD: `9562171a6185009eaa64ef69593866882e7a6b00` (`docs: record diagnostic APK handoff status`). No other branch was modified.
- Diagnostic source commit `f1cf21940eed9a9914fc163afde79633ba4550cb` is included in the current HEAD. It reports generated vertex/face counts, selected skin-texture readiness, and HUD atlas patch count in the Skin Studio status/log.
- Android source checks run `41955558582` completed successfully for HEAD `9562171a...`.
- Test APK workflow run `41955968685` completed successfully for HEAD `9562171a...`; arm64 build, audit, and artifact upload all succeeded. Artifact: `sylphra-launcher-arm64-test`, artifact ID `13384211698`, SHA-256 `26647d0f64f1d41e75ce0a7f2d47106c2ea0c35c9d0dc11cbd5af06fd7be08f6`, created 2026-11-05 02:52 UTC, expires 2026-11-12 02:52 UTC. This is a test artifact, not a Release.
- Important: successful build does not prove device rendering. The latest available user screenshot predates this diagnostic build and showed only the generic model-loaded status. Next required evidence is the status string from this diagnostic APK: `3D preview geometry empty: ...` or `3D preview submitted: ...`, including vertex/face counts, texture readiness, and atlas patch count. If the user already installed this artifact and still sees only the old generic status, verify package/version/install identity before further code changes.


## Skin Studio Android depth-pipeline fallback (2026-11-05)
- User supplied the diagnostic APK result: `288 vertices / 72 faces; skin texture ready: true; atlas patches: 1`, while the preview rectangle remained blank. This confirms model geometry and skin texture selection reach the HUD batch; it does not prove GPU draw output.
- Source trace followed `PlayerPreview::draw` -> `HudBatch::quad_3d` -> `HudBatch::depth_since` -> `HudRenderer::draw_range` -> the `HUD model` pipeline. This path requires a separate `Depth32Float` attachment, a depth clear/discard pass, and a per-preview scissor. The ordinary HUD path does not require that attachment.
- Applied a targeted Android compatibility fallback in `crates/horizon-client/src/mobs/preview.rs`: preview faces are already sorted back-to-front by camera depth in `player_preview_quads`, so they are now submitted as ordinary alpha HUD quads (2D positions) instead of `quad_3d` plus `depth_since`. This removes the depth attachment/pipeline/scissor as a dependency for the launcher preview. It is painter's-order rendering; intersecting surfaces may have minor sorting limitations, but it should reveal the mannequin if the Android depth pass was the blocker.
- Commit: `73ca7db94560a5b0d766b9fef6ed7a23b7ed285e` on authorized branch `android`.
- Verification boundary: change applied to source; source CI and a fresh test APK have not yet completed. Device rendering is unverified. Do not call the model fixed until the user tests the APK.
- Next: check Android source CI; if successful, trigger/build the test APK. User need not change any Skin Studio settings; just open the Skins page and inspect the preview after installing the new test artifact.


## Skin Studio coordinate-space correction from independent audit (2026-11-05)
- Independent read-only audit from Claudia identified a coordinate-space mismatch: `preview_rect` is calculated from launcher `Ui` dimensions in dp, while `PlayerPreview::draw` emits positions directly into `HudBatch` coordinates. The existing launcher UI background uses `scale = ui.density / ui.batch.scale` before writing to the batch, confirming the conversion expected by the HUD path.
- Confirmed current branch `android` and HEAD before edits: `09874dc661f499fa1aa97fdf2f5cf963989c5437`. CI run `41983998512` for that HEAD completed successfully. No other branch was touched.
- Applied in `crates/horizon-client/src/android/launcher_pages.rs`: convert all four preview rectangle coordinates using `ui.density / ui.batch.scale` before passing the rectangle to `PlayerPreview::draw`. Commit: `b90a53ba9c044a37dbc8ac0198a979a2172614e6`.
- Applied in `crates/horizon-client/src/mobs/preview.rs`: preserve the ordinary HUD alpha submission introduced in `73ca7db`, then call `HudBatch::clip_since` on the generated range using the same batch-space rectangle. This prevents painter-order faces from spilling outside the preview panel. Commit: `66c33e35360555731f7ccee19a174f0da63e3bdb`.
- Deliberately did not change body yaw: the audit flagged a possible front/back orientation concern, but it is a separate visual question and should be assessed only after the coordinate fix makes the model visible.
- Verification boundary: edits are committed to `android`; source CI and a new test APK are pending. Device rendering is not yet verified. The previous diagnostic values (288 vertices, 72 faces, texture ready, one atlas patch) came from the earlier APK and do not verify these changes.
- Next: run source CI for the new code, then build a fresh ARM64 test APK if CI passes. On-device validation should confirm mannequin visibility, containment within the preview panel, initial front-facing orientation, and touch rotation. If still blank, inspect batch-space vertex bounds against the converted rectangle before investigating GPU state again.


## Skin Studio front-facing depth orientation (2026-11-05)
- Read Claudia's second read-only audit and independently rechecked current source at HEAD `174f5d56011ea3392a1ea639b1b32378526a2615`. Confirmed the GUI transform used positive Z scale while HUD coordinates define +Z toward the viewer. The vanilla rest yaw remains `180.0 + yaw`; changing yaw is unnecessary for this correction.
- Confirmed `player_preview_quads` computes transformed face depth and sorts ascending (`quads.sort_by(|a, b| a.0.total_cmp(&b.0))`). With the depth-free painter's-order fallback, near (+Z) faces therefore need to be submitted later. Mirroring the model's Z axis places the vanilla front (-Z before GUI transform) toward +Z and restores expected front/back ordering.
- Applied in `crates/horizon-client/src/mobs/preview.rs`: changed GUI scale from uniform positive scale to `(model_scale, model_scale, -model_scale)`; updated comments to explain +Z viewer direction and why winding no longer causes face rejection. Commit: `864ca42c69a7c4b3da7beaff6dfcaf5de716de0a`.
- Retained the coordinate conversion (`b90a53b`), clipping (`66c33e3`), and depth-free HUD submission (`73ca7db`). No yaw change.
- Verification boundary: source edit committed to `android`; compilation, CI, APK build, and device orientation are pending. This is a source-level correction supported by transform and painter-order analysis, not proof of visual success.
- Next: run source checks; if successful, build a fresh ARM64 test APK. Verify that the model appears front-facing, skin torso/front details are visible, layers toggle correctly, and touch rotation remains natural.


## Skin Studio — device acceptance and next launcher redesign stage (2026-11-05)
- User installed and tested the build on the Android device and explicitly confirmed: the 3D player model renders; touch rotation works; skin layer toggles work; Classic/Slim model selection works. This is physical-device acceptance for those listed behaviours, not for untested devices or layouts.
- The user clarified the remaining Skin Studio feature: import elytra wing/cape layers, separate from the player's six skin overlay toggles. This feature is not implemented yet and must remain a distinct data/rendering path; do not reuse or rename Hat/Jacket/Sleeve/Pants controls.
- Verified current authorized branch android HEAD: d51ea97e3fe9792c7f0eaf166506f78946c6d537.
- Android source checks run #321 (41988650597) completed successfully on this HEAD.
- Isolated test APK workflow #15 (41989130135) completed successfully; arm64 build job succeeded. Artifact sylphra-launcher-arm64-test, ID 13398296511, SHA-256 69fcfccf0866fbaa73d4eaff38a11239d7ce76296837885ea66b8c37616731bc, created 2026-11-05 07:56 UTC, expires 2026-11-12 07:56 UTC. No GitHub Release was created.
- Next launcher redesign milestone: background media customization, previously deferred until adaptive navigation was completed. Begin with a source-level design/audit of Android document selection, safe URI/descriptor loading, image decoding/texture upload, and renderer lifecycle. Plan static image first, then GIF, then video only if the current renderer/platform architecture supports it without blocking the UI or leaking resources. Keep media optional and fall back cleanly to the existing theme.
- Existing adaptive navigation, wide-screen sidebar, expanded phone navigation, and shared page geometry remain in place. Preserve drawing/input rectangle alignment and all already accepted Skin Studio behaviour.
- No background media implementation has been applied in this session. The audit of gallery.rs, platform.rs, document_bridge.rs, launcher.rs, and launcher_pages.rs is exploratory only; Android gallery code currently documents screenshot publishing, not user media import.
- Continue only on android; no changes to main, no PR merges, no Release publishing. Record elytra import as a separate feature track.


## Skin Studio visual polish from video review (2026-11-05)
- User supplied a 52-second device recording; user independently tested the working viewer, and Claudia's frame-by-frame review identified text clipping in compact controls, an unsupported checkmark glyph in active skin labels, the wide-preview footer GitHub button overlapping the preview, and truncated skin names.
- Applied on authorized branch android, commit 7639734a34730aa067bb03a9c0f7efff8a9d017a, file crates/horizon-client/src/android/launcher_pages.rs:
  - button labels now center vertically based on font line height; controls shorter than 36 dp use 13 dp text.
  - removed the unsupported checkmark prefix from skin names; active state remains indicated by the green button surface.
  - increased skin row natural width allowance from 28 to 36 dp while retaining 24 dp text-fit inset.
  - hide the footer Sylphra GitHub button when the wide Skin Studio preview is active; it remains available elsewhere.
- Status: source edits applied and committed. CI/build and device visual verification are pending; do not describe the visual defects as fixed on-device until the new APK is tested.
- Preserve the user's device-verified model rendering, rotation, six skin layer toggles, imported/default skin switching, and Classic/Slim behaviour.
- Next: inspect source-check workflow result; if successful, build a fresh arm64 test APK. Ask user to inspect button baselines, skin name rows, and preview corner after installing that artifact.


## Elytra/back texture import investigation (2026-11-05)
- User clarified the requested feature: import custom back designs (Elytra), separate from player skin overlays Hat/Jacket/L/R sleeve/L/R pants. Do not merge these controls or records.
- Read-only source audit at HEAD b3480f4ae2e9a253df76a14e4bed705947a97377; detailed report created at docs/SYLPHRA_ELYTRA_IMPORT_RESEARCH.md, commit a459449ae201887ebc61b458955e3942aa79c8ff.
- Findings: profile parser already recognizes ELYTRA and Kind::Elytra, but the texture download completion currently discards Elytra (Kind::Elytra => {}) and the atlas requeue path only recognizes skins/ and capes/. Local offline custom texture support exists for Cape via HORIZON_OFFLINE_CAPE; no corresponding offline Elytra path was found.
- Player preview loads player#main plus player_slim#main and player#cape; it does not load player#elytra. The dedicated cape render function uses player#cape; no equivalent Elytra preview function was found in the inspected code.
- Android SAF import, bounded PNG reads, SHA-256 storage and manifest persistence are reusable patterns. Current skin-library v1 supports only skin records and 64x64/64x32 skin images; it has no Cape/Elytra record category.
- No product code was modified, no compilation/tests run, and no APK built in this investigation.
- Next research: inspect actual prepared game resources for Elytra geometry/UV and texture dimensions; locate all Cape render call sites and the best avatar-preview integration point. Then design separate profile-bound Elytra records, SAF import and renderer/cache support. Ask whether the user wants Elytra only or both Cape and Elytra as separate categories before finalizing UI.
- Continue only on branch android. Preserve the existing verified Skin Studio model/rotation/skin layers/Classic-Slim behaviours.


## Skin Studio — persistent Skins/Capes layout (2026-11-05)
- User clarified with reference screenshots that the requested back cosmetic is a textile **Cape** (Aurora-style), not Elytra. The earlier Elytra interpretation was incorrect; Elytra remains a separate research note and is out of scope for this feature.
- User approved a two-category “Skins & Wardrobe” design inspired by the supplied launcher: separate **Skins** and **Capes** tabs, with the 3D player preview always visible in both categories.
- Preview must retain rotation and display the current skin/model plus currently equipped cape. Changing tabs must not reset either selection.
- Skins retains current PNG import, appearance profiles, Classic/Slim, default skin and six skin-overlay toggles. Capes gets its own PNG import/library, active state and “None” option. Cape is independent from skin overlays and Elytra.
- Design specification: `docs/SYLPHRA_CAPE_STUDIO_DESIGN.md`. Prior research correction appended to `docs/SYLPHRA_ELYTRA_IMPORT_RESEARCH.md`.
- Source audit confirmed existing runtime primitives: `offline_cape()` reads `HORIZON_OFFLINE_CAPE`, player model loading includes `player#cape`, and `mobs/render.rs` has dedicated Cape pose/render code. Current Android skin library and SAF import only support skins; selected cape is not yet persisted or connected to preview atlas/rendering.
- Repository preflight: branch `android` exists and is the only authorized branch. HEAD before this documentation update was `1774f348e521a67b8f6b8243a7ef944ca7508381`. Source file hashes and current docs were inspected from `android`. `docs/PROJECT_CONTEXT.md`, `docs/KNOWN_ISSUES.md` and `docs/DECISIONS.md` are not present at those paths (404); do not assume they exist.
- Changes in this update are documentation only. No product source modified; no formatter, tests, CI, APK build or device validation performed.
- Next: audit Cape atlas initialization/refresh and `render::cape` call sites, then implement independent profile-bound Cape storage/import/runtime integration and persistent-preview tab layout as one coherent feature. Preserve the device-verified model, rotation, skin overlays and Classic/Slim behaviour. Continue only on `android`; no PR merge or Release.


## Cape runtime audit update (2026-11-05)
- Verified the Android launch request only passes selected skin path/model; there is no Cape field or CLI argument in `game_launch::LaunchRequest`. Existing `launcher/src/skins.rs::env()` supports Cape for a separate offline-account look helper, but is not connected to the Android launch request inspected here.
- Verified `offline_cape()` reads `HORIZON_OFFLINE_CAPE` once via `OnceLock`; this is not a suitable live tab-selection mechanism for the launcher library.
- Verified the player preview loads `player#cape`, but its draw path only invokes `render_with` for the body; it does not invoke `render::cape`. The specialized Cape helper requires an atlas UV supplied by its caller. Generic `render_visible` does not automatically submit this helper.
- The compact launcher atlas has a dedicated skin preview slot patched by the UI. Cape preview requires a parallel Cape slot and normalized RGBA data, plus a call to `render::cape` using that slot's UV.
- Standard/legacy Cape normalization already exists: 64×32 and 22×17 map into the upper half of a 64×64 transparent slot; oversized inputs are downscaled. Android import should have a clear tested allowlist and retain the 1 MiB cap.
- Current page-owned preview rect, touch hitbox and rotation state can be retained across tabs if preview drawing is moved outside the tab-specific controls. Narrow-screen layout requires a deliberate reflow to keep both the model and useful controls visible.
- No product code or tests changed; no CI/APK/device validation.


## Player preview and game launch trace — 2026-11-05
- Full `PlayerPreview::draw` path confirmed: clones the player mob, sets active skin/model/overlay visibility, calls `mobs::render::render_with`, converts vertices to clipped GUI quads. Although `player#cape` is loaded in the mob type, no Cape geometry is submitted. The Cape helper requires a root transform and a separate Cape sprite UV; preview integration must share the body transform and append Cape vertices before GUI conversion.
- Android `LaunchRequest` still carries only skin path and Slim flag. Add an independent optional Cape selection argument for gameplay; do not mutate `HORIZON_OFFLINE_CAPE` at runtime because `offline_cape()` uses `OnceLock`.
- Game frame composition calls `mob_geometry`, then separately `player_model_geometry` for the local player and `guest_model_geometry` for guest players. The exact internals of these methods still require targeted inspection; actual gameplay Cape call-site remains unconfirmed. Do not state that Cape rendering in-world is already working.
- Existing preview object/rotation/touch state is page-owned. Preserve it across Skins/Capes sub-tab switches; keep preview lifecycle outside tab-specific control rendering.
- No source code modified, tests/CI/APK/device verification not performed.


## Local-player render and atlas ownership audit — 2026-11-05
- Confirmed `Game::geometry` submits generic mobs first, then marks/submits the local player via `player_model_geometry`, and finally submits guest players via `guest_model_geometry`. The local player is a distinct rendering path, which is the correct place to apply the launcher's local Cape without touching remote/guest players.
- The implementations of `mob_geometry`, `player_model_geometry` and `guest_model_geometry` are not directly declared in the monolithic `game.rs` text inspected; they are supplied through the game's split/module organization or generated includes. A full repository search for their definitions via the available GitHub code-search connector returned no matches. Their exact implementation and the call-site of `mobs::render::cape` remain unresolved; do not edit or assume a remote-player path until local definitions are located through a complete source checkout or better search.
- Confirmed `game/atlas_slots.rs` is a generic LRU allocator for dynamic GUI atlas images (skins/maps), not a fixed Cape registry. It forgets assignments on atlas reload. A Cape texture can use the same slot lifecycle only if the caller patches pixel data and obtains the matching sprite UV after any eviction/reload.
- Confirmed Android `LaunchRequest` currently has `skin_file` and `skin_slim`, and argument construction emits skin flags only. `Launcher` creates that request from the active skin. Adding `cape_file: Option<PathBuf>` plus a dedicated CLI argument is a small, explicit transport change, but the receiving game-side argument parser and texture initialization still must be traced before implementing it.
- Confirmed `PlayerPreview::draw` already builds a body vertex buffer, then applies a GUI transform and clips it. The Cape helper `mobs::render::cape` needs a root transform and UV rectangle. The preview currently computes its GUI model transform after body vertex generation, while the renderer's root transform is internal/private; safe integration likely requires exposing/factoring the player root transform or a preview-specific renderer API rather than guessing matrix order.
- No product source changes, tests, CI, APK or device validation were performed. The investigation remains read-only except for documentation.


## Cape library foundation — 2026-11-06
- Preflight verified the only authorized branch android, with starting HEAD fa01ad6289b80680d064c070f73cd071618b4a09; docs and the current skin-library/SAF/catalog implementation were reviewed first.
- Added crates/horizon-client/src/android/cape_library.rs: separate cape-library.json, content-addressed private PNGs in capes/, supported dimensions 64×32 and 22×17, 1 MiB input limit, decoded-image bound, SHA-256 deduplication, per-profile cape selection, rename and remove.
- Registered the module from android/catalog.rs.
- Commits: 2a9102243b911b9d1f8b5c81523c89958c432f4f (library) and 7634ce5c38c1de1f70409f67a78163018bc240fe (module registration).
- Applied, not verified: GitHub accepted the writes. No formatter, compiler, unit tests, CI, APK or device run has been performed. The module is registered but not yet connected to Catalog, SAF, profiles in the existing SkinLibrary, launcher UI, preview or game runtime.
- Destructive deletion API clears profile selections in the Cape manifest and saves before deleting the PNG; the eventual UI must require explicit confirmation.
- User decisions: custom Cape replaces account Cape; choosing None preserves account Cape; hiding remains vanilla modelPart_cape; custom Cape must not retint Elytra; accept 64×32 and 22×17; 1 MiB maximum.
- Next: independent read-only code review of the new module; confirm actual player#cape UV; integrate import, profile linkage, preview, local-only runtime and persistent Skins/Capes tabs. Keep remote-player capes unchanged.


## Sylphra launcher visual redesign — 2026-11-06
- User supplied a 1536×691 Minecraft Java Launcher screenshot as the target visual reference.
- Created docs/SYLPHRA_LAUNCHER_REDESIGN.md on branch android (commit 0e59a312381768de5fc95f762ab20bf5f81508b7). It defines the wide-screen 23/77 sidebar/main-canvas composition, account/navigation rail, full-bleed Minecraft-themed background, compact top-right actions, bottom-left version selector, bottom-right green Play button, responsive behavior and non-regression constraints.
- The current source still uses the older panel-based Play page; the redesign has not yet been implemented in product code.
- No source changes, build, tests, CI, APK or device validation were performed in this documentation step.
- Next: implement the shared shell/sidebar and Play-page composition in launcher_pages.rs, preserving all existing Action behavior and adaptive layouts. Then implement persisted background selection/import only after tracing the existing SAF and texture-atlas APIs. Static image first; GIF/video deferred.
- Parallel task: Claudia is asked for a read-only Cape audit only. Do not let that investigation modify launcher_pages.rs or overlap the visual redesign.


## Sylphra launcher redesign — first native UI implementation (2026-11-06)
- Preflight verified branch android at HEAD 3b0750ed31336e48e4cff33fe81851c472211846; continuity, development log, redesign specification and relevant launcher source were reviewed before editing.
- Applied crates/horizon-client/src/android/launcher_pages.rs in commit 5b0c7a691f22f9e0b1ab97e582f037836d3b0fa4. Wide layout now targets a 23% sidebar (248–360 dp), account area at top, stacked navigation labels, and a green outline for the selected page. Wide Play now has a dark full-canvas treatment, Files and Multiplayer/server-entry actions at top right, an informational Horizon version label, and a large Play/Install action at bottom right. Compact and expanded layouts retain their previous page rendering.
- Limitation: custom background selection/import is not implemented. The canvas currently uses a dark fallback with Sylphra text, not a Minecraft scene image. The version label is not a dropdown; Multiplayer opens the existing server field, not a server browser.
- Re-fetch confirmed the source blob; compare reports one source file changed (120 additions, 77 deletions). Android source checks run 42169014756 was queued when recorded; outcome not yet verified.
- No local formatter, compilation, tests, APK build, install or device visual validation performed. Do not claim compile success or visual equivalence until checks and device review succeed.
- Next: inspect source-check result, correct failures, then implement persisted static background import through verified SAF/document/texture APIs. Keep Claudia's Cape audit read-only and separate; preserve tested Skin Studio behavior.


## Selected Play background — 2026-11-06
- User selected the uploaded Minecraft Rust Edition scene as the definitive launcher Play background.
- Inspected source asset: 1536×864, RGB, JPEG encoding even though the uploaded filename ends in `.png`. Preserve its artwork and embedded logo; use the correct `.jpg` extension if committed, or transcode intentionally.
- Intended presentation: full-bleed aspect-fill/cover behind Play content; keep the 23% dark sidebar separate; only a subtle legibility overlay, no large opaque panel.
- Design spec updated in commit 71339263a6264b10598bfbc8c151417c32237e86. Binary asset is not yet in the repository and runtime texture loading is not implemented.
- Latest source-check run 42169082001 passed for source HEAD fe40b1c964d18e53943f42fc4e8a1f0cc5be1490. This does not establish APK build or device rendering.
- Next: add the actual image asset to the authorized android branch, then trace/use the existing atlas texture update path to render it in the Play background. Do not claim background is integrated until the binary and runtime path are committed and verified.


## 2026-11-06 — Sidebar icon pass
- Preflight: branch `android` at `6c440043bb2d00dda76d9ea8049f143a81db4efb`; the selected-background metadata is already documented. The preceding Android Source Checks run `42170897755` succeeded for the documentation-only commit `6c44004`; it does not cover the new UI source change.
- Applied `crates/horizon-client/src/android/launcher_pages.rs` in commit `69067bca2a47dcd569cecfb13c1aea778a886ef7`: wide sidebar navigation rows now use item-atlas icons (grass block, crafting table, player head, writable book, iron pickaxe, chest, glass bottle), a dark-green selected fill, bright-green outline, and left-aligned labels. The original action targets and compact/expanded layouts remain intact.
- Source re-fetch confirms the icon mapping and `sprite_scaled` calls are present. Commit status was pending when this note was prepared. No local Rust compilation, APK build, or device visual check has been performed.
- Background asset is still not in the repository/runtime. The chat upload is JPEG-encoded (1536x864) despite its `.png` display name. Static background import/rendering remains the next major milestone; do not claim it is integrated yet.


## 2026-11-06 — Fix catalog-page overlap in wide sidebar mode
- User-provided screenshots of Mods, Shaders and Resource Packs showed the catalog header, source selectors, search/import controls and result rows overlapping. Profile and Play pages did not show the same defect.
- Root cause confirmed in `crates/horizon-client/src/android/launcher_pages.rs`: catalog-page controls used legacy absolute Y positions (title 142, source selectors 134, search 174, folder/import 222, hint 270), while the new Sidebar layout starts the content canvas at Y=0. More importantly, catalog results began at `content_rect.y + 152`, i.e. Y=152 in Sidebar mode, colliding with the header. The old offsets remain correct for Compact/Expanded modes, where content starts around Y=144–146.
- Applied source fix in commit `e080a5ddf2466e751f785d7360bbf0aeac97a1c7`: introduced `CatalogPageLayout::for_launcher`. Sidebar mode now stacks title/source row at Y=18–24, search at Y=68, folder/import at Y=116, hint at Y=164 and result list at Y=210. Compact/Expanded offsets are explicitly preserved.
- Added unit tests in commit `1172268cef3d7a6522b37c0646ef30ac0f6c238e` to assert sidebar catalog controls/results do not overlap and legacy offsets remain unchanged.
- GitHub re-fetch confirms the geometry helper and tests are present. Android Source Checks run `42176498819` was pending for HEAD `1172268c`; another run `42176470226` was still running for the source-only fix commit. Tests are not claimed passing until the matching HEAD run completes.
- No APK build or device visual retest has been performed for this fix yet. Next: verify source-check run, then build/install a test APK and inspect Mods, Shaders and Resource Packs at the user's 1536×691 landscape viewport plus a narrower phone layout. The fix is scoped to catalog-page vertical geometry and leaves Skin Studio/Play logic untouched.


## 2026-11-06 — Cape library connected to launcher catalog state (foundation)
- Preflight: verified authorized branch `android`, HEAD `e518b6d7a792bde58d3dd539952226a023048369`, and read `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `docs/SYLPHRA_CAPE_STUDIO_DESIGN.md`, `android/cape_library.rs`, `android/skin_library.rs`, `android/catalog.rs`, `android/game_launch.rs`, `game.rs`, `game/remote.rs` and `mobs/preview.rs` before editing.
- CI verification of the prior catalog-overlap fix is now confirmed: Android Source Checks run `42176534828` passed for `e518b6d`; Sylphra Launcher test APK run `42177345901` also completed successfully for that same source HEAD. This is CI evidence only; it does not replace the user's device retest (the user reported the overlap is fixed).
- Confirmed an existing but previously disconnected `android/cape_library.rs` already provides a separate `cape-library.json`, SHA-256 PNG storage, 64×32/22×17 validation, 1 MiB input cap, profile selection, rename/remove and unit tests. It was already declared as a module in `catalog.rs`, but `Catalog` did not load or expose it.
- Applied `crates/horizon-client/src/android/catalog.rs` in commit `3bbe4a4ace1e615df9463cd6f0ef6e5953343c59`: `Catalog::new` now loads the independent cape library and reports its own read-only/error state; `refresh` reloads it; `save_cape_library` persists it; `active_cape_file` resolves the cape associated with the active appearance profile and rejects symlink/non-regular/empty/over-1-MiB files. Skin manifest and existing skin behavior are unchanged.
- Re-fetch of the committed source confirms the cape load/reload and safe active-file resolver are present. The new commit's CI had not yet appeared in the Actions listing at documentation time; do not claim it passes until a run for this exact commit completes.
- Not implemented yet: SAF import route, Capes tab/actions in Skin Studio, preview texture/3D cape rendering, launch argument and game-side local cape loading, and LAN peer sharing. Existing `update_remote_skins` only applies the Sylphra skin to the local player's client-side model; this is not evidence that a custom skin is transmitted to other LAN clients. LAN sharing therefore needs a separate protocol/metadata design for both custom skin and cape, preserving vanilla/server account textures and never overriding unrelated remote players.
- Background: selected Minecraft Rust Edition artwork is documented as 1536×864 JPEG-encoded, but the selected binary is not present in the repository/runtime. A repository image `docs/media/android/snow-landscape.png` exists (3,320,463 bytes) but has not been verified as the user's selected artwork; do not substitute it. Ask the user to reattach the exact selected image before committing it.
- Next: validate CI for `3bbe4a4`; implement SAF Cape import + Skin/Capes tab state; add the 3D preview using the verified player#cape model UV and shared root transform; then implement launch/runtime local cape. Separately design explicit Sylphra-to-Sylphra LAN appearance exchange (skin + cape) and test two devices. Only one APK build should be scheduled after these source changes and tests are ready.


## 2026-11-06 — Cape SAF import route started
- Preflight: re-verified authorized branch `android` and read current continuity, development log, Cape Studio design, `catalog.rs`, `cape_library.rs`, `documents.rs`, `launcher.rs`, `launcher_pages.rs`, `game_launch.rs`, and `mobs/preview.rs`. Starting HEAD was `79016a33eb977410543adac56f4568cc17e5db49`; catalog's independent cape-library load and safe active-file resolver were present. Android Source Checks run `42180249685` passed for that starting HEAD.
- Applied `crates/horizon-client/src/android/documents.rs` in commit `59635acab50a0f6d29470f06d0b640636077c8dc`: added `ImportKind::Cape`, `begin_cape`, bounded SAF read using the cape library's 1 MiB cap, `CapeLibrary::import_png` validation/storage, and a distinct `cape_changed` notice. Picker result code remains the existing single SAF request 4101; Cape uses the existing PNG fallback name and no parallel request path.
- Applied `crates/horizon-client/src/android/launcher.rs` in commit `ea7f3379a5145013f0fdaaaf879072bdac04a65a`: cape import notices reload the cape library and return the launcher to Skin Studio, without changing the skin import flow.
- Re-fetched `documents.rs` and checked every Notice initializer includes `cape_changed`. Android Source Checks run `42182173409` was in progress for `59635ac`; run `42182185880` was pending for the latest commit `ea7f337`. Do not claim the new SAF path compiles until the latest-HEAD check completes.
- Not yet added: visible Skins/Capes tabs and cape library controls, active selection/deselect/rename/delete UI, dedicated 64×64 cape atlas slot, preview `render::cape` invocation, game launch argument/runtime texture application, or LAN appearance exchange. No APK build/device verification.
- Design guardrails retained: custom cape is a textile cape, not Elytra or skin overlays; None means no Sylphra override; no remote-player texture changes until a separate authenticated/validated Sylphra-to-Sylphra LAN appearance protocol is designed.
- Next: wait for checks on `ea7f337`, then add independent Skin/Capes tab state and management controls while keeping the 3D preview mounted across both tabs. In parallel, inspect the installed `entity-models.json` `player#cape` UV on device/build artifacts before committing preview UV assumptions. Continue with launch/runtime path only after the texture update lifecycle and renderer call site are verified.


## 2026-11-06 — Compile fix and Cape preview/runtime audit
- Preflight: verified branch `android`, HEAD `8661cef5f579e82aa26b1e34989d1a4b0cd8e7d3`, and re-read continuity, development log, Cape Studio design, `documents.rs`, `cape_library.rs`, `catalog.rs`, `launcher_pages.rs`, `launcher.rs`, `game_launch.rs`, `mobs/preview.rs`, `mobs/render.rs`, `game/remote.rs`, and `game.rs` before continuing.
- Confirmed APK workflow `42182555858` failed at Rust compilation because the generic `Notice` initializer in `documents.rs` omitted new field `cape_changed` (E0063). Applied the one-field initializer fix in commit `8661cef5f579e82aa26b1e34989d1a4b0cd8e7d3`.
- Android Source Checks run `42183821691` for `8661cef` completed `success`: `git diff --check`, public-tree/history check, Android Python unit tests, diagnostics, performance, content, and workflow export check all passed. The `native` job was skipped by workflow conditions. This is not an APK build or Android runtime test.
- Preview audit: `PlayerPreview::draw` currently renders only player-body vertices through `render_with`, then applies its GUI transform and clipping. `mobs::render::cape` exists and accepts a player root transform plus cape UV; it looks up `player#cape` and silently returns if the model layer is absent. The current preview function does not append cape geometry. The previously exposed `render::base_transform` is available, but the preview's GUI matrix is not identical to the world root transform; a preview-specific composition must be derived/tested before adding cape geometry.
- Runtime audit: `LaunchRequest` currently transports only `skin_file`/`skin_slim` through `--sylphra-skin` arguments; no cape argument exists. `update_remote_skins` resolves account/offline capes and assigns them to player mobs; the local offline cape is applied only when no account cape exists. No Sylphra custom-cape injection or LAN appearance protocol is present.
- Catalog already exposes `active_cape_file`, but the launcher page does not yet offer Cape selection controls, and the launch request builder does not yet pass a cape path. Avoid presenting the catalog foundation as an end-to-end feature.
- Important unresolved evidence: the installed/exported `entity-models.json` was not available in the inspected repository files, so exact `player#cape` UV range remains unverified. Do not hardcode the half-texture crop until the runtime asset can be inspected. The cape model layer may also be absent in a resource set; the UI/runtime should report that explicitly rather than silently claiming preview success.
- No new source changes were made in this audit pass. Next implementation should first add safe profile-level cape selection and management UI/actions, then wire `LaunchRequest` and parser/runtime loading, then add preview geometry using verified model UV and transform; LAN sharing remains a separate protocol task. Do not ask the user to compile until the full planned batch and source checks are complete.


## 2026-11-06 — Cape Studio interface wiring

- [Applied] Added `Skins` and `Capes` tabs to the existing Skin Studio, preserving the shared 3D player preview and existing profile controls.
- [Applied] Cape tab lists imported cape records with dimensions, exposes PNG import through the existing bounded Android document picker, and supports selecting a cape for the active appearance profile.
- [Applied] Added `None · keep account cape`, which clears only the Sylphra profile override via `select_cape(profile, None)`; it does not modify account cape data.
- [Applied] Added tab state to the Android launcher and action dispatch for import, selection, and clearing the profile cape.
- [Applied] Kept tabs on the title row after reviewing the compact-phone layout to avoid overlapping import/profile controls.
- [Not yet verified] No Rust build, APK build, emulator/device test, cape 3D preview, in-game cape rendering, rename/delete controls, or LAN appearance exchange was run in this change batch. The preview still uses the existing skin mannequin; cape geometry is a separate next stage.
- Commits: `5b33791` (studio tabs/list), `0df249c` (launcher tab state), `4a209f9` (tab placement correction).
- Next: run source checks / compile checks, then implement cape preview geometry and launch-time cape injection only after verifying the actual player cape UV mapping.

## 2026-11-06 — Cape Studio tab cleanup
- Verified branch `android` at `ca03019d0db3b1d3d38994e578063bf22444a951`; Android Source Checks run `42185225542` completed `success` for that exact HEAD. This workflow is source validation only, not an APK build or device test.
- Applied `crates/horizon-client/src/android/launcher_pages.rs` in commit `a4a2345afef942de6b1e9896f9d80661f28deffd`: hide the six skin-overlay visibility controls while the Capes tab is active and reclaim their vertical space for the cape list. The shared mannequin/preview remains visible.
- This is a UI consistency correction only. Cape texture is still not rendered on the mannequin, selected cape is not passed to the game launch request, and rename/delete controls, build/runtime validation, and LAN appearance exchange remain pending.
- Latest recorded source HEAD after this change: `a4a2345afef942de6b1e9896f9d80661f28deffd` (branch `android`). CI for this new commit is pending/not yet checked; no compilation or device test was run.
