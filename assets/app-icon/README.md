# Toma app icon

**Figure:** Georges Méliès. A pioneer of film editing and special effects.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Méliès portrait (cropped).png" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Méliès_portrait_(cropped).png) (public domain). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

**Linux:** the hicolor icons (`hicolor/scalable` and every PNG size) put the tile on the Nodalix
app grid, like the Colloid icons around it: a 224 px tile inside a 256 px canvas (16 px margin),
a 51.5 px corner radius and a soft shadow. The scalable SVG is made by `tools/nodalia-app-icon.py`
in [nodalix-icon-theme](https://github.com/danielmigueltejedor/nodalix-icon-theme) and
`packaging/icons.sh` renders the PNG sizes from it. macOS icons use Apple's
own grid, with a transparent margin, and the Windows `.ico` and the 1024 px render keep the
full-bleed tile. The window and taskbar icon on Windows is the 256 px
hicolor PNG, so it has the Linux margin and shadow.

## Files

| File | What |
|---|---|
| `toma.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `toma-small.svg` | lighter 512 px version; also the web favicon where there is one |
| `toma-1024.png` | 1024 px render |
| `toma-macos-512.png` | runtime Dock icon on macOS (embedded by `apps/toma/src/main.rs`) |
| `toma.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `toma.ico` | Windows icon, 16–256 px (embedded in `toma.exe` by `apps/toma/build.rs`) |
| `hicolor/<size>/apps/dev.danielmiguelt.Toma.png`, `hicolor/scalable/…svg` | Linux icon theme, used by `packaging/linux/dev.danielmiguelt.Toma.desktop` |

The web app's `apps/toma-web/web/favicon.png` (128 px) comes from the same artwork.

## How the icon reaches each OS

- **macOS:** the window icon set at startup (`ViewportBuilder::with_icon`) is the Dock and app-switcher
  icon when the app runs unbundled. A future `.app` bundle points `CFBundleIconFile` at `toma.icns`.
- **Windows:** `build.rs` embeds `toma.ico` as the exe's resource icon (Explorer, Start menu,
  pinned taskbar), and the runtime icon covers the window, taskbar and Alt-Tab.
- **Linux:** the runtime icon plus `with_app_id("dev.danielmiguelt.Toma")`, which Wayland matches to
  the `.desktop` file. Install the `hicolor/` tree under `/usr/share/icons/hicolor/`, with a copy of each
  `dev.danielmiguelt.Toma.*` icon named `dev.danielmiguelt.Toma.*` (the `.desktop` file's `Icon=`), and
  the desktop file under `/usr/share/applications/`.

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `toma.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
