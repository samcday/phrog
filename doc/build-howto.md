Title: Compiling with libcall-ui
Slug: building

# Compiling with libcall-ui

If you need to build libcall-ui, get the source from
[here](https://gitlab.gnome.org/World/Phosh/libcall-ui/) and see the `README.md` file.

## Bundling the library

Libcall-ui is not meant to be used as a shared library. It should be embedded in your source
tree as a git submodule instead:

```
git submodule add https://gitlab.gnome.org/World/Phosh/libcall-ui.git subprojects/libcall-ui
```

Add this to your `meson.build`:

```meson
libcall_ui = subproject('libcall-ui',
  default_options: [
    'package_name=' + meson.project_name(),
    'package_version=' + meson.project_version(),
    'pkgdatadir=' + pkgdatadir,
    'pkglibdir=' + pkglibdir,
    'examples=false',
    'gtk_doc=false',
    'tests=false',
  ])
libcall_ui_dep = libcall_ui.get_variable('libcall_ui_dep')
```
