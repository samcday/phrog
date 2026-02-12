# phrog development tasks

# Bump version across all packaging files
bump version:
    #!/usr/bin/env bash
    set -euo pipefail

    version="{{version}}"
    cargo_version="$version"
    debian_version="$version"
    if [[ "$version" =~ _rc([0-9]+)$ ]]; then
        cargo_version="${version/_rc/-rc.}"
        debian_version="${version/_rc/~rc}"
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
    sed -i 's/^Version:        .*/Version:        {{version}}/' phrog.spec

    # Alpine APKBUILD (set base version, keep _git suffix for dev builds)
    sed -i 's/^pkgver=.*_git$/pkgver={{version}}_git/' APKBUILD

    # Debian changelog (add new entry)
    echo "DEBUG: debian_version=${debian_version}, full version=${debian_version}-1"
    echo "DEBUG: Running: dch --newversion \"${debian_version}-1\" --distribution unstable --urgency medium \"Release ${version}\""
    DEBEMAIL="phrog@beep.boop" DEBFULLNAME="Phrogbot" \
        dch --newversion "${debian_version}-1" --distribution unstable --urgency medium \
        "Release ${version}"

    if [[ ! "$version" =~ _rc[0-9]+$ ]]; then
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
    if [[ ! "$version" =~ _rc[0-9]+$ ]]; then
        echo "  README.md"
    fi
    echo "  Cargo.lock"
    echo ""
