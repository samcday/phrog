# 🐸

Greetd-compatible greeter for mobile phones

This is a fork of [phog](https://gitlab.com/mobian1/phog).

`phrog` uses Phosh to handle a greetd conversation

## Usage

### Installation

#### Fedora

```
sudo dnf copr enable samcday/phrog
# If you want to test the latest and/or greatest
# sudo dnf copr enable samcday/phrog-nightly
sudo dnf install phrog
```

#### Other

For now, you must build from source, see the Development section below.

### Running

`phrog` is primarily intended to run via greetd - your `/etc/greetd/config.toml` should
look like this:

```
[default_session]
command = "systemd-cat --identifier=phrog phrog"
```

You can also run/test it directly in a faked greetd session:

```
phrog --fake
```

## Development

Right now, this project depends on unversioned/WIP upstream changes. `phosh` and `libphosh-rs`
are subtrees of this repo.

You must first build the libphosh fork:

(so, hey, fam. this means you need to do all the stuff [over here][phosh-deps], alright? okay cool.)

```
meson setup -Dbindings-lib=true _build-phosh phosh
meson install --destdir=install -C _build-phosh
```

Now you can build with these flags:

```
export LD_LIBRARY_PATH=$(pwd)/_build-phosh/install/usr/local/lib64
export SYSTEM_DEPS_LIBPHOSH_0_SEARCH_NATIVE=$(pwd)/_build-phosh/install/usr/local/lib64
export PKG_CONFIG_PATH=$(pwd)/_build-phosh/install/usr/local/lib64/pkgconfig
```

Make sure the local project schema is installed:

```
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp resources/mobi.phosh.phrog.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
```

Build the app.

```
cargo build
```

Run the app in test mode.

```
cargo run -- --fake
```

[phosh-deps]: https://gitlab.gnome.org/World/Phosh/phosh#dependencies
