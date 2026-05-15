# Maintainer: Your Name <you@example.com>
pkgname=bellforge
pkgver=0.1.0
pkgrel=1
pkgdesc="Native Arch Linux kettlebell training timer with Obsidian Markdown logging"
arch=('x86_64')
url="https://github.com/yourname/bellforge"
license=('MIT' 'Apache-2.0')
depends=('alsa-lib' 'wayland' 'libx11' 'mesa' 'vulkan-icd-loader')  # vulkan-icd-loader for wgpu; falls back gracefully
makedepends=('cargo' 'rust' 'git')
optdepends=(
  'obsidian: for viewing exported workout logs'
  'kitty: or any terminal for development'
)
source=("$pkgname-$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
  cd "$srcdir/$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$srcdir/$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo test --frozen --all-features
}

package() {
  cd "$srcdir/$pkgname-$pkgver"

  # Binary
  install -Dm755 "target/release/bellforge" "$pkgdir/usr/bin/bellforge"

  # Desktop integration
  install -Dm644 "bellforge.desktop" "$pkgdir/usr/share/applications/bellforge.desktop"
  install -Dm644 "assets/icon.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/bellforge.svg"

  # Documentation
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 "DESIGN_PLAN.md" "$pkgdir/usr/share/doc/$pkgname/DESIGN_PLAN.md"

  # License files (if present)
  # install -Dm644 LICENSE* "$pkgdir/usr/share/licenses/$pkgname/"
}

# vim:set ts=2 sw=2 et: