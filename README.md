> **⚠️ Work in progress.** MPF is in early development (Phase 1: loader & ntdll core). APIs, crate layout, and behavior will change without notice, and it cannot run Windows applications yet. Do not use it for anything that needs to work.

# Mustum Protonus Ferrous (MPF)

A Rust port of Wine (Proton flavour), built incrementally, one component at a time.

- [ROADMAP.md](ROADMAP.md): phases and current progress
- [ARCHITECTURE.md](ARCHITECTURE.md): mapping from Wine/Proton components to MPF crates
- [COMPARE.md](COMPARE.md): comparing the port against reference sources
- [SOURCES.md](SOURCES.md): pinned reference revisions

## Development

```sh
cargo test --workspace
cargo xtask compare dlls/kernel32 mpf-kernel32
cargo xtask coverage
```

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
