# AI and Maintainer Context

## Purpose

This document provides the minimum context required to continue development of **Ultimate Echo Collection** safely and consistently.

Ultimate Echo Collection is a multi-project source repository for building visual Echo Fighter registration plugins for *Super Smash Bros. Ultimate*. The repository stores Rust source code, Cargo project files, GitHub Actions workflows, technical documentation, and project metadata. It does not store complete third-party skin mods, extracted game files, private golden images, or generated `plugin.nro` files in normal Git history.

## Read This First

Before modifying or creating a project:

1. Read the root `README.md`.
2. Read `docs/ARCHITECTURE.md`.
3. Read `docs/CREATE_NEW_ECHO.md`.
4. Read `docs/TROUBLESHOOTING.md`.
5. Inspect `template/plugin-source`.
6. Use `projects/dry-bowser` as the first known-good implementation.
7. Preserve a private copy of the latest hardware-validated mod package before making changes.
8. Prefer minimal and localized changes over complete rewrites.

## Repository Layout

```text
.github/workflows/
├── build-dry-bowser.yml
└── build-template.yml

template/
└── plugin-source/
    ├── src/lib.rs
    ├── Build.bat
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── ui_chara_db.xml
    └── ui_layout_db.xml

projects/
├── README.md
└── dry-bowser/
    ├── README.md
    ├── CREDITS.md
    └── plugin-source/
        ├── src/lib.rs
        ├── Build.bat
        ├── Cargo.lock
        ├── Cargo.toml
        ├── README.md
        ├── ui_chara_db.xml
        └── ui_layout_db.xml
```

## Core Concept

A visual Echo Fighter project registers a new UI character entry that redirects to one native `fighter_kind`.

Conceptually:

```text
New UI character entry
        ↓
One native fighter_kind
        ↓
A continuous range of marked physical color slots
        ↓
Local mod assets for those slots
```

The generated `plugin.nro` registers and redirects the character entry. The complete fighter assets remain in the locally assembled mod package.

## First Known-Good Reference

The reference implementation is:

```text
projects/dry-bowser
```

Dry Bowser uses:

```text
Project ID: dry-bowser
UI character: ui_chara_koopadry
Base UI character: ui_chara_koopa
Fighter kind: fighter_kind_koopa
Base fighter directory: koopa
Marker file: koopadry.marker
Physical slots: c20-c28
Detected styles: 9
```

The project was compiled from the reorganized repository, validated by GitHub Actions, and tested successfully on Nintendo Switch hardware.

Validated behavior included:

- the game started correctly;
- an independent Dry Bowser CSS entry appeared;
- all nine styles were selectable;
- Training Mode loaded correctly;
- models loaded correctly;
- custom effects worked correctly;
- returning to the CSS worked;
- a second match started correctly;
- the results screen loaded correctly;
- no crashes were observed.

The final NRO produced from the reorganized `main` branch was SHA-256 identical to the previously tested NRO:

```text
9F081883E36A42DB45F4C9B815EEB70CD11EF32D2C5797E4C9D2A38627F68F54
```

## Golden Image Policy

A private golden image of the first hardware-validated Dry Bowser build is preserved outside the repository.

The golden image contains the complete local mod package, authorized assets, marker files, configuration, the tested `plugin.nro`, and source snapshots. It must not be overwritten when later presentation fixes or experiments are made.

Do not commit personal cloud paths, vault names, private backups, or the complete golden image to the repository.

## Source of Truth

Use these sources in this order:

1. The stable project under `projects/<project-id>`.
2. The generic template under `template/plugin-source`.
3. The project documentation and credits.
4. A private hardware-validated golden image, when available.
5. A generated GitHub Actions artifact and its `SHA256SUMS.txt`.

Do not treat a filename, folder name, or detected asset as proof that its behavior was fully validated.

## Editing Rules

- Preserve the last functional version before changing code.
- Make one logical change at a time.
- Do not rebuild a working project from scratch to fix a small detail.
- Keep the generic template generic.
- Keep character-specific identifiers inside the character project.
- Do not modify Dry Bowser merely to make the code look cleaner unless the change is separately compiled and tested.
- Do not silently change markers, slots, UI identifiers, base fighter kinds, or dependency behavior.
- Record known limitations honestly.

## Required Project Information

Every future project should document:

```text
Project ID
Display name
UI character ID
Base UI character ID
Base fighter kind
Base fighter directory
Marker filename
Physical slot range
Number of styles
Narration behavior
Required runtime dependencies
Original asset author
Build status
Hardware validation status
Known issues
```

## Files That Must Not Be Committed

Unless redistribution is explicitly authorized, do not commit:

- complete third-party skin mods;
- extracted game files;
- proprietary models, textures, animations, sounds, voices, effects, or UI files;
- private golden images;
- generated `plugin.nro` files in normal source history;
- `target/` and `dist/` build directories;
- archives and backups;
- credentials, tokens, private keys, sessions, or personal information.

## Build and Validation Standard

A project is not stable merely because Rust compiles.

The preferred validation chain is:

```text
Source review
→ GitHub Actions build
→ NRO identifier validation
→ SHA-256 verification
→ Local mod integration
→ Nintendo Switch hardware test
→ Private golden image
→ Stable main branch
```

When two `plugin.nro` files share the same SHA-256 hash, the files are identical bit for bit.

## Current Known Dry Bowser Presentation Issues

The first functional build intentionally preserves these non-blocking issues:

- the displayed name is `Bowser Skelet`;
- the Punch-Out title is `Le roi des os`;
- the VS render requires a layout adjustment;
- Final Smash uses regular Giga Bowser.

These issues do not invalidate the functional reference build. Address them in separate, reversible changes.

## AI-Specific Guidance

When an AI continues this project:

1. Do not assume every visual mod can become an Echo Fighter without analysis.
2. Identify the native base fighter and actual `fighter_kind` first.
3. Inspect the complete mod tree before recommending slot changes.
4. Distinguish source code from local assets and generated binaries.
5. Treat escaped HTML entities in technical content as possible presentation artifacts before diagnosing syntax errors.
6. Never invent inaccessible file contents.
7. Cite the exact files and constants that support technical conclusions.
8. Preserve the working geometry, IDs, paths, and marker sequence.
9. Request only the specific missing project or file when evidence is insufficient.
10. Use Dry Bowser as a reference, not as proof that every fighter will require identical values.
