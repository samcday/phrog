all:
	git checkout -f *.gir
	./fix.sh
	cd libphosh/sys && gir -o .
	cd libphosh && gir -o .
	cargo build --examples
	cargo test

Phosh-0.gir:
	meson setup _build .
	meson subprojects update
	meson compile -C _build
	cp _build/subprojects/phosh/src/Phosh-0.gir ./

.PHONY: Phosh-0.gir
