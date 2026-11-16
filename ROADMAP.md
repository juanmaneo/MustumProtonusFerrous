# Roadmap

Port strategy: incremental, per-component, with Wine's own test suite as the
conformance target. We do not mechanically translate C; each component is
re-implemented in safe Rust with `unsafe` confined to FFI/loader edges, and the
reference tree (`../repos/wine`, tag `wine-11.22`) stays mounted side-by-side
in the dev container for direct comparison.

## Phase 0 — scaffold (current)

- [x] workspace + crates: mpf-pe, mpf-ntdll, mpf-kernel32, mpf-loader, mpf-cli
- [x] PE/COFF parser with synthetic-image tests
- [x] RTL unicode strings + NTSTATUS
- [x] PE mapper (headers + sections into a flat image)
- [x] podman/docker dev container + `cargo xtask compare`

## Phase 1 — loader & ntdll core

- [ ] base relocations, import (IAT) resolution, TLS directory, delay imports
- [ ] LDR module list, PEB/TEB structures
- [ ] ntdll: RtlAllocateHeap/RtlFreeHeap, critical sections, full RTL unicode
      string API, object manager basics, exception handling design
      (SEH <-> Rust panic boundary)
- [ ] process init equivalent of `programs/wineboot` + `loader/main.c`

## Phase 2 — first runnable Win32 app

- [ ] kernel32/kernelbase subset (process, thread, module, file, registry)
- [ ] conformance harness running wine's `dlls/*/tests` against MPF builds
- [ ] mpf-wineboot: prefix creation (`drive_c`, registry hives)

## Phase 3 — graphics stack

- [ ] user32/gdi32/winex11.drv-equivalent surface
- [ ] DXVK: initially a native companion (exactly what Proton ships), with a
      Rust configuration layer; port D3D core to Rust later
- [ ] vkd3d-proton: same strategy — native companion first, Rust port later

## Phase 4 — Proton flavour & umu

- [ ] Rust equivalent of Proton's `proton` script (prefix setup, env vars,
      DXVK/VKD3D selection, run-in-prefix)
- [ ] Rust port of umu-launcher (currently Python): UMU_ID, protonfixes,
      runtime bootstrap

## Phase 5 — coverage

- [ ] all 744 `dlls/` components, prioritized by Proton game compatibility
- [ ] continuous reference-vs-port comparison in the dev container
