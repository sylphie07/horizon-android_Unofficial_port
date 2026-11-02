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