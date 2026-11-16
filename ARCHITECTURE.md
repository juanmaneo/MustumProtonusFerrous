# Architecture

## Mapping: reference Wine/Proton -> MPF crates

| Reference `repos/wine` @ wine-11.22 | Reference Proton component | MPF crate |
|-------------------------------------|----------------------------|-----------|
| `loader/`                           | `proton` script            | `mpf-loader` |
| `dlls/ntdll/`                       | —                          | `mpf-ntdll` |
| `dlls/kernel32/`, `dlls/kernelbase/`| —                          | `mpf-kernel32`, `mpf-kernelbase` (planned) |
| `dlls/user32`, `gdi32`, `winex11.drv` | dxvk submodule           | `mpf-user32`, `mpf-gdi32` (planned); dxvk native initially |
| —                                   | vkd3d-proton submodule     | native companion initially, Rust port later |
| `programs/wineboot`                 | `default_pfx.py`           | `mpf-wineboot` (planned) |
| —                                   | umu-launcher (Python)      | `mpf-umu` (planned) |
| `include/winnt.h` etc.              | —                          | `mpf-pe` + `mpf-winnt` (planned) |

## Rules

- Core crates have **zero external dependencies** and build on stable Rust.
- `unsafe` only at the edges: PE loader, FFI to native companions (dxvk /
  vkd3d-proton), and raw Win32 ABI surfaces. Core logic is safe Rust.
- Errors: `NTSTATUS` (i32) inside ntdll-style APIs (`mpf_ntdll::ntstatus`);
  Rust `Result` at the loader/CLI boundary.
- Memory: PE images are mapped by `mpf_loader::map_image` into a flat,
  section-aligned buffer; the process model mirrors ntdll's LDR.
- Tests: unit tests per crate; synthetic PE images from `mpf_pe::testing`;
  conformance against wine's own `dlls/*/tests` arrives in Phase 2.

## Comparison workflow

See [COMPARE.md](COMPARE.md): the dev container mounts `../repos` next to the
project and `cargo xtask compare` (in `xtask/`) diffs a reference directory
against its port crate.
