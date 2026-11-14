# Nodalia migration: Toma

Category: secondary app.
Status: Base Nodalia preparada, as of 2026-11-13. Evidence: the pull request that adds this file.

## Current state

Pure-Rust video editor (egui), MIT OR Apache-2.0. Cargo git dependencies come from getnodalia/foundation and nodalia-design. The gate, FreeBSD,
Nix, packaging lint, AGENTS.md and git-sources checks call getnodalia/foundation's reusable
workflows pinned by commit. SonarCloud was removed; CodeQL and Scorecard remain.

## Desired state

No workflow, dependency or document depends on storytold; Toma has its own release workflow; the
interface follows Foundation's [NDL standard](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md).

## Inherited dependencies still pending

| Dependency | Where | Replacement | Blocked on |
|---|---|---|---|
| `storytold/craft-common-actions` | `.github/workflows/windows-arm64.yml` | Foundation reusable workflows (release, refresh-contributors, windows-arm64) | writing and testing them |
| `storytold/craft-fonts` | `crates/text/build.rs` | getnodalia/foundation `vendor/craft-fonts/main` | repointing |
| `storytold/craft-libs` | `packaging/web/README.md` | to decide | — |
| `storytold/filmcraft` | `.github/ISSUE_TEMPLATE/bug_report.yml`, `.github/ISSUE_TEMPLATE/config.yml`, `.github/workflows/upstream-sync.yml`, `packaging/linux/ai.storyteller.filmcraft.metainfo.xml.in`, `packaging/linux/flatpak-oci.sh`, `packaging/linux/package.sh`, `packaging/windows/sign.ps1` | none: intended upstream sync | — |
| `storytold/vectorcraft` | `packaging/macos/dmg/README.md` | to decide | — |

## Technical debt

- Internal names inherited from upstream (crate, binary and package names) remain where saved
  settings, files and the upstream sync depend on them.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, translations, reusable workflows);
getnodalia/nodalia-design (theme, icons).

## NDL work pending

Check the interface against Foundation's NDL standard: hamburger menu with About, Preferences and
Keyboard shortcuts; theme selector only in Preferences; NDL window controls on Linux only; global
menu exported over D-Bus on Linux.

## Risks and priorities

1. Releases still run the inherited release workflow.
2. Windows ARM64 packaging still runs the inherited workflow.

## Recommended order

1. A Foundation release workflow, tested with a dry run, then Toma's release on it.
2. A Foundation Windows ARM64 workflow.
3. NDL review of the interface.

## What the first agent should do

1. Read AGENTS.md and this file.
2. Pick the first item of "Recommended order".

## Releases

Toma releases through `release-auto.yml` like every Nodalia app (Foundation's `release-tag` and
`release-publish`, with `release-linux.yml`, `release-macos.yml` and `release-windows.yml`). The
packages are still named `filmcraft-X.Y.Z-…`: rename them (packaging scripts, the release workflows,
`app=` in `scripts/check-release-notes.sh`) before the first release. Until then the notes check
refuses any notes, since they would have to name `filmcraft` files.
