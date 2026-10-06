# Create a New Visual Echo Fighter

## Goal

This guide describes the preferred process for creating a new visual Echo Fighter project without modifying the stable template or the Dry Bowser reference.

## Prerequisites

Before starting, obtain:

- the current stable repository;
- an authorized visual skin mod;
- the original mod credits and redistribution terms;
- the native base fighter name and fighter kind;
- a local working directory outside the repository;
- a Nintendo Switch modding environment capable of testing the result.

Do not upload private credentials, game dumps, or unauthorized third-party assets.

## Phase 1: Preserve Sources

Create a local working structure:

```text
NewEchoWork/
├── OriginalMod/
├── WorkingMod/
├── RepositorySource/
└── Backups/
```

Rules:

- Keep `OriginalMod` unchanged.
- Make all asset changes inside `WorkingMod`.
- Keep the repository source separate from the installable mod.
- Preserve a dated local backup before each major change.

For multi-file projects, a ZIP may be used locally to preserve folder structure. Exclude secrets, private databases, caches, `node_modules`, build output, and other unnecessary files.

## Phase 2: Audit the Visual Mod

Inspect the complete folder tree before changing slots.

Identify:

```text
Base fighter directory
Existing physical color slots
Number of genuinely distinct styles
Model paths
Motion paths
Effect paths
Sound and voice paths
UI paths and names
Existing config.json behavior
Existing marker files
Required runtime plugins
```

Do not classify distinct color assets as redundant merely because filenames are similar.

Do not create a second block of slots if the original mod already provides the required style set.

## Phase 3: Choose a Project ID

Use a lowercase ID with hyphens:

```text
dry-bowser
galacta-knight
example-character
```

Create:

```text
projects/<project-id>/
├── README.md
├── CREDITS.md
└── plugin-source/
```

Copy the complete generic template:

```text
template/plugin-source
```

to:

```text
projects/<project-id>/plugin-source
```

Do not edit `template/plugin-source` for one character.

## Phase 4: Configure Cargo

Open:

```text
projects/<project-id>/plugin-source/Cargo.toml
```

Set a unique package name, for example:

```toml
[package]
name = "example_character_autoslotting"
```

Preserve the validated dependency structure unless a specific project requires a documented change.

The GitHub Actions runner currently patches an obsolete `skyline-smash` source and regenerates dependency resolution in the temporary build environment.

## Phase 5: Configure `src/lib.rs`

Update the character-specific constants.

Example pattern:

```rust
pub const YOUR_CHARA_ID: &str = "ui_chara_example";
pub const YOUR_NAME_ID: &str = "example";
pub const YOUR_CHARA_LAYOUT: &str = "ui_chara_example_00";
pub const BASE_CHARA_ID: &str = "ui_chara_base";
pub const BASE_FIGHTER_KIND: &str = "fighter_kind_base";
pub const CHARA_SERIES: &str = "ui_series_example";
pub const BASE_CHARACALL: &str = "vc_narration_characall_base";
pub const IS_CHARA_CALL: bool = false;
pub const YOUR_CHARACALL: &str = "vc_narration_characall_example";
```

Configure the marker scanner:

```rust
const FIGHTER_NAME: &str = "base";
const MARKER_FILE: &str = "example.marker";
```

### Required checks

- `YOUR_CHARA_ID` is unique.
- `YOUR_NAME_ID` matches the UI message labels in the local mod.
- `BASE_CHARA_ID` points to the intended native UI character.
- `BASE_FIGHTER_KIND` is the real native fighter kind.
- `FIGHTER_NAME` matches the base fighter directory.
- `MARKER_FILE` matches the exact marker filename, including case.
- No `[CHARACTER]` placeholders remain in the character project.

## Phase 6: Create Marker Files

Place an empty marker file inside each intended physical body slot:

```text
fighter/<base>/model/body/cXX/<marker>.marker
```

Example for three styles:

```text
fighter/base/model/body/c20/example.marker
fighter/base/model/body/c21/example.marker
fighter/base/model/body/c22/example.marker
```

The range should be continuous.

Verify the marker count before compiling.

Windows CMD example:

```bat
dir "C:\Path\To\WorkingMod\*.marker" /S /B | find /C /V ""
```

Do not create unnecessary additional slot ranges.

## Phase 7: Document the Project

Create `projects/<project-id>/README.md` containing:

