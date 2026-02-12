# Maintainer: Sam Day <me@samcday.com>
pkgname=greetd-phrog
pkgver=0.53.0_git
pkgrel=0
pkgdesc="Mobile device greeter"
url=https://github.com/samcday/phrog
# s390x: blocked by greetd
# armhf: blocked by phosh
arch="all !s390x !armhf"
license="GPL-3.0-only"
depends="
	phosh
	greetd
	greetd-phrog-schemas
	libphosh"
makedepends="
	cargo
	cargo-auditable
	foot
	libphosh-dev"
checkdepends="xvfb-run"

_gitrev=main
source="https://github.com/samcday/phrog/archive/$_gitrev/phrog-$_gitrev.tar.gz"
subpackages="$pkgname-schemas::noarch"
builddir="$srcdir/phrog-$_gitrev"
# net: cargo fetch
options="net"

# Tests are flaky on loongarch64 + armv7
if [ "$CARCH" = "loongarch64" ] || [ "$CARCH" = "armv7" ]; then
	options="$options !check"
fi

export RUSTFLAGS="$RUSTFLAGS --remap-path-prefix=$builddir=/build/"

prepare() {
	default_prepare
	cargo fetch --target="$CTARGET" --locked
}

build() {
	cargo auditable build --release --frozen
}

package() {
	install -Dm644 data/mobi.phosh.phrog.gschema.xml -t "$pkgdir"/usr/share/glib-2.0/schemas/
	install -Dm644 data/phrog.session -t "$pkgdir"/usr/share/gnome-session/sessions/
	install -Dm644 data/mobi.phosh.Phrog.desktop -t "$pkgdir"/usr/share/applications/
	install -Dm644 dist/alpine/greetd-config.toml -t "$pkgdir"/etc/phrog/
	install -d "$pkgdir"/usr/share/phrog/autostart
	install -d "$pkgdir"/etc/phrog/autostart
	install -Dm755 target/release/phrog -t "$pkgdir"/usr/bin/
	install -Dm755 data/phrog-greetd-session -t "$pkgdir"/usr/libexec/
}

check() {
	export XDG_RUNTIME_DIR="$builddir"
	dbus-run-session xvfb-run -a phoc -E "cargo test --frozen"
}

schemas() {
	pkgdesc="Phrog schema files"
	depends=""
	amove usr/share/glib-2.0/schemas
}
