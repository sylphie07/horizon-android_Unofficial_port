# Sylphar — diseño de Skin Studio: Skins y Capes

**Fecha:** 2026-11-05  
**Rama autorizada:** `android`  
**Estado:** especificación de diseño aprobada por el usuario; implementación pendiente.

## Alcance corregido

La función solicitada es importar y equipar **Minecraft cosmetic capes** (capas textiles que cuelgan de la espalda), como la capa promocional Aurora mostrada por el usuario. **No** son Elytra/élitros ni las seis capas de overlay de la skin (Hat, Jacket, L/R sleeve, L/R pants).

La investigación previa titulada `SYLPHRA_ELYTRA_IMPORT_RESEARCH.md` estudió Elytra por una interpretación incorrecta de la frase “capas de la espalda”. Sus hallazgos sobre Elytra quedan archivados como investigación separada y no son requisitos de esta implementación.

## Referencia visual y comportamiento

Tomar como referencia la pantalla “Skins & Wardrobe” proporcionada por el usuario:
- Dos categorías principales claramente separadas: **SKINS** y **CAPES**.
- El visor 3D del personaje permanece visible en ambas categorías y no se destruye ni se oculta al cambiar de pestaña.
- La categoría activa controla únicamente el panel de la derecha (o inferior en ventanas estrechas).
- La selección de skin y la selección de cape son independientes y se conservan al alternar categorías.
- El avatar del visor refleja ambas selecciones simultáneamente: skin/modelo actual + cape equipada.
- La orientación/rotación actual del visor debe conservarse al cambiar de categoría o seleccionar un elemento.
- En Skins se conservan importación PNG, perfiles de apariencia, Classic/Slim, skin default y los seis controles de overlay ya verificados.
- En Capes se ofrece importación de PNG, biblioteca de capas personalizadas, estado activo y opción explícita para equipar “None”.
- La interfaz no debe presentar Elytra como cape ni mezclar cape con los seis overlays de skin.

## Layout adaptable

### Pantallas anchas
- Zona de preview fija a la izquierda, con modelo 3D grande, fondo/panel dedicado y controles de giro existentes.
- Panel de gestión a la derecha.
- Selector horizontal superior de dos pestañas: Skins / Capes.
- La lista/biblioteca puede desplazarse dentro de su panel sin mover ni ocultar el visor.

### Pantallas estrechas
- Mantener el visor siempre visible como primer bloque, con altura limitada pero suficiente para distinguir el avatar y la cape.
- Pestañas Skins / Capes inmediatamente debajo o encima del panel de gestión, según encaje con los límites actuales.
- Biblioteca y controles debajo del selector, con scroll propio.
- No comprimir el modelo hasta hacerlo ilegible ni superponer controles al visor.
- Drawing bounds, touch hitboxes, gamepad focus y scroll viewport deben derivarse de la misma geometría compartida.

## Estado y persistencia

- Mantener `selected_skin` y añadir `selected_cape` de forma independiente al perfil de apariencia.
- Añadir registros Cape separados de `SkinRecord`, con ID/hash, nombre, archivo y dimensiones originales validadas.
- Añadir carpeta privada dedicada para cape PNGs.
- Migrar el manifiesto actual de forma explícita y conservadora; preservar bibliotecas antiguas, datos desconocidos y manifiestos corruptos. La migración no debe borrar ni reescribir datos si la validación falla.
- Importar mediante el Android SAF existente, lectura acotada, PNG real decodificado, SHA-256 y escritura atómica.
- Validar las dimensiones permitidas según el contrato que ya usa el runtime Cape (actualmente `process_cape` normaliza a 64×64 con contenido en la mitad superior; `launcher/src/skins.rs` acepta 64×32 y 22×17). Antes de fijar restricciones, alinear la validación de la nueva biblioteca con ese contrato y añadir pruebas para ambos formatos.
- El botón “None” deselecciona la cape sin alterar la skin.

## Integración de renderizado

La investigación de código encontró:
- `crates/horizon-client-render/src/skins.rs` tiene `process_cape` y `offline_cape()`, que lee `HORIZON_OFFLINE_CAPE`.
- `crates/horizon-client/src/mobs/preview.rs` ya carga el modelo `player#cape` en el tipo de jugador.
- `crates/horizon-client/src/mobs/render.rs` contiene una función especializada `cape(...)` que posa y dibuja la capa.
- `launcher/src/skins.rs` ya conoce la variable de entorno `HORIZON_OFFLINE_CAPE`.
- La biblioteca Android actual solo gestiona skins y el visor recibe la skin/modelo, por lo que aún falta enlazar la selección Cape persistida al atlas y al render del preview.