```text
Project status
Base fighter
UI character ID
Base UI character ID
Fighter kind
Fighter directory
Marker filename
Physical slots
Style count
Build instructions
Runtime dependencies
Known issues
Hardware validation status
Asset availability policy
```

Create `projects/<project-id>/CREDITS.md` containing:

- original mod and asset author;
- original auto-slotting template author;
- integration and maintenance credits;
- runtime and development ecosystem;
- redistribution limitations.

## Phase 8: Add a Build Workflow

For the current repository structure, copy the known-good Dry Bowser workflow:

```text
.github/workflows/build-dry-bowser.yml
```

Create:

```text
.github/workflows/build-<project-id>.yml
```

Update:

- workflow display name;
- observed source path;
- `working-directory`;
- build step name;
- artifact name;
- optional identifier checks.

Example target path:

```yaml
working-directory: projects/example-character/plugin-source
```

Add binary validation strings that are unique to the project, such as:

```text
custom ui_chara ID
base fighter kind
marker filename
```

Do not delete the working Dry Bowser or template workflows while introducing a new workflow.

## Phase 9: Build Through GitHub Actions

Push only source code and documentation.

Do not push:

```text
plugin.nro
target/
dist/
complete mod assets
private golden images
archives
```

Run the new workflow and confirm:

```text
All build steps are green
plugin.nro exists
SHA256SUMS.txt exists
identifier validation passes
```

Download the artifact.

## Phase 10: Verify the Artifact

Check the extracted files:

```text
plugin.nro
SHA256SUMS.txt
```

PowerShell verification:

```powershell
$plugin = "C:\Path\To\plugin.nro"
Get-Item -LiteralPath $plugin | Select-Object FullName, Length
Get-FileHash -LiteralPath $plugin -Algorithm SHA256
Get-Content "C:\Path\To\SHA256SUMS.txt"
```

The computed hash and the hash recorded in `SHA256SUMS.txt` must match.

## Phase 11: Assemble a Test Mod

Copy the working mod to a separate test folder.

Place the generated file at the root:

```text
WorkingMod-Test/
├── plugin.nro
├── config.json
├── info.toml
├── effect/
├── fighter/
├── sound/
└── ui/
```

Do not place `plugin.nro` loose directly under `ultimate/mods`.

Disable conflicting mods during the first test:

- other mods of the same base fighter;
- other projects using the same physical slots;
- other Echo registrations for the same character;
- incompatible custom movesets or parameter hooks.

## Phase 12: Hardware Validation

Minimum test checklist:

```text
1. Game starts.
2. Independent CSS entry appears.
3. Expected style count appears.
4. Training Mode loads.
5. Model loads correctly.
6. Custom effects and audio work.
7. Return to CSS works.
8. A second match starts.
9. Results screen loads.
10. No crash occurs.
```

Additional recommended tests:

```text
Echo versus base fighter
Echo versus itself
Multiple styles in one match
Different player ports
Stage selection and rematch
Final Smash
Kirby copy behavior, when applicable
Online allowance behavior, when appropriate
```

Record exactly where a failure occurs.

## Phase 13: Preserve a Golden Image

After a successful hardware test, create a private archive containing:

```text
Complete tested mod package
Tested plugin.nro
SHA256SUMS.txt
Exact source snapshot
Validation notes
Known issues
```

Use a clear name, for example:

```text
ExampleCharacter_100_Functional_Golden_Image.zip
```

Do not overwrite the first known-good archive when creating improved versions.

## Phase 14: Stabilize the Repository

After successful testing:

1. keep the project under `projects/<project-id>`;
2. ensure documentation is complete;
3. run the project workflow from `main`;
4. compare the final NRO hash with the tested NRO;
5. remove temporary development branches only after the final build is validated;
6. retain the private golden image externally.

## Dry Bowser as Reference Zero

Use Dry Bowser to compare:

- folder layout;
- Cargo project organization;
- marker scanning;
- style counting;
- dependency checks;
- Chara DB registration;
- workflow packaging;
- artifact validation;
- hardware test procedure.

Do not copy Dry Bowser-specific IDs into another character.


## Phase 15: Advanced UI, Series, and Announcer

For `chara_X` portraits, BNTX handling, custom series registration, WAV/IDSP preparation, the `vc_narration/` path, Switch Toolbox quality settings, and release packaging, see `UI_AUDIO_SERIES.md`. That guide records the hardware-validated Baller workflow.
