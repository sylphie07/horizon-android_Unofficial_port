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
