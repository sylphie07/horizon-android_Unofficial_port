# Sylphra skin library foundation

**Date:** 2026-11-02  
**Status:** S1 data-model/storage foundation applied; Android import UI and gameplay integration are not implemented.

## Scope

This change adds a launcher-owned data model for appearance profiles and imported skin records. It does not change game resources, authentication, player UUIDs, network protocol, worlds, or renderer precedence.

## Applied source changes

- `crates/horizon-client/src/android/skin_library.rs`
  - Versioned manifest format (`format_version = 1`) stored as `skin-library.json` in the app-private launcher home.
  - Appearance profiles are separate from Horizon's numbered game-data folders. Each appearance profile can select its own skin.
  - Imported PNGs are stored under app-private `skins/<sha256>.png`; stable content hashes avoid duplicate copies and user filenames are not used as paths.
  - Import validation enforces a 1 MiB input limit, fully decodes the PNG, and accepts only 64×64 or legacy 64×32 dimensions.
  - Classic/Slim model choice is stored as metadata; imports default to Classic unless the caller explicitly selects Slim.
  - Manifest validation rejects unsupported versions, duplicate IDs, invalid paths/references and malformed profile records. Missing metadata yields a clean default profile. Corrupt/newer metadata is reported rather than silently replaced.
  - Added tests for first-run defaults, modern and legacy PNG imports, malformed/oversized/wrong-size PNG rejection, independent profile selections, and corrupt-manifest preservation.
- `crates/horizon-client/src/android/catalog.rs`
  - Loads the skin library into launcher state at startup.
  - Preserves the existing manifest and surfaces a status error if metadata cannot be read.

## Verification boundary

**Applied:** source files and tests above; fork branch `android`.

**Not yet verified:** Rust formatting, compilation, test execution, Android CI, SAF import flow, UI, APK build, device behaviour, and actual in-world skin rendering. No CI run or APK result is claimed.

## Next steps

1. Connect the existing Android Storage Access Framework picker to a dedicated PNG import route (without routing skins into the add-on folders).
2. Add a Skin Studio UI for appearance-profile selection, import, model choice, and safe removal.
3. Verify data persistence and file/path safety with CI tests.
4. Keep Android LaunchRequest and renderer integration in separate stages. The decision about local custom-skin precedence during authenticated play remains pending; do not change it in this stage.


## Follow-up correctness fix
- Re-importing identical PNG bytes now updates the existing skin record's display name and Classic/Slim metadata rather than leaving stale model metadata.
- The independent-profile test fixture now uses different PNG pixel data for its two skins, so the test exercises two distinct content IDs rather than accidentally selecting the same hash twice.
- Applied in fork commit `f126284b7566970e4e81fe2071eae83cd9662bd1`.
- This is a source/test correction only. The test has not yet been executed.


## Per-profile model correction
- Moved Classic/Slim selection from the shared skin-image record onto each appearance profile. A single PNG can now be selected as Classic in one profile and Slim in another without conflicting metadata.
- The active profile exposes its own model selection; the independent-profile test now checks both skins and model choices after serialization/reload.
- Applied in code commit `f6a8624fd8108e3ba77628bfdf6afed6d1e761d2`.
- Tests remain unexecuted.


## S1-B — Android SAF import and Skin Studio UI (2026-11-02)

**Applied in fork branch `android`; current source tip:** `6163699d94f7039deb27bb580bf32affb08e3838`.

### Applied changes
- `crates/horizon-client/src/android/documents.rs`
  - Added a separate `ImportKind::Skin` request alongside the existing archive imports.
  - Reuses Android's existing SAF Activity result bridge and opens the selected `content://` URI through the existing document bridge; it does not derive a filesystem path from the URI.
  - Reads at most 1 MiB + 1 byte, checks cancellation, then passes bytes through the skin library's PNG decoder/dimension validation and stores them in the app-private skin directory.
  - Skin imports do not enter mods, resource packs, shaders, worlds, or the game profile's numbered data directory.
  - Added a distinct `skin_changed` notice so import completion returns to Skin Studio rather than the Installed add-on list.
- `crates/horizon-client/src/android/catalog.rs`
  - Added reload/save helpers for the library.
  - Save is blocked while metadata is unreadable, preventing the fallback default state from overwriting a corrupt or unsupported manifest.
  - Added a non-tab `SkinStudio` page route; the existing six main navigation tabs remain unchanged.
- `crates/horizon-client/src/android/launcher_pages.rs`
  - Added a Settings entry and Skin Studio screen.
  - Added active appearance profile cycling and creation, PNG import, Classic/Slim toggle, default-skin selection, and selection of imported skins.
  - Added a read-only warning when the library manifest cannot be loaded.
- `crates/horizon-client/src/android/launcher.rs`
  - Reloads the skin library and returns to Skin Studio after a successful SAF skin import.

### Current limitations
- No thumbnail rendering or 3D preview yet.
- Skin selection is persisted locally but is **not yet connected to the Android game LaunchRequest or renderer**. The game will not be claimed to use these selected skins yet.
- Skin deletion/garbage collection and rename controls are not implemented.
- Compact/landscape UI layout has not been validated on a device.
- The local custom-skin precedence decision for authenticated play remains pending and has not been changed.
- No authentication, player UUID, server protocol, world, downloaded resource or add-on import behaviour was intentionally changed.

