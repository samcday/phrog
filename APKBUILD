# Maintainer: Sam Day <me@samcday.com>
pkgname=greetd-phrog
pkgver=0.7.0
pkgrel=1
pkgdesc="Greetd-compatible greeter for mobile phones"
url="https://github.com/samcday/phrog"
# riscv64: blocked by greetd
# s390x: blocked by greetd & phosh
# ppc64le: blocked by phosh
# armhf: blocked by phosh
arch="all !s390x !riscv64 !armhf !ppc64le"
license="GPL-3.0-only"
depends="
	phosh
	greetd
	greetd-phrog-schemas"
makedepends="
	cargo
	cargo-auditable
	libphosh-dev
	"
source="${url}/archive/$pkgver/phrog-$pkgver.tar.gz"
subpackages="$pkgname-schemas::noarch"

export RUSTFLAGS="$RUSTFLAGS --remap-path-prefix=$builddir=/build/"

prepare() {
    cd phrog-$pkgver
	cargo fetch --target="$CTARGET" --locked
}

build() {
	cargo auditable build --release --frozen
}

package() {
	install -Dm755 src/phrog-$pkgver/resources/mobi.phosh.phrog.gschema.xml -t "$pkgdir"/usr/share/glib-2.0/schemas/
	install -Dm755 src/phrog-$pkgver/target/release/phrog -t "$pkgdir"/usr/bin/
}

schemas() {
    pkgdesc="Phrog schema files"
    depends=""
    amove usr/share/glib-2.0/schemas
}

sha512sums="
d1ccf6c1d2ce0a4d6df4283cce720a70ac47d6da12b193bf492be7944c120c108b2db49afb1b44f3b651966c4592c623671d8dc6212081de3598fc2cc7e09faf  phrog-0.7.0.tar.gz
"
