# This image is used as the build environment in Github Actions.

FROM registry.gitlab.gnome.org/world/phosh/phosh/debian:v0.0.2025-11-06

# GTK with the custom surface / GtkPlain work that the GTK4 phosh build needs.
# Pinned to a commit rather than a branch so image builds stay reproducible.
# Bump this together with the phosh/ subtree in the phrog repo.
ARG GTK_REMOTE=https://gitlab.gnome.org/guidog/gtk.git
ARG GTK_REF=a4cba2e3c84fafc4745c7d4cacbe062b32774e0e

# phosh-gtk4: build-deps feed the vendored phosh build, and the installed
# libphosh serves as a fallback for non-static builds.
ARG PHOSH_REMOTE=https://gitlab.gnome.org/guidog/phosh.git
ARG PHOSH_REF=37d48c69b2414fb8b8f9cee19eae60a700c4c551

# The custom GTK in /usr/local must win over the distro GTK for building and
# running phrog. phoc comes from the base image (phosh's own CI image ships
# it); building it here would drag in wlroots, which Debian does not package.
ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
ENV LD_LIBRARY_PATH=/usr/local/lib

RUN export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y update \
   && apt-get -y install --no-install-recommends \
    curl \
    dh-cargo \
    foot \
    wf-recorder \
   && eatmydata git clone ${GTK_REMOTE} \
   && cd gtk \
   && git checkout ${GTK_REF} \
   && eatmydata apt-get --no-install-recommends -y build-dep . \
   && eatmydata meson setup --prefix=/usr/local -Dlibdir=lib -Dbuild-tests=false -Dbuild-demos=false -Dbuild-examples=false -Ddocumentation=false -Dintrospection=disabled _build . \
   && eatmydata meson compile -C _build \
   && eatmydata meson install -C _build \
   && cd .. \
   && eatmydata git clone ${PHOSH_REMOTE} \
   && cd phosh \
   && git checkout ${PHOSH_REF} \
   && eatmydata apt-get --no-install-recommends -y build-dep . \
   && apt-get --no-install-recommends -y install \
    libgcr-4-dev \
    libgnome-desktop-4-dev \
    phoc \
   && eatmydata meson setup --prefix=/usr -Dbindings-lib=true -Dtests=false _build . \
   && eatmydata meson compile -C _build \
   && eatmydata meson install -C _build \
   && eatmydata apt-get clean