### Verification boundary
**Applied:** source and documentation commits exist on fork branch `android`.

**Not verified:** Rust formatting, compilation, unit-test execution, Android source CI, native ABI checks, APK build, installation, SAF behaviour on-device, persistence on-device, UI touch behaviour, preview or in-game appearance. The latest code commit has no reported combined status checks. No passing CI is claimed.

**Next:** inspect the full diff and run the Android source workflow; then pin the reviewed fork revision in Sylphra's authorized branch. Keep preview and runtime application as later, separately validated stages.


## Compact-layout follow-up
- Skin Studio now uses a compact control layout on short screens, moves the skin list into the remaining viewport, and hides the ordinary footer/status controls while that compact page is open so they do not cover skin-selection rows.
- Selected-skin text is constrained to avoid overlapping the default-skin action.
- Applied in fork code commits `ba97c6bed133215430c220c28aeab2c99d03e17b`, `9836de2fe324d6fd38a3c0d2cb994ad7414dad24`, and `34bb922192b987abfd5d0ef0fbc166c4b38fc4ac`.
- This is source-level layout work only. Touch targets, scrolling and visibility remain unverified on a physical Android device.


## S2 — Shared 3D player preview (2026-11-02)

**Code tip reviewed:** `8ff34736168299c30de8ae007a069cb7c6aec036`.

### Applied
- Added `crates/horizon-client/src/mobs/preview.rs` with the cached player model type, a reusable `PlayerPreview` object, inventory-equivalent entity lighting, and depth-sorted GUI quads.
- Moved the inventory preview's quad conversion/lighting helpers into the shared module. The existing in-game inventory preview now calls those shared helpers, preserving its prior rendering path.
- The launcher loads the baked player model and animation definitions lazily from the prepared version resources. Missing model data is not negatively cached, so the launcher can retry after the first game-resource installation.
- Skin Studio uses the reserved `horizon/skin0` atlas slot for the selected skin, normalizes legacy 64×32 images through the existing skin processor, patches the atlas, and renders the avatar in a depth/scissor-tested HUD viewport.
- Pointer movement rotates the preview's body and head; the selected appearance profile controls the Classic/Slim arm model.
- Added a Catalog cache for normalized skin pixels. It rejects symlink/non-regular files and checks the 1 MiB size limit before reading.

### Verification
- Source workflow run #55, ID `41397954496`, completed successfully on code commit `8ff34736168299c30de8ae007a069cb7c6aec036`.
- Passed: `git diff --check`, public-tree history validation, Android Python unit tests, diagnostics tests, performance tests, content tests, and workflow-export consistency.
- Native Android ABI job was skipped because it requires manual workflow dispatch. No Rust compilation, Rust unit-test run, APK build, install, physical-device preview or gameplay test is verified.
- No authentication, player UUID, network protocol, world data or multiplayer skin visibility behaviour was changed.
- The selected skin is still a launcher preview/selection only; the Android game LaunchRequest and live renderer do not yet consume it.

## Native Android CI failure and correction (2026-11-02)

