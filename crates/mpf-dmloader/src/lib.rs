//! Port of the portable logic in Wine's `dlls/dmloader/loader.c` and `dlls/dmloader/debug.c`.
//!
//! Ported:
//! - `classes` table and `index_from_class`: [`index_from_class`]
//! - `is_cache_enabled`: [`is_cache_enabled`]
//! - `loader_EnableCache` state transition: [`enable_cache`], [`CacheToggle`]
//! - `get_search_path`: [`get_search_path`]
//! - `IS_VALID_DMFORM`: [`is_valid_dmform`]
//! - `resolve_STREAM_SEEK`: [`resolve_stream_seek`]
//!
//! Skipped:
//! - `find_cache_object`: `DMUS_OBJ_MEMORY` matches by pointer identity and walks a Wine `struct list`.
//! - `loader_GetObject`, `loader_SetObject`, `loader_SetSearchDirectory`, `loader_ScanDirectory`,
//!   `get_default_gm_path`, `get_system_default_gm_path`: registry, file attributes and file I/O.
//! - `DMUSIC_CopyDescriptor`: copies `IStream` references with `IStream_Clone`.
//! - `container.c`, `loaderstream.c`, `dmloader_main.c`: COM vtables, stream I/O, the class factory.
//! - `debugstr_*` helpers: they format pointers and build trace text.

mod debug;

pub use debug::{is_valid_dmform, resolve_stream_seek};

/// `S_OK`.
pub const S_OK: i32 = 0;
/// `S_FALSE`.
pub const S_FALSE: i32 = 1;

/// Number of entries in `classes`, and of `search_paths` in `struct loader`.
pub const CLASS_COUNT: usize = 12;

/// `MAX_PATH`, the buffer size `get_search_path` copies into.
const MAX_PATH: usize = 260;

/// A 128-bit GUID, laid out like the Windows `GUID` structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `classes` in loader.c, in table order. Index 0 is `GUID_DirectMusicAllTypes`, which is also
/// the fallback class.
const CLASSES: [Guid; CLASS_COUNT] = [
    // GUID_DirectMusicAllTypes
    Guid {
        data1: 0xd2ac2893,
        data2: 0xb39b,
        data3: 0x11d1,
        data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
    },
    // CLSID_DirectMusicAudioPathConfig
    Guid {
        data1: 0xee0b9ca0,
        data2: 0xa81e,
        data3: 0x11d3,
        data4: [0x9b, 0xd1, 0x00, 0x80, 0xc7, 0x15, 0x0a, 0x74],
    },
    // CLSID_DirectMusicBand
    Guid {
        data1: 0x79ba9e00,
        data2: 0xb6ee,
        data3: 0x11d1,
        data4: [0x86, 0xbe, 0x00, 0xc0, 0x4f, 0xbf, 0x8f, 0xef],
    },
    // CLSID_DirectMusicContainer
    Guid {
        data1: 0x9301e380,
        data2: 0x1f22,
        data3: 0x11d3,
        data4: [0x82, 0x26, 0xd2, 0xfa, 0x76, 0x25, 0x5d, 0x47],
    },
    // CLSID_DirectMusicCollection
    Guid {
        data1: 0x480ff4b0,
        data2: 0x28b2,
        data3: 0x11d1,
        data4: [0xbe, 0xf7, 0x00, 0xc0, 0x4f, 0xbf, 0x8f, 0xef],
    },
    // CLSID_DirectMusicChordMap
    Guid {
        data1: 0xd2ac288f,
        data2: 0xb39b,
        data3: 0x11d1,
        data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
    },
    // CLSID_DirectMusicSegment
    Guid {
        data1: 0xd2ac2882,
        data2: 0xb39b,
        data3: 0x11d1,
        data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
    },
    // CLSID_DirectMusicScript
    Guid {
        data1: 0x810b5013,
        data2: 0xe88d,
        data3: 0x11d2,
        data4: [0x8b, 0xc1, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xb6],
    },
    // CLSID_DirectMusicSong
    Guid {
        data1: 0xaed5f0a5,
        data2: 0xd972,
        data3: 0x483d,
        data4: [0xa3, 0x84, 0x64, 0x9d, 0xfe, 0xb9, 0xc1, 0x81],
    },
    // CLSID_DirectMusicStyle
    Guid {
        data1: 0xd2ac288a,
        data2: 0xb39b,
        data3: 0x11d1,
        data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
    },
    // CLSID_DirectMusicGraph
    Guid {
        data1: 0xd2ac2884,
        data2: 0xb39b,
        data3: 0x11d1,
        data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
    },
    // CLSID_DirectSoundWave
    Guid {
        data1: 0x8a667154,
        data2: 0xf9cb,
        data3: 0x11d2,
        data4: [0xad, 0x8a, 0x00, 0x60, 0xb0, 0x57, 0x5a, 0xbc],
    },
];

/// `index_from_class`: the position of `class` in `classes`, or `None` for an unknown class
/// (`-1` in C).
pub fn index_from_class(class: &Guid) -> Option<usize> {
    CLASSES.iter().position(|known| known == class)
}

/// `is_cache_enabled`. For an unknown class the C shift is `1 << -1`, which is undefined
/// behaviour. The x86 `shl` that Wine ships masks the count to 5 bits, so an unknown class tests
/// bit 31. `wrapping_shl` reproduces that.
pub fn is_cache_enabled(cache_class: u32, class: &Guid) -> bool {
    let shift = index_from_class(class).map_or(u32::MAX, |index| index as u32);
    (cache_class & 1u32.wrapping_shl(shift)) != 0
}

/// Result of `loader_EnableCache`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CacheToggle {
    /// The new `cache_class` word.
    pub cache_class: u32,
    /// Whether the C code calls `IDirectMusicLoader8_ClearCache` for the class.
    pub clear_cache: bool,
    /// `S_OK` or `S_FALSE`.
    pub hr: i32,
}

/// `loader_EnableCache`. `GUID_DirectMusicAllTypes` sets or clears every bit. An unknown class
/// changes nothing and returns `S_FALSE` without clearing the cache.
pub fn enable_cache(cache_class: u32, class: &Guid, enable: bool) -> CacheToggle {
    let current = is_cache_enabled(cache_class, class);
    let cache_class = if *class == CLASSES[0] {
        if enable {
            u32::MAX
        } else {
            0
        }
    } else {
        let Some(index) = index_from_class(class) else {
            return CacheToggle {
                cache_class,
                clear_cache: false,
                hr: S_FALSE,
            };
        };
        if enable {
            cache_class | (1u32 << index)
        } else {
            cache_class & !(1u32 << index)
        }
    };
    CacheToggle {
        cache_class,
        clear_cache: !enable,
        hr: if current == enable { S_FALSE } else { S_OK },
    }
}

/// `get_search_path`. A class that is unknown or has no path set falls back to
/// `GUID_DirectMusicAllTypes`. The stored path is copied with `lstrcpynW(path, src, MAX_PATH)`,
/// so it stops at the first NUL and at `MAX_PATH - 1` units. An unset fallback gives an empty path. Entries hold the
/// path units without the terminating NUL.
pub fn get_search_path(class: &Guid, search_paths: &[Option<Vec<u16>>; CLASS_COUNT]) -> Vec<u16> {
    let index = match index_from_class(class) {
        Some(index) if search_paths[index].is_some() => index,
        _ => 0,
    };
    search_paths[index]
        .as_deref()
        .unwrap_or_default()
        .iter()
        .take_while(|&&unit| unit != 0)
        .take(MAX_PATH - 1)
        .copied()
        .collect()
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
