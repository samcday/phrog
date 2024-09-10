## 🐸 (phrog)

<img align="right" width="180" height="360" src="https://github.com/samcday/phrog/releases/download/0.10.0/demo.webp">

<br />
<br />
<br />

A greeter that works on mobile devices and also other kinds of computers.

🤓 `phrog` uses [Phosh][] to conduct a [greetd][] conversation.

It is the spiritual successor of [phog][].

<br clear="right"/>

## Installation

### Fedora

```
sudo dnf copr enable samcday/phrog
# If you want to test the latest and/or greatest
# sudo dnf copr enable samcday/phrog-nightly
sudo dnf install phrog
```

### Alpine

The package has been [requested](https://gitlab.alpinelinux.org/alpine/aports/-/issues/16430) in Alpine.

For now, you must build it yourself:

```
sudo apk add alpine-sdk
abuild deps
abuild -K
sudo apk add ~/packages/dist/$(uname -m)/greetd-phrog-*.apk
```

### Other

You must build from source, see the Development section below.

## Running

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

If your system has `libphosh` packaged and available:

```sh
# Install libphosh:
# Fedora: sudo dnf install -y libphosh-devel
# Alpine (edge): sudo apk add libphosh

# Run 🐸 from source
cargo run -- --fake

# Run 🐸 tests
cargo test
```

You can also run with a statically linked libphosh from the vendored `./phosh/` subtree. This is useful if you want to work on a feature that also requires changes to upstream libphosh.

```sh
# Install the (many) Phosh build dependencies:
# Fedora: sudo dnf4 build-dep --define 'with_static 1' ./phrog.spec
# Debian (trixie): sudo apt-get build-dep -y ./phosh/
# Alpine: abuild deps

# Then it's mostly the same as before.
# More features may be visible and more tests may run, since the local tree pulls ahead of upstream.
cargo run --features=static -- --fake
cargo test --features=static
```

[phog]: https://gitlab.com/mobian1/phog
[Phosh]: https://gitlab.gnome.org/World/Phosh/phosh
[greetd]: https://sr.ht/~kennylevinsen/greetd/
