# phrog development tasks

# Bump version across all packaging files
bump version:
    #!/usr/bin/env bash
    set -euo pipefail

    version="{{version}}"
    cargo_version="$version"
    package_version="$version"
    debian_version="$version"
    is_rc=false

    if [[ "$version" =~ _rc([0-9]+)$ ]]; then
        echo "Unsupported RC format '$version'. Use canonical semver '-rc.N' (for example: 1.2.3-rc.1)."
        exit 1
    fi

    if [[ "$version" =~ ^([0-9]+\.[0-9]+\.[0-9]+)-rc\.([0-9]+)$ ]]; then
        base="${BASH_REMATCH[1]}"
        rc="${BASH_REMATCH[2]}"
        package_version="${base}_rc${rc}"
        debian_version="${base}~rc${rc}"
        is_rc=true
    fi

    echo "Bumping phrog to $version"

    # Cargo.toml workspace version (package section only)
    CARGO_VERSION="$cargo_version" python - <<'PY'
    import os
    import re
    from pathlib import Path

    cargo_version = os.environ["CARGO_VERSION"]
    path = Path("Cargo.toml")
    text = path.read_text()

    match = re.search(r"(?ms)^\[package\]\n.*?(?=^\[|\Z)", text)
    if not match:
        raise SystemExit("package section not found in Cargo.toml")

    block = match.group(0)
    updated = re.sub(r"(?m)^version = \".*\"$", f'version = "{cargo_version}"', block, count=1)
    if block == updated:
        raise SystemExit("package version not found in Cargo.toml")

    path.write_text(text[: match.start()] + updated + text[match.end() :])
    PY

    # RPM spec
    sed -i "s/^Version:        .*/Version:        ${package_version}/" phrog.spec

    # Alpine APKBUILD (set base version, keep _git suffix for dev builds)
    sed -i "s/^pkgver=.*_git$/pkgver=${package_version}_git/" APKBUILD

    # Debian changelog (add new entry)
    DEBEMAIL="phrog@beep.boop" DEBFULLNAME="Phrogbot" \
        dch --newversion "${debian_version}-1" --distribution unstable --urgency medium \
        "Release ${version}"

    if [[ "$is_rc" != "true" ]]; then
        # README demo video URL (skip for RCs)
        sed -i "s|releases/download/[^/]\+/demo.webp|releases/download/{{version}}/demo.webp|" README.md
    fi

    # Update lockfile
    cargo generate-lockfile

    echo "Done. Files updated:"
    echo "  Cargo.toml"
    echo "  phrog.spec"
    echo "  APKBUILD"
    echo "  debian/changelog"
    if [[ "$is_rc" != "true" ]]; then
        echo "  README.md"
    fi
    echo "  Cargo.lock"
    echo ""