La implementación debe seguir el flujo real de renderizado del jugador local y del preview. No debe asumir que cambiar el archivo de entorno en tiempo de ejecución actualiza una textura que se carga con `OnceLock`; investigar el momento de inicialización y proporcionar un mecanismo de actualización/atlas apropiado. Evitar alterar capes de jugadores remotos o identidad/autenticación.

## Secuencia de implementación

1. Auditar el ciclo de vida de `offline_cape()`, carga de atlas y call-sites de `render::cape`; determinar cómo refrescar una selección sin reiniciar el juego ni romper la vista previa.
2. Ampliar modelo de biblioteca y perfiles con selección Cape independiente, migración segura y pruebas de compatibilidad.
3. Añadir importación SAF de Cape PNG con validación, almacenamiento privado y mensajes de resultado.
4. Integrar textura seleccionada al jugador local y al visor 3D; asegurar actualización inmediata y fallback None.
5. Rediseñar la página con pestañas Skins/Capes y geometría persistente del preview, manteniendo sincronizados dibujo, input, foco y scroll.
6. Ejecutar pruebas de biblioteca/migración/PNG, Android source checks y matriz nativa; construir un único APK arm64 de prueba cuando el conjunto esté listo.
7. Validación del usuario en dispositivo: preview visible en ambas pestañas, rotación intacta, cape importada/equipada/retirada, skin y Classic/Slim intactos, selección persistente entre pestañas y reinicio.

## Restricciones

- Trabajar exclusivamente en rama `android`.
- No modificar `main`, no fusionar PRs y no crear GitHub Releases.
- No tocar archivos de mundo, autenticación, UUID, perfiles de cuenta ni texturas de jugadores remotos.
- No reemplazar ni renombrar las seis capas de overlay existentes.
- No generar APK por cambios aislados de documentación o por cada ajuste menor; agrupar la implementación funcional.
- Diferenciar siempre cambios aplicados, CI verificado y comportamiento confirmado en dispositivo.

## Estado de esta especificación

Solo documentación e inspección de código. No se ha modificado código de producto, ejecutado pruebas, compilado ni generado APK para la función Cape.


## Runtime and preview integration audit — 2026-11-05
- Confirmed the 3D preview loads the baked `player#cape` model into `MobType.layers`, but `PlayerPreview::draw` calls `render_with` for the player body and does not call the dedicated `mobs::render::cape` function. The cape geometry is therefore not currently submitted by the launcher preview.
- Confirmed `mobs::render::cape` itself handles the cape pose (movement/flap/lean) and draws `player#cape` using a caller-supplied atlas UV rectangle. It is a helper, not an automatic layer in `render_visible`; inspected `render_visible` iterates generic `layers(m, age)` and has no Cape submission call. A game-renderer call site still needs to be located/verified before claiming Android gameplay cape rendering is active.
- Confirmed offline texture pipeline: `horizon-client-render::skins::offline_cape()` reads `HORIZON_OFFLINE_CAPE` once using `OnceLock` and normalizes it through `process_cape` into a 64×64 slot. The general texture cache separately supports downloaded Cape textures under `capes/<hash>` and queues them to the atlas.
- Confirmed Android launcher launch request currently carries `skin_file` and `skin_slim` only; `game_launch::LaunchRequest::arguments()` emits `--sylphra-skin` and `--sylphra-skin-model`, with no Cape field/argument. The existing `launcher/src/skins.rs::env()` helper supports Cape for an older offline-account look flow, but that helper is not the Android `LaunchRequest` route inspected here. Do not assume the current Android offline Cape selection is connected.
- Preview atlas is the installer's compact GUI atlas, not the in-game full atlas. The Skin Studio currently patches a dedicated `horizon/skin-preview` 64×64 slot each frame. Cape preview needs its own stable atlas slot and 64×64 normalized pixels, then a dedicated call to `render::cape` using that slot's UV rectangle. The selected Cape texture should be supplied as data to preview drawing rather than relying on the global `OnceLock` environment path.
- Existing Cape texture normalization: `process_cape` accepts nonzero dimensions and scales oversized images to 64×32, copying them into the top half of a 64×64 transparent slot; it accepts small legacy 22×17 and standard 64×32 formats. The Android library's own import validator should use a stricter, documented allowlist (at minimum 64×32 and 22×17) or explicitly test any higher-resolution support, while retaining the 1 MiB bound.
- UI audit: current Skin Studio layout is one continuous branch in `launcher_pages.rs`, with a 42% preview column only at width ≥640 and a vertically stacked preview otherwise. The preview rect, drag hit-test and rotation state are already page-owned and persistent across draws. A tab switch can preserve these if the preview is kept outside the tab-specific control branch; narrow-screen layout must be rebalanced so preview remains visible without pushing all controls off-screen.
- No code changes, tests, CI, APK or device verification were performed during this audit.


