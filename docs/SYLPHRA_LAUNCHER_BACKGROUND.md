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
