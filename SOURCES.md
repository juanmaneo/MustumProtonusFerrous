# Reference sources (cloned into `../repos`)

| Repo | URL | Pinned at | Notes |
|------|-----|-----------|-------|
| Wine | https://gitlab.winehq.org/wine/wine | tag `wine-11.22` (9f8c4ae) | main port target |
| Valve Proton | https://github.com/ValveSoftware/Proton | HEAD 1f089f5 | "Proton flavour": patches, dxvk/vkd3d-proton submodules, `proton` script |
| DXVK | https://github.com/doitsujin/dxvk | HEAD 71bf4ba | D3D8/9/10/11 -> Vulkan |
| vkd3d-proton | https://github.com/HansKristian-Work/vkd3d-proton | HEAD 5b1089c | D3D12 -> Vulkan |
| umu-launcher | https://github.com/Open-Wine-Components/umu-launcher | HEAD e2b203a | Python launcher for games/tools in a UMU prefix |

Release pages: https://gitlab.winehq.org/wine/wine/-/releases/wine-11.22
(mirror: https://www.winehq.org/)

## wine-11.22 highlights (from `ANNOUNCE.md` at the tag)

- Locale data updated to Unicode CLDR 49
- AVX support in x86 emulation on ARM64
- Address space randomization
- Improved glyph metrics in Tahoma font
- DTD validation reimplemented in MSXML
- 34 bugs fixed (Tahoma Bold, Guild Wars 2, Overwatch 2 SAPI, ...)

## Proton notes

Proton carries Wine patches on top of upstream plus git submodules for dxvk,
vkd3d-proton, dxvk-nvapi, ffmpeg, gstreamer, etc., and the `proton` python
script that drives prefix setup and env (DXVK_*, PROTON_*, WINEESYNC...). The
MPF Phase 4 target is a Rust equivalent of that script plus a Rust umu port.