## Player-preview and in-world renderer trace — 2026-11-05 (continued)
- Re-read the complete `PlayerPreview::draw` implementation. It clones the player mob, applies skin/model/hidden overlay state, builds an atlas lookup closure, calls `mobs::render::render_with` for the body and generic layers, then converts vertices to clipped GUI quads. The preview's mob type includes `player#cape`, but this call never submits it. This establishes the precise preview integration point: after the body call, obtain the selected Cape sprite UV and call `mobs::render::cape` with the same cloned mob, matching base transform, preview light and vertex buffer; then include those vertices in the same GUI conversion/clip path. The base transform is currently private inside `mobs::render`, so expose a narrowly scoped helper or factor preview geometry assembly without duplicating transform math.
- The Cape helper itself is fully explicit: it takes the mob, partial tick, root transform, Cape UV, light, MobData and output vertices; reads the baked `player#cape` layer; applies the avatar's Cape flap/lean pose; and draws the geometry. The atlas texture must be a distinct sprite/UV and should not reuse the skin UV.
- Traced Android game launch again: `Launcher` constructs `game_launch::LaunchRequest` with active skin path and Slim flag. `LaunchRequest::arguments` serializes only `--sylphra-skin` and `--sylphra-skin-model`. A Cape path/identifier must be added as an independent optional argument and consumed by the game resource/texture setup. Do not rely on changing `HORIZON_OFFLINE_CAPE` after startup.
- Traced game frame composition: `Game::geometry` invokes `mob_geometry(camera, alpha, out)` for mobs, then separately submits the local player model via `player_model_geometry` and guest players via `guest_model_geometry`. The exact implementation of `mob_geometry` and the local/guest model methods is still in the large `game.rs` source and needs a targeted extraction before deciding whether to extend common mob rendering or the avatar-specific methods. No verified claim is made yet that the dedicated Cape helper is called during gameplay; its actual game call-site remains unconfirmed.
- Existing preview state in `Launcher` includes `skin_preview`, `skin_preview_rect`, `skin_preview_touch`, and `skin_preview_rotation`; page transitions can reset some preview state on full launcher restore, but changing a sub-tab should not recreate the preview or reset rotation. Current preview only loads while `Page::SkinStudio` is active.
- No product source changes, tests, CI, APK or device validation were performed.


## Local-player render and atlas ownership audit — 2026-11-05
- Confirmed `Game::geometry` submits generic mobs first, then marks/submits the local player via `player_model_geometry`, and finally submits guest players via `guest_model_geometry`. The local player is a distinct rendering path, which is the correct place to apply the launcher's local Cape without touching remote/guest players.
- The implementations of `mob_geometry`, `player_model_geometry` and `guest_model_geometry` are not directly declared in the monolithic `game.rs` text inspected; they are supplied through the game's split/module organization or generated includes. A full repository search for their definitions via the available GitHub code-search connector returned no matches. Their exact implementation and the call-site of `mobs::render::cape` remain unresolved; do not edit or assume a remote-player path until local definitions are located through a complete source checkout or better search.
- Confirmed `game/atlas_slots.rs` is a generic LRU allocator for dynamic GUI atlas images (skins/maps), not a fixed Cape registry. It forgets assignments on atlas reload. A Cape texture can use the same slot lifecycle only if the caller patches pixel data and obtains the matching sprite UV after any eviction/reload.
- Confirmed Android `LaunchRequest` currently has `skin_file` and `skin_slim`, and argument construction emits skin flags only. `Launcher` creates that request from the active skin. Adding `cape_file: Option<PathBuf>` plus a dedicated CLI argument is a small, explicit transport change, but the receiving game-side argument parser and texture initialization still must be traced before implementing it.
- Confirmed `PlayerPreview::draw` already builds a body vertex buffer, then applies a GUI transform and clips it. The Cape helper `mobs::render::cape` needs a root transform and UV rectangle. The preview currently computes its GUI model transform after body vertex generation, while the renderer's root transform is internal/private; safe integration likely requires exposing/factoring the player root transform or a preview-specific renderer API rather than guessing matrix order.
- No product source changes, tests, CI, APK or device validation were performed. The investigation remains read-only except for documentation.


