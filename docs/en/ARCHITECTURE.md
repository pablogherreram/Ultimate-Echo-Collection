# Architecture

## Overview

Ultimate Echo Collection separates character registration code from local character assets.

The repository contains independently compilable Rust projects. A generated `plugin.nro` registers a new character entry and redirects it to a native base fighter. The local mod package supplies the marked slot assets used by that entry.

## Main Components

```text
Source repository
├── Generic Rust template
├── Character-specific Rust projects
├── GitHub Actions workflows
└── Documentation and credits

Local mod package
├── Models and textures
├── Animations
├── Effects
├── Sounds and voices
├── UI files
├── Marker files
├── Mod metadata
└── Generated plugin.nro
```

The source repository and local mod package are related but intentionally separate.

## Character Registration Flow

A visual Echo Fighter follows this model:

```text
Custom ui_chara entry
        ↓
CharacterDatabaseEntry
        ↓
One native fighter_kind
        ↓
Base fighter directory
        ↓
Continuous physical color range
        ↓
Marker-driven selection
        ↓
Local slot-specific assets
```

### Dry Bowser Example

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
9 selectable styles
```

## Important Constants

The template exposes character-specific constants in `src/lib.rs`.

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

The marker scan also depends on:

```rust
FIGHTER_NAME
MARKER_FILE
```

### Meaning

- `YOUR_CHARA_ID`: new UI character hash name.
- `YOUR_NAME_ID`: name identifier used by character UI messages.
- `YOUR_CHARA_LAYOUT`: layout identifier for presentation data.
- `BASE_CHARA_ID`: base character UI identifier.
- `BASE_FIGHTER_KIND`: native fighter kind used for gameplay.
- `CHARA_SERIES`: UI series assignment.
- `BASE_CHARACALL`: narration used when a custom call is disabled.
- `IS_CHARA_CALL`: enables or disables a custom narration entry.
- `YOUR_CHARACALL`: custom narration identifier when enabled.
- `FIGHTER_NAME`: base fighter directory used during marker scanning.
- `MARKER_FILE`: exact marker filename expected in physical slots.

## Marker Architecture

The plugin scans physical color directories for a marker file.

Expected pattern:

```text
mods:/fighter/<base>/model/body/cXX/<marker>.marker
```

Dry Bowser example:

```text
fighter/koopa/model/body/c20/koopadry.marker
fighter/koopa/model/body/c21/koopadry.marker
...
fighter/koopa/model/body/c28/koopadry.marker
```

The plugin determines:

- the lowest marked physical slot;
- the number of continuously marked slots starting at that slot.

For `c20-c28`:

```text
lowest_color = 20
color_num = 9
```

A missing marker inside the range can truncate the detected style count. Markers after a gap are not necessarily exposed as part of the same continuous range.

## Chara DB and Layout DB

### Chara DB

The character database entry registers information such as:

- custom `ui_chara_id`;
- display name ID;
- base fighter kind;
- series;
- color count;
- first physical color;
- narration entry;
- display and metadata flags.

This registration is sufficient for the basic Dry Bowser Echo Fighter to function.

### Layout DB

Layout DB controls presentation details such as render position, offsets, scale, and related UI layout values.

Dry Bowser's first functional source preserves the layout registration block in a disabled template state. As a result, gameplay and character registration function, while the VS render is displaced.

Layout work must be treated as a separate presentation change and validated independently.

## NRO Responsibilities

The generated `plugin.nro` is responsible for registration and redirection logic. It does not contain the complete character mod.

The NRO:

- checks required runtime dependencies;
- waits for the mod filesystem mount event;
- scans marker files;
- calculates slot metadata;
- registers online allowance for the custom UI hash;
- optionally registers narration;
- registers the character database entry.

The local mod package remains responsible for the actual slot assets.

## Runtime Dependencies

The Dry Bowser reference checks for:

```text
libparam_config.nro
libthe_csk_collection.nro
libarcropolis.nro
libnro_hook.nro
libsmashline_plugin.nro
```

A future project may have additional dependencies. Document those dependencies in the project's README instead of assuming the Dry Bowser list is universal.

## Build Architecture

Each project is an isolated Cargo project.

```text
template/plugin-source
projects/dry-bowser/plugin-source
projects/<future-project>/plugin-source
```

The current stable workflows build:

```text
.github/workflows/build-template.yml
.github/workflows/build-dry-bowser.yml
```

Each workflow:

1. checks out the repository;
2. installs Linux build dependencies;
3. installs `cargo-skyline`;
4. prepares the Skyline standard library;
5. replaces the obsolete `blu-dev/skyline-smash` URL with the maintained source;
6. removes the old lockfile in the runner;
7. builds the selected Cargo project;
8. locates the generated NRO;
9. writes `plugin.nro` and `SHA256SUMS.txt` into `dist/`;
10. uploads the files as a GitHub Actions artifact.

The workflow modifies only the runner's temporary checkout. It does not rewrite the source repository.

## One Fighter Kind Per Entry

A registered character entry points to one `fighter_kind`.

A normal visual Echo configuration cannot directly express:

```text
Color 0 → fighter_kind_pickel
Color 1 → fighter_kind_mewtwo
```

Different visual styles can exist under one entry, but changing native fighter architecture by color requires custom gameplay logic far beyond the standard visual Echo template.

## Custom Moveset Compatibility

A visual entry may activate an existing custom moveset when:

- the moveset uses a known native `fighter_kind`;
- the moveset activates safely for the intended slots or markers;
- required scripts, statuses, articles, parameters, and assets are available;
- the added UI entry does not violate assumptions made by the moveset plugin.

Do not assume that a custom character name corresponds to a real custom `fighter_kind`. Many custom movesets operate on top of a native fighter kind.

## Stable Reference Boundary

The Dry Bowser project is the stable reference for:

- project organization;
- Cargo structure;
- marker scanning;
- continuous style counting;
- Chara DB registration;
- GitHub Actions packaging;
- hardware validation methodology.

Dry Bowser is not a universal source of character-specific IDs, slots, layout values, assets, or gameplay compatibility.
