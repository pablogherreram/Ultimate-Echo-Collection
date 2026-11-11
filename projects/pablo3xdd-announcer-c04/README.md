# Pablo3xDD announcer call for Steve C04

Minimal Skyline plugin for Super Smash Bros. Ultimate.

It does not create an Echo Fighter, CSS entry, series, layout, model slot, or global narration bank. It registers one individual CSK Collection announcer call and assigns it only to `ui_chara_pickel` C04.

## Registered label

`vc_narration_characall_pablo3xdd`

## Runtime mod layout

Place the compiled `plugin.nro` and your IDSP in one ARCropolis mod folder:

```text
Pablo3xDD-Announcer-C04/
├── plugin.nro
└── vc_narration/
    └── vc_narration_characall_pablo3xdd.idsp
```

The filename and label must remain completely lowercase as shown.

## Expected result

- C00: Steve
- C01: Alex
- C02: Steve
- C03: Alex
- C04: Pablo3xDD custom call
- C05: Alex
- C06: Zombie
- C07: Enderman

## Dependencies on the console

- ARCropolis / `libarcropolis.nro`
- CSK Collection / `libthe_csk_collection.nro`
- NRO Hook / `libnro_hook.nro`

## Build locally

```text
cargo skyline build --release
```

## Build on GitHub

Add this directory at `projects/pablo3xdd-announcer-c04` and add the included workflow at `.github/workflows/build-pablo3xdd-announcer-c04.yml`. Run the workflow manually or push changes under this project.

## Scope

This source intentionally contains no marker scanner and no auto-slotting logic. It patches the existing `ui_chara_pickel` entry and only changes the C04 narration label while explicitly preserving the original C00-C07 labels.