## 2026-11-06 — Cape library connected to launcher catalog state (foundation)
- Preflight: verified authorized branch `android`, HEAD `e518b6d7a792bde58d3dd539952226a023048369`, and read `docs/CONTINUITY.md`, `docs/DEVELOPMENT_LOG.md`, `docs/SYLPHRA_CAPE_STUDIO_DESIGN.md`, `android/cape_library.rs`, `android/skin_library.rs`, `android/catalog.rs`, `android/game_launch.rs`, `game.rs`, `game/remote.rs` and `mobs/preview.rs` before editing.
- CI verification of the prior catalog-overlap fix is now confirmed: Android Source Checks run `42176534828` passed for `e518b6d`; Sylphra Launcher test APK run `42177345901` also completed successfully for that same source HEAD. This is CI evidence only; it does not replace the user's device retest (the user reported the overlap is fixed).
- Confirmed an existing but previously disconnected `android/cape_library.rs` already provides a separate `cape-library.json`, SHA-256 PNG storage, 64×32/22×17 validation, 1 MiB input cap, profile selection, rename/remove and unit tests. It was already declared as a module in `catalog.rs`, but `Catalog` did not load or expose it.
- Applied `crates/horizon-client/src/android/catalog.rs` in commit `3bbe4a4ace1e615df9463cd6f0ef6e5953343c59`: `Catalog::new` now loads the independent cape library and reports its own read-only/error state; `refresh` reloads it; `save_cape_library` persists it; `active_cape_file` resolves the cape associated with the active appearance profile and rejects symlink/non-regular/empty/over-1-MiB files. Skin manifest and existing skin behavior are unchanged.
- Re-fetch of the committed source confirms the cape load/reload and safe active-file resolver are present. The new commit's CI had not yet appeared in the Actions listing at documentation time; do not claim it passes until a run for this exact commit completes.
- Not implemented yet: SAF import route, Capes tab/actions in Skin Studio, preview texture/3D cape rendering, launch argument and game-side local cape loading, and LAN peer sharing. Existing `update_remote_skins` only applies the Sylphra skin to the local player's client-side model; this is not evidence that a custom skin is transmitted to other LAN clients. LAN sharing therefore needs a separate protocol/metadata design for both custom skin and cape, preserving vanilla/server account textures and never overriding unrelated remote players.
- Background: selected Minecraft Rust Edition artwork is documented as 1536×864 JPEG-encoded, but the selected binary is not present in the repository/runtime. A repository image `docs/media/android/snow-landscape.png` exists (3,320,463 bytes) but has not been verified as the user's selected artwork; do not substitute it. Ask the user to reattach the exact selected image before committing it.
- Next: validate CI for `3bbe4a4`; implement SAF Cape import + Skin/Capes tab state; add the 3D preview using the verified player#cape model UV and shared root transform; then implement launch/runtime local cape. Separately design explicit Sylphra-to-Sylphra LAN appearance exchange (skin + cape) and test two devices. Only one APK build should be scheduled after these source changes and tests are ready.


