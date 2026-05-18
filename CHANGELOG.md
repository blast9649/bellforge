# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [0.1.0] - 2026-05-17

First public release of **bellforge**.

### Added

- Real audible chime (using `rodio`) when rest timers naturally complete
- Secondary "Skip Rest" button (visible only while resting)
- Post-session review screen with editable actual reps (`DragValue`)
- Rich Obsidian-compatible Markdown export from the review screen
  - Full YAML frontmatter (`title`, `aliases`, `date`, `time`, `datetime`, `type`, `workout_type`, `focus`, `tags`, `status`, `progress`, `source`, `exercises[]`, `created`)
  - Clean 5-column exercises table
  - Under/extra notes for edited reps
  - `**Notes:**` placeholder section
- Support for both natural session completion and "End Session" (partial/aborted) flows in the review screen
- Fully automated GitHub release workflow
  - Builds optimized x86_64 binary
  - Produces release tarball + zip
  - Builds real Arch `.pkg.tar.zst` inside an official `archlinux` container
  - Automatically attaches all artifacts to the GitHub Release
- Two ready-to-use AUR packages:
  - `bellforge-bin` (recommended — downloads prebuilt binary)
  - `bellforge` (builds from source)
- Comprehensive release documentation (`RELEASE.md`)
- Developer guidelines (`CONTRIBUTING.md`)
- Helpful release automation script (`scripts/release.sh`)

### Changed

- Major improvements to the active session UI flow (Perform → Start Rest → live timer → contextual next action)
- Post-session review is now the primary way to review and correct actual reps after a workout

### Fixed

- Multiple timer visibility and state management issues in the live session runner
- Borrow safety improvements around session start/end transitions

---

[Unreleased]: https://github.com/blast9649/bellforge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/blast9649/bellforge/releases/tag/v0.1.0

---

**Note**: This is the initial public release. Future releases will follow semantic versioning and will be documented here.