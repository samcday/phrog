## 🐸 (phrog)

<img align="right" width="180" height="360" src="https://github.com/samcday/phrog/releases/download/0.42.0/demo.webp">

<br />
<br />
<br />

A greeter that works on mobile devices and also other kinds of computers.

🤓 `phrog` uses [Phosh][] to conduct a [greetd][] conversation.

It is the spiritual successor of [phog][].

<br clear="right"/>

## Installation

### Alpine/postmarketOS (edge)

`phrog` is available in the [Alpine edge testing package repository][alpine-pkg].

```sh
# If you haven't yet setup the testing repo on your device, see here:
# https://wiki.alpinelinux.org/wiki/Repositories#Using_testing_repository
apk add greetd-phrog@testing
```

### Debian

Not yet available in official repositories, but [an MR is pending](debian-mr).

### Fedora

Not yet available in official repositories, but [a COPR][copr] is maintained:

```
sudo dnf copr enable samcday/phrog
sudo dnf install phrog
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
[alpine-pkg]: https://pkgs.alpinelinux.org/packages?name=greetd-phrog&branch=edge&repo=&arch=&origin=&flagged=&maintainer=
[copr]: https://copr.fedorainfracloud.org/coprs/samcday/phrog/
[debian-mr]: https://salsa.debian.org/DebianOnMobile-team/phrog/-/merge_requests/1