## 2026-11-06 — Cape SAF import route started
- Preflight: re-verified authorized branch `android` and read current continuity, development log, Cape Studio design, `catalog.rs`, `cape_library.rs`, `documents.rs`, `launcher.rs`, `launcher_pages.rs`, `game_launch.rs`, and `mobs/preview.rs`. Starting HEAD was `79016a33eb977410543adac56f4568cc17e5db49`; catalog's independent cape-library load and safe active-file resolver were present. Android Source Checks run `42180249685` passed for that starting HEAD.
- Applied `crates/horizon-client/src/android/documents.rs` in commit `59635acab50a0f6d29470f06d0b640636077c8dc`: added `ImportKind::Cape`, `begin_cape`, bounded SAF read using the cape library's 1 MiB cap, `CapeLibrary::import_png` validation/storage, and a distinct `cape_changed` notice. Picker result code remains the existing single SAF request 4101; Cape uses the existing PNG fallback name and no parallel request path.
- Applied `crates/horizon-client/src/android/launcher.rs` in commit `ea7f3379a5145013f0fdaaaf879072bdac04a65a`: cape import notices reload the cape library and return the launcher to Skin Studio, without changing the skin import flow.
- Re-fetched `documents.rs` and checked every Notice initializer includes `cape_changed`. Android Source Checks run `42182173409` was in progress for `59635ac`; run `42182185880` was pending for the latest commit `ea7f337`. Do not claim the new SAF path compiles until the latest-HEAD check completes.
- Not yet added: visible Skins/Capes tabs and cape library controls, active selection/deselect/rename/delete UI, dedicated 64×64 cape atlas slot, preview `render::cape` invocation, game launch argument/runtime texture application, or LAN appearance exchange. No APK build/device verification.
- Design guardrails retained: custom cape is a textile cape, not Elytra or skin overlays; None means no Sylphra override; no remote-player texture changes until a separate authenticated/validated Sylphra-to-Sylphra LAN appearance protocol is designed.
- Next: wait for checks on `ea7f337`, then add independent Skin/Capes tab state and management controls while keeping the 3D preview mounted across both tabs. In parallel, inspect the installed `entity-models.json` `player#cape` UV on device/build artifacts before committing preview UV assumptions. Continue with launch/runtime path only after the texture update lifecycle and renderer call site are verified.


## 2026-11-06 — Compile fix and Cape preview/runtime audit
- Preflight: verified branch `android`, HEAD `8661cef5f579e82aa26b1e34989d1a4b0cd8e7d3`, and re-read continuity, development log, Cape Studio design, `documents.rs`, `cape_library.rs`, `catalog.rs`, `launcher_pages.rs`, `launcher.rs`, `game_launch.rs`, `mobs/preview.rs`, `mobs/render.rs`, `game/remote.rs`, and `game.rs` before continuing.
- Confirmed APK workflow `42182555858` failed at Rust compilation because the generic `Notice` initializer in `documents.rs` omitted new field `cape_changed` (E0063). Applied the one-field initializer fix in commit `8661cef5f579e82aa26b1e34989d1a4b0cd8e7d3`.
- Android Source Checks run `42183821691` for `8661cef` completed `success`: `git diff --check`, public-tree/history check, Android Python unit tests, diagnostics, performance, content, and workflow export check all passed. The `native` job was skipped by workflow conditions. This is not an APK build or Android runtime test.
- Preview audit: `PlayerPreview::draw` currently renders only player-body vertices through `render_with`, then applies its GUI transform and clipping. `mobs::render::cape` exists and accepts a player root transform plus cape UV; it looks up `player#cape` and silently returns if the model layer is absent. The current preview function does not append cape geometry. The previously exposed `render::base_transform` is available, but the preview's GUI matrix is not identical to the world root transform; a preview-specific composition must be derived/tested before adding cape geometry.
- Runtime audit: `LaunchRequest` currently transports only `skin_file`/`skin_slim` through `--sylphra-skin` arguments; no cape argument exists. `update_remote_skins` resolves account/offline capes and assigns them to player mobs; the local offline cape is applied only when no account cape exists. No Sylphra custom-cape injection or LAN appearance protocol is present.
- Catalog already exposes `active_cape_file`, but the launcher page does not yet offer Cape selection controls, and the launch request builder does not yet pass a cape path. Avoid presenting the catalog foundation as an end-to-end feature.
- Important unresolved evidence: the installed/exported `entity-models.json` was not available in the inspected repository files, so exact `player#cape` UV range remains unverified. Do not hardcode the half-texture crop until the runtime asset can be inspected. The cape model layer may also be absent in a resource set; the UI/runtime should report that explicitly rather than silently claiming preview success.
- No new source changes were made in this audit pass. Next implementation should first add safe profile-level cape selection and management UI/actions, then wire `LaunchRequest` and parser/runtime loading, then add preview geometry using verified model UV and transform; LAN sharing remains a separate protocol task. Do not ask the user to compile until the full planned batch and source checks are complete.


