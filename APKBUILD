# Maintainer: Sam Day <me@samcday.com>
_static=1
pkgname=greetd-phrog
pkgver=0.8.0_git
_commit=main
pkgrel=1
pkgdesc="Mobile device greeter"
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
	cargo-auditable"
if [ -n "$_static" ]; then
makedepends="$makedepends
	callaudiod-dev
	elogind-dev
	evince-dev
	evolution-data-server-dev
	feedbackd-dev
	gcr-dev
	gettext-dev
	glib-dev
	gmobile-dev
	gnome-bluetooth-dev
	gnome-desktop-dev
	gtk+3.0-dev
	libadwaita-dev
	libgudev-dev
	libhandy1-dev
	libsecret-dev
	libunistring-dev
	linux-pam-dev
	meson
	networkmanager-dev
	polkit-elogind-dev
	pulseaudio-dev
	py3-docutils
	upower-dev
	wayland-dev
	wayland-protocols"
else
makedepends="$makedepends
	libphosh-dev"
fi
checkdepends="xvfb-run"

source="${url}/archive/$_commit/phrog-$_commit.tar.gz"
subpackages="$pkgname-schemas::noarch"
builddir="$srcdir/phrog-$_commit"
options="net" # cargo fetch

export RUSTFLAGS="$RUSTFLAGS --remap-path-prefix=$builddir=/build/"

prepare() {
	cargo fetch --target="$CTARGET" --locked
}

build() {
    features=""
    [ -n "$_static" ] && features="$features --features=static"
	cargo auditable build $features --release --frozen
}

package() {
	install -Dm755 resources/mobi.phosh.phrog.gschema.xml -t "$pkgdir"/usr/share/glib-2.0/schemas/
	install -Dm755 target/release/phrog -t "$pkgdir"/usr/bin/
}

check() {
    features=""
    [ -n "$_static" ] && features="$features --features=static,test"
    export XDG_RUNTIME_DIR=/tmp
	dbus-run-session xvfb-run -a phoc -E "cargo test $features --frozen"
}

schemas() {
    pkgdesc="Phrog schema files"
    depends=""
    amove usr/share/glib-2.0/schemas
}