### Failure observed
- Workflow run: [Android source checks #57](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41400684974).
- Tested commit: `29affd3f0fbde425220114eebac3f499b65de6e5`.
- The `source` job passed. All four native ABI jobs failed at the Rust `cargo check -p horizon-android` step.
- All four failures shared the same five Rust E0433 errors in `crates/horizon-client/src/mobs/preview.rs`: references to `horizon_client::hud::HudBatch`, `horizon_client::hud::HudAtlas`, `horizon_client::mesher::Vertex`, `horizon_client::hud::Rect`, and `horizon_client::fire_render::flames` could not resolve from inside the `horizon-client` crate.
- This was a crate-local path issue, not four separate ABI-specific failures.

### Applied correction
- In commit `d052fdfaa50332f0c9ca7e4fca6228b8d9c1f882` on branch `android`, replaced those five `horizon_client::` paths with `crate::` paths in `crates/horizon-client/src/mobs/preview.rs`.
- No renderer behaviour, skin data, authentication, network protocol, player identity, world data, or Android ABI configuration was changed.

### Verification status
- **Applied:** source correction committed to branch `android`.
- **Not yet verified:** Rust formatting, compilation, native ABI checks, APK build, installation, or physical-device behaviour. The failing workflow has not been rerun.
- **Next:** run the Android source checks against commit `d052fdfaa50332f0c9ca7e4fca6228b8d9c1f882`; inspect any new compiler error before proceeding with S3-A.



## Native Android compile corrections (2026-11-02)
- Run #61 (`41404037242`) passed the source job but exposed three Rust compile errors in every native ABI job.
- Corrected the armor-stand preview helper path to the shared `horizon_client::mobs::preview` module.
- Corrected PNG decoded-buffer sizing to handle the optional size returned by `output_buffer_size()`; moved the test-only `PathBuf` import into the test module.
- Commits: `3de10f78d00c7583f079d582fb194a09aacb237a` and `9b4135df830cb664d0fe3a8d476bbc46178dd5a5`.
- These are compile/validation corrections only. Native compilation remains **pending** until a fresh manual four-ABI workflow run passes. No APK or device verification is claimed.


## Native Android CI verification after compile fixes (2026-11-02)
- [Android source checks #66](https://github.com/sylphie07/horizon-android_Unofficial_port/actions/runs/41405537016) completed successfully on commit `09cb65890102fbac97edde9a8e2dd258e1524be6`.
- The source job and all four native Rust check jobs passed: `arm64-v8a`, `armeabi-v7a`, `x86_64`, and `x86`.
- This supersedes the pending native-compilation status recorded above: the corrected source now passes the configured Android Rust type-check for all four targets.
- **Still not verified:** APK build/sign/install, SAF import on device, skin-library persistence on device, touch/layout behaviour, preview rendering on device, and live in-game skin application. The native workflow explicitly checks Rust code without building an APK.
- Next stage: audit the current Android `LaunchRequest` and renderer skin selection path before proposing S3-A runtime integration. Keep authentication, UUID/player identity, worlds and protocol semantics unchanged.


## S3-A — Runtime integration audit (2026-11-02; no code changes)

**Inspected source tip:** fork branch `android`, commit `f1b0a4a3e8c41d77d56ed2be2d31aee790c60242`.

### Findings
- The launcher already has the active appearance profile and selected skin record. The image is stored app-privately at `home/skins/<record.file>`; model choice is per appearance profile.
- Android constructs `LaunchRequest` in `android/launcher.rs`. The request currently has no appearance field. Its argument vector is parsed into the game's `Options` in `game.rs`.
- Player rendering consumes `Mob.skin` and `Mob.slim`; `game/remote.rs::update_remote_skins` resolves the server-provided profile, authenticated account fallback, and finally the old offline skin fallback.
- The old fallback uses environment variables plus a process-global `OnceLock`, and only applies when no server/account skin is resolved. It is not sufficient for Sylphra's selected profile, particularly when Microsoft authentication is active.

### Proposed S3-A boundary (not implemented)
Pass an optional, validated app-private skin path and the profile's Classic/Slim model through the immutable launch request and game options. Decode/normalize once per game session. Apply it only to the local player's rendered avatar, after normal server/account skin resolution; leave all remote player skins untouched. With no selected custom skin, preserve existing behaviour. No Microsoft session, UUID, player name, world, server protocol or authentication changes.

### Required tests and verification
- Launch request argument tests for selected and absent skins.
- Local-UUID precedence test, including proof that a remote player's server skin remains unchanged.
- Invalid/missing/oversized/symlink file handling.
- Source workflow plus native Rust checks for all four Android ABIs.
- APK and physical-device rendering remain separate, unverified gates.

This entry records an audit and a proposal only. No source implementation or tests were changed/executed in this step.


## S3-A1 — LaunchRequest transport (2026-11-02; partial implementation)

### Applied
- `LaunchRequest` carries an optional skin path and Classic/Slim model selection. It emits dedicated command-line arguments only when a skin is selected.
- The Android launcher snapshots the active appearance profile's selected image path and model into that request.
- The game parser stores the arguments in per-launch `Options` fields and rejects model values other than `classic` or `slim`.
- Added launch argument tests for selected-skin transport and the no-selection compatibility path.

**Commits:** `40b21226b8a24cb1f974f6044e0421e9accc2af8`, `bc68e674dfdb34dba310e7f25589307c0b1bd5a1`, `ba041e4c49a89fa1ddcdacbf7c8c5e00c9b88cba`, formatting follow-up `7c600084c0f48155daac7a05925177d8d1c91bcb`.

### Not yet implemented or verified
- The game options currently carry the selection but the renderer does not consume it yet; no in-game custom skin application is claimed.
- Safe PNG file validation/decoding, per-session normalized texture creation, and local-player-only precedence are pending.
- No rustfmt, compilation, test execution, CI, APK or device verification has occurred for this implementation yet.

Next: complete the runtime consumer and local-player-only appearance override, then validate the source and four Android ABI checks.

## S3-A2 — local avatar runtime application (2026-11-02)

**Applied on branch `android`:**
- The Android launch path now carries the selected app-private skin path and the active profile's model to the game options.
- The game loads the selected PNG once per session, validates the content-addressed filename and skin-directory/file type, enforces the 1 MiB limit and 64×32/64×64 dimensions, then uses the renderer's existing PNG decoder and legacy-skin normalization.
- The resolved custom texture takes precedence for the local UUID only. Remote player textures and cape handling are not replaced.
- The active appearance profile's Classic/Slim setting controls the local model.

**Code commits:** `6520b49c29be123c01f01d6f95cad81f265fad8f`, `4452e70a47454580d54c98946ed3f145ba302a3b`.

**Not yet verified:** compilation, formatting, tests, Android CI, atlas eviction/re-upload, APK, and physical-device rendering. Do not treat the selected skin as device-verified until those checks pass. The custom multiplayer relay and server plugin remain separate future work.
