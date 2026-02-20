## 🐸 (phrog)

<img align="right" width="180" height="360" src="https://github.com/samcday/phrog/releases/download/0.50.0/demo.webp">

<br />
<br />
<br />

A greeter that works on mobile devices and also other kinds of computers.
 
🤓 `phrog` uses [Phosh][] to conduct a [greetd][] conversation.

It is the spiritual successor of [phog][].

<br clear="right"/>

## Usage

### Alpine/postmarketOS

```
sudo apk add greetd-phrog

# Configure greetd to run phrog:
cat <<HERE | sudo tee -a /etc/conf.d/greetd
cfgfile="/etc/phrog/greetd-config.toml"
HERE

rc-update add greetd
```

### Debian

Currently only in [sid][debian-sid-phrog], but you can also get it from the [Phosh nightly repo][phosh-nightly].

```
sudo apt install phrog
sudo systemctl enable greetd
```

### Fedora

```
# Phrog is not available in Fedora's repos
# But an unofficial COPR is provided:
sudo dnf copr enable samcday/phrog
sudo dnf install phrog

sudo systemctl enable phrog
```

### Other

Want to use phrog on another distro? [Please get in touch.](#getting-help)

If you want to run it manually, you'll need to build from source (see below), and then take a look at the existing packaging to understand how to wire up the necessary bits to spawn a functional greeter session under greetd.

### crates.io / cargo-install

`phrog` is also published on crates.io, so you can install it directly with Cargo:

```sh
cargo install phrog
```

[`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) is supported:

```sh
cargo binstall phrog
```

## Development

`libphosh` 0.45 is required to build this project.

* Alpine (v3.21+): `sudo apk add libphosh`
* Debian (trixie): `sudo apt install libphosh-0.45-dev`
* Fedora: `sudo dnf install libphosh-devel`

If `libphosh` is not packaged for your distro, you need to build Phosh+libphosh manually. See the [Phosh][] README for more info.

Once `libphosh` is installed, building and running 🐸 should be quite simple:

```sh
# To run phrog without greetd, pass --fake
# You can "login" to any user with the password "0" 
phoc -S -E "cargo run -- --fake"

phoc -S -E "cargo test"
```

## Getting help

Found a bug or want to request a feature? [Please file an issue!][issues]

You can also come chat in Matrix: [#phosh:phosh.mobi][Matrix]

[phog]: https://gitlab.com/mobian1/phog
[Phosh]: https://gitlab.gnome.org/World/Phosh/phosh
[greetd]: https://sr.ht/~kennylevinsen/greetd/
[COPR]: https://copr.fedorainfracloud.org/coprs/samcday/phrog/
[issues]: https://github.com/samcday/phrog/issues
[Matrix]: https://matrix.to/#/#phosh:talk.puri.sm
[debian-sid-phrog]: https://packages.debian.org/sid/phrog
[phosh-nightly]: https://phosh.mobi/posts/phosh-nightly/
