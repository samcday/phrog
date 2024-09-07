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
	greetd-phrog-schemas
	libphosh"
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
b637aeb79834e134a1c47f579098c48d4631213a6b8ecab7566ead5403eaa6104df6524c0d9bbee79a53663415aa92967b3ffaac5fc24f8224edc9e8527ab1c5  phrog-0.7.0.tar.gz
"
