# Toma upstream

| Field | Value |
|---|---|
| Upstream | `storytold/filmcraft` |
| Last observed | `2d536e6af1c886887e8299edfb34b5d565e30a69` |
| Last evaluated | `2d536e6af1c886887e8299edfb34b5d565e30a69` |
| Last integrated | `2d536e6af1c886887e8299edfb34b5d565e30a69` merged on 2026-11-10 |
| Skipped | none in the baseline import |
| Adapted | desktop binary name `toma` |
| Conflicting | `README.md` (kept ours), `AGENTS.md` (upstream plus our quality contract) |

Every name is Toma's, internal ones included: crates are `toma-*`, the binaries `toma`,
`toma-cli` and `toma-web`, and the app id `dev.danielmiguelt.Toma`. File signatures stay as
FilmCraft wrote them (`"format": "filmcraft.project"` and the preset and template formats, Toma's
metadata key in OTIO and FCPXML), so files saved before the rename keep opening.

## How the sync works

Upstream's code arrives renamed: `scripts/rebrand.py` applies `.github/rebrand.toml` (FilmCraft
becomes Toma, internal names included) to upstream's tree, and the weekly sync
(`.github/workflows/upstream-sync.yml`) records the result on the `upstream-toma` branch. Each
commit there has the previous mirror commit and the upstream commit it renames as parents, and
names that commit in an `Upstream-Commit:` trailer. The sync merges the newest mirror commit
into a branch and opens a pull request; files listed in `.github/upstream-keep` keep this
repository's version.

The mirror starts at the renamed `2d536e6a`, merged into `main` (with this repository's tree)
by the pull request that renamed the app. `python3 scripts/rebrand.py check` fails when an old
name comes back; a line that must keep one says `rebrand: keep`.
