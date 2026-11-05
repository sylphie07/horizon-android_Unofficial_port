# Sylphar — investigación técnica de importación de texturas de espalda (Cape / Elytra)

**Fecha:** 2026-11-05  
**Rama inspeccionada:** `android`  
**HEAD inspeccionado:** `b3480f4ae2e9a253df76a14e4bed705947a97377`  
**Estado:** investigación de solo lectura; no se modificó código de producto.

## Alcance y terminología

La petición del usuario es importar diseños de la espalda del personaje, no las seis capas de superposición de la skin (Hat, Jacket, L/R sleeve, L/R pants). En esta investigación se distinguen dos elementos:
- **Cape (capa/capa de espalda):** pieza textil colgante.
- **Elytra (élitros):** par de alas rígidas plegables que se muestran en la espalda.

El usuario ha usado “capas de élitros” y “capas de la espalda”; antes de cerrar UI se debe confirmar si quiere una biblioteca que admita **solo Elytra**, o tanto **capes como Elytra** como dos categorías. La arquitectura debe mantenerlas separadas en cualquier caso.

## Estado real encontrado en Horizon

### 1. El protocolo de perfiles ya reconoce Elytra

Archivo: `crates/horizon-client-render/src/skins.rs`
- `Textures` contiene `skin`, `cape` y `elytra` (líneas 15–25).
- El parser de la propiedad de perfil reconoce `SKIN`, `CAPE` y `ELYTRA` (líneas 65–90).
- `Kind` tiene `Skin`, `Cape` y `Elytra` (líneas 687–693).
- `lookup` considera los tres tipos de URL (líneas 695–727).
- El descargador decodifica Elytra como RGBA sin procesador específico (líneas 871–917), pero al recibir una descarga lista hace `Kind::Elytra => {}` en lugar de registrarla en el atlas (líneas 871–885).
- `again(location)` solo reconoce ubicaciones `skins/` y `capes/`; no hay ubicación Elytra (líneas 940–974).

**Conclusión:** el parser de metadatos reconoce la URL Elytra, pero el pipeline de caché/atlas de texturas no termina su integración. No afirmar que las texturas Elytra del perfil ya se renderizan.

### 2. Cape sí tiene integración local, pero Elytra no

Archivo: `crates/horizon-client-render/src/skins.rs`
- Existe `cape_location` y `process_cape` (líneas 585–627).
- `process_cape` normaliza la imagen a una ranura RGBA de 64×64 y coloca el diseño en la mitad superior; acepta imágenes de distintos tamaños y escala a 64×32 cuando corresponde (líneas 609–627).
- `offline_cape()` lee la variable `HORIZON_OFFLINE_CAPE` (líneas 629–644).

Archivo: `launcher/src/skins.rs`
- El comentario inicial documenta `HORIZON_OFFLINE_SKIN`, `HORIZON_OFFLINE_SKIN_MODEL` y `HORIZON_OFFLINE_CAPE` (líneas 1–14).
- Hay validación de Cape de 64×32 o 22×17 (líneas 90–108).
- Los archivos locales se guardan como skin y cape por nombre, y el entorno solo exporta variables para esos dos tipos (líneas 41–53 y 170–202).
- No se encontró equivalente `HORIZON_OFFLINE_ELYTRA` en el archivo inspeccionado.

**Conclusión:** existe un precedente para Cape, pero no se debe reutilizar su conversión UV para Elytra. Los mapas UV y la geometría son diferentes.

### 3. Renderizado de jugador y visor

Archivo: `crates/horizon-client/src/mobs/preview.rs`
- `load_player_type` carga `minecraft:player#main` y añade las capas de modelo `player_slim#main` y `player#cape` (líneas 23–80).
- No se añade una capa `player#elytra` en el código inspeccionado.
- `PlayerPreview::draw` renderiza el cuerpo mediante `render::render_with`, aplica la skin seleccionada, el modelo Classic/Slim, la rotación y recorte del visor (líneas 110–180).

Archivo: `crates/horizon-client/src/mobs/render.rs`
- Hay una función dedicada `cape(...)` que busca `player#cape`, calcula la pose de la capa y dibuja su geometría (líneas 1378–1419).
- La lista genérica `layers(m, age)` retorna vacía para avatares/jugadores (líneas 522–529); la capa de jugador tiene una ruta especializada.
- No se encontró una función de renderizado Elytra equivalente en los fragmentos inspeccionados.

**Conclusión:** el visor 3D actual no demuestra que pueda dibujar Elytra simplemente al seleccionar un PNG. Hace falta verificar el modelo/UV Elytra del paquete de recursos del juego y añadir una ruta explícita de renderizado, o reutilizar una implementación existente solo si se demuestra que sus geometrías y poses corresponden al élitro.

