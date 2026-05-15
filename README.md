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

## Building on Arch Linux

### Quick build (for development)

```bash
git clone https://github.com/yourname/bellforge.git
cd bellforge
cargo run --release
```

### Proper Arch packaging (recommended)

```bash
# Install build dependencies
sudo pacman -S --needed rust cargo base-devel

# Build the package
makepkg -si
```

This will produce a proper `bellforge` binary installed system-wide with a working `.desktop` entry.

---

## Running

After installation:

```bash
bellforge
```

Or launch from your application menu (search "bellforge").

---

## Roadmap (from DESIGN_PLAN.md)

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