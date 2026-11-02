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
