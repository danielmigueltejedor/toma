# Toma app icon

**Creature:** an owl with ear tufts, in a frontal head-and-shoulders portrait. It is the template the
other Crafting App icons follow.

## Palette

Exactly three colours:

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | line work and the figure's contour |
| Paper | `#efe9dc` | the figure (the owl) |
| Toma violet (app colour) | `#8b5cf6` | the full-bleed field behind the figure |

## Geometry and style

- A 512-unit viewBox tile, rounded square `rx=112`, with the app colour full bleed (no frame or border).
- An engraving-style portrait in the owl-template framing: the top of the head near y 80–100, the
  eyes at about 40 % of the height, and the shoulders running off the bottom edge.
- macOS renders (`toma.icns`, `toma-macos-512.png`) put the tile in Apple's grid (824 of
  1024 px, transparent margin). Windows, Linux and the web use the full-bleed tile.

## Provenance

The owner drew it in Nodalia as a 2880 px engraving-style image, vectorised with potrace, which keys
the drawing to the three-colour palette and traces the ink. Licence: [LICENSE.txt](LICENSE.txt) (MIT OR Apache-2.0).

## Files

| File | What |
|---|---|
| `toma.svg` | canonical artwork, traced at 2048 px (about 1 MB) |
| `toma-small.svg` | lighter trace at 1024 px (about 450 KB), for when size matters |
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
  the `.desktop` file. Install the `hicolor/` tree under `/usr/share/icons/hicolor/` and the desktop
  file under `/usr/share/applications/`.

## Regenerate

`packaging/icons.sh` (needs `resvg`; `iconutil` on macOS for the `.icns`; the `.ico` is packed by
`cargo xtask ico`). To change the artwork, replace `toma.svg` and `toma-small.svg`, then run the script. Every derived file needs its `.attribution` sidecar
and a row in `ATTRIBUTION.md`.
