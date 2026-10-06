# UI, series personalizadas y narración para Echo Fighters

## Propósito

Esta guía conserva las lecciones técnicas validadas durante la creación de **Baller**, un Echo Fighter visual basado físicamente en Min Min `c02`. Documenta la organización de recursos UI, el significado de los archivos `chara_X`, la creación de una serie personalizada, la preparación de una llamada del narrador y el proceso de validación en hardware.

No sustituye la guía general de creación de Echo Fighters. Debe leerse junto con `ARCHITECTURE.md`, `CREATE_NEW_ECHO.md` y `TROUBLESHOOTING.md`.

## 1. Separar código, fuentes editables y paquete instalable

Mantener tres espacios distintos:

```text
Repositorio fuente
├── projects/<id>/plugin-source/
└── projects/<id>/assets/

Paquete instalable
├── plugin.nro
├── fighter/
├── sound/
├── ui/
└── vc_narration/

Golden image privada
└── ZIP completo validado en hardware
```

Los PNG, WAV y archivos editables son fuentes. Los BNTX, IDSP y NRO forman parte del paquete final. No sobrescribir una golden image funcional mientras se experimenta.

## 2. Organización recomendada de assets

```text
projects/<id>/assets/
├── ui/
│   └── chara/
│       ├── source/
│       └── final/
├── series/
│   ├── source/
│   └── final/
└── audio/
    └── announcer/
        ├── source/
        └── final/
```

- `source/`: renders 4K, documentos editables y WAV maestros.
- `final/`: PNG recortados, BNTX probados e IDSP finales, únicamente cuando su redistribución esté autorizada.
- No guardar `plugin.nro` en el historial normal. Distribuirlo mediante GitHub Actions o Releases.

## 3. Mapa validado de `chara_X`

```text
chara_0  GSP, registros, puntuaciones y algunas pantallas de datos
chara_1  render grande del selector CSS, Echo apilado grande y tips
chara_2  stock icon o icono de vidas
chara_3  retrato de VS y pantalla de victoria/resultados
chara_4  retrato de combate
chara_5  retrato de espíritu de luchador
chara_6  primer plano del Smash Final
chara_7  icono pequeño de la cuadrícula o Echo apilado pequeño
```

Hallazgos importantes:

- `chara_0` no sustituye a `chara_7`.
- Un `chara_0` típico mide `128 x 128`.
- Los tres `chara_7` comparados medían `454 x 300`, `BC7_UNORM`, sin sRGB y con un mipmap.
- No crear `chara_7` renombrando un contenedor `chara_0`. Usar un BNTX `chara_7` real como donante y reemplazar únicamente la textura.
- El sufijo final sigue siendo el estilo lógico. Ejemplo correcto: `chara_7_baller_00.bntx`, no `chara_7_baller_07.bntx`.

## 4. Rutas UI: `replace` frente a `replace_patch`

Para una entrada UI nueva:

```text
ui/replace/chara/chara_X/chara_X_<nuevo-id>_00.bntx
```

Para reemplazar recursos nativos ya existentes del slot físico:

```text
ui/replace_patch/chara/chara_X/chara_X_<base>_cXX.bntx
```

En Baller:

```text
ui/replace/chara/chara_7/chara_7_baller_00.bntx
ui/replace_patch/chara/chara_7/...  # no era necesario
```

Los recursos exclusivos de Baller se editaron bajo `ui/replace`. Los `tantan_02` de `replace_patch` se conservaron como recursos del slot físico de Min Min.

## 5. Preparación de renders y encuadres

Usar el render maestro de máxima resolución por separado para cada lienzo. No reducir una vez y volver a ampliar.

Reglas:

1. Exportar la textura original a PNG para conocer el lienzo exacto.
2. Conservar ancho, alto y transparencia.
3. Editar el documento completo, no recortar al contenido.
4. Exportar PNG RGBA con fondo transparente.
5. Reemplazar la textura dentro de un BNTX ya funcional.
6. Reabrir el BNTX y comprobar que el cambio persistió.
7. Probar en consola. La vista de Toolbox no sustituye la prueba en hardware.

`chara_4` puede contener una silueta diagonal incrustada en el canal alfa. Si el juego no aplica la máscara, reutilizar el alfa original como máscara en Affinity y exportar el lienzo completo.

`chara_3` alimenta VS y resultados. Si ambos aparecen desplazados, corregir primero la posición y escala del render dentro del PNG. En Baller, mover el render hacia la derecha y arriba resolvió ambas pantallas sin modificar Layout DB.

## 6. Switch Toolbox

Flujo recomendado:

```text
Abrir un solo BNTX
→ seleccionar la textura interna
→ Replace
→ seleccionar PNG final
→ conservar dimensiones, formato, sRGB y mipmaps
→ máxima calidad de compresión
→ Save
→ cerrar y reabrir
```

Precauciones:

- Desactivar `Add Files to Active Editor` para editar y guardar contenedores individuales.
- El editor combinado puede no tener destino de guardado.
- Usar la opción de compresión de mayor calidad. `Fast (Lower Quality)` puede introducir artefactos BC7 visibles en bordes diagonales, degradados y alfa.
- Si una textura ya fue comprimida con baja calidad, no exportarla y recomprimir esa exportación. Reimportar el PNG maestro original.
- No cambiar el nombre interno salvo que una prueba controlada demuestre que es necesario.

## 7. Serie personalizada

El recurso visual se instala en:

```text
ui/replace/series/series_0/series_0_<serie>.bntx
```

Ejemplo:

