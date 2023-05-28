# Maintainer: pwnwriter < hey@pwnwriter.xyz >

pkgname=oran
pkgver=0.1.0
pkgrel=1
pkgdesc="An x11 window swallower"
arch=('x86_64')
url="https://github.com/metis-os/oran"
license=('MIT')
depends=('openssl' 'gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/metis-os/oran/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release 
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}


