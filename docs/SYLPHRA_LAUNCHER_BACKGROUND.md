# Sylphra Launcher background

## Goal
Use the user-supplied Minecraft Rust Edition artwork as the launcher background without changing the in-game HUD or world renderer.

## Asset and loading path
- Source artwork is bundled as four byte chunks under `crates/horizon-client/src/android/launcher_background/` because the repository's public-tree policy reserves the repository-root `assets/` directory for private game artifacts.
- `ensure_sylphra_background` assembles the chunks into a 600x338 indexed PNG at `home/installer/assets/minecraft/textures/gui/sylphra/background.png`.
- The same helper creates `home/installer/assets/minecraft/atlases/sylphra.json`, declaring a directory source `gui/sylphra/` with sprite prefix `sylphra/`.
- The atlas therefore exposes the sprite key `sylphra/background` to `HudAtlas`.

## Rendering
- `launcher_pages.rs` draws the sprite across the full launcher viewport.
- UV cropping uses aspect-cover behavior to avoid stretching on screens whose aspect ratio differs from the artwork.
- A subtle dark veil is drawn above the artwork to preserve text and control contrast.
- If the sprite is unavailable, the old solid launcher background remains as a fallback.
- The implementation is launcher-only; no gameplay HUD, world renderer, cape gameplay layer, or skin rendering path is modified.

## Verification status
- Source CI run #42304527339 passed on code HEAD `d551f02aeb8dd7ade1ee7ebb88cddf68a3165cfc`, before the atlas-registration correction.
- Atlas-registration correction commit: `0db826f608497f231eddd112b950eaba7a0b96dc`.
- Pending: source CI on the latest commit, native Rust checks for all four Android ABIs, then one arm64 test APK and physical-device visual verification.
- No APK build or device verification is claimed yet.



## 2026-11-07 — Play panel occlusion correction
- [Confirmed root cause] The background image was bundled, installed and registered with the private Sylphra atlas, and the Play renderer drew it. The image was then hidden by a full-content panel painted with opaque `color::RAISED`.
- [Applied] Commit `db995e8ea15ca0376a854dbb3de1dad9a92a846d` changes the Play panel fill to translucent dark `base_color::hex(0x101810, 88)`. Its border and geometry remain unchanged. This lets the artwork show through the panel while retaining a dark readability veil.
- [Not visually verified] No new APK/device screenshot after this correction. The source change must pass source CI and current-HEAD four-ABI native checks before a fresh ARM64 test APK is requested.
- [Previous baseline] Four-ABI native run #483 and ARM64 test APK run #22 passed on prior HEAD `266bc38da10dc64d836422d0c469972498d0c903`, before this panel change.
