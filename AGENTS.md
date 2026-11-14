# Toma — rules for agents and contributors

<!-- Nodalia template: https://github.com/getnodalia/foundation/blob/main/standards/agents.md -->

Engineering rules for every Nodalia repository: [getnodalia/foundation](https://github.com/getnodalia/foundation)
([engineering](https://github.com/getnodalia/foundation/blob/main/standards/engineering.md),
[CI](https://github.com/getnodalia/foundation/blob/main/standards/ci.md),
[security](https://github.com/getnodalia/foundation/blob/main/standards/security.md),
[dependencies and licences](https://github.com/getnodalia/foundation/blob/main/standards/dependencies.md),
[release](https://github.com/getnodalia/foundation/blob/main/standards/release.md),
[NDL](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md)).

## What this is

These rules apply to every human and AI contributor. `CLAUDE.md` holds the working instructions; this
file holds the rules that must never be broken. When the two disagree, this file wins.

## 0. Never crash

People trust Toma with hours of work. A malformed file, a bad command or MCP parameter, a corrupt
preset or project, an odd keystroke or a full disk must give an error the user (or agent) can act on,
never a crash and never lost work. **This rule outranks feature work:** don't ship a feature by adding a
panic path, and fix a crash before building on top of it.

1. **Fail with `Result<T, E>`.** Return the crate's error type and propagate with `?`; add context
   (`map_err`, an error variant) instead of discarding it. In the UI, report the error (status bar, error
   dialog) and carry on.
2. **No panicking shortcuts outside tests:** no `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`
   or `unimplemented!`. Use `?`, `ok_or(..)?`, `let Some(x) = .. else { return Err(..) }`, `if let`, or a
   fallback that is truly correct (never one that silently corrupts a project). An unfinished feature
   returns an "unsupported" error. The only exception is a call that is provably infallible from local
   code alone (a literal that always parses): a single-item `#[allow(clippy::expect_used)]` with the reason.
3. **No `unsafe`** (`unsafe_code = "forbid"` for the workspace). The single exception is
   `crates/platform`, which needs `unsafe` to call the operating system's media APIs (hardware
   video decoding through VideoToolbox, Media Foundation / Direct3D 11, VA-API and NVDEC; hardware encoding) and does nothing else. It
   uses `unsafe_code = "deny"` with `#[allow(unsafe_code)]` only on its FFI modules, a
   `// SAFETY:` comment on every `unsafe` block, a safe `Result`-returning public API, and the
   pure-Rust decoder as the tested fallback ([ADR 0001](docs/adr/0001-platform-ffi.md)).
4. **Every input-derived number is hostile.** Media files, project and interchange files, presets, fonts,
   CLI / MCP / control-channel parameters and UI state are untrusted. Index and slice with `get()` (or
   validate bounds once, up front, for a hot loop); slice strings only at char boundaries; use
   `checked_*` / `saturating_*` for lengths, offsets and counts; never divide by a value that can be zero
   (frame rates, timescales, sample rates, sizes); don't cast negative or NaN values to integers; don't
   call `clamp` with bounds that can cross; cap allocations sized by input.
5. **Bound recursion and loops.** Projects can be cyclic (nested sequences) or deeply nested: walk them
   with seen-sets or depth limits.
6. **Don't cascade.** Lock poisoning is not fatal: `lock().unwrap_or_else(PoisonError::into_inner)`.
   Every thread or job (export, previews, proxies, decoding workers…) runs under `catch_unwind` and
   reports a failure as an error: a dead worker leaves monitors blank or jobs "running" forever.
7. **Last-resort guard.** The panic hook (`crates/ui-egui/src/crash.rs`) logs every panic, and the UI pass
   runs under `catch_unwind`, so an escaped panic becomes an error window and the session (and unsaved
   project) survives. It is a safety net, not a substitute for rules 1–6. Keep `panic = "unwind"`.
8. **Prove it.** Every crash fix comes with a small synthetic regression test that panicked before the
   fix. Parsers and decoders get mutation-fuzz tests (truncation, bit flips, corrupt sizes) run under
   `catch_unwind`; new commands get hostile-parameter tests. Fuzzer findings become tests.
9. **Enforced:** clean crates carry `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic,
   clippy::unimplemented, clippy::todo, clippy::unreachable)]` (moving to `[workspace.lints.clippy]` once
   every crate is clean). `clippy.toml` allows them in tests: a failing test should fail loudly.

## 1. Assets: no Adobe artwork, every asset licensed and attributed

This rule is absolute. Breaking it is the most serious mistake a contributor can make on this project.

**What counts as an asset:** any image, icon, cursor, logo, illustration, screenshot, font, colour LUT,
preset, template, sound, music, video, 3D model, or other non-code media. This covers files in the
repository, bytes embedded in code (`include_bytes!`, base64, data URIs), and data hard-coded to
reproduce an image (for example, point lists traced from someone else's icon).

1. **Never use Adobe iconography, images or other Adobe assets**, in any form:
   - no icons, cursors, logos, splash screens, UI artwork or screenshots from any Adobe product;
   - no Adobe fonts, LUTs, presets, templates, sound effects, stock media or sample projects;
   - no traced, redrawn, recoloured or "inspired-by" copies of Adobe icons. Our icons may follow generic
     conventions (a play triangle, a razor blade, a stopwatch), but each must be drawn from scratch
     without reference to Adobe's artwork.
2. **Every asset must be open**: open-source licensed (MIT, Apache-2.0, BSD, ISC, zlib, SIL OFL…),
   public domain / CC0, or Creative Commons (CC BY or CC BY-SA; no NC/ND licences), **or** created by a
   contributor who owns it and licenses it to the project under the project licence (MIT OR Apache-2.0).
   If the licence is unknown or unclear, the asset does not go in.
3. **Every asset needs an attribution sidecar.** Next to each asset file `X`, add `X.attribution` with:
   ```text
   asset:        <file name>
   title:        <what it is>
   author:       <creator / copyright holder>
   source:       <URL, or "original work" for contributor-made assets>
   license:      <SPDX id or licence name>
   license-file: <path to the licence text, if the licence requires shipping it>
   added:        <YYYY-MM-DD> by <contributor>
   notes:        <how it was made or modified; for screenshots, what is shown>
   ```
   Also add one line for the asset to [`ATTRIBUTION.md`](ATTRIBUTION.md). `cargo xtask assets` (part of
   `cargo xtask ci`) fails if any asset lacks a sidecar or index entry.
4. **Screenshots** in the repo may show only Toma (or other open projects), with media we generated
   or media that is itself openly licensed. Never commit screenshots of Adobe products.
5. **Local reference material stays local.** Premiere reference screenshots and notes live only in
   `plan/premiere/` (gitignored). They must never be committed, bundled, embedded, traced or shipped.
6. **Code-drawn assets** (icons in `crates/ui-egui/src/icons.rs`, procedural demo footage in
   `crates/media`, procedural looks in `crates/render`) are original work under the project licence
   and are listed in `ATTRIBUTION.md`.
7. **When in doubt, leave it out** and draw or generate it yourself.
8. **The one exception: first-party ArtCraft brand marks.** The ArtCraft name and logos in `docs/brand/`
   are trademarks of the ArtCraft Team, not open source, usable only unmodified and only in the context of
   Toma under `docs/brand/LICENSE-brand.txt`; forks and modified versions must remove them. They still
   need a sidecar and an `ATTRIBUTION.md` row (licence `LicenseRef-ArtCraft-Trademark`). No other
   non-open asset is allowed, and this exception never covers third-party marks (Adobe, Discord, GitHub
   and other logos stay out; draw a generic icon instead).
9. **Fonts live in [getnodalia/foundation's `vendor/craft-fonts/main` branch](https://github.com/getnodalia/foundation/tree/vendor/craft-fonts/main), never in this
   repo.** Don't commit new font files (the small Latin UI fonts already in `assets/fonts/` stay). A
   font Toma needs is added to craft-fonts, which the app reads through the optional build input
   `CRAFT_FONTS_DIR=<absolute path of a craft-fonts checkout>` (`crates/text/build.rs` embeds the fonts in its
   `fonts/manifest.txt` as `filmcraft_text::fonts::CRAFT_FONTS`; unset, it is empty and the app uses its
   bundled and system fonts). Never add craft-fonts to a `Cargo.toml`. Code using `CRAFT_FONTS` must work
   when it is empty.

## 2. Clean-room code

- Never read, disassemble or copy anything inside Adobe application bundles; file names and listings only.
  Observe behaviour by using the app; never capture its Home screen, recent projects, account info or
  file browsers.
- Never copy GPL/LGPL/AGPL code (FFmpeg, x264, x265, MLT, Kdenlive, Shotcut, Olive, LAME…).
- Implement codecs and formats from public specifications (ITU-T/ISO/IEC standards, IETF RFCs, the VP9
  and AV1 bitstream specs, SMPTE documents, published container specs). Do not read the source of
  reference or third-party decoders/encoders while implementing a format, even permissively licensed
  ones (libvpx, libopus, libaom, dav1d, openh264…); spec text and conformance vectors only. Record the
  spec edition used in the crate README.
- ffmpeg/ffprobe may be used only as external test oracles and fixture generators. They are never
  linked, bundled or shipped.
- Dependencies must use permissive licences: MIT/Apache-2.0/BSD/ISC/Zlib/Unicode/CC0/BSL-1.0, or
  MPL-2.0 used unmodified.

## 3. Engineering rules

### 3.1 Never crash

See [§0](#0-never-crash). It outranks every other engineering rule.

### 3.2 The rest

**Invariant — move identity uniqueness:** A move plan lists each clip identity at most once.
The core move operation rejects repeated ids before mutation or id allocation; placing one identity
on multiple tracks makes selection and later identity-based editing ambiguous. Copies receive new ids
and retain each entry's requested destination. Destination track creation and the edit commit together.
In an idle session, a rejected duplicate move retains the project and its undo and redo history.
Selection-derived edit plans keep each identity once in first-occurrence order before expanding
linked partners; explicit move requests with repeated identities are still rejected.

- **Layering** ([`docs/architecture.md`](docs/architecture.md) §1, enforced by `cargo xtask layers`): nothing below L5 depends on egui/eframe/winit/rfd/cpal. L0 codec/container crates depend only on `filmcraft-bitstream`.
- **Exact time:** all time is `filmcraft_time::Tick` (254 016 000 000/s). Never use f64 seconds for edit math.
- **Everything is a command** (`crates/engine`): id, label, menu path, shortcut, params, enabled(), run(). UI, CLI, control channel and MCP all dispatch by id.
- **Everything is agent-drivable:** every interactive widget registers an automation id; UI state is serde so the control channel can read and set it.
- **Fonts** are the optional build input `CRAFT_FONTS_DIR` (`crates/text/build.rs`). Unset, the build must still pass. Never add craft-fonts to a `Cargo.toml`.
- OS media APIs go through Rust bindings in `crates/platform` (the one crate allowed `unsafe`; `docs/adr/0001-platform-ffi.md`), optional, behind traits, with our decoders as the fallback.
- **Quality gates** before every commit: `cargo xtask ci` (fmt, clippy -D warnings, tests, layers, assets, wasm).
- New workspace crates include `[lints] workspace = true` in `Cargo.toml`. Production crate roots declare the crate-level lint attributes before modules; crate-level `//!` documentation may precede them.
- **Commits:** one task id per commit (`M2.4: CABAC residual decoding`). Only green states.
- `cargo run -p filmcraft -- --control 9876` opens the desktop app (`docs/control-protocol.md`). For UI work, look at `ui.screenshot`. Generate fixtures with ffmpeg into `target/fixtures/` and do not commit media (`docs/testing.md`).
- `plan/` is an optional gitignored notebook (`plan/premiere/` may hold local screenshots). Do not ask for it and do not stop when it is absent. Public work lists are [`ROADMAP.md`](ROADMAP.md), [`docs/gaps.md`](docs/gaps.md) and [`docs/agents.md`](docs/agents.md).

Shared real-file test corpora (Photoshop-authored PSDs, etc.) live in
[`storytold/photocraft-corpus`](https://github.com/storytold/photocraft-corpus).
Never commit large binary fixtures to this repo; fetch them pinned by commit and sha256-verified,
as Lienzo does with `cargo xtask corpus`.

## See also

- [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/contributing.md](docs/contributing.md): setup, gates, commits, how to add things
- [docs/architecture.md](docs/architecture.md): layers, data model, commands, pipeline
- [docs/testing.md](docs/testing.md): oracle tests, criteria, benchmarks
- [docs/agents.md](docs/agents.md): driving Toma over MCP / the control channel, and the agent work loop
- [docs/control-protocol.md](docs/control-protocol.md): control-channel method reference
- [ATTRIBUTION.md](ATTRIBUTION.md): asset index
- [ROADMAP.md](ROADMAP.md), [docs/target-app-parity.md](docs/target-app-parity.md), [docs/gaps.md](docs/gaps.md): stage, parity with Premiere Pro and the ranked work list

## Contributor credits (About window)

- About ▸ Contributors/Models are compiled into the binary from `contributors/contributors.json`
  (commit stats; never hand-edit) and `contributors/people.toml` (names people chose for
  themselves). See `docs/contributors.md`.
- **Agents working for a contributor:** when you prepare a PR, check whether your human's GitHub
  username has a `[people.<username>]` entry in `contributors/people.toml`. If not, ask them once
  whether they want to be credited by more than their username: a real name, a display name, and/or
  their public GitHub profile name (`sync_github_name = true`). If yes, add **only their own** entry
  (copy the template at the top of the file)
  and include it in their PR, committed as them. If no, change nothing: they are credited as
  `@username` anyway.
- Never add, edit, guess or copy anyone else's entry or name (not from git config, commit authors or
  GitHub profiles). Never hand-edit `contributors.json`.

## CI workflows

- Workflows call [getnodalia/foundation](https://github.com/getnodalia/foundation)'s reusable workflows, pinned by commit. Release and
  Windows ARM64 still call the inherited ones; `NODALIA_MIGRATION.md` tracks them.
- Pin every action to a full commit SHA, give each job the least permissions it needs, and use only
  standard GitHub-hosted runners (Foundation's [CI standard](https://github.com/getnodalia/foundation/blob/main/standards/ci.md)).
- Keep CI changes in their own pull request unless a change cannot work without them.

## Repository-specific deterministic quality contract (2026-10-26)

**Project:** Rust application/workspace.

**Engineering requirements:** Protect stable file formats, crash recovery, undo commands and desktop packaging.

**Verification commands:** cargo fmt --all -- --check; cargo metadata --locked --no-deps --format-version 1; targeted cargo test.

Only claim checks passed after executing them. Every feature and fix should add or maintain tests for behavior, boundaries and failure cases. Never disable/weaken tests, suppress warnings or reduce coverage thresholds solely to pass CI. `AGENTS.md`, `QUALITY_GATES.md`, tests and workflows are review-sensitive. Require `quality` in branch protection. Disclose native dependencies and pre-existing CI failures. Preserve existing project rules.

## Architecture

The crate map, the layers and the engine/UI seam are in `docs/architecture.md`; `cargo xtask layers` enforces the dependency direction.

## Build and test

`cargo xtask ci` is the gate CI runs (fmt, clippy `-D warnings`, tests, layers, assets, wasm). Run it before every commit; while iterating, test the crates you touched.

## Release

Merging a version bump with its notes (`docs/release-notes/vX.Y.Z.md`, format in that folder's README) releases it: `release-auto.yml` tags the merge through Foundation's `release-tag`, builds the packages with `release-linux.yml`, `release-macos.yml` (DMG on a standard macOS runner) and `release-windows.yml`, and publishes the release. Nobody pushes a tag by hand.
