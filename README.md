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
FAKEGREET=1 phoc -E "fakegreet ./target/debug/phrog"
```


## Development
