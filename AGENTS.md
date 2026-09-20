# Repository Guidelines

## Project Structure & Module Organization
- `src/` contains the Rust application code (entry in `src/main.rs` and modules like `src/lockscreen.rs`).
- `tests/` holds integration tests plus shared fixtures in `tests/fixtures/` and test data in `tests/data/`.
- `data/` packaging metadata; `debian/`, `copr/`, and `APKBUILD` cover distro packaging.
- `resources/` houses assets that Glib postprocesses into the resulting
- `build.rs` ensures that phrog's schemas are compiled with `glib-build-tools` and put in `~/.local/share/glib-2.0/schemas`.

## Build, Test, and Development Commands
- `cargo build` builds the greeter (requires `libphosh` from your distro).
- `cargo run -- --fake` runs without greetd; pair with `phoc` for Wayland:
  - `phoc -S -E "cargo run -- --fake"`
- `cargo test` runs the test suite; for Wayland-dependent tests:
  - `phoc -S -E "cargo test"`

## Coding Style & Naming Conventions
- Rust 2021 edition; toolchain pinned in `rust-toolchain.toml`.
- Format with `cargo fmt` (rustfmt) and lint with `cargo clippy`.
- Use `snake_case` for files, functions, and modules; keep modules small and focused.

## Testing Guidelines
- Integration tests live in `tests/` and should be named with `snake_case.rs`.
- Place reusable helpers in `tests/common/` and data fixtures in `tests/fixtures/`.
- If a test relies on Wayland/Phosh behavior, document it in the test and use `phoc` when running locally.

## Commit & Pull Request Guidelines
- Recent history uses short, scoped subjects like `ci/debian: rename`. Prefer `area: description` or `area/subarea: description`.
- PRs should include a concise summary, test results, and screenshots or a short recording for UI changes.
- Link related issues when applicable.

## Configuration & Runtime Notes
- `phrog` is a greetd greeter; packaging in `debian/`, `copr/`, and `APKBUILD` shows how it is wired into services.
- Local testing without greetd uses `--fake` and the default login password `0`.

## Vendored Sources
- Keep patches to `phosh/` and `libphosh-rs/` minimal and limited to changes required for embedding. Do not reformat generated/imported code, add formatting exclusions, or change upstream dependency pins incidentally. Keep phrog-specific tooling outside those trees where practical.
- Import and update these directories with `git subtree add` / `git subtree pull --squash`, keeping local fixes in separate commits scoped to one subtree.
- Preserve the subtree merge commits and `git-subtree-*` trailers when landing or restacking: use merge commits, not GitHub squash/rebase merging.
- Update to an explicit upstream commit, then review conflicts and run the relevant build checks:
  - GTK3 Phosh: `git subtree pull --prefix=phosh --squash https://gitlab.gnome.org/World/Phosh/phosh.git <commit>`
  - GTK4 Phosh: use `https://gitlab.gnome.org/guidog/phosh.git` with the same command.
  - Bindings: `git subtree pull --prefix=libphosh-rs --squash https://gitlab.gnome.org/World/Phosh/libphosh-rs.git <commit>`
- Export downstream fixes with `git format-patch --stdout --relative=<prefix> <import-merge>..HEAD -- <prefix>`. Plain `git subtree split` traverses obsolete imports in phrog's old history and can fail on unavailable fork commits.