### 4. Importación y biblioteca Android

Archivo: `crates/horizon-client/src/android/documents.rs`
- El selector SAF y puente de URI ya admiten `ImportKind::Skin`.
- La importación se ejecuta en un hilo separado, lee con límite de tamaño y pasa los bytes a la biblioteca (líneas 22–41, 132–160, 199–245).

Archivo: `crates/horizon-client/src/android/skin_library.rs`
- La biblioteca usa manifiesto `skin-library.json`, `FORMAT_VERSION = 1`, directorio privado `skins/` y registros `SkinRecord` (líneas 11–48).
- Los perfiles guardan `selected_skin` y el modelo Classic/Slim (líneas 33–40).
- La importación valida PNG y limita a 1 MiB; acepta 64×64 o 64×32 (líneas 119–197 y 347–369).
- No existe colección, registro, selector ni almacenamiento para Cape/Elytra.

**Conclusión:** el SAF, la validación PNG, el almacenamiento por SHA-256 y el manifiesto son buenos patrones reutilizables. La categoría de espalda debe tener sus propios registros/directorio/selección, sin contaminar `SkinRecord` ni las seis capas actuales.

## Referencia externa sobre el formato y separación

- La propiedad de perfil de Java distingue `skin`, `cape` y `elytra` como overrides separados. Referencia: Minecraft Wiki, protocolo Java, sección Resolvable Profile: https://minecraft.wiki/w/Java_Edition_protocol/Packets
- Mojang documenta que `use_player_texture` aplica a capas de tipo `wings` y usa la textura Elytra personalizada del jugador; además indica la ruta de texturas de equipo para wings. Referencia oficial: https://feedback.minecraft.net/hc/en-us/articles/29941004144525-Minecraft-Java-Edition-Snapshot-24w36a
- Estas referencias confirman que Cape y Elytra son conceptos distintos, pero no sustituyen la inspección de los recursos/modelos que Horizon realmente carga.

## Arquitectura propuesta (no aplicada)

1. Añadir una colección independiente de registros de espalda al modelo de biblioteca, preferiblemente con categoría explícita `Elytra` y, si el usuario lo desea, otra categoría `Cape`.
2. Mantener compatibilidad del manifiesto v1 mediante migración explícita a v2; no sobrescribir manifiestos corruptos ni desconocidos.
3. Reutilizar el selector SAF y el puente de URI, pero añadir un tipo de importación dedicado.
4. Reutilizar lectura limitada, validación PNG, SHA-256, almacenamiento privado y persistencia atómica.
5. No asumir dimensiones de Cape para Elytra. Determinar las dimensiones reales y el layout UV desde los recursos/modelos de la versión instalada; validar el PNG con ese contrato.
6. Guardar la selección de espalda por perfil de apariencia, independiente de `selected_skin`.
7. Integrar la textura en un slot/cache propio (p. ej. `elytra/`) y una función de renderizado dedicada que use el modelo/UV correctos.
8. Añadir un control de vista previa de espalda o un modo de giro del avatar, sin alterar las seis capas de skin ni el comportamiento ya verificado.
9. La compilación debe esperar a que importación, persistencia, textura y renderizado (si es viable) estén implementados y cubiertos por pruebas.

## Pendientes de investigación antes de implementar

- Localizar el modelo Elytra en los recursos de juego realmente preparados y confirmar su clave de modelo, geometría, UV y dimensiones de textura.
- Confirmar si el motor ya contiene geometría Elytra en otra ruta (item model, armor/equipment model o attachable).
- Seguir la llamada de renderizado de Cape hasta el renderer de avatar y definir un punto de extensión simétrico para Elytra.
- Determinar si el usuario quiere Elytra únicamente o también Cape como segunda categoría.
- Revisar los cuatro ajustes visuales del commit `7639734a34730aa067bb03a9c0f7efff8a9d017a` en la próxima APK; siguen sin verificación visual posterior al cambio.

## Archivos inspeccionados

- `crates/horizon-client-render/src/skins.rs`
- `crates/horizon-client/src/mobs/preview.rs`
- `crates/horizon-client/src/mobs/render.rs`
- `crates/horizon-client/src/android/documents.rs`
- `crates/horizon-client/src/android/skin_library.rs`
- `launcher/src/skins.rs`
- `crates/horizon-client/src/android/catalog.rs`
- `docs/SYLPHRA_SKIN_LIBRARY.md`

**Cambios de producto:** ninguno. **Pruebas:** ninguna ejecutada en esta auditoría.