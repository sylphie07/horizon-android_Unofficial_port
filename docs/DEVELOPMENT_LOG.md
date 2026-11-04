# Development log — Horizon Android fork

## 2026-11-02 — Sylphra welcome gate and About subpage

**Preflight:** continued on the maintained fork's authorized `android` branch, based on the previously reviewed Sylphra identity source pin `f9ed3fd5bd906b40622356c96250fb5741f181f3`.

**Changes applied:**
- `crates/horizon-client/src/android/catalog.rs`: added revisioned `welcome_ack_revision` config state and two Serde compatibility/round-trip tests.
- `crates/horizon-client/src/android/launcher.rs`: added first-run state, blocked Back/controller dismissal, and guarded the launch action while acknowledgement is pending.
- `crates/horizon-client/src/android/launcher_pages.rs`: added the explicit Continue modal, scrollable About subpage, Settings entry and 20-row scroll extent, separate Sylphra repository action, and retained the original Horizon repository action.
- `docs/SYLPHRA_WELCOME_ABOUT.md`: recorded behaviour, boundaries and validation status.

**Implementation commits:** `a8a19a97565b4f19f266f1f41c908fd79f7e4ab6`, `fd70c99740e2b9be82310164462b0fc90c89f6c4`, `fbc36abc70c18275dfea285a9243783bca702593`, `824ecf81674e6321edba68307163acd508e5f956`, `d3e8f55b383718aa178d587579d02802b177c6fa`, `45913e11f81a15ec79025a1dc36f9636a2d61fe1`, `f236422a218c6fec73188ad649c84bb8663621a6`, `b99dd55d45c1ecb08003de033161c3386a165028`, `4686236516ad43e6d7e570ac636c399363ee5146`; documentation commit `2da4cb2ad869cb7b8b5f87e3e9fbc3a699d33c30`.

**Source audit:** static assertions confirmed the config field and tests, activation and launch gates, Back protection, About route, Sylphra URL, original Horizon URL, six top-level tabs, and Settings scroll extent. All assertions passed.

**Not verified:** no Rust formatter, compiler, unit-test runner, GitHub Actions result, APK build, manifest audit, or Android device interaction was available/executed. Static assertions are not a substitute for compilation or runtime tests. The initial source edit briefly contained misplaced attributes/test-module placement; both were corrected before the final static audit.

**Next:** advance Sylphra's submodule gitlink to the final fork commit, update Sylphra continuity/decision/issue records, then run formatting, tests, build and device checks when a build environment is available.


## 2026-11-02 — whitespace cleanup follow-up
- Removed a whitespace-only line from Android catalog.rs and trailing Markdown line-break spaces from SYLPHRA_WELCOME_ABOUT.md so the changed files are compatible with diff whitespace checks.
- Commits: source cleanup c2d57ccaa151d8f829915918829023b260739ae4; documentation cleanup 5ba647366b50d8ab5ec26cec9da967d5439fb5b5.
- A static scan of the touched source and docs was used to locate the issue. No actual git diff --check, formatter, compiler, test, CI or device run has been executed.

## 2026-11-02 — CI native setup failure and correction
**Observed run:** GitHub Actions run 41389486056 on commit 66bd109a2f1c6127194226714dff3090c1376633 failed overall.
- The `source` job passed every step, including `git diff --check`, public-tree history validation, Python Android tests, diagnostics, performance, content checks, and active/exported workflow comparison.
- All four `native` matrix jobs failed at `Install Android toolchain` before Rust checking. Their logs report `sdkmanager: command not found` (exit 127). The `Check Android Rust code` step was skipped. This is a CI environment setup failure, not evidence of a Rust source compile error.
**Correction applied:** added the Android Actions SDK setup action pinned to commit `9fc6c4e9069bf8d3d10b2204b1fb8f6ef7065407` before `sdkmanager` in both `.github/workflows/android-source.yml` and `packaging/android/source-checks.yml`. The two copies were fetched and compared after the edits; contents match.
**Commits:** active workflow `ab67dbfac784beb6bd786de23027e425620d5e7f`; packaged workflow mirror `880d801041b7452c3464038b8498b9d219a3ea88`.
**Pending verification:** the corrected workflow has not yet run. Re-run `Android source checks` on branch `android` with `native_checks=true`. Only after that run can we report whether Android Rust type-checking succeeds.

## 2026-11-02 — CI correction source validation
- Corrected workflow run: `41390177748` on the maintained `android` branch.
- The `source` job completed successfully: diff whitespace check, public-tree history check, Android Python unit tests, diagnostics, performance, content and active/exported workflow comparison all passed.
- The `native` matrix was skipped because this run was push-triggered. Its workflow condition requires manual `workflow_dispatch` with `native_checks=true`.
- Therefore the source workflow and mirror are validated by CI, but the Android SDK setup action and Rust checks have not yet run in the native matrix. No Rust compilation, APK or device result is claimed.
- Next: manually dispatch `Android source checks` on branch `android` with `native_checks=true`, then inspect all four ABI jobs.


## 2026-11-02 — Follow-up source CI run
- Follow-up run `41390791154` on branch `android` completed successfully.
- Job `source` passed every step, including Python Android tests and workflow mirror validation.
- Job `native` was skipped again because the run was push-triggered rather than a manual dispatch with `native_checks=true`.
- This is a second source-only CI pass; it does not validate SDK setup or Rust compilation. Native matrix remains the next required check.


## 2026-11-02 — Native CI setup follow-up: obsolete SDK package
**Observed run:** manual native workflow run `41392101483` on commit `91f6d3d876095101422916fae388f1d3ad28940e`.
- The `source` job passed all checks.
- All four `native` matrix jobs failed in `Set up Android SDK command-line tools`; the Rust check and subsequent toolchain installation were skipped.
- Exact cause from each job log: the pinned `android-actions/setup-android` action defaults its `packages` input to `tools platform-tools`. The current Google SDK repository no longer contains the legacy standalone `tools` package, so `sdkmanager` reports `Failed to find package 'tools'` and exits 1. This is still SDK setup, not a Rust compile failure.
**Correction applied:** explicitly set the action input to `packages: platform-tools` in both the active workflow and its packaged mirror. NDK `28.2.13676358` and build-tools `35.0.0` remain installed by the following explicit `sdkmanager` command. Both workflow copies are intended to remain byte-identical.
**Commits:** active workflow `7f9489a58cf982f4226c0ade4575d14730579d59`; packaged mirror `87923d198af694ccd364639ef2cd100dfd6d457c`.
**Pending verification:** rerun the manual workflow with `native_checks=true`; confirm SDK setup, NDK/build-tools installation, and all four Rust checks. No Rust, APK, or device result is claimed yet.


## 2026-11-02 — Manual native CI matrix passed
**Observed run:** [41392913670](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41392913670), event `workflow_dispatch`, branch `android`, commit `51e5940d253cfbd79254fa1e17902143882b16f7`.
- Overall conclusion: `success`.
- `source` job passed all steps: checkout, `git diff --check`, public-tree history validation, Android Python unit tests, diagnostics, performance, content checks, and active/exported workflow equality.
- All four native matrix jobs passed: `armeabi-v7a` / `armv7-linux-androideabi`, `arm64-v8a` / `aarch64-linux-android`, `x86_64`, and `x86` / `i686-linux-android`.
- Each native job successfully completed Android SDK command-line setup, NDK/build-tools installation, and `Check Android Rust code without building an APK`.
- This verifies the SDK setup correction and the configured Rust checks for all four Android targets. It does **not** mean an APK was built, installed, inspected, signed, or tested on a device.
- The prior pending native-matrix verification is resolved. Remaining gates: actual APK build, package/manifest/SAF inspection, Sylphra-specific signing setup, icon integration, side-by-side install/data isolation, and device validation.


