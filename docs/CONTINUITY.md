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
- Main navigation now includes the existing Skin Studio page as **Skins**; desktop-facing Installed label is **Installations**. Code commits: `5e4146cf14921e17bd2753dc06ffc69b162ca031` and `f00f9602f6fc2e0908ecf42f83414c3866534cb1`.
- Detailed chronological record: `docs/DEVELOPMENT_LOG.md`.
- The Skins navigation entry only exposes existing Skin Studio functionality; it does not add new skin runtime or multiplayer-sharing behaviour.

## Verification state
- Android source-check run `41523801686` passed on commit `2b1f4e43afbe753dab9be12a8a4f6e717e236cd0`.
- In that run, all source checks passed; native matrix was skipped because the event was a push, not a manual dispatch with `native_checks=true`.
- That CI result predates the Settings pass and does not validate it.
- No formatter, Rust compile, unit test, APK build, install, or device verification is claimed for the latest Settings pass.
- Required next validation: manually run **Android source checks** on branch `android` with `native_checks=true`; inspect `source`, `armeabi-v7a`, `arm64-v8a`, `x86`, and `x86_64` jobs.

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
