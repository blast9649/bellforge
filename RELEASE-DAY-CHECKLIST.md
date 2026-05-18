# bellforge v0.1.0 Release Day Checklist

**Repo**: blast9649/bellforge (private)  
**When**: The day you are ready to ship the first public release.

This is the condensed, copy-paste-ready version of RELEASE.md. Follow it in order.

---

## 1. Final Local Validation

```bash
cd /home/andrew/Projects/bellforge

# Everything committed and pushed?
git status
git log --oneline -3

# Full test + lint + build
cargo test
cargo clippy -- -D warnings
cargo build --release

# Last manual smoke test
cargo run --release
# → Create a workout, run a short session, export Markdown, verify the file looks good.
```

---

## 2. Create and Push the Annotated Tag

This single action triggers the entire automated release.

```bash
git tag -a v0.1.0 -m "v0.1.0 – First public release

- Real rodio chime when rest timers complete
- Post-session review screen with editable actual reps
- Rich Obsidian-compatible Markdown export
- Fully automated release pipeline (binary tarball + real Arch .pkg.tar.zst)
- Complete Arch Linux packaging (bellforge + bellforge-bin) ready for AUR"

git push origin v0.1.0
```

---

## 3. Monitor the GitHub Workflow

Open and watch:

**https://github.com/blast9649/bellforge/actions**

Two jobs must both succeed:

- **build** — compiles on Ubuntu, creates tar.gz + zip + raw binary, creates the GitHub Release
- **build-arch-package** — runs inside official Arch container, produces real `bellforge-0.1.0-x86_64.pkg.tar.zst` + .SRCINFO files, attaches everything to the release

When both are green, the GitHub Release page will have all the artifacts.

---

## 4. Update Checksums and Packaging Files

Once the release artifacts exist, run the helper:

```bash
./scripts/release.sh 0.1.0
```

What it does:
- Downloads the official tarball
- Computes the real SHA256
- Patches `PKGBUILD` and `bellforge-bin/PKGBUILD` with correct `pkgver` + `sha256sums`
- Regenerates both `.SRCINFO` files

Review the changes:

```bash
git diff PKGBUILD bellforge-bin/PKGBUILD .SRCINFO bellforge-bin/.SRCINFO
```

---

## 5. Commit Packaging Updates to Main Repo

```bash
git add PKGBUILD bellforge-bin/PKGBUILD .SRCINFO bellforge-bin/.SRCINFO
git commit -m "chore(release): update checksums and .SRCINFO for v0.1.0"
git push
```

---

## 6. Push to AUR (Two Packages)

You maintain two packages on the AUR:

- `bellforge-bin` (recommended – downloads prebuilt binary)
- `bellforge`     (builds from source)

Assuming you have local AUR working directories (e.g. `~/aur/bellforge-bin` and `~/aur/bellforge`):

```bash
# bellforge-bin (most users will install this)
cp bellforge-bin/PKGBUILD bellforge-bin/.SRCINFO ~/aur/bellforge-bin/
cd ~/aur/bellforge-bin
git add . && git commit -m "Update to v0.1.0" && git push

# bellforge (source package)
cp PKGBUILD .SRCINFO ~/aur/bellforge/
cd ~/aur/bellforge
git add . && git commit -m "Update to v0.1.0" && git push
```

(If you use `aurpublish`, the workflow is even shorter.)

---

## 7. Polish the GitHub Release Notes (Optional but Nice)

Go to: https://github.com/blast9649/bellforge/releases/tag/v0.1.0

Edit the auto-generated release and add the highlights from the tag message or from the template in RELEASE.md.

---

## 8. Post-Release Quick Checks

- [ ] `yay -S bellforge-bin` (or `paru`) works cleanly
- [ ] The installed app launches and the chime plays
- [ ] Markdown export still looks correct
- [ ] Celebrate. First release is done.

---

**One-page version complete.**  
Keep this file. For future releases (v0.2.0+) just repeat the same flow after bumping the version in Cargo.toml.

**Prepared**: 2026-05-17
