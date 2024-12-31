# This image is used as the build environment in Github Actions.

FROM registry.gitlab.gnome.org/world/phosh/phosh/debian:v0.0.2024-11-04

ARG PHOSH_REMOTE=https://gitlab.gnome.org/World/Phosh/phosh.git
ARG PHOSH_REF=v0.44.0

# Workaround for https://gitlab.gnome.org/World/Phosh/phosh/-/issues/1161
# Official upstream can be used when phoc 0.45 is released
#ARG PHOC_REMOTE=https://gitlab.gnome.org/World/Phosh/phoc.git
ARG PHOC_REMOTE=https://gitlab.gnome.org/guidog/phoc.git
ARG PHOC_REF=05f9cfbe4c4c0e74732d8817e1e2959053d40e6d

RUN export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y update \
   && apt-get -y install --no-install-recommends \
    dh-cargo \
    rust-clippy \
    wf-recorder \
   && eatmydata git clone ${PHOC_REMOTE} \
   && cd phoc \
   && git checkout ${PHOC_REF} \
   && DEB_BUILD_PROFILES=pkg.phoc.embedwlroots eatmydata apt-get --no-install-recommends -y build-dep . \
   && eatmydata meson setup --prefix=/usr -Dembed-wlroots=enabled _build . \
   && eatmydata meson compile -C _build \
   && eatmydata meson install -C _build \
   && cd .. \
   && eatmydata git clone ${PHOSH_REMOTE} \
   && cd phosh \
   && git checkout ${PHOSH_REF} \
   && eatmydata apt-get --no-install-recommends -y build-dep . \
   && eatmydata meson setup --prefix=/usr -Dbindings-lib=true _build . \
   && eatmydata meson compile -C _build \
   && eatmydata meson install -C _build \
   && eatmydata apt-get clean
