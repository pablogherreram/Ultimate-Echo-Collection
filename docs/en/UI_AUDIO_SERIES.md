# UI, Custom Series, and Announcer Audio for Echo Fighters

## Purpose

This guide records hardware-validated lessons from the Baller visual Echo Fighter project. It covers UI portrait containers, custom series registration, announcer-call preparation, tooling, packaging, and hardware validation.

## Asset organization

```text
projects/<id>/assets/
├── ui/chara/source/
├── ui/chara/final/
├── series/source/
├── series/final/
└── audio/announcer/
    ├── source/
    └── final/
```

Keep editable masters separate from installable assets and compiled binaries.

## Validated `chara_X` mapping

```text
chara_0  GSP, records, score and data-screen icon
chara_1  large CSS render, large stacked Echo render, tips portrait
chara_2  stock icon
chara_3  versus and victory/results portrait
chara_4  battle portrait
chara_5  fighter spirit portrait
chara_6  Final Smash headshot
chara_7  small CSS or stacked Echo icon
```

`chara_0` and `chara_7` are not interchangeable. Compared `chara_7` donors used a `454 x 300`, `BC7_UNORM`, non-sRGB, one-mipmap texture. Use a real `chara_7` container as a donor.

## UI editing workflow

Open one BNTX at a time in Switch Toolbox, disable `Add Files to Active Editor`, replace only the internal texture, preserve all original metadata, choose the highest-quality compression mode, save, close, and reopen the file. Never recover a high-quality source by exporting an already low-quality BC7 conversion. Reimport the original PNG master.

Preserve the full canvas and transparency. `chara_4` may require preserving an alpha-mask shape embedded in the source image. `chara_3` controls both versus and results portraits; correct the PNG composition before changing Layout DB.

## `replace` and `replace_patch`

Use `ui/replace` for assets owned by a newly registered logical character. Use `ui/replace_patch` for replacements targeting native resources already present in game data. Baller-specific `baller_00` UI assets were placed under `ui/replace`.

## Custom series

Install the icon at:

```text
ui/replace/series/series_0/series_0_<series>.bntx
```

The plugin must register `ui_series_<series>` through CSK and assign it through `CHARA_SERIES`. Register the series before registering the character entry that references it. A separate JSON patch may lose to plugin registration order.

## Custom announcer call

```rust
pub const BASE_CHARACALL: &str = "vc_narration_characall_base";
pub const IS_CHARA_CALL: bool = true;
pub const YOUR_CHARACALL: &str = "vc_narration_characall_character";
```

The hardware-validated asset path is:

```text
<Mod>/vc_narration/vc_narration_characall_<id>.idsp
```

Do not place it under `sound/bank/narration` for this CSK workflow.

Prepare a mono, 48 kHz, PCM 16-bit WAV without a loop. Measure each source rather than applying a fixed gain blindly. Convert with VGAudioCli:

```cmd
VGAudioCli.exe -i "input.wav" -o "vc_narration_characall_character.idsp"
```

Batch conversion:

```cmd
VGAudioCli.exe -b -i "WAV_READY" -o "IDSP_READY" -r --out-format idsp
```

The Baller IDSP was validated as mono, 48 kHz, 1.3584 seconds, GameCube DSP 4-bit ADPCM, with `0x10` interleave.

## Tools

- Switch Toolbox for BNTX texture export and replacement.
- Affinity for composition, alpha masks, and transparent PNG masters.
- FFmpeg/FFprobe for audio preparation and measurement.
- VGAudioCli for WAV and IDSP conversion.
- 7-Zip for release ZIP creation and testing.
- GitHub Actions for reproducible NRO compilation.
- PowerShell for SHA-256 verification and exact tree comparison.

## Packaging

Publish a ZIP using Deflate, relative paths, and no password. The archive root should directly contain `plugin.nro`, `fighter/`, `sound/`, `ui/`, `vc_narration/`, and documentation. Extract the ZIP and compare relative paths, sizes, and SHA-256 hashes before deleting the working directory.

## Hardware checklist

Validate game startup, CSS load, custom slot, `chara_7`, all UI portraits, stock icon, versus/results framing, custom series, base fighter regression, custom announcer, full match, results, return to CSS, and crash-free behavior.
