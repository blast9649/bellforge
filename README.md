# bellforge

**Native Arch Linux kettlebell training timer with Obsidian Markdown logging.**

Run precise, guided kettlebell sessions on your desktop with beautiful big timers, flexible workout templates, and automatic export to your Obsidian vault with rich frontmatter.

Built the Arch way: lightweight, keyboard-first, Wayland native, trivial to package.

---

## Status

**v0.1.0** is the first public release. It includes the complete feature set:

- Flexible workout template editor (exercises, sets, repeats, per-block rests)
- Live guided sessions with large visible countdown timer
- Real audio chime (rodio) at the end of each rest
- Post-session review with editable actual reps + rich Obsidian Markdown export
- Fully automated Arch Linux packaging + release pipeline

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
git clone https://github.com/blast9649/bellforge.git
cd bellforge
makepkg -si
```

### Development

```bash
git clone https://github.com/blast9649/bellforge.git
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

**GitHub Release**: https://github.com/blast9649/bellforge/releases/tag/v0.1.0

The release workflow automatically builds and attaches:
- `bellforge` (raw binary)
- `bellforge-0.1.0-x86_64.tar.gz` (binary + .desktop + icon — used by `bellforge-bin`)
- `bellforge-0.1.0-x86_64.zip`
- `bellforge-0.1.0-x86_64.pkg.tar.zst` (real Arch package built in an official container)

**AUR Packages**:
- `bellforge-bin` (recommended — downloads prebuilt binary)
- `bellforge` (builds from source)

---

## Creating a Release (for maintainers)

See `RELEASE-DAY-CHECKLIST.md` for the exact one-page process used for v0.1.0 (including the automated GitHub workflow that builds both the tarball and real Arch package).

---

## Future Plans

See `DESIGN_PLAN.md` for the original roadmap. Possible future enhancements:

- Better history / past workout browser
- Notifications and sleep inhibit during sessions
- aarch64 builds
- More built-in presets and template sharing

---

## Philosophy

- **Local first** — Your training data lives in your Obsidian vault.
- **Correct timers** — Wall-clock `Instant`-based, survives suspend.
- **Pure timer UX** — Big visible countdown, automatic chime, contextual next-step buttons. No friction during training.
- **Arch native** — Follows XDG, works great on Wayland + fractional scaling, tiny binary.

---

## License

MIT or Apache-2.0 (your choice).

---

