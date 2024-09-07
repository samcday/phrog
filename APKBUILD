# Maintainer: Sam Day <me@samcday.com>
pkgname=greetd-phrog
pkgver=0.7.0_git
_commit=main
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
source="${url}/archive/$_commit/phrog-$_commit.tar.gz"
subpackages="$pkgname-schemas::noarch"

export RUSTFLAGS="$RUSTFLAGS --remap-path-prefix=$builddir=/build/"

prepare() {
    cd phrog-$_commit
	cargo fetch --target="$CTARGET" --locked
}

build() {
	cargo auditable build --release --frozen
}

package() {
	install -Dm755 src/phrog-$_commit/resources/mobi.phosh.phrog.gschema.xml -t "$pkgdir"/usr/share/glib-2.0/schemas/
	install -Dm755 src/phrog-$_commit/target/release/phrog -t "$pkgdir"/usr/bin/
}

schemas() {
    pkgdesc="Phrog schema files"
    depends=""
    amove usr/share/glib-2.0/schemas
}
