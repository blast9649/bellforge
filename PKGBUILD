# Maintainer: blast9649 <https://github.com/blast9649>
pkgname=bellforge
pkgver=0.1.0
pkgrel=1
pkgdesc="Native Arch Linux kettlebell training timer with Obsidian Markdown logging"
arch=('x86_64')
url="https://github.com/blast9649/bellforge"
license=('MIT OR Apache-2.0')
depends=(
  'alsa-lib'           # rodio audio backend
  'dbus'               # rfd + notifications
  'gcc-libs'
  'glibc'
)
makedepends=(
  'cargo'
  'git'
  'rust'
)
optdepends=(
  'obsidian: for viewing exported workout logs'
  'xdg-desktop-portal: for native file dialogs on Wayland/KDE'
)
source=("$pkgname-$pkgver.tar.gz::https://github.com/blast9649/bellforge/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')   # Update these after creating the v0.1.0 GitHub release tarball

prepare() {
  cd "$srcdir/$pkgname-$pkgver"
  export CARGO_HOME="$srcdir/.cargo"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$srcdir/$pkgname-$pkgver"
  export CARGO_HOME="$srcdir/.cargo"
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  cd "$srcdir/$pkgname-$pkgver"
  export CARGO_HOME="$srcdir/.cargo"
  cargo test --frozen --all-features
}

package() {
  cd "$srcdir/$pkgname-$pkgver"

  # Binary (strip for size)
  install -Dm755 "target/release/bellforge" "$pkgdir/usr/bin/bellforge"

  # Desktop integration
  install -Dm644 "bellforge.desktop" "$pkgdir/usr/share/applications/bellforge.desktop"
  install -Dm644 "assets/icon.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/bellforge.svg"

  # Documentation
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"

  # License
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE" 2>/dev/null || true
}

# vim:set ts=2 sw=2 et: