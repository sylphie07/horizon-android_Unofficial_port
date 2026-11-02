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
