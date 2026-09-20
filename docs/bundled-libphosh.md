# Experimental bundled libphosh

The default build uses the published Rust bindings and the installed
`libphosh-0.45` ABI through pkg-config. `cargo vendored-phosh build` instead selects our
vendored Rust bindings, compiles the bundled GTK3 Phosh source, and statically
links libphosh and its internal helper libraries. GTK3, libhandy,
and other native dependencies remain shared. This is not a standalone binary.
Distro packages continue using system libphosh.

Vendoring and linking are separate choices: the repository records an exact
Phosh source snapshot, while the Cargo configuration selects how that snapshot is
built into phrog. Merely having the subtree present does not select embedding.
Linking an independently installed static libphosh is not a supported mode here.

## Build

Install Rust, a C toolchain, Meson, Ninja, pkg-config, and the native build
dependencies of the bundled Phosh. On Debian, `apt-get build-dep ./phosh` is a
convenient starting point; on Fedora, `dnf builddep phosh` provides the distro's
Phosh dependencies, which may differ from this snapshot. Meson's configuration
errors identify missing or incompatible dependencies. The embedding build
turns off Phosh tests and both plugin sets; their GTK4 preferences and document
viewer dependencies are not required.

Use the generic command from the repository root:

```sh
cargo vendored-phosh build
phoc -S -E 'cargo vendored-phosh test -- --test-threads=1'
```

`cargo vendored-phosh <command> [arguments...]` forwards the command to Cargo with
`.cargo/vendor.toml` loaded. For example:

```sh
cargo vendored-phosh run -- --fake
cargo vendored-phosh fmt --all --check
cargo vendored-phosh tree
cargo vendored-phosh clippy --all-targets -- -D warnings
```

Cargo reserves `cargo vendor` for downloading Rust dependencies, so this alias
uses the distinct name `vendored-phosh`. Its small, dependency-free dispatcher
lives in an independent workspace: launching it cannot switch the application's
lockfile back to registry dependencies. It preserves arguments and exit status.
Clippy receives the configuration after its command name so it can forward it to
its internal Cargo invocation. Rustfmt uses Cargo's global configuration option,
because its own parser rejects Cargo flags; third-party Cargo plugins may have
their own configuration conventions.

The configuration patches both binding crates to local paths, forces
`PHROG_LIBPHOSH_BUILD_INTERNAL=always`, and uses `target/vendor` to keep native
artifacts separate.

The native build script stages Phosh under its Cargo output directory, then runs
`meson subprojects download` for the pinned gvc and libcall-ui revisions. It does
not download into or modify the checkout. Existing downloads are reused when
their wrap files still match; changing a pin invalidates that cached subproject.
Staging retains a recovery copy until the replacement source tree is ready, so
an interrupted build does not discard prepared downloads.
Meson setup uses `--wrap-mode=nodownload`, so other native dependencies must be
installed rather than implicitly fetched.

Switching between registry and local crates changes `Cargo.lock`. Run the first
build after a switch without `--locked`; subsequent builds/tests in that mode
can use `--locked`. The committed lockfile describes the default registry build.
CI starts from that lockfile, builds through `cargo vendored-phosh build`, then runs
`cargo vendored-phosh test --locked` and lint with the same vendor configuration.
Fork PRs retain test-recording artifacts, but hosted demo publication is limited
to same-repository runs because forks cannot access the publication credentials.
Returning to `cargo build` restores registry resolution. Do not commit the
vendor-mode lockfile as the default lockfile.

For a fully offline rebuild after preparing both Cargo and native dependencies:

```sh
PHROG_VENDOR_OFFLINE=1 cargo vendored-phosh build --frozen
```

`PHROG_VENDOR_OFFLINE=1` disables Meson's download step. Cargo's `--offline` or
`--frozen` only governs Cargo's own network access. A fresh offline build needs
the prepared `phosh/subprojects/gvc` and `phosh/subprojects/libcall-ui` source
directories (or a compatible `PHOSH_SRC` tree containing them), as well as the
Cargo dependency cache. A GitHub source archive alone is not a complete offline
build input.

`PHOSH_SRC` can select another compatible source tree. Its default is relative
to the bindings crate. The internal selector accepts `always` or `never`; it
rejects implicit fallback (`auto`) and unknown values. The vendor configuration
forces `always`, even if the shell has selected `never`. Ordinary builds do not
load this configuration and use the public bindings.

This configuration-based interface avoids forwarding an unpublished Cargo
feature. The published phrog archive excludes both native subtrees and the
checkout-only dispatcher, configuration, and runner. No patched binding release or
publication-time manifest rewrite is needed.

Only native builds are currently supported for embedding. Cross builds should
use system libphosh in a properly configured target sysroot. Meson cross-file
support is future work.

## Resources and runtime data

The existing local Phosh patch registers its compiled GResources explicitly
from `phosh_shell_init`, ensuring the resource object is linked from the static
archive. It registers once per process. Native library paths use a private
`/usr/lib/phrog` prefix so the snapshot does not scan the system Phosh plugin
directory. No external plugin ABI is supported by this embedding mode.

The native build compiles selected library and schema targets under Cargo's
`OUT_DIR`. It does not run `meson install`, overwrite installed libraries, or
copy Phosh schemas into the builder's home directory. The pre-existing phrog
build script still installs phrog's own development schema into the user's
local schema directory; that behavior is shared with the default build and is
not changed by this experiment.

An embedded library still needs runtime data: Phosh schemas and translations,
GNOME schemas, icons, and the services already required by phrog. Static linking
does not supply these. The vendor configuration's runner automatically selects
the compiled Phosh schemas for each test/run command. Cargo and Clippy can create separate native
output directories; the runner requires their compiled schemas to match. If
stale copies disagree, run `cargo vendored-phosh clean -p libphosh-sys`
and rebuild. GLib still falls back to system schema sources for other schemas.
A distributor opting into embedding must ship matching Phosh
schema and translation data and arrange schema discovery explicitly. This PR
does not switch any distro package to that model. Packaged static embedding and
its data upgrade policy remain separate follow-up work.

The embedding interface requires a repository checkout with these patched Rust
bindings. Published crates use the registry bindings and the system-library
build; the native and bindings subtrees are excluded from phrog's crate archive.
Git source archives retain them. Coordinating an embedding interface with the
upstream bindings is future work.

## Upstream tracking and GTK4 reconciliation

- Phosh subtree: `62fde093044d90b82154e68a39feb8e6ea967ef5` (Phosh 0.53.0).
- Rust bindings subtree: `5a356759a8f087263c0656d6a11dce2e76cab551`.
- gvc: `d2442f455844e5292cb4a74ffc66ecc8d7595a9f`.
- libcall-ui: `7389b4ae90e101620ef8e790e76a98e434bd920c` (v0.1.5).

Keep subtree updates separate from local embedding patches. The GTK4 branch
already includes the original static-build prototype. When reconciling this
work, preserve its GTK4 sources and regenerated bindings, adopt the vendor
dispatcher/configuration, and adapt its native targets and schema setup.
Do not replace its Phosh subtree with this GTK3 snapshot. Static embedding does
not settle the GTK4 migration or its release schedule.
