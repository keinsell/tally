# Contributing to Tally

Thanks for hacking on Tally! This guide covers everything you need to build,
test, and package the app locally.

## Required toolchain

| Tool | Minimum version | What for |
| --- | --- | --- |
| Rust + Cargo (`rustup`) | stable | builds the app (`src/`, `build.rs`) |
| `blueprint-compiler` | ≥ 0.22 | compiles `src/ui/*.blp` → GTK UI XML (via `build.rs`) |
| `glib-compile-resources` (GLib 2) | ≥ 2.66 | bundles UI + CSS + icons into a GResource (via `build.rs`) |
| GTK 4 dev files | ≥ 4.0 | `gtk4` crate system dependency |
| libadwaita dev files | ≥ 1.1 | `libadwaita` crate system dependency |
| `desktop-file-validate` | any | desktop entry validation |
| `appstreamcli` | any | metainfo validation |
| Flatpak + `flatpak-builder` | any recent | Flatpak builds (`build-aux/flatpak.sh`) |
| `cargo-deb`, `cargo-generate-rpm` | any recent | native `.deb` / `.rpm` packages (release only) |

A C compiler (`gcc`/`clang`) plus `pkg-config` is also required — the GTK
crates link against system libraries.

## Installing dependencies

### Ubuntu / Debian (apt)

```sh
sudo apt-get update
sudo apt-get install -y \
  build-essential pkg-config \
  libgtk-4-dev libadwaita-1-dev \
  libglib2.0-dev-bin libgdk-pixbuf-2.0-dev \
  blueprint-compiler \
  desktop-file-utils appstream \
  flatpak flatpak-builder \
  librsvg2-common \
  curl git
```

Rust via `rustup` (do **not** use the distro `rustc` package — it is usually
outdated):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

Optional, for native packages:

```sh
cargo install cargo-deb cargo-generate-rpm --locked
```

### Fedora (dnf)

```sh
sudo dnf install -y \
  gcc pkg-config \
  gtk4-devel libadwaita-devel \
  glib2-devel gdk-pixbuf2-devel \
  blueprint-compiler \
  desktop-file-utils appstream \
  flatpak flatpak-builder \
  librsvg2 \
  curl git
```

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

Optional:

```sh
cargo install cargo-deb cargo-generate-rpm --locked
# .rpm builds additionally need:
sudo dnf install -y rpm-build
```

> `cargo-deb` produces Debian packages; on Fedora it is only useful for
> cross-checking. The native Fedora artifact is the `.rpm`.

### openSUSE Tumbleweed / Slowroll (zypper)

```sh
sudo zypper install -y \
  gcc pkgconf \
  gtk4-devel libadwaita-devel \
  glib2-devel-tools gdk-pixbuf-devel \
  blueprint-compiler \
  desktop-file-utils appstream \
  flatpak flatpak-builder \
  librsvg-devel \
  curl git
```

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

Optional:

```sh
cargo install cargo-deb cargo-generate-rpm --locked
sudo zypper install -y rpm-build
```

> **openSUSE quirk — SVG icon loader.** Tumbleweed's gdk-pixbuf can ship
> without an SVG loader, in which case `flatpak-builder` fails during
> `build-export` with `… is not a valid icon: Format not recognized`. Make
> sure an SVG loader is registered on the host (e.g. install the distro's
> `librsvg` pixbuf loader package) and retry.

## Quick start

```sh
git clone https://github.com/keinsell/tally
cd tally

cargo build --locked --release
cargo test
cargo run       # debug run (build.rs bundles UI + GResource itself)
```

## Common tasks

| Command | What it does |
| --- | --- |
| `cargo build --locked --release` | release build into `target/` |
| `cargo run` | debug build and run |
| `cargo test` | test suite |
| `./build-aux/stage-package-assets.sh` | stage desktop/metainfo/icons for native packagers |
| `./build-aux/flatpak.sh` | build Flatpak and export `tally-<arch>.flatpak` |
| `cargo fmt --check` | check formatting |
| `cargo clippy --all-targets -- --deny warnings` | lint |

Validate the desktop entry and metainfo with:

```sh
./build-aux/stage-package-assets.sh
desktop-file-validate target/package-data/applications/com.keinsell.Tally.desktop
appstreamcli validate --no-net target/package-data/metainfo/com.keinsell.Tally.metainfo.xml
```

Before opening a PR, please run:

```sh
cargo fmt --check
cargo clippy --all-targets -- --deny warnings
cargo test
```

This mirrors what CI runs (`.github/workflows/ci.yml`).

## Flatpak notes

- Manifest: `build-aux/com.keinsell.Tally.json` (GNOME 51 runtime, `simple`
  buildsystem driving `cargo build --release --offline` against the vendored
  sources in `build-aux/cargo-sources.json`). The `.cargo/config.toml` that
  points Cargo at `cargo/vendor` is generated inside the sandbox by the
  manifest itself.
- Metadata installed into the Flatpak comes from
  `data/com.keinsell.Tally.desktop.in` and
  `data/com.keinsell.Tally.metainfo.xml.in` — `flatpak-builder
  --appstream-compose` validates and embeds them, so keep them passing
  validation (see above).
- The desktop entry is intentionally **not** `DBusActivatable` — the app
  ships no `dbus-1/services/*.service` file, and adding the key without one
  breaks `flatpak-builder` export.
- `build-aux/flatpak.sh` is the single place where the build + `build-bundle`
  commands live — run it instead of invoking `flatpak-builder` by hand.
- After changing `Cargo.lock`, regenerate the vendored sources so the
  offline Flatpak build keeps working (the generator lives upstream in
  `flatpak-builder-tools` and is intentionally not vendored here):

```sh
curl -sSLO https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
python3 flatpak-cargo-generator.py Cargo.lock -o build-aux/cargo-sources.json
rm flatpak-cargo-generator.py
git add build-aux/cargo-sources.json
```

## Native packages (.deb / .rpm)

```sh
cargo build --release --locked
./build-aux/stage-package-assets.sh   # stages desktop/metainfo/icons for cargo packagers
cargo deb --no-build
cargo generate-rpm
```

Package metadata lives in `Cargo.toml` under `[package.metadata.deb]` and
`[package.metadata.generate-rpm]`.

## Cutting a release

1. Bump the version in `Cargo.toml` (single source of truth — CI enforces
   the tag matches it).
2. Add a `<release version="…" date="…">` entry to
   `data/com.keinsell.Tally.metainfo.xml.in`.
3. Commit, then `git tag vX.Y.Z && git push origin vX.Y.Z`. CI
   (`.github/workflows/release.yml`) publishes the `.deb`, `.rpm`, and
   `.flatpak` to the GitHub release.

## Project layout

- `src/` — Rust application (`main.rs`, `lib.rs`, `config.rs`, `ui/*.blp`)
- `data/` — desktop entry, AppStream metainfo, icons, CSS, GResource
- `build-aux/` — Flatpak manifest, vendored cargo sources, packaging scripts
- `build.rs` — compiles Blueprint UI + GResource bundle at build time
