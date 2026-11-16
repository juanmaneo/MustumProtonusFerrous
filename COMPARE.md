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

## The whole port at once

```sh
cargo xtask compare-all              # one pass, report in target/compare/report.txt
cargo xtask compare-all --watch 30   # continuous, in the dev container: re-runs every 30 s
```

Every row of `coverage/mapping.txt` that names a crate is diffed
(`repos/wine/<wine-subdir>` against `crates/<crate>/src`). The report lists
each component's reference/port file counts and differing files, then the
raw diffs, and ends with totals. Rows whose reference dir or port crate is
missing are listed as skipped; a `diff` that itself fails (exit above 1)
makes the command exit 1 after writing the report.

## The dlls inventory and priorities

```sh
cargo xtask dlls
```

Lists every component of `repos/wine/dlls` with its port status
(`ported`/`partial`/`planned`/`native`/`todo`) and its Proton
game-compatibility priority, sorted P1 first, plus a per-status summary.
The classification comes from `coverage/mapping.txt` (format in the file
header); components with no mapping row show as `todo` / `-`.

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
