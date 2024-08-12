all: gir/target/release/gir
	git checkout -f *.gir
	./fix.sh
	cd libphosh/sys && ../../gir/target/release/gir -o .
	cd libphosh && ../gir/target/release/gir -o .
	cargo build --examples
	cargo test

gir/target/release/gir:
	git submodule update --init --checkout gir/
	cd gir && cargo build --release

Phosh-0.gir:
	meson setup _build .
	meson subprojects update
	meson compile -C _build
	cp _build/subprojects/phosh/src/Phosh-0.gir ./

.PHONY: Phosh-0.gir
