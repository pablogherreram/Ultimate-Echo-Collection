# Dr. Luigi auto-slotting plugin

Skyline plugin source for registering Dr. Luigi as an independent
visual Echo Fighter entry in Super Smash Bros. Ultimate.

## Configuration

UI character: ui_chara_drluigi
Logical layout: ui_chara_drluigi_00
Base UI character: ui_chara_luigi
Base fighter: fighter_kind_luigi
Fighter directory: luigi
Physical slot: c20
Marker: drluigi.marker
Styles: 1
Series: ui_series_mario
Narration call: vc_narration_characall_drluigi

The physical Luigi c20 slot is exposed as logical Dr. Luigi style c00.

## Build

Run cargo skyline build --release or execute Build.bat.

The GitHub Actions workflow publishes plugin.nro and SHA256SUMS.txt.

## Credits

See the project-level CREDITS.md.
