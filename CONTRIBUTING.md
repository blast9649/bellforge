# Contributing to bellforge

Thank you for your interest in contributing to bellforge! This document provides guidelines and instructions for contributing.

---

## Code of Conduct

Be respectful and inclusive. We want bellforge to be a welcoming project for everyone interested in Arch Linux, Rust, and training tools.

---

## Getting Started

### Prerequisites

- Rust (stable toolchain)
- Cargo
- Basic familiarity with egui (or willingness to learn)
- For Arch packaging work: `base-devel`, `namcap`, and an AUR helper is helpful

### Local Development

```bash
git clone https://github.com/blast9649/bellforge.git
cd bellforge
cargo run --release
```

For faster iteration during development:

```bash
cargo run
```

### Project Structure

- `src/main.rs` — Main egui application, UI views (Dashboard, Editor, Active Session, Review)
- `src/session.rs` — Core `SessionRunner` engine (cue compilation, timer, `actual_reps`, etc.)
- `src/models.rs` — `WorkoutTemplate`, `FlowItem`, and related domain types
- `src/persistence.rs` — Loading/saving templates as TOML
- `bellforge-bin/` — Packaging for the prebuilt binary AUR package
- `.github/workflows/release.yml` — Automated release pipeline

---

## Development Guidelines

### Code Style

- Follow standard Rust formatting (`cargo fmt`).
- Run `cargo clippy -- -D warnings` before submitting a PR.
- Prefer small, focused functions.
- Use the existing patterns for egui immediate-mode code (especially around borrow safety and repaint requests).

### Adding New Features

- Keep new logic inside the existing `SessionRunner` when possible (see `src/session.rs`).
- Prefer pure functions for anything that generates output (e.g. Markdown, calculations).
- For UI state changes, be careful with borrows — use deferred request flags when needed (see how `end_session_requested` and `request_review_export` are handled).

### Testing

- Add unit tests in `src/session.rs` under `#[cfg(test)] mod tests` when adding new engine behavior.
- For new pure helpers (e.g. markdown builders), write property-style or table-driven tests.
- The project currently has no GUI tests — manual verification via `cargo run` is expected for UI changes.

### Commits

- Use clear, descriptive commit messages.
- Reference issues when applicable.
- Keep commits focused (one logical change per commit when possible).

---

## Packaging & Releases

See `RELEASE.md` for the full release process.

When working on packaging:

- Both `bellforge` (source) and `bellforge-bin` (binary) must be kept in sync.
- Use `namcap` to validate packages before pushing to the AUR.
- Prefer small, reviewable changes to `PKGBUILD` files.

---

## Reporting Issues

When opening an issue, please include:

- Your Arch Linux version / kernel
- How you installed bellforge (`bellforge-bin`, `bellforge`, or from source)
- Steps to reproduce
- Any relevant logs or screenshots

Feature requests are welcome! Especially around:

- Additional built-in workout templates
- Better aarch64 support
- Enhanced post-session analysis / graphs
- Integration with other training log formats

---

## Pull Requests

1. Fork the repository.
2. Create a feature branch from `master`.
3. Make your changes.
4. Run `cargo fmt`, `cargo clippy`, and `cargo test`.
5. Open a Pull Request with a clear description.

We appreciate contributions of all sizes — from small bug fixes and documentation improvements to new features.

---

Thank you for helping make bellforge better! 🏋️

_Maintained by blast9649_