```text
ui/replace/series/series_0/series_0_roblox.bntx
```

Una plantilla validada usó:

```text
256 x 256
BC7_SRGB
Use SRGB: true
Mip Count: 1
Fondo transparente
```

El BNTX por sí solo no cambia la serie del personaje. El plugin debe:

1. Registrar `ui_series_<serie>` mediante `add_series_db_entry_info` o un mecanismo equivalente de CSK.
2. Asignar ese ID mediante `CHARA_SERIES` a la entrada personalizada.

Ejemplo conceptual:

```rust
pub const CHARA_SERIES: &str = "ui_series_roblox";
```

La entrada de serie debe registrarse antes de registrar la entrada de personaje que la utiliza. Un JSON externo puede no sobreescribir una entrada creada posteriormente por el NRO; Baller requirió el registro y asignación desde el plugin.

## 8. Narrador personalizado

Constantes relevantes:

```rust
pub const BASE_CHARACALL: &str = "vc_narration_characall_base";
pub const IS_CHARA_CALL: bool = true;
pub const YOUR_CHARACALL: &str = "vc_narration_characall_personaje";
```

Cuando `IS_CHARA_CALL` es `true`, el plugin registra `YOUR_CHARACALL` mediante `add_narration_characall_entry` y lo asigna a `characall_label_c00`.

### Ruta válida del IDSP

La ruta comprobada en hardware es:

```text
<Mod>/vc_narration/vc_narration_characall_<id>.idsp
```

No usar:

```text
<Mod>/sound/bank/narration/
```

Esa ruta no fue detectada por el flujo actual de CSK usado en Baller.

### Preparar el WAV maestro

Objetivo validado:

```text
Codec: PCM signed 16-bit little endian
Frecuencia: 48000 Hz
Canales: 1, mono
Loop: ninguno
Pico máximo aproximado: -2.5 dBFS
```

Para el audio de Baller se aplicó `+9 dB` al MP3 fuente porque medía aproximadamente `-23.9 LUFS` y alcanzaba solo `-11.5 dBFS`. Después del ajuste quedó cerca de `-17.7 dB` de volumen medio y `-2.5 dBFS` de pico.

Conversión con FFmpeg:

```powershell
ffmpeg -y -i "entrada.mp3" -af "volume=9dB" -ac 1 -ar 48000 -c:a pcm_s16le "salida.wav"
```

No aplicar `+9 dB` ciegamente a todas las voces. Medir cada fuente o normalizar un lote con un objetivo controlado. Para futuros lotes, conservar los WAV resultantes y escuchar cada variante antes de elegir.

### Convertir WAV a IDSP

Usar `VGAudioCli`:

```cmd
VGAudioCli.exe -i "entrada.wav" -o "vc_narration_characall_personaje.idsp"
```

Propiedades del IDSP validado de Baller:

```text
Sample count: 65202
Duración: 1.3584 s
Sample rate: 48000 Hz
Channel count: 1
Encoding: GameCube DSP 4-bit ADPCM
Interleave: 0x10
```

Ver metadatos:

```cmd
VGAudioCli.exe -m "vc_narration_characall_personaje.idsp"
```

Conversión por lote recursiva:

```cmd
VGAudioCli.exe -b -i "WAV_LISTOS" -o "IDSP_LISTOS" -r --out-format idsp
```

El nombre del archivo debe coincidir exactamente con el identificador registrado por el plugin.

## 9. Herramientas usadas

- Switch Toolbox: exportar y reemplazar texturas BNTX.
- Affinity: composición, transparencia, máscaras y recortes.
- FFmpeg y FFprobe: conversión, remuestreo y medición de WAV.
- VGAudioCli: conversión WAV ↔ IDSP y metadatos.
- 7-Zip: empaquetado ZIP e integridad.
- GitHub Actions: compilación reproducible del NRO.
- PowerShell: SHA-256 y comparación exacta de árboles.

## 10. Empaquetado y validación

Para GitHub Releases usar ZIP, Deflate, rutas relativas y sin contraseña. La raíz del ZIP debe contener directamente:

```text
plugin.nro
fighter/
sound/
ui/
vc_narration/
README.txt
CREDITS.txt
```

Validar el ZIP extrayéndolo y comparando cada archivo mediante ruta relativa, tamaño y SHA-256. En Baller, 80 archivos originales y 80 extraídos resultaron idénticos bit a bit.

## 11. Lista de validación en hardware

```text
Juego inicia
CSS carga
Entrada personalizada aparece
Icono chara_7 aparece
Render grande aparece
Stock icon aparece
VS y resultados están encuadrados
Serie personalizada aparece
Personaje base conserva su serie
Narrador custom suena
Personaje base conserva su narrador
Combate completo funciona
Resultados cargan
Regreso al CSS funciona
No hay crash
```

## 12. Lecciones específicas de Baller

- Baller usa `fighter_kind_tantan` y el slot físico `c02`.
- La entrada lógica utiliza estilo `00` aunque el modelo físico pertenezca a `c02`.
- Los BNTX exclusivos usan `baller_00` en `ui/replace`.
- La miniatura ausente era `chara_7`, no `chara_0`.
- El recurso de VS/resultados era `chara_3`, no `chara_6`.
- `chara_6` es el primer plano del Smash Final.
- La serie Roblox requirió registro y asignación desde el plugin.
- El narrador solo cargó cuando el IDSP se colocó en `vc_narration/`.
- El custom Final Smash queda fuera del alcance de v1.0.0 y debe desarrollarse como una función separada.
