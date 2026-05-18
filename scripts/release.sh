#!/usr/bin/env bash
#
# bellforge release helper
# Usage:
#   ./scripts/release.sh 0.1.0
#
# This script:
#   1. Downloads the release tarball from GitHub
#   2. Computes the SHA256 checksum
#   3. Updates both PKGBUILD files (root + bellforge-bin/)
#   4. Regenerates the two .SRCINFO files
#
# After running this, you only need to git add + commit + push to the AUR.

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <version>   (e.g. 0.1.0)"
    exit 1
fi

VERSION="$1"
REPO="blast9649/bellforge"
TARBALL="bellforge-${VERSION}-x86_64.tar.gz"
URL="https://github.com/${REPO}/releases/download/v${VERSION}/${TARBALL}"

echo "==> Downloading ${TARBALL}..."
curl -fL -o "/tmp/${TARBALL}" "${URL}"

echo "==> Computing SHA256..."
CHECKSUM=$(sha256sum "/tmp/${TARBALL}" | awk '{print $1}')

echo "==> Checksum: ${CHECKSUM}"

echo "==> Updating root PKGBUILD..."
sed -i "s/^pkgver=.*/pkgver=${VERSION}/" PKGBUILD
sed -i "s|^source=.*|source=(\"$pkgname-\$pkgver.tar.gz::https://github.com/${REPO}/archive/refs/tags/v\$pkgver.tar.gz\")|" PKGBUILD
sed -i "s/^sha256sums=.*/sha256sums=('${CHECKSUM}')/" PKGBUILD

echo "==> Updating bellforge-bin/PKGBUILD..."
sed -i "s/^pkgver=.*/pkgver=${VERSION}/" bellforge-bin/PKGBUILD
sed -i "s|bellforge-.*-x86_64::https://github.com/${REPO}/releases/download/v.*|bellforge-\$pkgver-x86_64::https://github.com/${REPO}/releases/download/v\$pkgver/bellforge-x86_64-unknown-linux-gnu|" bellforge-bin/PKGBUILD
sed -i "s|bellforge-.*\.desktop::https://raw.githubusercontent.com/${REPO}/v.*|bellforge-\$pkgver.desktop::https://raw.githubusercontent.com/${REPO}/v\$pkgver/bellforge.desktop|" bellforge-bin/PKGBUILD
sed -i "s|bellforge-.*\.svg::https://raw.githubusercontent.com/${REPO}/v.*|bellforge-\$pkgver.svg::https://raw.githubusercontent.com/${REPO}/v\$pkgver/assets/icon.svg|" bellforge-bin/PKGBUILD
sed -i "s/^sha256sums=.*/sha256sums=('${CHECKSUM}' 'SKIP' 'SKIP')/" bellforge-bin/PKGBUILD

echo "==> Regenerating .SRCINFO files..."
makepkg --printsrcinfo > .SRCINFO
cd bellforge-bin
makepkg --printsrcinfo > .SRCINFO
cd ..

echo ""
echo "✅ Done! Review the changes, then:"
echo "   git add PKGBUILD bellforge-bin/PKGBUILD .SRCINFO bellforge-bin/.SRCINFO"
echo "   git commit -m \"Update to v${VERSION}\""
echo "   git push"
echo ""
echo "Don't forget to also push the updated packaging to the AUR repositories."