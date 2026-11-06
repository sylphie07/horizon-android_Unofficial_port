# Sylphra Launcher — visual redesign specification

Status: approved visual direction; implementation not yet verified.
Reference: user-provided Minecraft Java Launcher screenshot (1536×691), supplied 2026-11-06.
Authorized branch: android only.

## Goal

Rework the native Android launcher UI to follow the reference composition and visual hierarchy while retaining Sylphra's own identity. This is a layout and presentation target, not a request to copy Minecraft branding, logos, proprietary assets, or exact artwork.

## Main screen composition

For wide landscape layouts, target an approximately 23% left navigation rail and 77% main canvas. Adapt proportions at narrower widths without clipping or making controls unreachable.

### Left rail
- Dark charcoal, near-black surface, visually distinct from the main canvas.
- Account area at the top: avatar, player name and offline/online status.
- Large vertically stacked navigation rows with small pixel-art-style icons where feasible:
  - Play
  - Installations
  - Skins
  - Patch Notes (future/optional; do not invent release content)
  - News (development builds only, or hide when empty)
  - Mods
  - Profile / account
- Selected row: muted Minecraft green fill and clear bright-green outline.
- Keep labels readable and spacing generous. Do not reproduce overlapping bars or oversized controls.

### Main canvas / Play
- Full-bleed Minecraft-themed background image, cropped with cover behavior and a subtle dark gradient/overlay for legibility.
- No large opaque gray card over the background.
- Top-right compact actions: Files, Multiplayer, and one secondary icon action if an existing safe action maps to it. Do not invent nonfunctional controls.
- Bottom-left compact version selector showing selected channel/version and dropdown affordance. Horizon currently targets one engine version; do not add fake version cards or imply unsupported versions.
- Bottom-right prominent green PLAY button.
- During install/download, replace the normal Play action with clear progress state while preserving the same visual hierarchy and preventing accidental launch.
- If no custom background is configured, use a bundled safe fallback or a clean dark Minecraft-inspired treatment. Never show a broken image placeholder.

## Other pages
- Keep the same left navigation rail and shared palette.
- Installations: compact useful list; no decorative version cards for versions the engine cannot run.
- Skins: preserve verified native 3D preview, rotation, Classic/Slim model and layer toggles. The preview must remain visible and large; target about 60% of the usable vertical space where layout allows.
- Cape Studio: separate Skins/Capes tabs; do not implement until the independent Cape audit clears UV/rendering questions. Keep existing preview state when switching tabs.
- Mods, shaders and resource packs: compact cards/list rows with clear installed/enabled state and actions.
- News/Patch Notes: optional, hide when there is no real content; reserve for genuine development notes/changelogs.
- Profile: account and local profile controls, not duplicated navigation clutter.

## Visual language
- Charcoal / near-black rail and overlays.
- Minecraft-inspired muted greens for selected states and primary action.
- White primary text, gray secondary text.
- Pixel-art icon language where assets are available; avoid emoji as final UI icons.
- Avoid copying Minecraft logo/wordmark, exact proprietary launcher assets, or implying affiliation. Sylphra branding remains independent.

## Technical constraints
- Native Rust UI only; no WebView.
- Preserve game engine, authentication, install, files, profile, skin and settings behavior.
- Do not regress the already device-tested skin preview, rotation, model selection or layer toggles.
- Responsive: wide landscape, compact landscape, portrait and OEM navigation insets.
- Touch targets must remain practical and all visible controls must have working actions.
- Background decode/loading must not run every frame; cache decoded pixels/texture and handle failure gracefully.
- Static image support first. GIF/video are separate later investigations; do not promise or implement them as part of the first background milestone.
- Do not add large image files or generated assets without checking licensing, repository size and attribution.
- Keep development documentation current: distinguish proposed, applied, compiled/tested, CI-verified and device-verified work.

## Milestones
1. Shared shell and responsive sidebar matching the reference.
2. Play canvas composition and working controls.
3. Background image setting/import with persistence and fallback.
4. Restyle Installations, Skins, Mods, Packs, Shaders and Profile consistently.
5. Optional Patch Notes/News presentation only when real content exists.
6. Tests, CI and device validation before declaring completion.

## Implementation status (2026-11-06)
- First source pass applied in commit 5b0c7a691f22f9e0b1ab97e582f037836d3b0fa4, file `crates/horizon-client/src/android/launcher_pages.rs`.
- Wide layouts now have a 23%-target dark sidebar, account area, stacked navigation labels and green selected outline. The wide Play page uses a dark full-canvas fallback, top-right Files/server-entry actions, an informational Horizon version tile and a large bottom-right Play/Install action.
- Compact/expanded layouts retain the previous layout and Play rendering.
- Still missing: selectable Minecraft background image, pixel-art icons/avatar treatment, final visual spacing polish and consistent redesign of non-Play pages. Multiplayer currently opens the existing server field; there is no server browser. Version tile is informational, not a dropdown.
- CI run 42169014756 was queued at documentation time. Compilation, tests, APK and device appearance remain unverified.

## Current verified starting point
- Branch android; pre-redesign HEAD was 3b0750ed31336e48e4cff33fe81851c472211846.
- Existing launcher is native Rust in crates/horizon-client/src/android/launcher_pages.rs.
- Existing adaptive layout and navigation modes are in launcher_pages.rs.
- Existing launcher palette is local to launcher_pages.rs; gameplay HUD theme must not be changed.
- Existing main Play page is currently a content panel with status/instructions and a Play button, not the reference full-bleed composition.
- Existing Config persists android-launcher.json and can be extended compatibly with serde defaults if needed.
- Existing Page set includes Play, News (development only), Mods, Packs, Shaders, Installed, Settings and SkinStudio. User-facing navigation labels may be adjusted without deleting functionality.
- No source implementation or visual verification is implied by this specification.
