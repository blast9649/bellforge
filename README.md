# 🛎️ bellforge

**Native Arch Linux kettlebell training timer with Obsidian Markdown logging.**

Run precise, guided kettlebell sessions on your desktop with beautiful big timers, flexible workout templates, and automatic export to your Obsidian vault with rich frontmatter.

Built the Arch way: lightweight, keyboard-first, Wayland native, trivial to package.

[![Latest Release](https://img.shields.io/github/v/release/blast9649/bellforge?style=flat-square)](https://github.com/blast9649/bellforge/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)

---

## ✨ Features

- **Flexible workout template editor** — exercises, sets, repeats, per-block rests
- **Live guided sessions** — large visible countdown timer
- **Real audio chime** — rodio-powered chime at the end of each rest
- **Post-session review** — editable actual reps + rich Obsidian Markdown export with full YAML frontmatter
- **Arch-native packaging** — automated builds for binaries, tarballs, zips, and real `.pkg.tar.zst` packages

---

## 📦 Installation

### Prebuilt Releases (Easiest)

Download the latest release from **[GitHub Releases](https://github.com/blast9649/bellforge/releases/latest)**:

- `bellforge` — raw optimized binary
- `bellforge-*.tar.gz` / `.zip` — binary + `.desktop` + icon
- `bellforge-*.pkg.tar.zst` — ready-to-install Arch package (built in official container)

Extract or install the package and run `bellforge`.

### From Source

```bash
git clone https://github.com/blast9649/bellforge.git
cd bellforge

# Build the binary
cargo build --release

# Or build the full Arch package (recommended for system integration)
makepkg -si
```

The binary ends up at `target/release/bellforge` (or installed system-wide after `makepkg`).

### Development

```bash
git clone https://github.com/blast9649/bellforge.git
cd bellforge
cargo run --release
```

---

## ▶️ Running

```bash
bellforge
```

Or launch it from your application menu / desktop environment.

---

## 📋 Releases & Changelog

- **Latest release**: [v0.1.1](https://github.com/blast9649/bellforge/releases/tag/v0.1.1)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **All releases**: [GitHub Releases page](https://github.com/blast9649/bellforge/releases)

v0.1.1 focused on packaging reliability and build fixes (including Cargo.lock tracking and container build improvements).

---

## 🛠️ For Maintainers

- [RELEASE-DAY-CHECKLIST.md](RELEASE-DAY-CHECKLIST.md) — one-page release process
- [RELEASE.md](RELEASE.md) — detailed release automation notes

---

## 🔮 Future Plans

See [DESIGN_PLAN.md](DESIGN_PLAN.md) for the full roadmap. Some ideas:

- Better history / past workout browser
- Notifications + sleep inhibit during sessions
- aarch64 builds
- More built-in presets and template sharing

---

## 💭 Philosophy

- **Local first** — Your training data lives in your Obsidian vault.
- **Correct timers** — Wall-clock `Instant`-based, survives suspend.
- **Pure timer UX** — Big visible countdown, automatic chime, contextual next-step buttons. No friction during training.
- **Arch native** — Follows XDG, works great on Wayland + fractional scaling, tiny binary.

---

## 📄 License

Licensed under the [MIT License](https://opensource.org/licenses/MIT).

---
