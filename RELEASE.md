# Release Checklist for bellforge

This document describes the process for creating a new release of bellforge.

---

## 1. Preparation

- [ ] All desired changes are merged into the `main` branch.
- [ ] Update the version in `Cargo.toml` if it hasn't been bumped yet.
- [ ] Run `cargo test` and `cargo clippy` locally and ensure everything is green.
- [ ] Update `CHANGELOG.md` (if it exists) or add release notes in the GitHub release.
- [ ] Make sure `README.md` and `DESIGN_PLAN.md` reflect the current state if needed.

---

## 2. Create and Push the Tag

```bash
# Example for v0.1.0
git tag -a v0.1.0 -m "v0.1.0 - First public release"
git push origin v0.1.0
```

Pushing a tag starting with `v` will automatically trigger the release workflow.

---

## 3. GitHub Release Workflow

The workflow (`.github/workflows/release.yml`) will automatically:

- Build a release binary on `ubuntu-22.04`
- Strip the binary
- Create the following artifacts:
  - `bellforge` (raw executable)
  - `bellforge-vX.Y.Z-x86_64.tar.gz` (binary + desktop file + icon)
  - `bellforge-vX.Y.Z-x86_64.zip` (same as above)
- Create a GitHub Release and attach the artifacts
- (Optional future step) Build the real `.pkg.tar.zst` using an Arch container

After the workflow finishes (usually 4–8 minutes), go to the Releases page and verify the artifacts are attached.

---

## 4. Update Packaging Checksums (Important)

After the release is published, download the tarball and compute its SHA256:

```bash
sha256sum bellforge-0.1.0-x86_64.tar.gz
```

Then update the `sha256sums` fields in:

- `PKGBUILD` (root, for the source package)
- `bellforge-bin/PKGBUILD`

Also update `.SRCINFO` files if you maintain them manually.

---

## 5. Publish to the AUR (Recommended)

You should maintain two AUR packages:

### `bellforge-bin` (recommended for most users)

- Uses the prebuilt binary from the GitHub release.
- Fast installation.
- Update the `PKGBUILD` and `.SRCINFO` with the new version and checksums.
- Push to the AUR.

### `bellforge` (source package)

- Builds from the release tarball.
- Good for users who want to verify the build.
- Update `PKGBUILD` + `.SRCINFO`.

**Tip**: Use `git` + `ssh` to push to the AUR, or tools like `aurpublish`, `paru`, or `yay` with AUR support.

---

## 6. Post-Release Tasks

- [ ] Announce the release (if desired) on relevant forums, Reddit, etc.
- [ ] Update the AUR packages as soon as possible.
- [ ] If this is a major release, consider updating the `version` in `Cargo.toml` and preparing the next development version (e.g., `0.2.0-dev` or just bump to `0.2.0` later).

---

## Future Improvements (Optional)

- [ ] Add an Arch Linux container job that actually builds and attaches the real `.pkg.tar.zst`.
- [ ] Add automatic `.SRCINFO` generation and commit in the workflow.
- [ ] Add binary signing (e.g., with `cosign` or GPG).
- [ ] Support aarch64 builds.
- [ ] Add a changelog generation step using `git-cliff` or similar.

---

## Quick Commands Reference

```bash
# Create and push a new release tag
git tag -a v0.2.0 -m "v0.2.0"
git push origin v0.2.0

# After release, update checksums and push to AUR
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to v0.2.0"
git push
```

---

**Maintained by**: The bellforge maintainers  
**Last updated**: 2026-05-17 (for v0.1.0)