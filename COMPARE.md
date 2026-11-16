# Comparing reference code vs the MPF port

## Dev container (podman or docker — any host OS)

```sh
podman build -t mpf-dev .
podman run -it --rm \
  -v "$PWD":/work/project \
  -v "$PWD/../repos":/work/repos:ro \
  mpf-dev
```

Inside the container: the project is at `/work/project`, the reference sources
(`wine`, `proton`, `dxvk`, `vkd3d-proton`, `umu-launcher`) at `/work/repos`
(override with `MPF_REFS`).

## Diff one component

```sh
# reference: repos/wine/dlls/kernel32   vs   port: crates/mpf-kernel32/src
cargo xtask compare dlls/kernel32 mpf-kernel32
```

## Inspect a PE with the port's own parser

```sh
cargo run -p mpf-cli -- info /work/repos/proton/.../some.dll
# or any Windows .exe/.dll on the host, or a synthetic one:
cargo run -p mpf-pe --example dump_test_pe -- /tmp/mpf_test.dll
cargo run -p mpf-cli -- info /tmp/mpf_test.dll
```

## Side-by-side browsing

```sh
cd /work
ls repos/wine/dlls | head     # reference (744 components)
ls project/crates             # port
```