## 2026-11-06 — Cape Studio interface wiring

- [Applied] Added `Skins` and `Capes` tabs to the existing Skin Studio, preserving the shared 3D player preview and existing profile controls.
- [Applied] Cape tab lists imported cape records with dimensions, exposes PNG import through the existing bounded Android document picker, and supports selecting a cape for the active appearance profile.
- [Applied] Added `None · keep account cape`, which clears only the Sylphra profile override via `select_cape(profile, None)`; it does not modify account cape data.
- [Applied] Added tab state to the Android launcher and action dispatch for import, selection, and clearing the profile cape.
- [Applied] Kept tabs on the title row after reviewing the compact-phone layout to avoid overlapping import/profile controls.
- [Not yet verified] No Rust build, APK build, emulator/device test, cape 3D preview, in-game cape rendering, rename/delete controls, or LAN appearance exchange was run in this change batch. The preview still uses the existing skin mannequin; cape geometry is a separate next stage.
- Commits: `5b33791` (studio tabs/list), `0df249c` (launcher tab state), `4a209f9` (tab placement correction).
- Next: run source checks / compile checks, then implement cape preview geometry and launch-time cape injection only after verifying the actual player cape UV mapping.

## 2026-11-06 — Cape Studio tab cleanup
- The Capes tab no longer displays the skin-only overlay toggles; its list starts higher to use the available space. The Skins tab retains all six overlay controls.
- Applied in commit `a4a2345afef942de6b1e9896f9d80661f28deffd`.
- Source checks passed for the previous HEAD `ca03019`; this new commit still requires CI validation. The actual cape 3D preview and launch-time application remain unimplemented.

## 2026-11-06 — Renderer/atlas integration audit
- Engine `mobs::render::cape()` can generate cape mesh using installed `player#cape` model layer and avatar cape pose. `PlayerPreview::draw()` does not call it yet.
- `process_cape()` normalizes supported cape images into a 64x64 RGBA canvas with image data in the top 32 rows. Launcher needs its own `horizon/cape-preview` atlas slot, separate from `horizon/skin-preview`.
- Runtime `entity-models.json` is installed resource data and is not in the repository tree; exact model UVs must be read from installed document or verified with a fixture before hardcoding any crop.
- `LaunchRequest` only has skin fields and emits skin CLI arguments. A cape argument and game-side offline-cape override need separate tracing; `None` must preserve the account cape.
- This entry records audit findings and proposed implementation order only. No renderer, atlas, launch, or game code was modified in this audit.

## 2026-11-06 — Cape Studio mannequin rendering implementation
- The Capes tab now has a dedicated atlas texture slot, and the active profile cape is loaded and normalized with the engine's existing `process_cape()` path.
- The preview code calls the existing `render::cape()` geometry routine with the same player pose/root transform used for the mannequin body. UVs come from the installed `player#cape` model and the atlas rectangle; no guessed UV constants were added.
- Changes: `mobs/preview.rs`, `horizon-client-render/src/hud.rs`, `android/launcher_pages.rs`; commits `48953f0`, `d087ee8`, `c7e0914`.
- This is an implementation proposal applied to source, not a verified working feature. No build, APK, or device test has been run. Game-side application and account cape preview are still pending.


