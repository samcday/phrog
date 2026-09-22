# This image is used as the build environment in Github Actions.

FROM registry.gitlab.gnome.org/world/phosh/phosh/debian:v0.0.2025-11-06

# GTK with the custom surface / GtkPlain work that the GTK4 phosh build needs.
# Pinned to a commit rather than a branch so image builds stay reproducible.
# Bump this together with the phosh/ subtree in the phrog repo.
ARG GTK_REMOTE=https://gitlab.gnome.org/guidog/gtk.git
ARG GTK_REF=a4cba2e3c84fafc4745c7d4cacbe062b32774e0e

# The shared and embedded modes use the same checked-in Phosh subtree,
# including downstream GTK4 compatibility fixes.

# The custom GTK in /usr/local must win over the distro GTK for building and
# running phrog. phoc comes from the base image (phosh's own CI image ships
# it); building it here would drag in wlroots, which Debian does not package.
ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
ENV LD_LIBRARY_PATH=/usr/local/lib

# Keep dependency installation and native builds in separate layers so a Phosh
# setup failure does not discard a successfully built custom GTK.
RUN export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y update \
   && apt-get -y install --no-install-recommends \
    curl dh-cargo foot wf-recorder \
    meson ninja-build pkg-config glslc \
    libglib2.0-dev libgdk-pixbuf-2.0-dev libpango1.0-dev libcairo2-dev \
    libgraphene-1.0-dev libepoxy-dev libegl1-mesa-dev libwayland-dev \
    wayland-protocols libxkbcommon-dev iso-codes shared-mime-info

RUN eatmydata git clone ${GTK_REMOTE} /gtk \
   && git -C /gtk checkout ${GTK_REF}

RUN apt-get -y install --no-install-recommends libdrm-dev libgbm-dev

RUN eatmydata meson setup --prefix=/usr/local -Dlibdir=lib \
    -Dbuild-tests=false -Dbuild-demos=false -Dbuild-examples=false \
    -Ddocumentation=false -Dintrospection=disabled -Dmedia-gstreamer=disabled \
    /gtk/_build /gtk \
   && eatmydata meson compile -C /gtk/_build \
   && eatmydata meson install -C /gtk/_build

COPY phosh /phosh

RUN export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y update \
   && cd /phosh \
   && eatmydata apt-get --no-install-recommends -y build-dep . \
   && apt-get --no-install-recommends -y install \
    libgcr-4-dev libgnome-desktop-4-dev libgnome-bg-4-dev phoc

RUN eatmydata meson setup --prefix=/usr -Dbindings-lib=true -Dtests=false \
    -Dphoc_tests=disabled -Dlockscreen-plugins=false -Dquick-setting-plugins=false \
    /phosh/_build /phosh \
   && eatmydata meson compile -C /phosh/_build \
   && eatmydata meson install -C /phosh/_build \
   && eatmydata apt-get clean
