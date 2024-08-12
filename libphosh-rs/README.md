# libphosh-rs

The Rust bindings of [phosh][phosh]

## Development

You will need the following installed:

 * Rust
 * `xmlstarlet`
 * Meson
 * All [Phosh][phosh-deps] build dependencies

### Updating the introspection XML

If the upstream libphosh introspection `Phosh-0.gir` XML has changed, then run the following:

```
make Phosh-0.gir
```

The `main` branch of [Phosh][phosh] will be fetched as a Meson subproject, the introspection XML will be regenerated, and the result will be copied to `./Phosh-0.gir`. You should commit the changes to this repo.

### Updating the bindings

If you've updated the introspection XML, or made changes to the `Gir.toml` files, then run:

```
make
```

Note that you should *not* commit the changes that were made to `NM-1.0.gir` or `Phosh-0.gir`.

### Examples

There's two examples demoing the libphosh-rs usage:

- [hello-world.rs](./libphosh/examples/hello-world.rs) has the  minimum code to spawn a shell and can be run like

```sh
   WLR_BACKENDS=wayland phoc -E target/debug/examples/hello-world
```

- [custom-shell-and-lockscreen.rs](./libphosh/examples/custom-shell-and-lockscreen.rs) shows how to override the shell and lockscreen classes and can be run like

```sh
   WLR_BACKENDS=wayland phoc -E target/debug/examples/custom-shell-and-lockscreen
```

### Documentation

API documentation is at <https://guidog.pages.gitlab.gnome.org/libphosh-rs/git/docs/>

[phosh]: https://gitlab.gnome.org/World/Phosh/phosh
[phosh-deps]: https://gitlab.gnome.org/World/Phosh/phosh#dependencies
