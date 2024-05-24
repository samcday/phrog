#!/usr/bin/env sh

GTK_VERSION=$(wget -qO- "https://packages.msys2.org/api/search?query=gtk4" | jq -r ".results.exact.version")

wget -qO- "https://mirror.msys2.org/mingw/ucrt64/mingw-w64-ucrt-x86_64-gtk4-$GTK_VERSION-any.pkg.tar.zst" | \
    zstdcat - | tar xO "ucrt64/share/gir-1.0/GdkWin32-4.0.gir" > GdkWin32-4.0.gir
