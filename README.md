> **⚠️ Work in progress.** MPF is in early development and cannot run Windows applications yet. Phases 0–4 are done: the PE loader, a subset of ntdll, kernel32 and advapi32, headless user32 and gdi32, the dxvk/vkd3d configuration layer, and Rust versions of Proton's `proton` script and umu-launcher (`mpf-proton`, `mpf-umu`). Phase 5 is in progress: 474 of the 744 `dlls/` components have a port crate (460 partial, 14 fully ported), 8 are still marked native, and the other 262 are not ported yet (see `coverage/mapping.txt`). APIs, crate layout, and behavior will change without notice. Do not use it for anything that needs to work.

# Mustum Protonus Ferrous (MPF)

The full name is a joke. **Mustum** is Latin for young wine (must), so the project is the younger Wine. **Protonus** gives it a Latin ending to match Valve's Proton, the flavour it follows. **Ferrous** (iron) is for the Rust implementation. In short, the project is just called Mustum, since it is the younger Wine.

A Rust port of Wine (Proton flavour), built incrementally, one component at a time.

- [ROADMAP.md](ROADMAP.md): phases and current progress
- [ARCHITECTURE.md](ARCHITECTURE.md): mapping from Wine/Proton components to MPF crates
- [COMPARE.md](COMPARE.md): comparing the port against reference sources
- [SOURCES.md](SOURCES.md): pinned reference revisions

## Development

```sh
cargo test --workspace
cargo xtask compare dlls/kernel32 mpf-kernel32   # diff one reference dir vs its port
cargo xtask compare-all [--watch SECONDS]         # every mapped component, report in target/compare/report.txt
cargo xtask dlls                                  # the reference dlls/ inventory with port status
cargo xtask coverage
```

`cargo xtask coverage` needs [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) and fails unless **every file** is at 100% of regions, lines and functions, naming each file and metric that falls short.

## License

MPF is licensed under the GNU Lesser General Public License, version 2.1 or (at your option) any later version (`LGPL-2.1-or-later`). See [LICENSE](LICENSE).

## Acknowledgments

MPF is a port of, and builds on, the work of many others:

- **[Wine](https://www.winehq.org/)**: the main porting target. Wine is licensed under LGPL-2.1-or-later. Copyright the Wine project authors.
- **[Valve Proton](https://github.com/ValveSoftware/Proton)**: the Proton flavour, its `proton` script, and its patches on top of Wine. Copyright Valve Corporation. Proton is distributed under several licenses; see its `LICENSE` files.
- **[DXVK](https://github.com/doitsujin/dxvk)**: Direct3D 8/9/10/11 on Vulkan. Licensed under the zlib license. Copyright Philip Rebohle, Joshua Ashton, Robin Kertels, and Jeffrey Ellison.
- **[vkd3d-proton](https://github.com/HansKristian-Work/vkd3d-proton)**: Direct3D 12 on Vulkan. Licensed under LGPL-2.1.
- **[umu-launcher](https://github.com/Open-Wine-Components/umu-launcher)**: the Python launcher for games and tools in a UMU prefix. Licensed under GPL-3.0.

Thanks to everyone who contributed to these projects. MPF does not claim any rights to their work, and it follows the reference revisions listed in [SOURCES.md](SOURCES.md).
