# syntax=docker/dockerfile:1
# MPF dev image. Builds and runs identically under podman and docker on any
# host OS (Linux, macOS, Windows — podman/docker run a Linux VM where needed):
#
#   podman build -t mpf-dev .
#   podman run -it --rm \
#     -v "$PWD":/work/project \
#     -v "$PWD/../repos":/work/repos:ro \
#     mpf-dev
#
# Inside the container: project at /work/project, reference sources
# (wine, proton, dxvk, vkd3d-proton, umu-launcher) at /work/repos.
FROM docker.io/library/rust:1.99-bookworm

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       build-essential git curl ca-certificates diffutils file less pkg-config \
       flex bison perl python3 meson ninja-build \
    && rm -rf /var/lib/apt/lists/*

ARG USER_UID=1000
ARG USER_GID=1000
RUN groupadd -g "${USER_GID}" dev \
    && useradd -m -u "${USER_UID}" -g "${USER_GID}" -s /bin/bash dev

WORKDIR /work
COPY . /work/project
RUN chown -R dev:dev /work
USER dev
ENV CARGO_TERM_COLOR=always \
    MPF_REFS=/work/repos
CMD ["bash"]
