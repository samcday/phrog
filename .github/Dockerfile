# This image is used as the build environment in Github Actions.

FROM registry.gitlab.gnome.org/world/phosh/phosh/debian:v0.0.2024-11-04

ARG PHOSH_REMOTE=https://gitlab.gnome.org/World/Phosh/phosh.git
ARG PHOSH_REF=v0.44.0

ARG PHOC_REMOTE=https://gitlab.gnome.org/World/Phosh/phoc.git
ARG PHOC_REF=v0.44.1

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
