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

#### Other

For now, you must build from source, see the Development section below.

### Running

`phrog` is primarily intended to run via greetd. That is, your `/etc/greetd/config.toml` should
look something like this:

```
[default_session]
command = "systemd-cat --identifier=phrog phoc -E phrog"
```

You can also run it directly in a `greetd-fakegreet` session, assuming you have that binary installed:

```
FAKEGREET=1 phoc -E "fakegreet phrog"
```

## Development

Right now, this project depends on unversioned/WIP upstream changes. `phosh` and `libphosh-rs`
are subtrees of this repo.

You must first build the libphosh fork:

```
meson setup -Dbindings-lib=true _build-phosh phosh
meson install --destdir=install -C _build-phosh
```

Now you can build:

```
export LD_LIBRARY_PATH=$(pwd)/_build-phosh/install/usr/local/lib64
export PKG_CONFIG_PATH=$(pwd)/_build-phosh/install/usr/local/lib64/pkgconfig

phoc -E "cargo run"
```
