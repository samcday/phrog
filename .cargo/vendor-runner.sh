#!/usr/bin/env bash
# Cargo invokes this for vendored test binaries and `cargo run --config ...`.
set -euo pipefail

# The dispatcher must start before libphosh (and its schemas) has been built.
if [[ $(basename "$1") == xtask ]]; then
    exec "$@"
fi

profile_dir=$(dirname "$1")
if [[ $(basename "$profile_dir") == deps ]]; then
    profile_dir=$(dirname "$profile_dir")
fi
schemas=("$profile_dir"/build/libphosh-sys-*/out/phosh/data/gschemas.compiled)
for schema in "${schemas[@]}"; do
    if [[ ! -f "$schema" ]] || ! cmp -s "${schemas[0]}" "$schema"; then
        echo 'Missing or inconsistent vendored Phosh schemas; clean libphosh-sys with the vendor configuration and rebuild.' >&2
        exit 1
    fi
done
export GSETTINGS_SCHEMA_DIR
GSETTINGS_SCHEMA_DIR=$(cd "$(dirname "${schemas[0]}")" && pwd)
exec "$@"
