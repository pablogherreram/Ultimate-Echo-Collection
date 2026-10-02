# Crear un Echo Fighter visual nuevo

## Objetivo

Este documento define el proceso recomendado para crear un nuevo proyecto sin modificar la plantilla estable ni la referencia de Dry Bowser.

## Requisitos previos

Obtener:

- versión estable del repositorio;
- mod visual autorizado;
- créditos y condiciones de redistribución;
- nombre y fighter kind del luchador nativo base;
- carpeta de trabajo fuera del repositorio;
- entorno de Nintendo Switch capaz de probar el resultado.

No subir credenciales, dumps del juego ni assets no autorizados.

## Fase 1: preservar fuentes

```text
TrabajoNuevoEcho/
├── ModOriginal/
├── ModDeTrabajo/
├── FuenteRepositorio/
└── Respaldos/
```

- Mantener `ModOriginal` intacto.
- Modificar solamente `ModDeTrabajo`.
- Separar código fuente y mod instalable.
- Crear un respaldo antes de cada cambio importante.

## Fase 2: auditar el mod visual

Inspeccionar el árbol completo e identificar:

```text
Directorio del luchador base
Slots físicos existentes
Cantidad real de estilos distintos
Rutas de modelos
Rutas de animaciones
Rutas de efectos
Rutas de sonidos y voces
Rutas y nombres UI
Comportamiento de config.json
Markers existentes
Dependencias requeridas
```

No considerar redundantes archivos de color distintos solo porque sus nombres sean semejantes. No crear otro bloque de slots si el mod ya contiene todos sus estilos.

## Fase 3: elegir ID del proyecto

Usar minúsculas y guiones:

```text
dry-bowser
galacta-knight
personaje-ejemplo
```

Crear:

```text
projects/<project-id>/
├── README.md
├── CREDITS.md
└── plugin-source/
```

Copiar:

```text
template/plugin-source
```

a:

```text
projects/<project-id>/plugin-source
```

No configurar directamente la copia almacenada en `template/`.

## Fase 4: configurar Cargo

Editar `Cargo.toml` y asignar un nombre único:

```toml
[package]
name = "personaje_ejemplo_autoslotting"
```

Preservar la estructura de dependencias validada salvo que el proyecto requiera un cambio documentado.

## Fase 5: configurar `src/lib.rs`

Actualizar:

```rust
pub const YOUR_CHARA_ID: &str = "ui_chara_ejemplo";
pub const YOUR_NAME_ID: &str = "ejemplo";
pub const YOUR_CHARA_LAYOUT: &str = "ui_chara_ejemplo_00";
pub const BASE_CHARA_ID: &str = "ui_chara_base";
pub const BASE_FIGHTER_KIND: &str = "fighter_kind_base";
pub const CHARA_SERIES: &str = "ui_series_ejemplo";
pub const BASE_CHARACALL: &str = "vc_narration_characall_base";
pub const IS_CHARA_CALL: bool = false;
pub const YOUR_CHARACALL: &str = "vc_narration_characall_ejemplo";
```

Configurar el escaneo:

```rust
const FIGHTER_NAME: &str = "base";
const MARKER_FILE: &str = "ejemplo.marker";
```

Comprobar:

- `YOUR_CHARA_ID` es único;
- `YOUR_NAME_ID` coincide con las etiquetas del archivo de mensajes;
- `BASE_CHARA_ID` apunta al personaje correcto;
- `BASE_FIGHTER_KIND` es el fighter kind nativo real;
- `FIGHTER_NAME` coincide con el directorio base;
- `MARKER_FILE` coincide exactamente, incluyendo mayúsculas;
- no quedan placeholders `[CHARACTER]` en el proyecto específico.

## Fase 6: crear markers

Ruta:

```text
fighter/<base>/model/body/cXX/<marker>.marker
```

Ejemplo de tres estilos:

