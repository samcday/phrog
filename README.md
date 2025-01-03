## 🐸 (phrog)

<img align="right" width="180" height="360" src="https://github.com/samcday/phrog/releases/download/0.43.0/demo.webp">

<br />
<br />
<br />

A greeter that works on mobile devices and also other kinds of computers.
 
🤓 `phrog` uses [Phosh][] to conduct a [greetd][] conversation.

It is the spiritual successor of [phog][].

<br clear="right"/>

## Installation

* Alpine (v3.21+): `apk add greetd-phrog`
* Fedora ([COPR][]): `sudo dnf copr enable samcday/phrog && sudo dnf install phrog`
* Other: you must build from source, see the Development section below.

## Running

`phrog` is a [greetd][] "greeter". To use it, make sure your `/etc/greetd/config.toml` looks like this:

```
[default_session]
command = "systemd-cat --identifier=phrog phrog"
```

Then run greetd however your distro prefers you to.

You can also test it outside greetd, nested in your favourite Wayland desktop environment:

```
phoc -S -E "phrog --fake"
```

## Development

`libphosh` is required to build this project.

* Alpine (v3.21+): `sudo apk add libphosh`
* Fedora: `sudo dnf install libphosh-devel`
* Debian: `sudo apt install libphosh-0.44-dev`

If `libphosh` is not packaged for your distro, you need to build Phosh+libphosh manually. See the [Phosh][] README for more info.

Once `libphosh` is installed, building and running 🐸 should be as simple as:

```sh
phoc -S -E "cargo run -- --fake"
phoc -S -E "cargo test"
```

[phog]: https://gitlab.com/mobian1/phog
[Phosh]: https://gitlab.gnome.org/World/Phosh/phosh
[greetd]: https://sr.ht/~kennylevinsen/greetd/
[COPR]: https://copr.fedorainfracloud.org/coprs/samcday/phrog/