## Local player runtime integration — audit result (2026-11-06)
- The verified insertion point is `crates/horizon-client/src/game/remote.rs::Game::update_remote_skins()`.
- The method builds a `HashMap<UUID, cape texture>` from server/player profile textures, merges the signed-in account's active cape only when server texture data lacks one, and optionally adds the offline cape only when no local cape exists. It then assigns cape texture IDs to player mobs and the local `player_model`.
- The local player UUID is `Remote.profile`. Remote player UUIDs are distinct, so the custom override must be inserted only for that UUID; never replace the map globally or change remote mobs.
- Recommended precedence: server/account texture resolution → account profile fallback → offline cape fallback → optional Sylphra local override. The Sylphra override exists only when a CapeLibrary selection is explicitly passed at launch. If absent (`None` in Cape Studio), leave the map untouched so the account cape remains visible. Keep the existing `modelPart_cape` visibility setting and atlas `again_skin()` lifecycle.
- Launch chain to extend: `android/launcher.rs` constructs `LaunchRequest`; `android/game_launch.rs` emits CLI arguments; `game.rs` parses `--sylphra-cape`, validates and loads a bounded library-owned PNG; `remote.rs` applies the decoded 64x64 texture to local UUID only. Use `skins::process_cape()` for 64x32 / 22x17 normalization. Do not route through Elytra texture handling.
- No code was changed by this audit. Required tests: selected path argument, absent path emits no argument, invalid path/symlink/oversized or unsupported PNG rejected, selected local cape takes precedence over account/offline cape, no selection preserves account cape, remote UUID entries remain unchanged.
- Current source checks run `42187731379` passed on HEAD `a21cee999205bf28e1f3bb4e98fe299abd066ee3`; this is not compilation, APK, or device verification.


## Runtime implementation — first local-only pass (2026-11-06)
- `LaunchRequest.cape_file` now transports the active profile cape to the game as `--sylphra-cape`; no argument is emitted when selection is None. Launcher resolves the path through `Catalog::active_cape_file`, and refuses launch if the selected library file is invalid/unavailable.
- Game CLI parses the optional path. `load_sylphra_cape()` accepts only a non-symlink file in a `capes/` directory, a 64-hex content-addressed `.png` name, <=1 MiB, and supported 64x32 / 22x17 dimensions; it decodes and normalizes with existing `process_cape()`.
- `update_remote_skins()` inserts the custom atlas texture only under the local player's UUID, after existing server/account/offline cape resolution. With no Sylphra selection, account/offline behavior is unchanged. Other player UUIDs are not rewritten, and Elytra code is untouched.
- Source commits: `678aa23`, `38f0b26`, `e9985d1`, `4282427`, `2d06b12`.
- Added launch-argument tests, but they have not yet been run. No Rust compile, APK build, or device/gameplay validation has been performed. Source-check run `42189848848` was pending for code HEAD `2d06b12` when this entry was written.
- Remaining verification: compile the Rust workspace; test loader rejection/normalization; verify explicit cape overrides only the local player; verify None retains an official account cape; verify `modelPart_cape` still hides cape as before; inspect mannequin preview and in-game cape visually on device. LAN sharing remains separate.


## 2026-11-06 — Atlas UV integration correction
- `Game::store_skin()` applies cape-specific half-height atlas UVs only when the atlas key starts with `capes/`. Since `process_cape()` stores 64x32 cape pixels in the upper half of a 64x64 RGBA buffer, the custom cape must use the same namespace.
- Applied: `crates/horizon-client/src/game/remote.rs` now stores the selected local cape as `capes/sylphra-local` instead of `sylphra/local_cape` (commit `e1d5e2472f06c968f849216fe9a8cf134dc5accc`).
- This corrects the atlas-key/UV contract by inspection; visual rendering is not yet verified. No native compile, APK, or device test has been run for this change.


## 2026-11-06 — Cape preview consistency and content-address verification
- Preflight: verified branch `android`, HEAD `f2dd62f6e0c4196b95518ae7a822847007ad1dd8`, and reviewed current continuity/development/design docs plus CapeLibrary, Catalog, LaunchRequest, launcher page, preview renderer, CLI loader, and local-player texture assignment.
- [Confirmed] The Cape preview atlas patch and `PlayerPreview::draw` were guarded by `self.cape_tab`; therefore a selected cape disappeared from the 3D mannequin when switching back to Skins, despite the shared-preview design.
- [Applied] Removed that tab-only guard in `crates/horizon-client/src/android/launcher_pages.rs`. The active profile cape is now loaded into the preview atlas whenever one is selected, independent of the currently visible Skins/Capes management tab. The two tab lists and their actions remain independent.
- [Confirmed] `load_sylphra_cape` checked the content-addressed filename shape but did not recompute SHA-256 against the PNG bytes.
- [Applied] Added SHA-256 content verification in `crates/horizon-client/src/game.rs` before PNG signature/dimension decoding. A replaced or corrupted file whose contents no longer match its 64-hex filename is rejected.
- [Pending] CapeLibrary already implements `rename` and `remove`, but Skin Studio currently exposes neither action. The delete API clears profile selections before removing the PNG and explicitly requires caller confirmation; add safe UI confirmation before invoking it. Rename needs a proper text-entry flow.
- [Not verified] No Rust compilation, unit test, native CI job, APK build, or device visual/gameplay test was run in this change. The existing automated Android source-check workflow may run on the new commit; inspect its exact HEAD and job results before claiming validation.
- [Not changed] No branch other than `android`; no workflow rerun; no APK or release; no LAN protocol changes.


