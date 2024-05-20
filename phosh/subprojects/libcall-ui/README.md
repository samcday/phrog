# libcall-ui

Libcall-ui carries common user interface parts for call handling. It is meant
to be used as a git submodule.

## License

libcall-ui is licensed under the LGPLv2.1-or-later.

## Getting the source

```sh
    git clone git@gitlab.gnome.org:World/Phosh/libcall-ui.git
    cd libcall-ui
```

The main branch has the current development version.

## Dependencies
See `meson.build` for required dependencies.

## Building

We use the meson (and thereby Ninja) build system for phosh.  The quickest
way to get going is to do the following:

    meson . _build
    ninja -C _build
    ninja -C _build install

## Running the demo
You can run the contained demo via:

    _build/examples/call-ui-demo

# API docs
API documentation is available at https://world.pages.gitlab.gnome.org/Phosh/libcall-ui/

# Getting in Touch
* Issue tracker: https://gitlab.gnome.org/World/Phosh/libcall-ui/-/issues
* Matrix: https://im.puri.sm/#/room/#calls:talk.puri.sm

