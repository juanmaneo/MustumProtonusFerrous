# Architecture

## Mapping: reference Wine/Proton -> MPF crates

| Reference `repos/wine` @ wine-11.22 | Reference Proton component | MPF crate |
|-------------------------------------|----------------------------|-----------|
| `loader/`                           | `proton` script            | `mpf-loader` (map, relocate, IAT/delay-import resolution, TLS) |
| `dlls/ntdll/`                       | —                          | `mpf-ntdll` (ntstatus, unicode, heap, critical sections, object handles, exception bridge) |
| `dlls/kernel32/`, `dlls/kernelbase/`| —                          | `mpf-kernel32` (strings, process, thread, module, file), `mpf-kernelbase` (planned) |
| `dlls/advapi32/`                    | —                          | `mpf-advapi32` (`.reg` hives + Reg* API) |
| `dlls/user32`, `gdi32`, `winex11.drv` | dxvk submodule           | `mpf-user32`, `mpf-gdi32` (planned); dxvk native initially |
| —                                   | vkd3d-proton submodule     | native companion initially, Rust port later |
| `programs/wineboot`                 | `default_pfx.py`           | `mpf-wineboot` (prefix init + hives) |
| `dlls/*/tests`                      | —                          | `mpf-conformance` (seeded conformance cases) |
| —                                   | umu-launcher (Python)      | `mpf-umu` (planned) |
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
  conformance against wine's own `dlls/*/tests` arrives in Phase 2.

## Comparison workflow

See [COMPARE.md](COMPARE.md): the dev container mounts `../repos` next to the
project and `cargo xtask compare` (in `xtask/`) diffs a reference directory
against its port crate.
