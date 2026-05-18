# Release Process for bellforge

**Repository**: https://github.com/blast9649/bellforge (private)  
**First Release**: v0.1.0

This document contains the exact steps to publish the first public release of bellforge.

---

## 1. Pre-Release Checklist

- [ ] All changes are committed and pushed to `master`.
- [ ] Run the following locally:

```bash
cd /home/andrew/Projects/bellforge
cargo test
cargo clippy -- -D warnings
cargo build --release
```

- [ ] Manually test the app one last time (`cargo run --release`), especially the new post-session review screen and Markdown export.

---

## 2. Tag and Trigger the Release

```bash
git tag -a v0.1.0 -m "v0.1.0 – First public release

- Real rodio chime when rest timers complete
- Secondary Skip Rest button
- Post-session review screen with editable actual reps
- Rich Obsidian-compatible Markdown export
- Fully automated release pipeline (binary + real Arch package)
- Complete Arch Linux packaging (bellforge + bellforge-bin)"

git push origin v0.1.0
```

Pushing the `v0.1.0` tag will automatically start the GitHub Actions release workflow.

---

## 3. Monitor the Release Workflow

Watch the workflow here:  
https://github.com/blast9649/bellforge/actions

Two jobs will run:

- **build** — Builds the binary, creates `.tar.gz` and `.zip`
- **build-arch-package** — Builds the real `bellforge-*.pkg.tar.zst` inside an official Arch Linux container

Wait for both to finish successfully. The workflow will automatically create a GitHub Release and attach the artifacts.

---

## 4. Update Packaging Checksums

After the release is published, download the generated tarball:

```bash
wget https://github.com/blast9649/bellforge/releases/download/v0.1.0/bellforge-0.1.0-x86_64.tar.gz
```

Compute the SHA256 sum:

```bash
sha256sum bellforge-0.1.0-x86_64.tar.gz
```

Update the `sha256sums` fields in:

- `PKGBUILD` (root)
- `bellforge-bin/PKGBUILD`

Then regenerate the `.SRCINFO` files:

```bash
makepkg --printsrcinfo > .SRCINFO
cd bellforge-bin && makepkg --printsrcinfo > .SRCINFO
```

---

## 5. Publish to the AUR (Long-term Maintenance)

You should maintain **two** packages long-term:

### A. `bellforge-bin` (Recommended for most users)

This is the easiest and fastest way for people to install bellforge.

**Long-term maintenance tips:**
- Keep the `pkgver` and checksums in sync with GitHub releases.
- The `bellforge-bin` package should always point to the latest stable GitHub release tarball.
- When a new release is tagged, the workflow will attach a new `bellforge-vX.Y.Z-x86_64.tar.gz`.
- After updating the PKGBUILD, push to the AUR. Most people will use this package.

### B. `bellforge` (Source package)

This package builds from source using the release tarball. Useful for users who want to verify the build or prefer source packages.

**Long-term maintenance tips:**
- Keep this package in sync with `bellforge-bin`.
- Some users prefer building from source for security or customization reasons.
- The build dependencies are heavier (needs full Rust toolchain + system libraries).

**Recommended workflow for future releases (v0.2.0+):**

1. Tag and push the new version (`git tag -a vX.Y.Z -m "..."` + `git push origin vX.Y.Z`).
2. Wait for the GitHub workflow to finish and attach the new artifacts.
3. Download the new tarball and update both `PKGBUILD` files with the new version + checksums.
4. Regenerate both `.SRCINFO` files.
5. Commit and push to both AUR packages.

**Helpful tools for AUR maintenance:**
- `aurpublish` — very popular for managing multiple AUR packages from one git repo.
- `paru -Syu --aur` or `yay -Syu --aur` for testing.
- `namcap` to check package quality before pushing.

**Tip**: It is common and accepted to maintain both `bellforge` and `bellforge-bin` from the same source repository. Many popular Rust GUI tools on Arch do this.

---

## 6. Finalize the GitHub Release

Go to:  
https://github.com/blast9649/bellforge/releases

Edit the auto-created release and add nice release notes (you can expand the tag message).

Recommended release notes for v0.1.0:

```markdown
## bellforge v0.1.0 – First Public Release

First official release of bellforge, a native Arch Linux kettlebell training timer with Obsidian Markdown logging.

### Highlights
- Real audible chime when rest periods complete
- Secondary "Skip Rest" button
- Post-session review screen with editable actual reps
- Rich Obsidian-compatible Markdown export
- Fully automated release pipeline (binary + real Arch package)

### Installation (Arch Linux)
```bash
yay -S bellforge-bin     # Recommended
# or
yay -S bellforge         # Build from source
```

Full details in the repository.
```

---

## 7. Post-Release

- [ ] Verify both AUR packages build and install correctly.
- [ ] Test installing directly from the GitHub release artifacts.
- [ ] (Optional) Announce the release on r/kettlebell, r/Obsidian, r/archlinux, etc.
- [ ] Monitor for early feedback.

---

## Future Releases

For v0.2.0 and beyond:

1. Bump version in `Cargo.toml`
2. `git tag -a vX.Y.Z -m "..."` + push tag
3. Workflow runs automatically
4. Update checksums in both PKGBUILDs
5. Push packaging updates to AUR

---

## 8. aarch64 Support (Future)

Currently the release workflow only builds for `x86_64`.

Adding aarch64 support is possible but more complex because of the following dependencies:
- `wgpu` / Vulkan or OpenGL on aarch64
- `rodio` audio
- `rfd` (file dialogs)

### Recommended Approach for aarch64 (when you're ready)

1. Add a new job in `.github/workflows/release.yml` using cross-compilation or native aarch64 runners (GitHub has `ubuntu-22.04-arm` now).
2. Or build inside an `archlinuxarm` container (more involved).
3. Update `bellforge-bin/PKGBUILD` with an `aarch64` architecture and corresponding binary.

For v0.1.0 it is perfectly acceptable to ship **x86_64 only**. Most Arch users on ARM (M1/M2 Macs via Asahi or Raspberry Pi) are still a small minority for desktop GUI tools.

When you want to add it, I recommend starting with a separate `build-aarch64` job that produces `bellforge-aarch64-unknown-linux-gnu` and the corresponding tarball.

---

**Prepared for**: blast9649  
**Date**: 2026-05-17 (v0.1.0)

**Prepared for**: blast9649  
**Date**: 2026-05-17 (v0.1.0)