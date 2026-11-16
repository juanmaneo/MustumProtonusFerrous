# Architecture

## Mapping: reference Wine/Proton -> MPF crates

| Reference `repos/wine` @ wine-11.22 | Reference Proton component | MPF crate |
|-------------------------------------|----------------------------|-----------|
| `loader/`                           | `proton` script            | `mpf-loader` (map, relocate, IAT/delay-import resolution, TLS) |
| `dlls/ntdll/`                       | —                          | `mpf-ntdll` (ntstatus, unicode, heap, critical sections, object handles, exception bridge) |
| `dlls/kernel32/`, `dlls/kernelbase/`| —                          | `mpf-kernel32` (strings, process, thread, module, file), `mpf-kernelbase` (planned, not yet created) |
| `dlls/advapi32/`                    | —                          | `mpf-advapi32` (`.reg` hives + Reg* API) |
| `dlls/user32`                       | —                          | `mpf-user32` (headless windowing) |
| `dlls/gdi32`                        | —                          | `mpf-gdi32` (headless DCs/objects) |
| `dlls/winex11.drv`                  | dxvk submodule             | `mpf-graphics` dxvk module (detect + dxvk.conf + DXVK_* env); native dxvk initially |
| —                                   | vkd3d-proton submodule     | `mpf-graphics` vkd3d module (detect + version); native companion initially |
| `programs/wineboot`                 | `default_pfx.py`           | `mpf-wineboot` (prefix init + hives) |
| process startup / run-in-prefix     | `proton` script            | `mpf-proton` (prefix, env, DXVK/VKD3D selection, `run` via `mpf-loader`) |
| `dlls/*/tests`                      | —                          | `mpf-conformance` (seeded conformance cases) |
| —                                   | umu-launcher (Python)      | `mpf-umu` (UMU_ID/GAMEID, prefix + `pfx` symlink, runtime bootstrap, protonfixes, run via `mpf-proton`) |
| `include/winnt.h` etc.              | —                          | `mpf-pe` + `mpf-winnt` (PEB/TEB/LDR/critical-section/exception ABI structs) |
| process startup (`loader/main.c`, `LdrInitializeThunk`) | — | `mpf-cli run` (map → ASLR relocate → resolve imports → PEB/TEB/LDR) |

## Rules

- Core crates have **zero external dependencies** and build on stable Rust.
- `unsafe` only at the edges: PE loader, FFI to native companions (dxvk /
  vkd3d-proton), and raw Win32 ABI surfaces. Core logic is safe Rust.
- Errors: `NTSTATUS` (i32) inside ntdll-style APIs (`mpf_ntdll::ntstatus`);
  Rust `Result` at the loader/CLI boundary.
- Memory: PE images are mapped by `mpf_loader::map_image` into a flat,
  section-aligned buffer, then relocated (`MemoryModule::relocate`,
  HIGHLOW/DIR64) and their IAT/delay-IAT patched by `resolve_imports` /
  `resolve_delay_imports`; the process model mirrors ntdll's LDR
  (`mpf_winnt::PebLdrData` / `LdrDataTableEntry`), with PEB/TEB built by
  `mpf-cli run`.
- Tests: unit tests per crate; synthetic PE images from `mpf_pe::testing`;
  conformance against wine's own `dlls/*/tests` is seeded by `mpf-conformance` (Phase 2).

## Comparison workflow

See [COMPARE.md](COMPARE.md): the dev container mounts `../repos` next to the
project and `cargo xtask compare` (in `xtask/`) diffs a reference directory
against its port crate. The mapping of reference directories to port crates
lives in `coverage/mapping.txt` (status + Proton game-compatibility
priority per component); `cargo xtask compare-all` diffs every mapped
component into `target/compare/report.txt` (`--watch SECONDS` re-runs it
continuously in the container), and `cargo xtask dlls` prints the `dlls/`
inventory classified by the mapping.
