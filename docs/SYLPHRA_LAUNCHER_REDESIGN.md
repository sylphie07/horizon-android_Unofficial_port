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


## User-selected launcher background (2026-11-06)
- The user selected the uploaded Minecraft Rust Edition scene as the definitive Play-page background.
- Inspected upload: 1536×864 pixels, RGB, JPEG-encoded image (despite the uploaded filename ending in `.png`). Preserve the artwork and embedded `MINECRAFT RUST EDITION` logo; do not regenerate or redraw it.
- Rendering target: full-bleed background behind the Play content area, using aspect-fill/cover with centered crop; keep the sidebar as a separate opaque dark rail. Add only a subtle readability overlay where needed, not a large opaque card.
- Asset import into the repository and runtime texture wiring are still pending. Before adding it, store it with the correct `.jpg` extension or deliberately transcode it to PNG; do not label JPEG bytes as PNG.
- Latest branch check at this note: android HEAD `fe40b1c964d18e53943f42fc4e8a1f0cc5be1490`. Android Source Checks run `42169082001` completed successfully for that HEAD. This validates source checks only, not APK compilation, runtime rendering or device appearance.


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
