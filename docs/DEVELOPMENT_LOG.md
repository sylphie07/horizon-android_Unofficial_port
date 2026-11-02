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
