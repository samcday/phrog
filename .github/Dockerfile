# This image is used as the build environment in Github Actions.
# It's based on Phosh's CI image because we do a full (static) build of Phosh during the main CI run

FROM registry.gitlab.gnome.org/world/phosh/phosh/debian:v0.0.2024-05-08

RUN export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y update \
   && apt-get -y install --no-install-recommends \
    comerr-dev \
    dh-cargo \
    libgmobile-dev \
    libgssapi-krb5-2 \
    libk5crypto3 \
    libkrb5-dev \
    rust-clippy \
    wf-recorder \
   && eatmydata apt-get clean
