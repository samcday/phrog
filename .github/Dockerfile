# This image is used as the build environment in Github Actions.

FROM registry.gitlab.gnome.org/world/phosh/phosh/debian:v0.0.2024-11-04

ARG PHOSH_TAG=v0.43.1

RUN export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y update \
   && apt-get -y install --no-install-recommends \
    dh-cargo \
    rust-clippy \
    wf-recorder \
   && eatmydata git clone -b ${PHOSH_TAG} https://gitlab.gnome.org/World/Phosh/phosh.git \
   && cd phosh \
   && eatmydata apt-get -y build-dep . \
   && eatmydata meson setup --prefix=/usr -Dbindings-lib=true _build . \
   && eatmydata meson compile -C _build \
   && eatmydata meson install -C _build \
   && eatmydata apt-get clean