```text
fighter/base/model/body/c20/ejemplo.marker
fighter/base/model/body/c21/ejemplo.marker
fighter/base/model/body/c22/ejemplo.marker
```

El rango debe ser continuo.

Contar markers en CMD:

```bat
dir "C:\Ruta\Al\ModDeTrabajo\*.marker" /S /B | find /C /V ""
```

## Fase 7: documentar

El README específico debe incluir:

```text
Estado del proyecto
Luchador base
ID UI
ID UI base
Fighter kind
Directorio base
Marker
Slots
Cantidad de estilos
Instrucciones de compilación
Dependencias
Problemas conocidos
Validación en hardware
Política de assets
```

`CREDITS.md` debe incluir autor original, autor de la plantilla, integración, ecosistema y límites de redistribución.

## Fase 8: agregar workflow

Copiar como referencia:

```text
.github/workflows/build-dry-bowser.yml
```

Crear:

```text
.github/workflows/build-<project-id>.yml
```

Actualizar:

- nombre visible;
- rutas observadas;
- `working-directory`;
- nombre del paso de compilación;
- nombre del artifact;
- strings de validación únicos.

Ejemplo:

```yaml
working-directory: projects/personaje-ejemplo/plugin-source
```

Validar al menos el ID UI, fighter kind y marker del proyecto.

## Fase 9: compilar

Subir solo código y documentación. No subir:

```text
plugin.nro
target/
dist/
assets completos
golden images
archivos comprimidos
```

El workflow debe terminar en verde y generar:

```text
plugin.nro
SHA256SUMS.txt
```

## Fase 10: verificar artifact

```powershell
$plugin = "C:\Ruta\plugin.nro"
Get-Item -LiteralPath $plugin | Select-Object FullName, Length
Get-FileHash -LiteralPath $plugin -Algorithm SHA256
Get-Content "C:\Ruta\SHA256SUMS.txt"
```

Ambos hashes deben coincidir.

## Fase 11: ensamblar copia de prueba

```text
ModDeTrabajo-Prueba/
├── plugin.nro
├── config.json
├── info.toml
├── effect/
├── fighter/
├── sound/
└── ui/
```

No dejar `plugin.nro` suelto bajo `ultimate/mods`. Desactivar temporalmente mods conflictivos del mismo luchador, slots o Echo registrations.

## Fase 12: validación en hardware

Lista mínima:

```text
1. Inicia el juego.
2. Aparece la casilla independiente.
3. Aparece la cantidad esperada de estilos.
4. Carga entrenamiento.
5. Carga el modelo.
6. Funcionan efectos y audio.
7. Regresa al CSS.
8. Inicia una segunda partida.
9. Carga resultados.
10. No se cierra el juego.
```

Pruebas adicionales:

```text
Echo contra luchador base
Echo contra sí mismo
Varios estilos
Puertos distintos
Rematch
Final Smash
Kirby Copy, cuando aplique
Comportamiento en línea, cuando corresponda
```

Registrar el punto exacto de cualquier fallo.

## Fase 13: golden image

Guardar de forma privada:

```text
Mod completo probado
plugin.nro probado
SHA256SUMS.txt
Fuente exacta
Notas de validación
Problemas conocidos
```

Ejemplo:

```text
PersonajeEjemplo_100_Funcional_Golden_Image.zip
```

No sobrescribir la primera golden image al crear una versión mejorada.

## Fase 14: estabilizar

1. Mantener el proyecto bajo `projects/<project-id>`.
2. Completar documentación.
3. Ejecutar el workflow desde `main`.
4. Comparar el NRO final con el probado.
5. Borrar ramas temporales solo después de validar.
6. Conservar externamente la golden image.

## Dry Bowser como referencia cero

Usar Dry Bowser para comparar estructura, Cargo, markers, conteo de estilos, dependencias, registro Chara DB, workflow, artifact y pruebas. No copiar sus IDs específicos a otro personaje.
