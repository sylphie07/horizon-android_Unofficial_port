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
