# Sylphra first-run notice and About page

**Implementation branch:** `android`  
**Source base:** `f9ed3fd5bd906b40622356c96250fb5741f181f3`  
**Implementation status:** source changes applied; automated and device verification pending.

## Scope

This is derived-launcher branding and disclosure UI. It does not change the Horizon game engine, Mojang resource installation, account/session handling, worlds, downloaded game assets, JNI identifiers, or the original Horizon repository attribution.

## First-run notice

- `catalog::Config` stores `welcome_ack_revision: u32`.
- `WELCOME_NOTICE_REVISION` is currently `1`.
- Existing configurations deserialize with revision `0` because Config is Serde-defaulted.
- The acknowledgement is saved through the existing temporary-file/rename persistence path.
- The modal exposes an explicit **Continue** action. It is not dismissed by tapping outside, Android Back, Escape, or controller Back.
- Both the UI activation path and the launch action itself are gated while acknowledgement is pending.
- The modal closes only after the configuration save succeeds. A save error leaves the notice active.

When the notice materially changes, increment `WELCOME_NOTICE_REVISION` so the updated notice is shown again.

## About page

- A permanent **About Sylphra** row is added to Settings; the Settings scroll extent accounts for all 20 rows.
- About is a subpage, not a seventh top-level tab. `Page::ALL` remains the original six entries.
- The body uses the launcher's existing scroll state and viewport.
- It identifies Sylphra as independent and unofficial, based on Horizon Android, and disclaims affiliation, sponsorship, endorsement, or maintenance by the Horizon PC creators and original Android port author.
- It preserves project attribution and points to `LICENSE` and `THIRD_PARTY_NOTICES`; it does not replace component-specific licensing terms.
- It invites original creators to review or collaborate and commits to reviewing concrete rights/licensing concerns, including removal where warranted.
- **Sylphra GitHub** opens `https://github.com/sylphie07/Sylphra-Android`.
- The existing **GitHub** action still opens the original Horizon Android repository. It was not repointed.

## Source files changed

- `crates/horizon-client/src/android/catalog.rs`: revisioned Config field and serde compatibility tests.
- `crates/horizon-client/src/android/launcher.rs`: first-run state, Back/controller handling, and secondary launch guard.
- `crates/horizon-client/src/android/launcher_pages.rs`: modal, Settings row, scrollable About subpage, separate repository action, and activation gate.

## Verification record

**Applied:** source and unit-test changes are committed on the maintained fork's `android` branch. The latest implementation commit is `4686236516ad43e6d7e570ac636c399363ee5146`.

**Tests added, not executed:** two Rust unit tests cover deserializing an old config without the new field and round-tripping the acknowledgement revision.

**Not verified:** Rust formatting, compilation, test execution, GitHub Actions, APK build/manifest, first-run interaction, small-screen scrolling, touch/controller behaviour, and on-device persistence. Do not describe the feature as build- or device-verified until those checks pass.

## Regression boundaries

- Keep all six existing top-level tabs.
- Keep the original Horizon Android GitHub link.
- Keep the Sylphra repository link separate.
- Do not weaken sign-in, ownership, multiplayer, or session rules.
- Do not alter the Mojang installer, endpoint allowlist, metadata/hash verification, research lock, or downloaded game files.
- Do not substitute Horizon artwork for the Sylphra launcher icon.