## 2026-11-06 — Cape library management controls
- Preflight: verified branch `android`, starting HEAD `776603bb9b28d53cf9b3dcefadf9deb58a53425c`, read the latest continuity/development/design entries, and inspected `CapeLibrary::rename/remove`, launcher actions, chooser confirmation flow, and the Android text-input path.
- [Confirmed] Cape import, selection, per-profile selection, and `None` already existed. `CapeLibrary::rename` persists a sanitized name; `remove` first saves removal of metadata and all profile selections, then removes the PNG. The UI had no controls invoking either method.
- [Applied] Added per-cape Rename and Del controls to the Capes list. Rename opens the existing Android keyboard, restricts input to printable name characters (maximum 48), and persists through `CapeLibrary::rename`. Delete opens a dedicated confirmation chooser; only its explicit confirmation invokes `CapeLibrary::remove`. The library is reloaded after removal attempts so in-memory state reflects the persisted manifest even if PNG deletion reports an error.
- [Applied] Added `pending_cape_delete`, `cape_rename_id`, and `cape_rename_text` state to Catalog, plus `RenameCape`, `DeleteCape`, and `ConfirmDeleteCape` actions. No change to import formats, account cape fallback, game launch arguments, remote-player handling, or Elytra rendering.
- [Not verified] No Rust formatting, compilation, unit tests, native CI, APK build, or Android UI test has run for this change. Confirm keyboard behavior, list layout on narrow devices, rename persistence, delete confirmation/cancellation, and profile selection cleanup in a later device test.
- [Not changed] No other branch, workflow rerun, APK, release, or LAN protocol change.


## 2026-11-06 — Cape library persistence tests and dotted-name fix
- Preflight: verified branch `android`, starting HEAD `62a628df8bd67521d0d8611a01f8b74e86e2a1aa`, read all three continuity/design documents, and inspected CapeLibrary import/rename/remove/select/save/load plus the current UI actions and workflow status.
- [Confirmed] Existing library tests covered import/deduplication, supported dimensions, profile-local `None`, deletion and corrupt manifests, but did not assert that rename and profile selection survive a fresh manifest load or that deletion preserves other profiles' different cape selections.
- [Applied] Added focused tests in `crates/horizon-client/src/android/cape_library.rs` for rename persistence (including a dotted custom name), independent profile selection persistence, `None` persistence, and deletion of one cape without disturbing another profile's different cape.
- [Applied] Fixed `clean_name` so only a PNG file extension is stripped. User-assigned names such as `Cape.v2` no longer lose their final dotted suffix during rename.
- [Not verified] Tests were added but not executed; no formatter, Rust compiler, native CI job, APK build, or device test was run. The current source-check workflow is not a substitute for these checks.
- [Not changed] No other branch, workflow rerun, APK, release, runtime protocol, account cape fallback, or LAN behavior.


## 2026-11-06 — Runtime cape-loader regression test
- Preflight: verified branch `android`, starting HEAD `93958c45faec7faad8c3211615a2fb2723c0db62`, reviewed the cape loader and its current caller, and checked the latest source workflow status.
- [Confirmed] The game-side loader validates the `capes/<sha256>.png` location, rejects symlinks and oversized files, checks the filename digest against the PNG bytes, verifies the PNG header/dimensions, and normalizes with the existing cape processor. This critical path previously had no direct unit test.
- [Applied] Added a focused unit test in `crates/horizon-client/src/game.rs` that creates a valid 64×32 content-addressed PNG, verifies normalization to a 64×64 RGBA texture, then tampers with the bytes while retaining the original digest filename and verifies rejection.
- [Not verified] Test has not been executed; no Rust compiler, test runner, native CI job, APK build, or device test was run. The GitHub source workflow only checks its configured source-level tasks.
- [Not changed] No launch precedence, remote-player handling, account fallback, Elytra behavior, other branch, workflow rerun, APK, or release.
