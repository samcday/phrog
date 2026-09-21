# GTK4 development

`main` continues GTK3 stable releases alongside Phosh. `gtk4` is a long-running
integration branch for the GTK4/libadwaita migration. It becomes eligible for
integration into `main` when the required GTK/Phosh APIs are available upstream
and the migration's functional and packaging gaps are resolved.

## Branch maintenance

- The GTK4 series has two layers above #186: PR #213 (`codex/gtk4-phosh`)
  imports Guido's Phosh GTK4 subtree and native dependency pins; PR #209 (`gtk4`)
  carries the regenerated bindings, phrog migration, and build/CI integration.
- Target Phosh dependency updates at `codex/gtk4-phosh`, then merge that branch
  into `gtk4`. Target application and binding changes at `gtk4`.
- Both GTK4 layers remain draft while `main` continues GTK3 stable releases.
  The Phosh import layer alone still has GTK3 Rust bindings; build and test the
  complete GTK4 application at `gtk4`.
- Land shared fixes on `main` first, then merge `main` into `gtk4` regularly.
- Keep upstream source updates, local native patches, and application changes in
  separate commits. Avoid routine rebases of the shared integration branch.
- GTK4 currently produces CI artifacts identified by commit/run. Stable tags,
  release candidates, crates.io publication, and the existing distro/COPR
  channels remain GTK3. The inherited release/package workflows are disabled
  here until a separate GTK4 distribution channel is designed.

The current reconstruction is based on #186's explicit vendored-build interface.
It does not supersede #186. GTK4 uses that interface to build an unreleased
libphosh snapshot; static embedding remains an independent proposal.

## Dependency snapshots

| Component | Source | Commit |
| --- | --- | --- |
| GTK | guidog/gtk, custom-surface | `a4cba2e3c84fafc4745c7d4cacbe062b32774e0e` |
| Phosh | guidog/phosh, phosh-gtk4 | `37d48c69b2414fb8b8f9cee19eae60a700c4c551` |
| libphosh-rs import | upstream main | `580d6093` |
| gir generator | gtk-rs/gir | `601c9bb7f8f42a095df1740453fab3d4d6679788` |
| gvc | guidog/libgnome-volume-control, phosh/0.52.1 | `1cdc1cb2d622d64e9ad2781093bcc63719c5ea5b` |
| libcall-ui | World/Phosh/libcall-ui, v0.2.1 | `f66056ace818ff19b507335634dd67138a92c77f` |

The GTK custom-surface snapshot provides `GtkPlain`, which this Phosh snapshot
requires. A sufficiently high stock GTK version alone is not enough. GTK is
built in `.github/Dockerfile`; Phosh and its Rust bindings are repository
subtrees. Keep the Dockerfile's Phosh pin aligned with the Phosh subtree.

## Build and test

The supported CI environment builds the pinned GTK and a shared GTK4 libphosh,
then tests both dynamically linked and embedded phrog. Build that environment:

```sh
podman build -t phrog-gtk4-ci -f .github/Dockerfile .github
```

Within an environment providing those dependencies and the Rust toolchain from
`rust-toolchain.toml`, run from the repository root:

```sh
# Dynamic GTK4 libphosh; local GTK4 Rust bindings are used in both modes.
cargo build --locked --all-targets
cargo clippy --locked --all-targets --no-deps -- -D warnings
phoc -S -E 'cargo test --locked -- --test-threads=1'

# Embed the pinned Phosh source using the interface shared with #186.
cargo vendored-phosh build --locked --all-targets
cargo vendored-phosh clippy --locked --all-targets --no-deps -- -D warnings
phoc -S -E 'cargo vendored-phosh test --locked -- --test-threads=1'
```

For headless tests, use the `dbus-run-session xvfb-run ... phoc` invocation in
`.github/workflows/build.yml`. Host prefixes from earlier development sessions
are not assumed to exist. When using your own prefix, set `PKG_CONFIG_PATH`,
`LD_LIBRARY_PATH`, and `XDG_DATA_DIRS` to the custom GTK/Phosh installation.

GTK3 and GTK4 libphosh currently share the `libphosh-0.45` pkg-config name.
The build rejects metadata that selects GTK3, even if its version satisfies the
minimum. Never mix the GTK4 bindings with the GTK3 shared library.

The vendor command stages native sources and schemas under `target/vendor`;
its runner selects the matching schemas. `PHROG_VENDOR_OFFLINE=1` disables
native downloads once dependencies are prepared. Embedded libphosh still needs
shared GTK and other native libraries plus schemas, translations, and services.
Distribution recipes are not yet GTK4-ready.

## Local desktop with Toolbox

Create a Toolbox from the same patched GTK/Phosh environment used by CI:

```sh
podman build -t localhost/phrog-gtk4-ci -f .github/Dockerfile .github
podman build -t localhost/phrog-gtk4-toolbox -f tools/toolbox/Containerfile tools/toolbox
toolbox create --image localhost/phrog-gtk4-toolbox phrog-gtk4
```

Then, from this checkout in a desktop terminal:

```sh
toolbox run -c phrog-gtk4 ./tools/toolbox/run
```

The launcher builds into `target/toolbox` and opens phrog in a nested Phoc window.
It uses the host's Rust installation in `~/.cargo`, the container's patched native
libraries, software rendering, a private session bus, and in-memory settings.
It prefers the nested X11 backend when `$DISPLAY` is available, with Wayland as
a fallback. Upstream GTK4 diagnostics are saved to `target/toolbox/phrog.log`.
Authentication uses `--fake`; the test password is `0`. Close the nested window
or press Ctrl+C in the launching terminal to stop it. The first build takes
longer; subsequent launches reuse the Cargo build cache.

For an interactive development shell, use `toolbox enter phrog-gtk4` and change
to this checkout. Native Phosh/GTK dependency changes require rebuilding the
images and recreating the Toolbox; ordinary phrog edits only need another run
of the launcher.

## Local patches and migration gaps

- Static Phosh resources are registered explicitly in `phosh_shell_init`,
  inherited from #186. Native build output and schemas remain inside Cargo's
  target directory; the vendor runner selects the matching schema bundle.
- `libphosh-rs/fix.sh` rewrites the unnamed `GtkPlain` parent instance field to
  `Gtk.Widget` / `GtkWidget`. It preserves the field: deleting it makes generated
  instance structs too small for subclass registration. This temporary workaround
  needs replacement and ABI validation when GtkPlain gains introspection.
- The lockscreen uses the layer surface's `configured` signal to select the
  default page because the upstream GTK4 port still relies on widget `show`.
- The emergency menu currently crashes while constructing its upstream template,
  according to the original migration's local run. `tests/emergency_calls.rs`
  currently checks fixture setup and power-menu interaction only; it does not
  validate emergency dialling or incoming calls. Restore that coverage before
  treating the migration as feature-complete.
- GTK4 distro packaging, a separate snapshot distribution channel, and final
  upstream ABI compatibility remain outstanding. A passing development build is
  not a declaration that GTK4 is ready to replace GTK3 stable releases.

Bindings were regenerated from the GTK4 Phosh GIR using the pinned gir tool.
Build the generator outside this workspace, apply `fix.sh`, then use the existing
`libphosh-rs/Makefile` generation targets. Preserve the local sys crate's
`build.rs` and `native_source.rs`: generator output does not
replace the embedding implementation inherited from #186.
