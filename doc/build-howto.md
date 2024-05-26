Title: Compiling with gmobile
Slug: building

# Compiling with gmobile

If you need to build gmobile, get the source from
[here](https://gitlab.gnome.org/World/Phosh/gmobile/) and see the `README.md` file.

## Using pkg-config

Like other libraries, gmobile uses `pkg-config` to provide compiler
options. The package name is `gmobile`.


If you use Automake/Autoconf, in your `configure.ac` script, you might specify
something like:

```
PKG_CHECK_MODULES(GMOBILE, [gmobile])
AC_SUBST(GMOBILE_CFLAGS)
AC_SUBST(GMOBILE_LIBS)
```

Or when using the Meson build system you can declare a dependency like:

```meson
dependency('gmobile')
```

## Bundling the library

If you don't want to use the shared library gmobile can be bundled in
one of two ways:

### As a git submodule

To use it as a submodule add the submodule to git

```
git submodule add https://gitlab.gnome.org/World/Phosh/gmobile.git subprojects/gmobile
```

And then add this to your `meson.build`:

```meson
gmobile = subproject('gmobile',
  default_options: [
    'package_name=' + meson.project_name(),
    'package_version=' + meson.project_version(),
    'pkgdatadir=' + pkgdatadir,
    'pkglibdir=' + pkglibdir,
    'examples=false',
    'gtk_doc=false',
    'tests=false',
  ])
gmobile_dep = gmobile.get_variable('gmobile_dep')
```

### As a meson subproject

To use it as a meson subproject add this to `subprojects/gmobile.wrap`:

```ini
[wrap-git]
directory=gmobile
url=https://gitlab.gnome.org/World/Phosh/gmobile.git
revision=main
depth=1
```

You can then use `gmobile_dep` in your mesn build files like:

```meson
gmobile = dependency('gmobile',
                     fallback: ['gmobile', 'gmobile_dep'],
			         native: true)
```
