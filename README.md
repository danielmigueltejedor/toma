<div align="center">
  <a href="https://getnodalia.com">
    <img src="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-badge.svg" width="150" alt="Nodalia">
  </a>
  <br><br>
  <img src="https://raw.githubusercontent.com/danielmigueltejedor/toma/main/assets/app-icon/hicolor/512x512/apps/dev.danielmiguelt.Toma.png" width="192" alt="Toma">
  <h1>Toma</h1>
  <p><strong>The video editor of the Nodalia suite.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/status-early%20stage-orange" alt="Early stage">
    <img src="https://img.shields.io/badge/Rust-native-dea584?logo=rust&logoColor=black" alt="Native Rust">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea44f" alt="MIT OR Apache-2.0">
  </p>

  <p>
    <a href="#overview">Overview</a> ·
    <a href="#build-and-run">Build</a> ·
    <a href="#project-status">Status</a> ·
    <a href="#the-nodalia-suite">Suite</a> ·
    <a href="#license">License</a>
  </p>
</div>

---

## Overview

**Toma** is the video editor of [Nodalia](https://getnodalia.com): a native app written in Rust,
with no account and no cloud. It runs on Linux, macOS and Windows.

> [!IMPORTANT]
> **Toma is at an early stage.** There are no published builds yet, and the Nodalia interface is
> not wired in.

## Build and run

You need a recent stable Rust toolchain.

```bash
git clone https://github.com/getnodalia/toma.git
cd toma
cargo run --release -p toma
```

## Project status

| Area | State |
|---|---|
| Package and binary names | Done: the desktop binary is called `toma` |
| App icon | Done: portrait of Georges Méliès |
| Nodalia interface | Not started |
| Shared interface resources | Planned: the Nodalia apps will share interface components and styles |
| Installable packages | None published |

## App icon

Nodalia app icons are black-and-white portraits of historical figures over the app colour.
Toma's is Georges Méliès, a pioneer of film editing and effects. Provenance and licence are in [`assets/app-icon/`](assets/app-icon/).

## The Nodalia suite

Toma is one of the applications of [Nodalia](https://getnodalia.com), an independent software
project developed in public by [Daniel Miguel Tejedor](https://danielmigueltejedor.com).

## License

`MIT OR Apache-2.0`. See [`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) and
[`NOTICE`](NOTICE).
