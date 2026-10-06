# Arquitectura

## Descripción general

Ultimate Echo Collection separa el código de registro de personajes de los assets locales. Cada personaje tiene un proyecto Rust compilable de forma independiente. El `plugin.nro` generado registra una entrada nueva y la redirige hacia un luchador nativo; el paquete local del mod aporta los assets asignados a los slots marcados.

## Componentes principales

```text
Repositorio fuente
├── Plantilla Rust genérica
├── Proyectos específicos
├── Workflows de GitHub Actions
└── Documentación y créditos

Paquete local del mod
├── Modelos y texturas
├── Animaciones
├── Efectos
├── Sonidos y voces
├── UI
├── Markers
├── Metadatos del mod
└── plugin.nro generado
```

Ambos componentes están relacionados, pero deben mantenerse separados.

## Flujo de registro

```text
ui_chara personalizado
        ↓
CharacterDatabaseEntry
        ↓
Un fighter_kind nativo
        ↓
Directorio del luchador base
        ↓
Rango continuo de colores físicos
        ↓
Detección mediante markers
        ↓
Assets locales por slot
```

### Ejemplo de Dry Bowser

```text
ui_chara_koopadry
        ↓
fighter_kind_koopa
        ↓
fighter/koopa
        ↓
c20-c28
        ↓
koopadry.marker
        ↓
9 estilos seleccionables
```

## Constantes importantes

La plantilla expone constantes específicas en `src/lib.rs`:

```rust
YOUR_CHARA_ID
YOUR_NAME_ID
YOUR_CHARA_LAYOUT
BASE_CHARA_ID
BASE_FIGHTER_KIND
CHARA_SERIES
BASE_CHARACALL
IS_CHARA_CALL
YOUR_CHARACALL
```

El escaneo también depende de:

```rust
FIGHTER_NAME
MARKER_FILE
```

### Significado

- `YOUR_CHARA_ID`: identificador hash nuevo para la UI.
- `YOUR_NAME_ID`: identificador usado por los mensajes de nombre.
- `YOUR_CHARA_LAYOUT`: identificador de layout.
- `BASE_CHARA_ID`: personaje UI nativo usado como base.
- `BASE_FIGHTER_KIND`: fighter kind nativo que aporta el gameplay.
- `CHARA_SERIES`: serie asignada en la UI.
- `BASE_CHARACALL`: narración base cuando no se usa llamada personalizada.
- `IS_CHARA_CALL`: activa o desactiva una llamada personalizada.
- `YOUR_CHARACALL`: identificador de narración personalizada.
- `FIGHTER_NAME`: nombre del directorio base escaneado.
- `MARKER_FILE`: nombre exacto del marker esperado.

## Arquitectura de markers

Patrón esperado:

```text
mods:/fighter/<base>/model/body/cXX/<marker>.marker
```

Ejemplo:

```text
fighter/koopa/model/body/c20/koopadry.marker
fighter/koopa/model/body/c21/koopadry.marker
...
fighter/koopa/model/body/c28/koopadry.marker
```

El plugin determina:

- el slot marcado más bajo;
- la cantidad de slots marcados de forma continua desde ese punto.

Para `c20-c28`:

```text
lowest_color = 20
color_num = 9
```

Un marker faltante dentro del rango puede truncar la cantidad detectada. Los markers posteriores a una separación no necesariamente forman parte del mismo rango.

## Chara DB y Layout DB

### Chara DB

La entrada de la base de personajes registra, entre otros datos:

- `ui_chara_id` personalizado;
- ID del nombre;
- fighter kind base;
- serie;
- cantidad de colores;
- primer color físico;
- narración;
- flags de visualización y metadatos.

Esta entrada es suficiente para que la versión básica de Dry Bowser funcione.

### Layout DB

Layout DB controla posición, desplazamientos, escala y otros valores de presentación. En la primera fuente funcional de Dry Bowser, el bloque de registro del layout permanece desactivado. El gameplay funciona, pero el render VS aparece desplazado.

El trabajo de layout debe tratarse como una corrección visual separada.

## Responsabilidades del NRO

El NRO:

- comprueba dependencias de runtime;
- espera el montaje del sistema de mods;
- escanea markers;
- calcula slots y estilos;
- permite el hash UI en línea;
- registra narración si corresponde;
- registra la entrada del personaje.

El NRO no contiene el mod visual completo. Los assets permanecen en el paquete local.

## Dependencias de runtime

Dry Bowser comprueba:

```text
libparam_config.nro
libthe_csk_collection.nro
libarcropolis.nro
libnro_hook.nro
libsmashline_plugin.nro
```

Un proyecto futuro puede requerir dependencias adicionales. Deben documentarse en el README específico.

## Arquitectura de compilación

Cada proyecto es un Cargo independiente:

```text
template/plugin-source
projects/dry-bowser/plugin-source
projects/<proyecto-futuro>/plugin-source
```

Workflows estables actuales:

```text
.github/workflows/build-template.yml
.github/workflows/build-dry-bowser.yml
```

Cada workflow:

1. descarga el repositorio;
2. instala dependencias Linux;
3. instala `cargo-skyline`;
4. prepara la biblioteca estándar de Skyline;
5. sustituye la URL obsoleta de `skyline-smash` por la fuente mantenida;
6. elimina el lockfile antiguo solamente dentro del runner;
7. compila el Cargo seleccionado;
8. localiza el NRO;
9. genera `plugin.nro` y `SHA256SUMS.txt` dentro de `dist/`;
10. publica ambos como artifact.

Las modificaciones del workflow ocurren en la copia temporal del runner, no en el repositorio.

## Un fighter kind por entrada

Una entrada registrada apunta a un único `fighter_kind`. La plantilla visual no puede expresar directamente:

```text
Color 0 → fighter_kind_pickel
Color 1 → fighter_kind_mewtwo
```

Varios estilos visuales sí pueden coexistir, pero cambiar la arquitectura nativa por color requiere lógica de gameplay avanzada fuera del alcance de la plantilla.

## Compatibilidad con custom movesets

Una entrada visual puede activar un custom moveset existente si:

- utiliza un fighter kind nativo conocido;
- se activa de forma segura para los slots o markers elegidos;
- sus scripts, estados, artículos, parámetros y assets están disponibles;
- la nueva entrada UI no rompe supuestos internos del plugin del moveset.

Un nombre de personaje custom no implica que exista un `fighter_kind` custom auténtico.

## Límite de la referencia estable

Dry Bowser sirve como referencia para:

- organización del proyecto;
- estructura Cargo;
- escaneo de markers;
- conteo continuo de estilos;
- registro Chara DB;
- empaquetado mediante Actions;
- metodología de validación.

Dry Bowser no aporta automáticamente los IDs, slots, valores de layout, assets ni compatibilidad de otro personaje.
