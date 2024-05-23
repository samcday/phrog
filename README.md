# 🐸

Greetd-compatible greeter for mobile phones

This is a fork of [phog](https://gitlab.com/mobian1/phog).

`phrog` uses Phosh to handle a greetd conversation

## Usage

### Installation

#### Fedora

```
sudo dnf copr enable samcday/phrog
sudo dnf install phrog
```

#### Alpine

Hopefully this makes it into Alpine repos soon. For now:

```
sudo apk add alpine-sdk
cd dist/alpine
abuild deps
abuild
sudo apk add ~/packages/dist/$(uname -m)/greetd-phrog-*.apk
```

#### Other

For now, you must build from source, see the Development section below.

### Running

`phrog` is primarily intended to run via greetd. That is, your `/etc/greetd/config.toml` should
look something like this:

```
[default_session]
command = "systemd-cat --identifier=phrog phoc -E phrog"
# or this if you are using postmarketOS:
command = "dbus-run-session phoc -E phrog"
```

You can also run it directly in a `greetd-fakegreet` session, assuming you have that binary installed:

```
FAKEGREET=1 phoc -E "fakegreet phrog"
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

Now you can build:

```
export LD_LIBRARY_PATH=$(pwd)/_build-phosh/install/usr/local/lib64
export SYSTEM_DEPS_LIBPHOSH_0_SEARCH_NATIVE=$(pwd)/_build-phosh/install/usr/local/lib64
export PKG_CONFIG_PATH=$(pwd)/_build-phosh/install/usr/local/lib64/pkgconfig

cargo build
FAKEGREET=1 phoc -E "fakegreet ./target/debug/phrog"
```

[phosh-deps]: https://gitlab.gnome.org/World/Phosh/phosh#dependencies