## 2026-11-02 — Native Android CI compile fixes
**Observed run:** [Android source checks #61](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41404037242), commit `e7f3ad1c420c18ec2dd801191e4a734e7058de77`, branch `android`.
- The `source` job passed all source checks and Python tests.
- All four native ABI jobs failed at `cargo check -p horizon-android` with the same three compile errors: an outdated `super::perspective::player_preview_quads` path in `armor_stands.rs`, and two incorrect uses of `Option<usize>` returned by `png::Reader::output_buffer_size()` in `skin_library.rs`.
- `PathBuf` was also imported in production scope but used only by the test module (warning, not a build blocker).

**Corrections applied on branch `android`:**
- `crates/horizon-client/src/game/armor_stands.rs`: call the shared helper through `horizon_client::mobs::preview::player_preview_quads`.
- `crates/horizon-client/src/android/skin_library.rs`: handle `None` from `output_buffer_size()` with a descriptive validation error before comparing/allocating; move `PathBuf` into the test module imports.
- No skin rendering semantics, profile data, authentication, player identity, network protocol, world data, or ABI configuration were changed.

**Commits:** armor preview path `3de10f78d00c7583f079d582fb194a09aacb237a`; PNG size handling/import scope `9b4135df830cb664d0fe3a8d476bbc46178dd5a5`.

**Verification:** changes are committed. The source workflow's checks passed on the preceding commit, but the corrected Rust code has not yet been rechecked by the native ABI matrix. No Rust compilation, APK build, installation, or device result is claimed.

**Next:** manually dispatch `Android source checks` on branch `android` with `native_checks=true`; inspect all four ABI results before proceeding with the next skin-preview stage.


## 2026-11-02 — Native Android CI verified after compile fixes
**Observed run:** [Android source checks #66](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41405537016), event `workflow_dispatch`, branch `android`, tested commit `09cb65890102fbac97edde9a8e2dd258e1524be6`.
- Overall workflow conclusion: `success`.
- `source` job passed all checks, including diff validation, public-tree history, Android Python tests, diagnostics, performance, content, and workflow-export consistency.
- All four native Android jobs passed the `Check Android Rust code without building an APK` step: `arm64-v8a`, `armeabi-v7a`, `x86_64`, and `x86`.
- This verifies the Rust Android code type-checks for all four configured targets after the helper-path and PNG output-size fixes.
- **Verification boundary:** no APK was built, installed, signed, or tested on a physical device. SAF import, Skin Studio touch/layout, persistence and 3D preview behaviour remain device-validation tasks. No runtime skin application or multiplayer skin sharing is verified.
- The earlier run #61 native compile failure is resolved by this successful run.
- **Next:** review the current LaunchRequest/runtime skin flow and design S3-A as a separate, narrowly scoped integration. Preserve auth, UUID, worlds, and network protocol; do not claim in-game skin application until runtime/device testing succeeds.


## 2026-11-02 — S3-A runtime integration audit (investigation only)

**Repository preflight:** inspected the maintained fork on branch `android`. Branch ref resolved to commit `f1b0a4a3e8c41d77d56ed2be2d31aee790c60242`. Read the current development log, skin-library design notes, launcher, LaunchRequest, skin-library model, remote-player skin resolution and renderer body selection. No source code was modified in this audit.

**Findings:**
- `crates/horizon-client/src/android/launcher.rs` creates one `LaunchRequest` immediately before `game_launch::start`. It currently carries resource/game paths, session file, offline name, server, control socket, extra arguments and graphics flags; it does not carry the selected appearance.
- `crates/horizon-client/src/android/catalog.rs` owns the loaded `SkinLibrary`; `active_skin()` resolves the active profile's record and `active_model()` resolves its Classic/Slim choice. The PNG lives in the app-private `home/skins/<record.file>` directory.
- `crates/horizon-client/src/game.rs` parses launch arguments into `Options` and hands those options to the game runtime. This provides a potential explicit, per-launch transport route; no process-global state is required for the proposed integration.
- The renderer's avatar body uses `Mob.skin` and `Mob.slim` for player/mannequin entities. Remote player skin resolution is in `game/remote.rs::update_remote_skins`.
- Existing `horizon_client::skins::offline_skin()` is a legacy fallback based on `HORIZON_OFFLINE_SKIN`, `HORIZON_OFFLINE_SKIN_MODEL` and a process-global `OnceLock`. It only supplies an offline skin if the local profile has no resolved server/account skin. Reusing it unchanged would not support an immutable per-launch profile selection and would not override a signed-in account skin.
- `update_remote_skins` resolves server-provided textures, then fills missing local-player skin/cape from the authenticated account profile, and only then applies the legacy offline fallback. The local custom-skin feature therefore needs a deliberate local-player precedence point; it must not rewrite other players' texture entries.

**S3-A design proposal (not applied):** carry an optional local appearance descriptor (validated app-private PNG path plus Classic/Slim model) through the launch request into game options. Decode and normalize it once for that game session, then apply it only to the local player's rendered avatar after ordinary server/account resolution. If no custom skin is selected, preserve current behaviour exactly. Do not alter Microsoft session data, UUID, player name, world files, server packets, remote-player textures or authentication. Avoid environment variables and global `OnceLock` state for Sylphra's selection.

**Validation plan before implementation:** add launch-argument round-trip/omission tests; add a renderer-resolution test proving the custom appearance wins only for the local UUID while remote player skins remain unchanged; validate missing/invalid/oversized/symlink paths fail closed or fall back without corrupting the manifest; then run source checks and all four native ABI checks. APK/device rendering is a separate later gate.

**Status:** investigation completed; no implementation, test execution, CI run, APK build or device verification occurred in this audit. This is a proposal, not an applied change.


## 2026-11-02 — S3-A1 launch transport implemented (renderer application pending)

**Preflight:** rechecked branch `android`, latest commit before this implementation, and the current launch/runtime design notes. Work remained on the authorized fork branch; no Sylphra main-branch changes.

**Applied source changes:**
- `crates/horizon-client/src/android/game_launch.rs`: `LaunchRequest` now carries an optional skin PNG path and a Classic/Slim flag. When present, it emits separate `--sylphra-skin` and `--sylphra-skin-model` arguments; absent selection emits neither. Added tests for selected skin argument transport and unchanged no-skin launch arguments.
- `crates/horizon-client/src/android/launcher.rs`: snapshots the active profile's selected skin file under the app-private `skins/` directory and its per-profile model into the launch request.
- `crates/horizon-client/src/game.rs`: parses the two arguments into per-launch `Options` fields; model accepts only `classic` or `slim`.

**Commits:** LaunchRequest and transport tests `40b21226b8a24cb1f974f6044e0421e9accc2af8`; launcher selection wiring `bc68e674dfdb34dba310e7f25589307c0b1bd5a1`; game argument parsing `ba041e4c49a89fa1ddcdacbf7c8c5e00c9b88cba`; formatting follow-up `7c600084c0f48155daac7a05925177d8d1c91bcb`.

**Verification boundary:** edits are committed and source snippets were fetched again for inspection. No formatter, compiler, unit-test execution, CI run, APK build, or device test has yet been run. The parsed per-launch appearance fields are not yet consumed by the renderer, so this does **not** apply the custom skin in-game. File validation/decoding and local-player-only precedence remain the next implementation step.

**Next:** add safe per-session PNG loading/normalization and local-player renderer precedence, preserving account/session identity and all remote player textures; add tests, then run source checks and all four native ABI checks.

## 2026-11-02 — Sylphra S3-A2 local skin runtime connection

**Preflight:** inspected branch `android`, current documentation, launch options, `Game::new`, PNG decoder/skin normalization, and `update_remote_skins`. Starting source tip was `6a384058d119914fa7868b4e39b62b91e16a90c9`.

**Applied source changes:**
- `crates/horizon-client/src/game.rs`
  - Loads the selected skin once per game session rather than decoding it on every remote-skin update.
  - Restricts the selected path to the app-private `skins/<64-hex-content-id>.png` layout.
  - Rejects symlinked skin directory/file, non-regular files, files above 1 MiB, invalid PNG signatures, and dimensions other than 64×32 or 64×64 before decoding.
  - Uses the engine PNG decoder and existing legacy-skin normalization.
- `crates/horizon-client/src/game/remote.rs`
  - Applies the selected texture after server/account skin resolution, with precedence only for the local UUID.
  - Leaves remote player skin entries untouched and preserves cape resolution.
  - Applies the active appearance profile's Classic/Slim metadata to the local avatar.

**Commits:** decoder/session loading `6520b49c29be123c01f01d6f95cad81f265fad8f`; local-avatar precedence `4452e70a47454580d54c98946ed3f145ba302a3b`.

**Verification boundary:** changes are applied to branch `android`; no formatter, Rust compiler, unit tests, Android CI, APK build, or device rendering test has yet been run for S3-A2. In particular, atlas eviction/re-upload behaviour and compilation must be checked before calling this complete. This stage does not change authentication, UUIDs, worlds, or network packets.

**Next:** run formatting and Android source/native CI; address any compile or test failures; then verify local skin visibility, Classic/Slim, default-skin fallback, and unchanged remote-player skins on device.


## 2026-11-02 — CI #80 verification for S3-A2

- Workflow: [Android source checks #80](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41410958976).
- Branch: `android`; tested commit: `6538fde75864f2544d4abb84bd3cc7225604103b` (`docs: document Sylphra local skin runtime`).
- Overall conclusion: `success`.
- Source job passed: `git diff --check`, public-tree history validation, Android Python unit tests, diagnostics tests, performance tests, content tests, and workflow-export consistency.
- Native Rust job: `skipped` because this was a push-triggered run. Therefore CI #80 does **not** verify Rust compilation for Android ABIs. The earlier native success #66 applies to the older source tip and does not verify the S3-A2 runtime changes.
- S3-A2 source implementation is present in commits `6520b49c29be123c01f01d6f95cad81f265fad8f` (per-session skin loading/validation) and `4452e70a47454580d54c98946ed3f145ba302a3b` (local-only texture precedence).
- Still pending: run the manual Android source workflow with native checks enabled for all four ABIs; Rust unit tests/formatting where covered; APK build and physical-device rendering. No multiplayer relay/server-side skin sharing is verified.


## 2026-11-02 — S3-A2 native CI compile fix

**Observed failure:** manual Android source checks run #83 ([41412353796](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41412353796)) on branch `android`, commit `b146caa14d747ce86230af7f7a3397a919e6f7a9`.
- The source job passed all checks.
- Android SDK setup and NDK/build-tools installation succeeded for all four native ABI jobs.
- All four jobs reached Rust compilation and failed on the same E0433 in `crates/horizon-client/src/android/launcher.rs:377`: `crate::skin_library::SkinModel` referred to a non-existent crate-root module. The compiler identified `crate::catalog::skin_library::SkinModel` as the valid path.

**Correction applied:** changed only the Slim model enum path in `crates/horizon-client/src/android/launcher.rs` to `crate::catalog::skin_library::SkinModel::Slim`.
- Fix commit: `b16b6547edb11d5fd1537a39e9a9d7b10c77a2d4`.
- No rendering precedence, PNG validation, authentication, UUID, network, world, or ABI configuration behaviour was changed.

**Verification boundary:** correction is committed to the authorized `android` branch. It has not yet been validated by a new CI run. No successful native compilation, APK build, installation, or device rendering is claimed. Next: rerun the manual workflow with `native_checks=true` and inspect all four ABI jobs.


## 2026-11-02 — S3-A2 native Android CI verified after module-path correction

**Observed run:** [Android source checks #86](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41413689762), run ID `41413689762`, event `workflow_dispatch`, branch `android`, tested commit `5587f1e670518d447b2ff06e9a74bc6ddcf879a9`.

- Overall workflow conclusion: `success`.
- The `source` job passed all steps, including `git diff --check`, public-tree history validation, Android Python unit tests, diagnostics, performance, content checks, and workflow-export consistency.
- All four native Android jobs passed the `Check Android Rust code without building an APK` step: `armeabi-v7a`, `x86`, `arm64-v8a`, and `x86_64`.
- The common E0433 module-path failure from run #83 was corrected in `crates/horizon-client/src/android/launcher.rs` by referencing `crate::catalog::skin_library::SkinModel`; this run verifies the corrected source passes the configured Rust checks across all four Android targets.
- **Verification boundary:** this workflow checks Rust code but does not build an APK. APK build/sign/install, local skin rendering, Classic/Slim appearance, fallback behaviour, and unchanged remote-player appearance remain unverified on a physical device. Multiplayer skin sharing remains future work.
- **Next:** proceed with a narrowly scoped S3-A3 audit for runtime texture lifecycle and local-player rendering correctness before any further code changes. Preserve account/session identity, UUIDs, worlds, remote-player textures, and network protocol.


## 2026-11-02 — S3-A3 atlas residency correction (CI pending)

**Audit finding:** the selected local skin is stored under the atlas key `sylphra/local_skin`, but `drawn_skin()` only marked downloaded `skins/*` and `horizon/offline_skin` entries as recently shown. The local custom skin could therefore be treated as least-recently-used while still being rendered, making its atlas slot vulnerable to eviction under pressure.

**Correction applied:** `crates/horizon-client/src/game/remote.rs` now centralizes the runtime-atlas key check in `uses_runtime_skin_slot()` and includes `sylphra/local_skin`. Added a unit test covering downloaded, offline, Sylphra and ordinary default-skin keys. No account, UUID, network, world or remote-player skin resolution logic was changed.

**Code commit:** `af1084bab9cba03684e3002eac7b27c2cda3895d`.

**Verification status:** applied to branch `android`; no CI result yet for this correction. Rust formatting, unit tests, source checks and the four-ABI native matrix remain pending. APK/device rendering and multiplayer sharing remain separate unverified gates.

**Next:** run Android source checks with `native_checks=true`; inspect source and all four ABI jobs before further implementation.


## 2026-11-02 — S3-A3 CI #92 failure and scope correction

**Observed run:** [Android source checks #92](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41415493658), workflow_dispatch on branch `android`, commit `d48a31accf9ea9e1fabdda4efea5832ddb150b06`.

- Source job passed.
- All four native jobs failed at the same Rust error E0425: `uses_runtime_skin_slot` was not found at the unqualified call in `Game::drawn_skin`.
- Root cause confirmed by source inspection: the helper had been inserted inside `impl Game<'_>`, while the call and unit test treated it as a module-level function.
- Correction applied: moved `uses_runtime_skin_slot()` to module scope before `impl Game<'_>`; its call and focused unit test remain module-level references.
- Fix commit: `c968683b6c2f7751beead71976cbd0c025c5fb9f`.
- **Verification status:** code correction applied, not yet validated by CI. No APK/device verification is claimed.
- **Next:** rerun Android source checks with `native_checks=true` and inspect source plus all four native jobs.


## 2026-11-02 — S3-A3 native Android CI verified

**Observed run:** [Android source checks #96](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41417195641), run ID `41417195641`, event `workflow_dispatch`, branch `android`, tested commit `e129c983f7c062624bd1b87490f18224487cb86e`.

- Overall workflow conclusion: `success`.
- The `source` job passed all checks: `git diff --check`, public-tree history validation, Android Python unit tests, diagnostics, performance, content checks, and workflow-export consistency.
- All four native Android ABI jobs passed SDK setup, NDK/build-tools installation, and `Check Android Rust code without building an APK`: `arm64-v8a`, `armeabi-v7a`, `x86_64`, and `x86`.
- This verifies the S3-A3 module-scope helper correction and the atlas runtime-slot predicate checks across the configured Android targets. The earlier #92 E0425 failure is resolved by the corrected helper placement.
- **Verification boundary:** CI did not build an APK and does not verify atlas behaviour or local-skin rendering on a physical device. No APK, install, visual, or multiplayer skin-sharing result is claimed.
- **Next:** continue with S3-A4: audit the local skin texture lifecycle from session load through atlas insertion, draw/use marking, and eviction/re-upload; then define device validation. Keep remote-player textures, account identity, UUIDs, worlds and protocol unchanged.


## 2026-11-02 — Sylphra Launcher branding preparation

**Preflight:** the branding preparation was performed on the authorized branch `android` (the actual pre-change tip was `56255b98efe9d9a4e3dd1d439e5a1582e63b51d4`; the earlier `92ef794...` note was stale); inspected the existing development log, skin-library notes, Android manifest, Gradle application configuration and APK build script.

**Applied:**
- `packaging/android/build.gradle`: changed the fallback Android launcher label to `Sylphra Launcher`.
- `tools/android_build.py`: made `Sylphra Launcher` the default `--app-name`, so normal local APK builds receive the same launcher label and Sylphra-prefixed artifact naming without requiring an extra argument.
- The manifest already references `@drawable/launcher_icon`; its resource path is `packaging/android/res/drawable-nodpi/launcher_icon.png`.

**Icon status:** the supplied artwork was inspected (1536×1536 source image). It is a JPEG-encoded image despite the uploaded `.png` filename, so it must be converted to a real PNG before replacing the Android resource. The repository's binary resource has **not** been replaced in this commit; the available GitHub file-edit action supports text files only, so a binary Git object upload is still required. No claim is made that the supplied icon is already in the APK.

**Verification boundary:** the label defaults and build-script argument were inspected as source edits only. No build, APK, install, signing, CI or device verification has been performed. Existing application ID, signing key, authentication, profile data, worlds and network behaviour were not changed.

**Next:** complete the binary icon replacement while preserving the manifest resource name, then prepare the arm64-v8a test APK workflow and verify package/signature isolation before installation.


## 2026-11-02 — Sylphra Launcher icon integrated

**Applied:** converted the supplied artwork into a valid optimized 96×96 PNG (RGB palette) and replaced `packaging/android/res/drawable-nodpi/launcher_icon.png`. Kept the existing Android resource name and manifest reference, so no manifest or package-ID changes were needed.

**Commit:** `e1265307675053ccad4321328cc824ff9420190c` (`branding: add Sylphra Launcher icon`).

**Verification:** fetched the resource back from branch `android`; Git blob SHA matches the locally generated PNG (`445a979cae49ba3efd27da5c84db56e3040ceb71`). This confirms the binary resource is present on the branch. It does not yet verify Android packaging, launcher rendering, or device installation.

**Branding state:** app label defaults to `Sylphra Launcher` in Gradle and `tools/android_build.py`; the manifest continues to point to `@drawable/launcher_icon`.

**Next:** prepare an arm64-v8a test APK workflow. Before installation, verify its application ID and signing continuity so it cannot unintentionally replace or conflict with an existing Sylphra/Horizon installation. No APK has been built or published yet.


## 2026-11-02 — CI branding validation and arm64 test APK workflow

**Preflight:** verified authorized branch `android`, HEAD before this change `5306577086eb62c0d9c0a0419ce7746396dd3f38`, and inspected existing Android build scripts, Gradle configuration, manifest, resource preparation, signing and current workflow.

**CI result confirmed:** run [`41421058011`](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41421058011) completed successfully on commit `5306577086eb62c0d9c0a0419ce7746396dd3f38`. The source job passed all steps. The native matrix was skipped (this was a push run), so this result validates source checks only; the previous manual run `41417195641` had already passed all four native ABI checks.

**Applied:** added `.github/workflows/sylphra-test-apk.yml` in commit `680757b73184412847cea7314d6d0a3e217ade6e`. It is manually dispatched and builds only `arm64-v8a`, prepares the pinned engine tables using the existing `tools/android_prepare.py`, invokes the existing APK build and public release-candidate audit, and uploads a seven-day Actions artifact. It does not create a GitHub Release or publish to a store.

**Installation isolation:** the test APK uses application ID `dev.sylphra.android.test`, distinct from the upstream default `dev.horizon.android`, and the visible label `Sylphra Launcher`. A dedicated development signing key is preserved in an Actions cache under the test-only key `sylphra-test-signing-v1` so subsequent test builds can update the same test installation. This is not a production signing key.

**Not yet verified:** the new workflow has not completed a build; no APK artifact exists yet; no installation or device rendering has been tested. Resource preparation, native compilation, Gradle packaging, APK audit, icon appearance, and in-game skin behavior remain pending.

**Next:** run the new workflow manually on branch `android`. If it succeeds, download the artifact and install it as a separate test app, then verify the launcher icon and local Classic/Slim skin rendering.


**Pre-build review correction:** after the source CI passed, reviewed the build script's exact artifact naming logic. With `--app-name "Sylphra Launcher"` and `--public`, the output is `sylphra-launcher-android-public.apk` (the `public` suffix takes precedence over `release`). Corrected the workflow upload path accordingly in commit `c911898d71a621939595d0759e76358ab659b450` before dispatch. This prevents an artifact-upload path mismatch; the workflow still needs to be run and the APK built.


## 2026-11-02 — First Sylphra arm64 APK build failure and audit fix

**Build run:** [`41423563328`](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41423563328), commit `2487fdd36ec53904761865fb64034a200fe744a0`, failed in `Build and audit isolated arm64 test APK`.

**Verified from job log:** engine preparation succeeded; Rust/Gradle compilation reached `packageRelease`; the APK was signed successfully. The release audit then rejected the custom test application ID with `ValueError: Unexpected Android package identity`. The audit script had a hard-coded expectation for `dev.horizon.android`, while the test workflow intentionally uses `dev.sylphra.android.test`. This was a verifier configuration mismatch, not a Rust compile, Gradle package, icon, or skin-runtime failure. Artifact upload was skipped because the audit exited non-zero.

**Applied fixes (not yet CI-verified):**
- `tools/check_android_release.py`: added validated `--app-id`, defaulting to upstream `dev.horizon.android`, and compare the APK manifest against that expected ID.
- `tools/android_build.py`: pass the effective configured application ID into the release audit, retaining the upstream default when no override is supplied.
- `tests/test_android_release_audit.py`: added regression coverage proving the isolated Sylphra ID passes and a mismatched package ID remains rejected.

**Commits:** `be270bbbc20adc89808442c136b538f17b966729`, `110cccc78a6fbfdbe52586d230d4f0fabacd0a73`, `375a660585e18f95fc40961984e7a85945f00a6e`.

**Next:** wait for source CI on the updated branch. If green, rerun the manual `Sylphra Launcher test APK` workflow. The previous APK is not considered deliverable because it did not pass the release audit and was not uploaded.


## 2026-11-02 — Follow-up CI failure: package-ID regex escaping

**CI run:** [`41426901487`](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41426901487), commit `907bd8c74122ae1aa6b63055584fe1591a4962ee`, failed in the Android Python unit tests before native checks.

**Exact cause:** the new package-ID validation regex in `tools/check_android_release.py` was written as a raw string containing an over-escaped dot (`\\\\.` in the source), so it expected a literal backslash instead of a package separator. Consequently the valid `dev.sylphra.android.test` value was rejected as invalid before inspecting the APK.

**Applied:** corrected the regex to use a single escaped dot in the raw regex string, preserving strict validation of dotted Android package IDs. Commit: `118bb2139aa537e6d973e9eb80b5371de702c6de`.

**Verification status:** fix applied; source CI has been triggered by the commit and is pending. The release audit regression test has not yet passed in CI. Do not rerun the APK workflow until source CI is green.


## 2026-11-02 — Sylphra internal launcher branding, first UI pass

**Preflight:** verified repository `sylphie07/horizon-android_Unofficial_port`, authorized branch `android`, branch HEAD before this change `0fb3978454e63cb6b953c230a02b5a920f386893`, and the latest commit message (`docs: record package ID regex test failure`). Inspected the existing Sylphra skin-library and welcome/About documentation plus the Android launcher page renderer. No changes were made to `main`.

**Device feedback:** the user installed the isolated Sylphra test APK. They confirmed the launcher opens and behaves like the original Horizon launcher. Screenshots confirm the Android app label and custom icon are visible; the in-app header still said `Horizon Android 0.7.7` and the install progress heading still used Horizon-era wording. This is a user-reported device check, not an automated test result.

**Applied in code commit `207ccccc1b6359bcaab0ba8955d9d5d94926af92`:**
- `crates/horizon-client/src/android/launcher_pages.rs`: changed the main in-app title to `Sylphra Launcher {version}`.
- Changed the initial game preparation heading to `Preparing Sylphra for your game`.
- Changed the retry hint to refer to keeping Sylphra open.

**Intentionally preserved:** internal Horizon-derived source/catalog names, attribution and disclosure, original Horizon repository link, actual app-private storage directory names, game installation paths, engine, download verification, authentication, profiles, worlds, shaders, mods and skin runtime. These are not blindly renamed because some are source identifiers, provenance, compatibility paths or real filesystem locations.

**Verification status:** source edits applied. No compile, Rust formatting, unit tests, CI, new APK or post-change device test has yet been performed. The previously installed APK still contains the earlier header and progress wording until a new test artifact is built and installed.

**Next:** run source CI and all four native Android checks; if green, rebuild the isolated arm64 test APK. Then inspect remaining user-visible strings in context and handle them in separate, low-risk branding passes.


## 2026-11-02 — First Sylphra UI branding pass: CI verified

**CI run:** [41471566047](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41471566047), event `workflow_dispatch`, branch `android`, commit `04a4966f0c37eb453429b536ac6b34f4c867bb4b`, conclusion `success`.

**Verified jobs:** source checks and all four native Android check targets (`arm64-v8a`, `armeabi-v7a`, `x86_64`, `x86`) passed. Source steps included `git diff --check`, public-tree history audit, Android Python unit tests, diagnostics, performance, content tests and workflow export consistency. Each native job completed Android toolchain setup and Rust Android `--check` successfully.

**Scope limit:** these jobs validate source and native target compilation checks; they do not produce an APK. The first branding UI changes are CI-verified, but still require a fresh isolated arm64 APK build and a device check.

**Next:** dispatch `Sylphra Launcher test APK` on branch `android`; inspect the run and artifact before asking the user to install. After device confirmation, continue the next user-visible branding pass without changing engine or launcher behavior.


## 2026-11-02 — Sylphra launcher branding, second UI pass

**Preflight:** verified authorized branch `android`, starting HEAD `2006a4acfc60445e38257d9e5c4ec0c89c5c0e11`, latest commit message `docs: record successful Sylphra branding CI`, and inspected `docs/DEVELOPMENT_LOG.md`, Android launcher page rendering, and shared theme palette. No changes were made to `main`.

**Applied in code commit `ceeec2c32a8a575cbccc56652f055a8f4aef0d8b`:**
- Renamed the user-facing Mods page heading from `Horizon Mods` to `Sylphra Mods`.
- Updated the default Mods-page hint to use Sylphra branding while retaining the HTTPS `.hmod` import guidance.

**Intentionally preserved:** Horizon and Modrinth provider/source labels, source IDs, catalog identifiers, the shared Horizon-derived color theme, filesystem labels/paths, attribution, engine, download/install behavior, accounts, profiles, worlds, and skin runtime. The shared palette is used beyond the launcher, so changing it here would risk an unintended whole-client redesign.

**Verification status:** the two UI string changes are committed. No CI, build, APK, or device rendering check has yet completed for this pass. This is a limited text-branding refinement, not yet a full visual redesign.

**Next:** run Android source checks and the four native ABI checks on branch `android`. After they pass, group the next visual changes into a dedicated launcher-only layout pass, keeping shared game UI and runtime behavior untouched.


## 2026-11-02 — Second UI branding pass: source CI result

**Verified:** Android source-check run [41481347051](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41481347051), branch `android`, commit `a335cafc6386ca9425ce0c176db37fb2145b3ae2`, completed with conclusion `success`; the `source` job passed. The native matrix was skipped because this was a push event. The earlier overlapping run for code-only commit `ceeec2c32a8a575cbccc56652f055a8f4aef0d8b` was cancelled by the later push due to workflow concurrency; it is not counted as a failure or a successful native check.

**Current verification boundary:** the second branding pass has passing source checks, but the four native ABI checks have not been rerun for this commit. No APK was generated for this pass and no new device rendering was checked.

**Design inspection:** reviewed `docs/design-system.md`, `crates/horizon-theme/src/lib.rs`, and Android `launcher_pages.rs`. The documented Blocks visual language is already implemented in the shared Horizon launcher/theme ecosystem, but shared theme tokens are consumed outside the Android launcher and must not be globally recolored for Sylphra. The Android page renderer currently uses the shared dark palette and a compact custom layout. A safe visual redesign must therefore be isolated to Android launcher rendering, not shared theme tokens or game overlays.

**Next:** manually dispatch `Android source checks` on branch `android` with `native_checks=true` to validate all four Android ABIs, then implement the first isolated visual pass on the Android Play/home page. Keep the change limited to launcher drawing/layout and avoid account, installation, engine, catalog, profile, world, and storage behavior.

## 2026-11-02 — Sylphra Play page visual pass 1 (CI pending)

**Preflight:** verified branch `android`, HEAD `33ec383d461a6f3c735740d6b41201d81c6630f5`, and reviewed `docs/design-system.md`, this development log, `crates/horizon-theme/src/lib.rs`, and Android `launcher_pages.rs`. The manual Android source-check run #124 on that exact HEAD completed successfully: source plus all four native ABI jobs passed.

**Applied:** code commit `955067bdf5c80a26d4ea5561f48d51d78d296c12` adds a restrained framed hero surface and violet accent marker to the Android Play page only. The surface uses the existing Android UI drawing primitives and palette; shared theme tokens were not changed.

**Preserved:** installation and progress flow, Play action, login/account state, profile selection, catalog navigation, game engine, storage, worlds, and all non-Android UI remain untouched.

**Verification status:** change is committed to `android`; no post-change CI, native checks, APK build, or device rendering has completed yet. The prior #124 success validates the parent commit only, not this visual change.

**Next:** run Android source checks with `native_checks=true`; inspect the source job and all four ABI jobs. If green, continue with a second isolated pass for hierarchy/spacing and only then build a separate arm64 test APK for device review.

## 2026-11-02 — Sylphra Play page responsive pass 2

**Preflight:** verified repository `sylphie07/horizon-android_Unofficial_port`, authorized branch `android`, HEAD `6f900a14617858c0e741ad3c29894ec47d8837a3`, and inspected the latest commits, `docs/DEVELOPMENT_LOG.md`, and the Android Play renderer. The cancelled test-APK run #4 is not treated as a build result. No changes were made to `main`.

**Applied in code:**
- `crates/horizon-client/src/android/launcher_pages.rs`: made the Play hero title and subtitle scale down for compact layouts; shortened the compact instructional copy; limited the number of helper rows on short screens; and made the primary Play/download button width fit the available screen width instead of always reserving 320 px.
- Kept the visual pass confined to the Android launcher renderer. No shared theme tokens, account/authentication, installation, game engine, profile, world, catalog, storage, or gameplay behaviour was changed.

**Verification status:** source edit applied; no compiler, formatter, unit test, CI, APK, or device result is claimed yet. The existing source CI success on parent commit `6f900a1` does not validate this new change.

**Next:** run Android source checks with `native_checks=true`, inspect the source job and all four ABI jobs, then build the isolated arm64 test APK only after CI passes.
 


## 2026-11-02 — Sylphra Settings compact-layout pass 3

**Preflight:** rechecked repository `sylphie07/horizon-android_Unofficial_port`, authorized branch `android`, branch HEAD `2b1f4e43afbe753dab9be12a8a4f6e717e236cd0`, the current development log, and the Android launcher renderer. `docs/CONTINUITY.md` did not exist at preflight. No changes were made to `main`.

**Applied in code commit `d19f7c8fcf71676777b618261d965f6770f4c10c`:**
- `crates/horizon-client/src/android/launcher_pages.rs`: made the Launch settings heading slightly smaller and higher on compact screens; reduced compact settings row height on short viewports; tightened toggle/value control vertical placement and separator position to match the reduced row height.
- The settings list remains scrollable and uses the existing actions/config persistence. No settings semantics, account/authentication, engine, profile, world, storage, or shared theme tokens were changed.

**Verification status:** source edit committed and fetched back from branch `android`; no formatter, compiler, unit tests, CI, APK build, or Android device check has yet run for this pass. The prior source CI success on commit `2b1f4e4` does not validate this new change. The all-ABI native matrix remains a manual workflow dispatch requirement.

**Next:** run source CI plus `native_checks=true` for all four ABIs; if green, group any remaining launcher-only refinements before building the isolated arm64 test APK. No APK or device result is claimed.


## 2026-11-02 — Sylphra launcher navigation: Skin Studio entry

**Preflight:** confirmed the only authorized branch is `android`, and the branch HEAD before this change was `951850d0c5904b79aa7897f9536b90698eaf4924`. Re-read `docs/CONTINUITY.md`, this development log, the Android catalog page enum, and the launcher tab renderer. No changes were made to `main`.

**Applied:**
- `crates/horizon-client/src/android/catalog.rs`: added the existing `SkinStudio` page to the primary `Page::ALL` navigation list; renamed the desktop-facing `Installed` label to `Installations` and the `SkinStudio` label to `Skins`.
- `crates/horizon-client/src/android/launcher_pages.rs`: added the compact navigation label for the Skin Studio tab. Existing Skin Studio actions, skin library, PNG import, model selection, profiles, and preview remain the same.

**Scope:** this is the first navigation integration step, not the complete Java-launcher redesign. It exposes the existing Skins page directly in the main navigation. No engine, authentication, account identity, UUID, world, storage, catalog source, protocol, or rendering behaviour was intentionally changed.

**Commits:** catalog navigation `5e4146cf14921e17bd2753dc06ffc69b162ca031`; compact tab label `f00f9602f6fc2e0908ecf42f83414c3866534cb1`.

**Verification:** both changed source files were fetched again from branch `android` after their commits. No formatter, compiler, unit test, CI run, APK build, install, or visual device test has yet been run for this navigation change. The push should trigger source CI; all-ABI native checks still require manual dispatch with `native_checks=true`.

**Next:** inspect the source CI result; then design the responsive Java-style navigation shell as a separate pass, keeping compact phone navigation usable and avoiding a global theme or game UI change.


## 2026-11-02 — Navigation priority and compact labels

**Preflight:** confirmed repository `sylphie07/horizon-android_Unofficial_port`, authorized branch `android`, HEAD `8485d724ee6e6940422984ac1dee15f916bf0799`, and reviewed `docs/CONTINUITY.md`, this log, `catalog.rs`, and the Android launcher page renderer. No changes were made to `main`.

**Applied:**
- `crates/horizon-client/src/android/catalog.rs`: reordered the existing seven primary pages to `Play`, `Installed`, `SkinStudio`, `Mods`, `Packs`, `Shaders`, `Settings`, placing the core launcher destinations first.
- `crates/horizon-client/src/android/launcher_pages.rs`: shortened the compact `Shaders` navigation label to `FX`; existing full-width label remains unchanged.

**Scope:** navigation presentation/order only. No page actions, content installation, account/authentication, game engine, profiles, worlds, storage, network protocol, or shared theme tokens were changed.

**Commits:** catalog reorder ccb89774e6db8192c2420ed2d5525598fa7ca5f7; compact label update ddb17228df9c194bd7317be96bcf442c09490654.

**Verification:** GitHub accepted both source updates on `android`. No formatter, compiler, unit test, CI, APK build, installation, or device visual test has run for these changes. The previously recorded CI results do not validate this new commit.

**Post-change inspection:** fetched both source files again from branch `android`; confirmed the requested page order and compact `FX` label are present. Updated `docs/CONTINUITY.md` with the exact commits and verification boundary.

**Next:** manually dispatch Android source checks with `native_checks=true` on branch `android`; inspect the source job and all four ABI jobs before any test APK build.


## 2026-11-02 — Primary navigation surface

**Preflight:** confirmed only branch `android` exists in the branch search; latest HEAD before this change was `a2f4e977fb1072b9de880e74e9b217e9d6f8f52c`. Re-read `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `catalog.rs`, and `launcher_pages.rs`. `docs/PROJECT_CONTEXT.md` is not present at that path (404); continuity and development log are the active documentation sources.

**Applied:** `crates/horizon-client/src/android/launcher_pages.rs` now draws a restrained raised/bordered container behind the seven primary navigation buttons. Existing button rectangles, page actions, content coordinates, and touch/gamepad targets are unchanged. This is a visual grouping step, not yet a Java-style sidebar.

**Commit:** 995cb7f6967435a88d5c3edd50cdce055b6720ef.

**Verification:** GitHub accepted the source update on `android`. No formatter, compiler, unit test, CI, APK build, installation, or device visual check has run for this change.

**Next:** inspect the resulting diff and layout constraints; the full sidebar requires a coordinated content-layout refactor so hit-testing and page content remain aligned. Run source/native CI before preparing a test APK.


## 2026-11-02 — Adaptive navigation layout audit

**Preflight:** confirmed the only returned branch is `android`; HEAD before documentation changes was `22847b254ba4ea9d02de0c334480075d83cbd935`. Re-read continuity and development log, then inspected the launcher renderer, page drawing, and shared UI implementation. `docs/PROJECT_CONTEXT.md` is absent at the checked path (404).

**Investigation:** mapped the top-level header/navigation and page-specific coordinate patterns. Navigation buttons append rectangles to `self.targets`; list controls also use `self.list_targets` and `self.list_view`, which drive touch and gamepad behaviour. Page families use direct absolute coordinates and independent right/bottom anchors. A sidebar-only visual move would desynchronize hit targets and overlap content, so it is not safe as a one-line change.

**Code changes:** none in this audit. No source files were modified.

**Documentation applied:** added `docs/SYLPHRA_ADAPTIVE_LAYOUT_PLAN.md` with renderer findings, a staged geometry migration, validation gates, and scope guardrails. Background customization (image/GIF/video) remains a separate later feature.

**Verification:** branch and HEAD inspected; documentation source files fetched. No formatter, compiler, unit test, CI, APK build, install, or device test was run. This audit does not claim runtime validation.

**Next:** implement a launcher-only geometry model, first preserving the current layout exactly; then migrate page families to consume a content rectangle before enabling a wide-screen sidebar.

## 2026-11-02 — Shared launcher shell geometry (stage 1)

**Preflight:** confirmed branch search returns only `android`; HEAD before this implementation was `43a2131c7ba520e30b28445bf66758c2e85cb544`. Re-read `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `docs/SYLPHRA_ADAPTIVE_LAYOUT_PLAN.md`, and the launcher page renderer. No changes were made to `main`.

**Applied:** `crates/horizon-client/src/android/launcher_pages.rs` now defines a launcher-local `LauncherLayout` geometry model for the title origin, account button rectangle, navigation surface rectangle, tab button vertical bounds, and Play panel content top. The header, account button, navigation surface/border, tab buttons, and Play hero panel now consume these shared geometry values. The geometry currently reproduces the existing horizontal layout; no sidebar or expanded navigation has been enabled.

**Commits:** initial geometry model `72b99bcd8920e8a7cf221ab05e0e3f91526aa1c8`; navigation border alignment follow-up `36891d928fcf82917849a1a29a8e650793f237b0`.

**Static source inspection:** fetched the final source back from branch `android`; confirmed the model, header/account/nav/tab consumers and Play panel consumer are present. This is source inspection only. No formatter, compiler, unit test, CI, APK build, installation, or device visual check was run. No runtime equivalence is claimed until those checks and a device test are performed.

**Next:** migrate the next page families to explicit content bounds and update their drawn controls, list targets, scroll viewports, and bottom/right anchors together. Only after that migration should the adaptive sidebar be enabled.


## 2026-11-03 — Shared Installed-page viewport bounds (stage 2)

**Preflight:** confirmed branch `android` is the only returned branch and starting HEAD was `775a02b2d957b36edb5b492a4befeeae54f4d251`. Re-read `docs/CONTINUITY.md`, this log, `docs/SYLPHRA_ADAPTIVE_LAYOUT_PLAN.md`, and inspected the full Android launcher page renderer. No changes were made to `main`.

**Applied in `crates/horizon-client/src/android/launcher_pages.rs`:**
- Extended the launcher-local `LauncherLayout` with a content rectangle derived from the already-inset UI width and height.
- Migrated the Installed page's scroll viewport top and bottom to that shared rectangle. The same `top`/`bottom` values continue to drive scroll clamping, `list_view`, row visibility, and list hitbox placement.
- Kept all current visible coordinates and right/bottom anchors unchanged. This is a geometry-centralization step, not yet an adaptive sidebar or a visual layout change.

**Code commits:** `67bbd8015b558441f456b654a796472e6f45aa85` (shared viewport rectangle) and `29134294339fdefd810df7503df268f1aa9ddea2` (use the height parameter).

**Static source inspection:** fetched the final source from branch `android`; confirmed the content rectangle is initialized and the Installed page derives both viewport bounds from it, then uses those bounds for scroll clamping and `list_view`. These checks are source inspection only.

**Not verified:** no formatter, Rust compiler, unit tests, GitHub Actions run, APK build, install, or Android device test was executed. No runtime or visual equivalence is claimed.

**Next:** migrate the Installed page's horizontal content anchors (including matching draw and hitbox rectangles) to the same content model, then continue with catalog pages. Enable a sidebar only after page families and their input/scroll geometry have been migrated and CI passes.


## 2026-11-03 — Installed page horizontal content anchors (stage 3)

**Preflight:** branch search returned only `android`; starting HEAD was `1e145c01a7d2fdf5fd72891c87c2fe0a953a59f1`. Re-read continuity, development log, adaptive layout plan, and current launcher renderer before editing.

**Applied:** in `crates/horizon-client/src/android/launcher_pages.rs`, migrated Installed-page horizontal positions to values derived from `layout.content_rect`: section headers, import buttons, local-item enable/disable controls, remove buttons, text origins and title widths. The corresponding `self.list_targets` rectangles use the same derived positions as the drawn controls. Existing numerical geometry is preserved at current viewport sizes; no visual redesign is claimed.

**Code commit:** `a6d661a987459ae9114c123490098c29ecf26f30`.

**Static source inspection:** fetched the updated source and confirmed the old hard-coded Installed horizontal anchors are absent in that page block, while the shared left/right/import/toggle/title geometry and list viewport remain present. This is a source-level check only.

**Not verified:** no formatter, compiler, unit tests, GitHub Actions run, APK build, installation, or Android device test was executed.

**Next:** migrate catalog-page horizontal/vertical content bounds and keep catalog row rendering, list hitboxes, pagination and scroll viewport synchronized. Then continue through Skin Studio and Settings before enabling the adaptive sidebar.


## 2026-11-03 — Catalog page shared content geometry (stage 4)

**Preflight:** verified only branch `android`; read continuity, development log, adaptive layout plan, and current renderer before editing.

**Applied in `crates/horizon-client/src/android/launcher_pages.rs`:**
- Migrated catalog page title/search/source selectors/hints/import controls and row action columns to shared content-left/right values.
- Migrated catalog row text widths and action positions, preserving matching `list_targets` action rectangles.
- Derived catalog list viewport top/bottom and pagination Y from `layout.content_rect`; scroll clamping and `list_view` use the same viewport.
- Corrected derived offsets to preserve the prior numerical spacing at current viewport sizes. No visual redesign or adaptive sidebar is claimed yet.

**Code commits:** `62de6b668537b6face24c9c7f7fe8e9b11ac17a0` (horizontal anchors), `d8ed8bbb89d50e4fe8ff51953c4525931f5b6767` (viewport), and `ebf780dc119cee3aa23ffd35352668f4e21b7655` (preserve existing spacing).

**Static source inspection:** fetched final source and checked that previous hard-coded width/position anchors are absent from the catalog block and shared content bounds, row hitboxes, list viewport and pagination are present.

**Not verified:** no formatter, compiler, unit tests, GitHub Actions, APK build, installation, or Android device test was executed. Long-line formatting and compiler correctness remain unverified.

**Next:** inspect and migrate Skin Studio and Settings page families, including their scrollbars, bottom/right anchors and all corresponding targets. Then audit dialogs and input focus before enabling adaptive navigation.


## 2026-11-03 — Skin Studio shared content geometry (stage 5)

**Applied in `crates/horizon-client/src/android/launcher_pages.rs`:**
- Skin Studio control column, profile/error labels, profile/import/model/default buttons and selected-skin label now derive horizontal positions and widths from the shared content rectangle.
- The 3D preview panel is positioned from the shared content right edge; the existing preview width and control-column spacing are preserved.
- Skin list row drawing and `list_targets` now use the same shared horizontal rectangle.
- Skin list lower viewport bounds are expressed relative to the shared content bottom while preserving the existing compact and non-compact viewport endpoints.

**Code commits:** `bb356f49be3c57328c0c03c449bfc5863cfda14a` and `24aba85afd45d518743ef74264cb7d3d5f1c4129`.

**Static source inspection:** fetched the final Skin Studio block and confirmed old hard-coded horizontal anchors are absent, the preview uses shared edges, and the skin-row draw/hitbox rectangles match. No runtime equivalence is claimed.

**Not verified:** no formatter, compiler, tests, CI, APK build, install, or Android device test was run.

**Next:** migrate Settings page content and its list targets/viewport, then review About, dialogs, scrollbars and gesture/focus handling before implementing the actual adaptive navigation.


## 2026-11-03 — Settings page shared geometry (stage 6)

**Preflight:** branch `android` and HEAD `0ca4a6a43a69755f9536e0abc0ad39d039384094` were checked. Re-read continuity, development log, adaptive layout plan and Settings renderer block.

**Applied:** `crates/horizon-client/src/android/launcher_pages.rs` now derives Settings content left/right edges, row split, title position, list top/bottom, scroll maximum, row hitboxes, toggle/button rectangles, dividers and bottom Diagnostics/Account buttons from `LauncherLayout.content_rect`. The wide-screen split and compact full-width row dimensions preserve the prior coordinate formulas. Drawn controls and input targets remain paired.

**Code commit:** `1a2d0764d7e5a74b254e9a3464c0ff9afd8dcbeb`.

**Verification:** Android source checks #172, run `41580868881`, completed successfully. Passed `git diff --check`, public-tree history validation, Android Python unit tests, diagnostics, performance, content tests and workflow-export consistency.

**Not verified:** this push workflow does not run the native ABI matrix. No Rust formatter, Rust Android compile, APK, installation or device visual/touch test is claimed.

**Next:** migrate About and Play content to shared bounds; then audit footer/status, scrollbar and list drag hit regions, modal overlays, keyboard/gamepad focus and touch gestures. Enable the wide-screen sidebar only after all page families and input geometry are migrated and native checks pass.


## 2026-11-03 — About, Play and footer geometry (stages 7–8)

**Preflight:** checked branch `android`, current source and previous continuity/plan. Starting source tip was `051fb1d419f587b4cd0375ef20efc237ef0ad50b`.

**Applied in `crates/horizon-client/src/android/launcher_pages.rs`:** About content and buttons now use shared content bounds; Play panel, progress controls, text and primary button derive horizontal placement/width from the content rectangle; the status footer and Horizon GitHub action derive their horizontal bounds from the same content rectangle. Existing coordinates are preserved with the current non-sidebar layout.

**Commits:** initial page migration `9c04410b6c17e7afe3860cd2889b7ff124573dee`; static-review correction `b1980e9c8803ca5bb3caeda942158e05aa2a884a` moved footer geometry declarations into the enclosing scope so the GitHub button can access them.

**CI history:** run #174 was cancelled during overlapping push delivery. Run #175 (`41581418514`) completed successfully on the preceding `9c04410` commit, not the corrected HEAD. A fresh source run is required for `b1980e9` and this documentation tip.

**Not verified:** Rust formatting/compile, native ABI matrix, APK, installation or visual/touch behaviour on device.

**Next:** migrate list viewport hit regions to include X bounds, update drag-origin checks, then audit dialogs, scrollbar position, focus navigation and touch gestures before enabling adaptive navigation.


## 2026-11-03 — Responsive navigation and bounded list input

**Preflight:** branch `android` checked at `a784adcc24727cc45160e2b0a97e2c9c702f98c9`; continuity, development log and adaptive layout plan reread before final polish.

**Applied files:**
- `crates/horizon-client/src/android/launcher.rs`: list view stores full horizontal/vertical bounds; mouse-wheel input only scrolls when the pointer is inside the active list.
- `crates/horizon-client/src/android/launcher_pages.rs`: `NavigationMode` and `LauncherLayout::tab_rect` provide compact, expanded 4+3 phone/tablet, and wide sidebar navigation. Title/account header, content rectangle, tab hitboxes and footer follow the selected mode. Added layout unit tests.
- `crates/horizon-client/src/android/chooser.rs`: chooser exposes a horizontal viewport as well as vertical bounds.
- `crates/horizon-client/src/android/launcher_gesture.rs`: origin containment and updated focus reveal viewport; tests cover both horizontal and vertical boundaries and density scaling.

**Code commits:** `45493c0a6fad782ff93c225ea9a8dde5542ad728`, `a784adcc24727cc45160e2b0a97e2c9c702f98c9`, and `f629757e008003561d14704c4aae25ae123c5be3`.

**Verification:** source CI #185 (`41583666042`) completed successfully on `f629757e008003561d14704c4aae25ae123c5be3`. All configured source steps passed: diff, public-tree history, Android Python unit tests, diagnostics, performance, content and workflow-export consistency.

**Limitations:** this push workflow does not run Rust Android compilation or the four-ABI matrix. No APK, install, physical-device rendering, touch, gamepad or orientation validation is claimed. The manual native workflow input `native_checks=true` is still required.

**Next:** synchronize into Sylphra's authorized branch; request manual native matrix and device validation. Keep app background image/GIF/video feature out of scope until the user approves the next design stage.


## 2026-11-03 — Four-ABI native validation for responsive navigation

**Workflow:** Android source checks #188, run `41584912109`, manually dispatched on `android` with native checks enabled.

**Validated revision:** `b22e97b2ca0b3e16537e57efafc04a513abe4851` (responsive-navigation code is `f629757e008003561d14704c4aae25ae123c5be3`).

**Result:** overall `success`. Jobs `source`, `native (arm64-v8a)`, `native (armeabi-v7a)`, `native (x86)`, and `native (x86_64)` all completed successfully. Each native job passed Android SDK command-line setup, Android toolchain installation and `tools/android_build.py --check --public` for its ABI. Source checks passed diff, public-tree history, Android Python tests, diagnostics, performance, content and workflow-export comparison.

**Boundary:** this workflow checks native Rust code; it does not produce an APK. No APK build, artifact, manifest/package audit, install or physical-device interaction has been run or verified.

**Next:** use the separate manual test-only APK workflow on branch `android`; do not publish a Release. After artifact audit, request device validation. Background image/GIF/video remains deferred.


## Skin Studio responsive preview — 2026-11-03

**Applied on branch `android`:**
- Replaced the Skin Studio's fixed 144-unit preview column with a responsive layout: on sufficiently wide viewports the preview column receives 40% of the available content width; narrower viewports use a stacked preview instead of hiding it.
- Reordered the profile, import/model, selected-skin and default-skin controls to avoid the previous compact-layout overlap between the selected label and Default button.
- The 3D mannequin now scales from the available preview rectangle, with bounded minimum/maximum scale, instead of a fixed 78-unit transform.
- The preview still uses the shared player model and selected Classic/Slim skin path. No skin-layer toggles or drag-rotation behavior were added in this pass.

**Commits:** responsive page layout `170326f09ca0e19641925fb256a9c9ce25fad1e6`; preview scaling `7218728c11abbb39b977c7e1149a830a382deae5`.

**Verification status:** source changes are applied, but formatting, compilation, unit tests, Android ABI CI, APK build, and physical-device layout checks have not yet been run. Do not treat the responsive design as verified until CI and device checks pass. The player renderer currently returns no extra render layers for avatar/player entities, so meaningful hat/jacket/sleeve/pants toggles require a separate renderer-level implementation; decorative toggles are intentionally deferred. Touch drag still uses pointer-position look-at behavior and needs a separate interaction change.


## Skin Studio — cumulative touch rotation (2026-11-03)

**Applied on branch `android`:**
- `crates/horizon-client/src/android/launcher.rs`: added independent preview hitbox, active touch tracking and persistent yaw/pitch state; reset preview interaction state when returning from gameplay.
- `crates/horizon-client/src/android/launcher_pages.rs`: records the preview rectangle, labels the drag affordance, routes a touch that starts inside the 3D preview to rotation, and preserves the existing list-scroll gesture for touches that start outside it.
- `crates/horizon-client/src/mobs/preview.rs`: changed the preview API to consume explicit yaw/pitch values instead of deriving orientation from the absolute cursor position. Horizontal rotation accumulates; vertical rotation is clamped to a comfortable range.

**Code commits:** `3404878f09e19f66e11d06a592c4bfacfc9d6f1b`, `273003a9b2141b9a89d6e8b26af18081c7af33cd`, `3e7f11027fccd6f60548e9cab31b7422466c3d05`.

**Verification:** changes are applied to the authorized branch. No Rust formatting, compilation, unit test, Android ABI CI, APK build or physical-device interaction test has yet been confirmed for these commits. Do not describe touch rotation as verified until those checks complete.

**Known limitation:** actual player skin layer visibility (hat, jacket, sleeves, trousers) is still not implemented. The current preview renderer submits the baked player model as one body model, and the generic entity layer path does not expose these skin parts as independent overlays. The next investigation must identify the baked player model's child-part structure and implement visibility without changing the in-game player renderer or inventory preview. Do not add nonfunctional toggles.

**Next:** inspect source CI for this push; fix any compile/test failures; then implement isolated preview-only model-part visibility if the model data supports it. Native four-ABI checks and a test-only APK/device test remain required before declaring Skin Studio complete.


## Skin Studio — functional layer visibility controls (2026-11-03)

**Applied on branch `android`:**
- `crates/horizon-client/src/android/launcher.rs`: stores a preview-only hidden-part bitmask; clears an active preview gesture on focus loss.
- `crates/horizon-client/src/android/launcher_pages.rs`: added six functional controls for Hat, Jacket, Left sleeve, Right sleeve, Left pants and Right pants. The selected/visible state is reflected by the button styling and status message.
- `crates/horizon-client/src/mobs/preview.rs`: passes the bitmask to the cloned preview mob's existing `hidden_model_parts` field. The existing player animation visibility logic applies the same vanilla player-part flags to the preview model. No gameplay entity is modified.

**Code commits:** `ea74fc3299088746d13f458a900d3e6802a80e7b`, `28dfe575cbab230afb54834571cb6179a4a4080f`, `3db7cd5ce8f1c30ab58078d888c45b427a4f8820`, followed by fixes `c65d5f3fe6b20c4928e8e01a720e325c51e9212e`, `1d1baca8f6af2007dc498e105b3a71e13132f724`, and `b1b8f0918902e5e525a9f1f89f8f5ad01d7e2d43`.

**Technical basis:** `MobExtra.hidden_model_parts` already represents the vanilla `PlayerModelPart` bit flags, and `mobs/anim.rs::player_model_visibility` maps bits 1..6 to jacket, left/right sleeves, left/right pants and hat. The Skin Studio now controls those bits on its private preview clone only. This is preferable to inventing separate render layers.

**Verification:** code is applied, but formatting, Rust compilation, unit tests, Android ABI CI and physical-device testing have not yet been verified. Push-triggered source runs were cancelled by subsequent rapid commits; a clean run must be observed on the final documented tip. Native four-ABI checks require the manual `native_checks=true` workflow dispatch.


## Skin Studio visual correction (2026-11-03)

- **Applied on `android`:** `crates/horizon-client/src/android/launcher_pages.rs` moves the six layer toggles into a dedicated full-width strip below the preview, so they no longer overlap the skin list. The list begins after that strip, extends to the bottom of the available content viewport, uses 44-unit selection surfaces with compact spacing, and fits long names to the available width. Only fully visible rows receive drawing and hit targets, preventing partial rows from spilling past the viewport.
- **Commit:** `2325973ad65b53cfcb671ca10a703024b0545087`.
- **Still under investigation:** the 3D preview reports that the player model must be installed via Play. Its current load is attempted when entering Skin Studio from `home/.local/horizon-client/<VANILLA_VERSION>`. Need verify the actual model/resource path and why the current device still requires repeated launches. Do not claim fixed yet.
- **Verification:** no CI result or APK/device check has been confirmed for this commit yet.
- **Next:** inspect the model document loader and installation path, add a bounded retry only if the install timing requires it, then verify source/native CI before updating Sylphra's submodule.


## 3D Skin Studio preview diagnostics (2026-11-03)

- Applied on android: mobs/preview.rs now preserves the actual resource-loading error instead of collapsing every failure to None. It reports a missing/unreadable entity-models.json with the exact resource root, or a missing minecraft:player#main model entry. Successful model loads remain cached by root; failures are not cached, so later visits can retry after resource installation.
- android/launcher_pages.rs logs the diagnostic and displays it inside the preview panel rather than repeating the generic instruction to open Play. This is diagnostic instrumentation, not a claim that the preview is fixed.
- The immediately preceding Skin Studio layout fix remains in the same branch: dedicated six-toggle layer strip, compact skin rows, fitted labels, and list viewport bounds.
- Source workflow run #214 (41698244902) was queued for the UI diagnostic commit when this entry was written. Native ABI validation has not been run for the new changes. No APK or device verification is claimed.
- Next: wait for source checks, inspect the exact error shown by a test APK/device, then fix the real resource-path/installation mismatch rather than adding blind retries. Before integrating to Sylphra, run the required native matrix when the workflow can be manually dispatched.


## Verification — source checks passed (2026-11-03)
- Run #216 (`41698337604`) on branch `android`, tip `708f42cfcffc74804f680ad33291644550a13d24`: source job succeeded. Confirmed all source steps passed: git diff check, public-tree history audit, Android unit tests, diagnostics, performance, content, and workflow export parity.
- Native matrix was skipped (workflow condition requires manual dispatch with `native_checks=true`). Earlier code-triggered runs #213/#214 were cancelled due to concurrency; #216 checked out the current branch tip, so do not interpret those cancellations as failures.
- Preview diagnostics are implemented but not device verified. The root cause of the missing 3D model is still unknown until the diagnostic is seen on an APK/device.
- Next: run four-ABI native checks manually, then build/audit isolated arm64 test APK, then test on device. Keep Sylphra submodule unchanged until these validations pass.


## Native CI failure analysis and fix (2026-11-03)
- Manual workflow run #219 (`41715591765`) on commit `6c5670a4bcd0c0edbc690d62944e57d2a6bea442`: source job succeeded; native jobs for x86_64, arm64-v8a, armeabi-v7a and x86 failed identically.
- Compiler error E0308 at `crates/horizon-client/src/game/perspective.rs:146`: wrapper promised `Option<Arc<MobType>>` but returned `Result<Arc<MobType>, Box<dyn Error>>` after making preview loading fallible for diagnostics.
- Fixed in `88850c1afa60c386a63af38ec37caf78686834f9` by mapping `.ok()` at the gameplay compatibility boundary. This preserves the old gameplay Option contract and leaves Skin Studio's `Result` error reporting intact.
- Verification pending: source CI and all four native ABI checks must rerun. No APK or device result is claimed.


## Full Android native matrix green (2026-11-03)
- Authoritative run #223 (`41717605149`), branch `android`, commit `9abc241e5f73b2ef9d12bef96018beb7ce76cc69`: all jobs succeeded.
- Source checks: success. Native ABI checks: x86 (i686-linux-android), arm64-v8a (aarch64-linux-android), x86_64 (x86_64-linux-android), armeabi-v7a (armv7-linux-androideabi) all success.
- Confirms the previous E0308 mismatch is corrected at the gameplay compatibility boundary while retaining the detailed Result in the Skin Studio preview loader.
- Next: run isolated test APK workflow, audit APK contents/signature/metadata and only then request device test. The 3D preview itself remains unverified on-device.


## Test APK workflow failure: republished Minecraft manifest (2026-11-03)
- Observed workflow run #9 (`41720733718`) on branch `android`, commit `c549ea195131c743420df5d39f2188d4f271e9eb`: setup steps succeeded, but `Prepare pinned Minecraft-derived engine tables` failed before APK compilation. Build and artifact upload were skipped.
- Exact cause: `tools/android_prepare.py` selected the live Mojang version-catalog entry for 26.3 and downloaded its current JSON (SHA-1 `80608ff6753705b6a82e2cb86a5937d4d233abd4`). `horizon-prepare` rejected it because it is not among the engine's accepted manifests. The repository lock already pins the canonical manifest SHA-1 `11f1aefe473eefba0af62e3286c33b692807b2eb` and Mojang immutable package URL.
- Correction applied in commit `8cbd2ead693b8e03bcedad3e6ef251fbab39d88a`: preparation now downloads the version JSON from the immutable package URL constructed from `research-lock.json`'s `manifest_sha1`, and verifies it against that same pin. This removes dependence on a mutable version-catalog URL while preserving checksum verification and the existing client/server jar pins.
- Status: source edit applied; no rerun or APK build has yet verified the correction. Next: run source checks / Python syntax and then dispatch the isolated test APK workflow. Do not sync the submodule into Sylphra or publish a release until the APK and device gates pass.


## Skin Studio — larger 3D viewport and persistent load diagnostics (2026-11-03)

**User device result:** the test APK built and installed successfully. The user confirmed the skin list/control layout is more orderly, but the preview remains too small and still displays the generic “Player model is not loaded yet” message. Screenshot shows a 1536×691 launcher view; the wide preview rectangle was only 194 logical units high, and the renderer capped the model transform at 112 units.

**Applied on branch `android`:**
- `crates/horizon-client/src/android/launcher_pages.rs`: wide preview height is now responsive (42% of viewport height, bounded 260–340); stacked preview also uses a responsive height (30%, bounded 180–260). The preview-load error is read from dedicated state rather than the general catalog status, so unrelated status updates cannot hide the diagnostic.
- `crates/horizon-client/src/android/launcher.rs`: added `skin_preview_error` state, initialized independently from the general page status.
- `crates/horizon-client/src/mobs/preview.rs`: increased preview mannequin scale to use up to 72% of preview height and raised its maximum transform scale from 112 to 220 units.
- Load success clears the dedicated error; load failure stores and logs the full resource-root/error diagnostic.

**Code commits:** `81d6dc29457b127de3af02b0d0fee8607091c4fb` (diagnostic state), `e7a64827dfa56ce795eb732275884104614d5f55` (model scaling), `6c4ac1d99932a5d6587d27d44b3a22cf1baa8c17` and `91aa7a24dac5cae75cdb455b88fe66881b435a1e` (responsive viewport and persistent diagnostic display).

**Verification:** edits are committed, but no formatter, Rust compile, unit test, Android source/native CI, APK build, or device test has yet verified this pass. The device screenshot is evidence of the preceding APK's appearance only; it does not verify these new changes. The exact player-resource loading failure remains unknown until the next test APK displays the persistent diagnostic.

**Next:** run source checks and the four-ABI native matrix, build the isolated arm64 test APK, then inspect the exact diagnostic shown on device. Do not sync the submodule into Sylphra or publish a Release until validation gates pass.

- Follow-up scale-fit correction committed as `2534eb6f03f5487f470703b6850b03c83edf46ba`: model scale now uses 42% of preview height (rather than 72%) and 46% of preview width, with a 220-unit ceiling. This keeps the full 1.8-unit player model inside the taller viewport instead of enlarging it beyond the clip bounds. Still uncompiled/unverified.


## Skin Studio — full-height preview, compact skin rows, first-install model validation (2026-11-03)

**User design reference:** the 1536×691 screenshot marks the complete right-hand column for the 3D player and requests skin-selection rows sized to their names, with the remaining skins/layers using scrollable space below the controls.

**Applied on authorized branch `android`:**
- `crates/horizon-client/src/android/launcher_pages.rs`:
  - Wide layouts now allocate 42% of the content width to the 3D preview and stretch it from the top to the bottom of the available content area.
  - Profile/import/default controls and skin-layer toggles remain in the left column; layer controls are anchored below the controls rather than below the full-height preview.
  - Skin rows now use measured label width plus padding, capped to the left column, instead of stretching across the full page. Existing vertical list viewport and scroll calculations continue to use the remaining space below the layer strip.
- `crates/horizon-client/src/official_install.rs`:
  - During the first official resource installation, the installer now attempts to load the player preview from the staged engine resource directory. On success it writes `skin-preview-ready.json`; on failure it logs the exact resource error but does not block normal game installation.
  - This deliberately reuses the same pinned engine resource bundle already downloaded/extracted for the game; it does not download a duplicate model or add another remote dependency.

**Code commits:** `a51b97c80e15bcafa97be9090076126403073644` (wide preview and measured rows), `ce95e3ec38fadacdab2f83a1b17ee79b0cd7ea2e` (first-install preview resource validation), `a71b0bcaead6f0642a8ac86bd49885add156aab7` (keep layer controls in left column).

**Verification:** GitHub file fetch confirms the edits are present on `android`. No formatter, Rust compile, tests, Android source/native CI, APK build, install, or device visual test has run for this pass. The installer preflight is a code change, not yet evidence that the staged bundle contains a loadable player model on device.

**Next:** run source checks and all four native ABI jobs. If green, dispatch the isolated arm64 test APK, install it, and inspect both the full-height preview layout and the new `skin-preview-ready.json`/diagnostic behavior. Do not sync the Sylphra submodule or publish a Release before these validation gates.

- Follow-up viewport fill correction committed as `c88c2a7601c5016577d7bdf73bde3d161b7d1e02`: wide Skin Studio preview and skin-list viewport now extend to `h - 12` rather than stopping at the generic page content rectangle, reclaiming the unused lower area visible in the screenshot. Still uncompiled/unverified.


## Sylphra Java-inspired launcher identity and development-only news (2026-11-03)

**User reference and intent:** adapt the Minecraft Java Launcher visual language (charcoal panels, bright green Play action, profile in the sidebar, broad home/hero area) without copying its artwork or replacing Sylphra's own layout. The News destination should be absent from stable builds and available only to beta/development APKs for release notes, known issues, proposals, testing guidance and contributor information.

**Applied on authorized branch `android`:**
- `crates/horizon-client/src/android/launcher_pages.rs`: introduced a launcher-local charcoal/green palette. This affects launcher pages only; the shared Horizon theme and in-game HUD remain unchanged. The wide sidebar now places the account/profile control above the navigation. The Play page has a Java-inspired hero composition, a Files shortcut, and a prominent right-aligned green Play action on wide layouts.
- `crates/horizon-client/src/android/catalog.rs`: added a build-channel constant and News page. Stable/release channel page navigation excludes News; beta/development channels include it.
- `crates/horizon-client/build.rs`: validates `SYLPHRA_BUILD_CHANNEL` (stable, beta, development) and injects it at compile time; omitted environment defaults to stable.
- `.github/workflows/sylphra-test-apk.yml`: isolated test APK workflow now sets `SYLPHRA_BUILD_CHANNEL=development`.
- `docs/SYLPHRA_DEVELOPMENT_NEWS.md`: created the source feed compiled into the development News view, with explicit PROPOSED / IN PROGRESS / APPLIED-UNVERIFIED / VERIFIED semantics and safe tester-reporting guidance.
- `launcher_pages.rs` renders the feed as a scrollable page only when the build channel exposes News.

**Commits:** `ec75e42585a62b730b24bd86d9af66addf1e0f0e` (build-channel-gated News route), `df7b9d5f8b3789a3a9a088c903968e32a8cd0cb4` (build channel injection), `51364b6eaabc3b103a1edddb4161edd182ba488a` (development APK channel), `427a9803d5864318aa013fdcd67e8f35fcbc4179` (development feed), `351aea1d323dd6cc85c7adf39fadba310b8dbbcd` (local green palette), `e3762924902159ee1d1f1675983a0c30cf1133b5` (home layout and News rendering), `e8c9c5e424cf74d9671b1c25177f5002c3bfeb95` (build-script validation correction).

**Static verification:** fetched the updated branch files and confirmed the News route is conditional on the injected channel, the test APK workflow sets development, the feed path resolves in source, and launcher colors are local to the launcher page module. No rustfmt, Rust compilation, tests, Android source/native CI, APK build, install, or visual device test has run for this pass. The current test workflow is a development build; a separate beta workflow has not been created.

**Next:** run source checks and all four native ABI jobs. Fix any compiler/format/test issues before building the isolated development APK. On device, review the sidebar/profile placement, green palette, Play hero/button, and News page scrolling. Beta workflow wiring can be added when a beta distribution path is authorized. Do not sync the Sylphra submodule or publish a Release during this validation stage.


## GitHub destination and project independence clarification (2026-11-03)

**User concern:** the old footer GitHub button opened the upstream Horizon Android repository, which could mislead visitors into thinking Sylphra is an official Horizon project or is endorsed by Horizon.

**Applied on authorized branch `android`:**
- `crates/horizon-client/src/android/launcher_pages.rs`: changed the persistent footer destination from the generic `Action::Repository` / `open_repository()` path to the explicit `Sylphra GitHub` label and `Action::SylphraRepository`, which opens `https://github.com/sylphie07/Sylphra-Android`.
- Expanded the About Sylphra notice: Sylphra is independent and unofficial, uses Horizon Android as its technical foundation, is not affiliated with/sponsored by/endorsed by Horizon or the original Android port author, does not speak for them and does not know their position on Sylphra. It also explicitly expresses respect and thanks for the original Android port author and the substantial foundation Horizon provided.
- Existing license and attribution notices remain in place.

**Commit:** `9f1b3e5d39728f0b3bd1e848303bb7078bd4e744`.

**Static verification:** fetched the updated source and confirmed the footer no longer contains the generic `GitHub` / `Action::Repository` button; the only remaining `Action::Repository` occurrence is its now-unreferenced legacy handler. The About text and explicit Sylphra repository action are present. No formatter, compile, tests, CI, APK build, install or device tap test has run yet.

**Next:** validate through the already planned Android source/native CI and isolated development APK; on device, tap the footer button and confirm it opens the Sylphra repository. Do not change Horizon attribution or remove required third-party notices.


## Correction: public-facing Sylphra repository target (2026-11-03)
- User correctly pointed out that the previous link targeted the private development repository `Sylphra-Android`. That would be unsuitable for a public launcher button and could expose internal work if visibility changed later.
- Verified repository metadata: `sylphie07/Sylphra-Android-Public` is public; `sylphie07/Sylphra-Android` is private. The public repository description identifies Sylphra as an independent unofficial Android launcher based on Horizon Android.
- Updated `crates/horizon-client/src/android/launcher_pages.rs`: footer and About repository action now open `https://github.com/sylphie07/Sylphra-Android-Public`; status text says public Sylphra project page.
- Commit: `7ce3d959b4461107ea37d4980fc496a87c0f5022`.
- Static source edit applied; no build, CI or device tap test yet. Keep private implementation repository and speculative/internal development notes out of public links unless explicitly approved for publication.


## Validation checkpoint — launcher visual identity and public repository link (2026-11-03)
- Android source checks workflow run #261, run ID `41738307509`, completed successfully for commit `cc5ea8e5c5a599dce392ec3065eb6abd25f516d6`. Successful steps: `git diff --check`, public-tree history validation, Android unit tests, diagnostics tests, performance tests, content tests, and active/exported workflow comparison.
- The source workflow does not run native Rust checks unless manually dispatched with `native_checks=true`. No native ABI checks or APK build have run for the latest launcher changes.
- The code in the tested commit includes the public GitHub destination `Sylphra-Android-Public`, About/independence wording, green launcher palette, News channel gating, and Java-inspired home layout.
- Next gate: manually dispatch `.github/workflows/android-source.yml` on branch `android` with `native_checks=true`. If all four ABI jobs pass, manually dispatch `.github/workflows/sylphra-test-apk.yml` to build the isolated development APK. No release or submodule sync.


## Native CI failure analysis and build-channel correction (2026-11-03)
- Source checks #264 passed on commit `d767c088b9db69b2398ca113b72b142645c221e0`, but all four native ABI jobs failed at Rust compilation.
- Logs identified two common root causes in `crates/horizon-client/src/android/catalog.rs`: `SYLPHRA_BUILD_CHANNEL` was not defined for the `horizon-android` crate because it had been injected from the unrelated `horizon-client` build script; and comparing string literals in a const `matches!` expression is not supported by the pinned Rust toolchain.
- Applied correction: `crates/horizon-android/build.rs` now validates and injects the channel for the crate that actually compiles the Android launcher, and emits `cfg(sylphra_development)` for beta/development. `catalog.rs` now uses `env!(..., "stable")`, `cfg!` for the boolean, and a cfg-attributed News array entry rather than const string matching.
- Commits: `0b0bd77bdd0578b9d630a0a1e24ea9cd4e12c33b` (catalog compatibility), `d65b792f7fe229f4cca5e46bf8d12bd0016d3efb` (correct Android build script).
- Source checks #261 passed; #264 source job passed but its four native jobs failed as described. The correction has not yet been compiled or CI-verified. Next: rerun source checks with `native_checks=true`, then continue only if all ABI jobs pass.

## Native Android CI success and APK handoff (2026-11-03)
- User reported the rerun had finished positively; independently inspected GitHub Actions run #269 (41764300816) on branch `android`, tested commit `1eed09d156fb0f41b37cd05acf400ecfd6187273`.
- Verified all five jobs passed: source checks and native Rust checks for `x86_64`, `x86`, `arm64-v8a`, and `armeabi-v7a`. The native checks compiled/checked Android Rust code without building an APK.
- This confirms the previous build-channel/const-compatibility correction. It does not establish APK creation, installation, or physical-device UI behaviour.
- Reviewed `.github/workflows/sylphra-test-apk.yml`: it builds an isolated arm64 development test APK (`dev.sylphra.android.test`) with `SYLPHRA_BUILD_CHANNEL=development`, performs the release-candidate audit, and uploads an Actions artifact for seven days. It does not create a GitHub Release.
- No run of the APK workflow was present in the latest Actions history. Next action: manually dispatch that workflow on `android`; inspect the artifact and audit, then install and test the launcher on the Android device. Do not publish a Release or sync the Sylphra submodule.

## Skin Studio screenshots — layout diagnosis and correction (2026-11-04)
- User provided two screenshots after starting a game. The game launches, but the Skin Studio preview does not visibly show the selected skin/model. The first screenshot shows the not-loaded placeholder; the second shows a blank preview panel. Both show layer buttons crossing into the preview panel.
- Source inspection identified a deterministic layout defect: `layer_width` used the full `content_width` despite the wide two-column Skin Studio layout. The list scroll viewport also extended into the preview column although its selectable rows are confined to the control column.
- Applied in `crates/horizon-client/src/android/launcher_pages.rs`, commit `312a7b7968ad62ffd048803c1b7f108aaea3778b`: compute layer widths from `control_width` for wide layout and constrain list viewport right edge to the left column. This is an applied code change, not yet CI-verified or device-verified.
- Investigated preview path: `PlayerPreview::load` reads player model definitions from the versioned `.local/horizon-client` root; `skin_preview_rgba` reads the selected app-private PNG, decodes it and processes legacy 64×32 skins; `PlayerPreview::draw` patches atlas sprite `horizon/skin0` and submits preview quads. The screenshots do not identify which stage causes the blank model, so no speculative rendering fix was applied.
- Next: instrument/validate player model geometry generation, atlas sprite presence and selected skin pixel upload; add regression coverage for missing/empty preview geometry and fallback texture. Then run source checks and native four-ABI checks, build a new isolated development APK, and ask user to retest. Keep Release publication and submodule sync disabled.
## Player skin atlas diagnosis and correction (2026-11-04)
- Inspected `mobs/render.rs`: player avatar body texture resolves from `Mob::skin` (or the model's default `entity/player/wide/steve`). Inspected the Skin Studio draw path: it previously patched atlas sprite `horizon/skin0` and passed that same custom id to `PlayerPreview::draw`.
- Applied in commit `1812890524738441941e5696dd67ee617c4171c3`, file `crates/horizon-client/src/android/launcher_pages.rs`: patch the existing vanilla player atlas slot `entity/player/wide/steve` with the selected 64×64 skin pixels, and pass the same id to preview rendering. Missing atlas slot now logs a warning and updates visible status rather than silently skipping the custom texture.
- This is a reasoned code correction based on the renderer's texture-selection path. It has not yet passed CI and has not been built or tested on device. It may address the invisible skin but does not yet explain the blank model geometry in the screenshot.
- Previous source CI run `41772593884` passed source tests for layout commit `9551b032ad72c8b0c02f32896185a38830fcc401`; native ABI checks were skipped. APK run `41765962850` succeeded but its commit predates the layout/skin corrections, so it is not a valid test artifact for these changes.
- Next: inspect source CI for this commit; dispatch `android-source.yml` with `native_checks=true`; if all four ABIs pass, dispatch `sylphra-test-apk.yml`. If preview remains blank, instrument `render_with` output vertex count, post-cull quad count, atlas slot bounds, and depth batch submission before changing model/render transforms. Do not publish Release or sync submodule.
## Skin Studio face-winding diagnosis and correction (2026-11-04)
- Independently inspected run #277 (41773866734): source job passed on commit `0255865a66f4e650995fd3e117afe34d92a4543f`; native job was skipped. No four-ABI validation was performed for the latest skin atlas code.
- Inspected `crates/horizon-client/src/mobs/preview.rs`: `PlayerPreview::draw` applied a negative Z scale (`Vec3(model_scale, model_scale, -model_scale)`). `player_preview_quads` then culls transformed faces whose normal has `n.z >= 0`. A reflection reverses face winding and can invert the normal sign, potentially culling the complete player mesh.
- Applied correction in commit `456c2cb213476a558e7412e3d5cdd48be62b33ba`: replaced the reflected scale with positive uniform scale, preserving transform orientation/winding. Only the GUI preview transform is affected; general gameplay rendering is untouched.
- This is a strong source-level diagnosis, but the change remains unverified by CI and on-device. No claim that the model is now visible.
- Next: source CI, then manual `android-source.yml` dispatch with `native_checks=true`, then isolated development APK workflow if all four ABI jobs pass. On device verify actual mannequin, selected PNG skin, rotation, and layer toggles. If still blank, add vertex/quad counters and inspect depth submission. Do not publish Release or sync the submodule.
## Native Rust compile failure and correction (2026-11-04)
- Read job logs from run #281 (`41775392689`): source tests all passed, but `armeabi-v7a`, `x86`, `arm64-v8a`, and `x86_64` all failed compiling `horizon-android` with E0425 at `launcher_pages.rs:966–967`: `wide_preview` and `control_width` not found in scope.
- Diagnosis: a string replacement in the prior change had inserted the Skin Studio viewport condition into the Installed page's `self.list_view` assignment, rather than the Skin Studio block. The failed scope was unrelated to the new player transform.
- Applied correction commit `2a5342e2e186268021f0bc3054770af5bf1699b7` in `crates/horizon-client/src/android/launcher_pages.rs`: Installed page viewport restored; Skin Studio viewport now uses `list_right` within its own scope, and skin row hitbox/render width uses that same bound.
- Not yet verified. Need rerun source + native CI against current branch head. APK must wait until all four ABI checks pass. No Release publication or submodule sync.