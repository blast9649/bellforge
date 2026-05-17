# bellforge

**Native Arch Linux kettlebell training timer with Obsidian Markdown logging.**

Run precise, guided kettlebell sessions on your desktop with beautiful big timers, flexible workout templates, and automatic export to your Obsidian vault with rich frontmatter.

Built the Arch way: lightweight, keyboard-first, Wayland native, trivial to package.

---

## Current Status

This is **PR 1 Foundation** (see `DESIGN_PLAN.md`).

- Basic egui/eframe window boots correctly
- Dark "gym mode" theme
- Placeholder dashboard
- All major architectural decisions locked

Full functionality (template editor, live session runner, Obsidian export, packaging) is implemented across the PRs described in the design plan.

---

## Installation (Arch Linux)

### Recommended — Binary package (fastest)

```bash
# Using an AUR helper
yay -S bellforge-bin
# or
paru -S bellforge-bin
```

This downloads the prebuilt binary from the GitHub release.

### From source (AUR)

```bash
yay -S bellforge
```

### Manual build

```bash
git clone https://github.com/yourname/bellforge.git
cd bellforge
makepkg -si
```

### Development

```bash
git clone https://github.com/yourname/bellforge.git
cd bellforge
cargo run --release
```

---

## Running

```bash
bellforge
```

Or search for "bellforge" in your application menu.

---

## First Release (v0.1.0)

This is the initial public release.

**GitHub Release**: https://github.com/yourname/bellforge/releases/tag/v0.1.0

The release workflow automatically builds and attaches:
- `bellforge` (raw binary)
- `bellforge-vX.Y.Z-x86_64.tar.gz` (binary + .desktop + icon — used by `bellforge-bin`)
- `bellforge-vX.Y.Z-x86_64.zip`
- `bellforge-vX.Y.Z-x86_64.pkg.tar.zst` (real Arch package built in an official container)

**AUR Packages**:
- `bellforge-bin` (recommended — downloads prebuilt binary)
- `bellforge` (builds from source)

---

## Creating a Release (for maintainers)

1. Update version in `Cargo.toml` and commit.
2. Tag: `git tag -a v0.1.0 -m "v0.1.0"`
3. Push tag: `git push origin v0.1.0`
4. Create GitHub Release from the tag.
5. Attach a stripped binary (built with `cargo build --release`).
6. Update `sha256sums` in both `PKGBUILD` files.
7. Push updated `PKGBUILD` + `.SRCINFO` to AUR for `bellforge` and `bellforge-bin`.

---

## Roadmap

See `DESIGN_PLAN.md` for the original development plan.

- **PR 1** — Foundation + packaging skeleton (current)
- **PR 2** — Template Editor (flexible FlowItem + top-level Repeats)
- **PR 3** — Session Runner core + RestTimer
- **PR 4** — Beautiful during-workout UX (huge text, circular timer, keyboard)
- **PR 5** — Obsidian Markdown export with excellent frontmatter
- **PR 6** — History browser + basic stats
- **PR 7** — System tray, notifications, sleep inhibit, bundled chime
- **PR 8** — Final icon, polished PKGBUILD, AUR readiness, documentation

---

## Philosophy

- **Local first** — Your training data lives in your Obsidian vault.
- **Correct timers** — Wall-clock `Instant`-based, survives suspend.
- **Deliberate UX** — Requires confirmation after rests (you can change this later).
- **Arch native** — Follows XDG, works great on Wayland + fractional scaling, tiny binary.

---

## License

MIT or Apache-2.0 (your choice).

---

*Built with the arch-linux-gui-app-builder persona and Grok Build.*