# Roadmap

Port strategy: incremental, per-component, with Wine's own test suite as the
conformance target. We do not mechanically translate C; each component is
re-implemented in safe Rust with `unsafe` confined to FFI/loader edges, and the
reference tree (`../repos/wine`, tag `wine-11.22`) stays mounted side-by-side
in the dev container for direct comparison.

## Phase 0 — scaffold (done)

- [x] workspace + crates: mpf-pe, mpf-ntdll, mpf-kernel32, mpf-loader, mpf-cli
- [x] PE/COFF parser with synthetic-image tests
- [x] RTL unicode strings + NTSTATUS
- [x] PE mapper (headers + sections into a flat image)
- [x] podman/docker dev container + `cargo xtask compare`

## Phase 1 — loader & ntdll core (done)

- [x] base relocations (`mpf_loader::MemoryModule::relocate`, HIGHLOW/DIR64),
      import (IAT) resolution, TLS directory, delay imports (`mpf-pe` parsers
      + `mpf-cli run` pipeline with ASLR base selection)
- [x] LDR module list, PEB/TEB structures (`mpf-winnt`, offsets pinned by tests)
- [x] ntdll: RtlAllocateHeap/RtlFreeHeap (`heap`), critical sections
      (`critical_section`), full RTL unicode string API (`unicode`), object
      handle table (`object`), exception handling design (`exception::guard`
      = SEH <-> Rust panic boundary)
- [x] process init equivalent of `programs/wineboot` (`mpf-wineboot::init_prefix`)
      + `loader/main.c` (`mpf-cli run`: map -> relocate -> resolve -> PEB/TEB/LDR)

## Phase 2 — first runnable Win32 app (done)

- [x] kernel32 subset: process (`GetCurrentProcess[Id]`), threads
      (`CreateThread`/`WaitForSingleObject`, thread ids), modules
      (`GetModuleHandleW`/`GetModuleFileNameW`/`GetProcAddress` over mapped
      images), files (`CreateFileW`/`ReadFile`/`WriteFile`/`CloseHandle`)
- [x] registry: `mpf-advapi32` parses/formats wine `.reg` hives + Reg* key
      API; `mpf-wineboot` writes real `system.reg`/`user.reg` hives
      (`drive_c` prefix tree)
- [x] conformance harness `mpf-conformance`: `cargo run -p mpf-conformance`
      runs seeded cases derived from wine's `dlls/*/tests` expectations

## Phase 3 — graphics stack (done)

- [x] `mpf-user32`: headless windowing (class atoms, HWNDs, MessageBoxW,
      GetSystemMetrics, GetClientRect, DestroyWindow)
- [x] `mpf-gdi32`: headless DCs and GDI objects (GetDeviceCaps fixed
      1920x1080x32@96dpi, CreateCompatibleDC/CreateBitmap/SelectObject/DeleteObject)
- [x] `mpf-graphics`: dxvk companion detection in the prefix + dxvk.conf
      parse/format + DXVK_* env mapping; vkd3d-proton detection + changelog
      version. The D3D cores stay native companions (exactly what Proton
      ships); Rust ports come later

## Phase 4 — Proton flavour & umu (done)

- [x] `mpf-proton`: Rust equivalent of Proton's `proton` script — prefix
      setup, env vars (WINE*/PROTON_*/DXVK_*), DXVK/VKD3D companion
      selection, ASLR base, run-in-prefix (`mpf-proton run` → loader report)
- [x] `mpf-umu`: Rust port of umu-launcher — UMU_ID/GAMEID (umu-<id> →
      STEAM_COMPAT_APP_ID), `$HOME/Games/umu/<id>` prefix with `pfx`
      self-symlink, Steam Runtime bootstrap (sniper/soldier/steamrt4, no
      downloads), protonfixes (winetricks with WINEPREFIX/GAMEID), then
      run-in-prefix via `mpf-proton`

## Phase 5 — coverage (current)

- [x] the inventory: `coverage/mapping.txt` maps every tracked reference
      component to its port crate with a status; `cargo xtask dlls` prints
      the whole `dlls/` inventory classified by it
- [x] continuous reference-vs-port comparison in the dev container:
      `cargo xtask compare-all [--watch SECONDS]` diffs every mapped
      component into `target/compare/report.txt`, re-running on a timer
      with `--watch`
- [x] first port: `dlls/shlwapi` path helpers as `mpf-shlwapi`
      (string-only subset, conformance case seeded from `dlls/shlwapi/tests`)
- [x] `cargo xtask coverage` gate: every file of every crate at 100% of
      regions, lines and functions
- [ ] 505 of 744 `dlls/` components have a port crate (489 partial, 16
      fully ported); 8 are marked `native` (not a final status) and 231 are not ported yet
- [ ] all 744 `dlls/` components ported, each with conformance cases
      seeded from its `dlls/<dll>/tests`

Definition of done for Phase 5: every `dlls/` component is ported (no
permanent skip or `out-of-scope` status), each port has conformance cases,
and all gates are green. Components skipped during triage stay open until
ported. Alias and forwarder DLLs (for example `d3dx9_24` or `msvcr70`) also
get their own port crate, each with a conformance case that proves the alias
resolves to the ported implementation. `native` is not a final status for
them.

## Phase 6 — game acceptance (planned)

- [ ] Phase 5 done, and the port runs real Windows games and applications
      with results comparable to upstream Wine, verified per title
