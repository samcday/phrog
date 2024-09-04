# 🐸

Greetd-compatible greeter for mobile phones


`phrog` uses Phosh to handle a [greetd][] conversation

It is the spiritual successor of [phog][].

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

`phrog` is primarily intended to run via [greetd][] - your `/etc/greetd/config.toml` should
look like this:

```
[default_session]
command = "systemd-cat --identifier=phrog phrog"
```

You can also run/test it directly with a faked session, run this from a terminal in your favourite (Wayland) desktop environment:

```
phrog --fake
```

## Development

If your system has `libphosh` packaged and available, you should be able to simply:

```sh
# Run 🐸 from source
cargo run -- --fake

# Run 🐸 tests
cargo test
```

You can also run with the locally vendored libphosh source, statically linked. This is useful if you want to work on a feature that also requires changes to upstream libphosh.

```sh
# Install the (many) Phosh build dependencies:
# Fedora: sudo dnf4 build-dep --define 'with_static 1' ./phrog.spec
# Debian (trixie): sudo apt-get build-dep -y ./phosh/

# Then it's mostly the same as before.
# More features may be visible and more tests may run, since the local tree pulls ahead of upstream.
cargo run --features=static -- --fake
cargo test --features=static
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

[phog]: https://gitlab.com/mobian1/phog
[greetd]: https://sr.ht/~kennylevinsen/greetd/
[phosh-deps]: https://gitlab.gnome.org/World/Phosh/phosh#dependencies
