//! Oracle-derived test vectors for the ported dmloader logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/dmloader/loader.c`
//! (`classes`, `index_from_class`, `is_cache_enabled`, `loader_EnableCache`, `get_search_path`)
//! and `dlls/dmloader/debug.c` (`IS_VALID_DMFORM`, `resolve_STREAM_SEEK`), with the header
//! constants they use, under `gcc -fsanitize=address,undefined -g` in `target/canon/dmloader/`.
//! The harness printed 23 rows. Two of them, `is_cache_enabled` and `enable_cache` for
//! `CLSID_DirectMusicSegmentState` (an unknown class, so `1 << -1`), are flagged by UBSan as a
//! negative shift exponent and are excluded. That leaves 21 rows. ASan reported nothing.
//! `get_search_path` rows compare the output length and an FNV-1a hash of its units.

use super::*;

/// `CLSID_DirectMusicSegmentState`: not in `classes`.
const SEGMENT_STATE: Guid = Guid {
    data1: 0xd2ac2883,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

/// FNV-1a over 16-bit units, as printed by the harness.
fn fnv1a(units: &[u16]) -> u32 {
    units.iter().fold(0x811c_9dc5u32, |h, &unit| {
        (h ^ u32::from(unit)).wrapping_mul(0x0100_0193)
    })
}

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// Builds the `search_paths` array from `(class index, path)` pairs.
fn paths(entries: &[(usize, Vec<u16>)]) -> [Option<Vec<u16>>; CLASS_COUNT] {
    let mut out: [Option<Vec<u16>>; CLASS_COUNT] = Default::default();
    for (index, path) in entries {
        out[*index] = Some(path.clone());
    }
    out
}

#[test]
fn oracle_is_valid_dmform_rows() {
    // DMUS_FOURCC_AUDIOPATH_FORM "DMAP", FOURCC_DLS "DLS ", "RIFF"
    let rows = [(0x50414d44, true), (0x20534c44, true), (0x46464952, false)];
    for (fourcc, expected) in rows {
        assert_eq!(is_valid_dmform(fourcc), expected, "fourcc {fourcc:#010x}");
    }
}

#[test]
fn oracle_resolve_stream_seek_rows() {
    let rows = [(0, "STREAM_SEEK_SET"), (2, "STREAM_SEEK_END"), (3, "()")];
    for (flag, expected) in rows {
        assert_eq!(resolve_stream_seek(flag), expected, "flag {flag}");
    }
}

#[test]
fn oracle_index_from_class_rows() {
    // GUID_DirectMusicAllTypes, CLSID_DirectMusicBand, CLSID_DirectMusicSegmentState
    assert_eq!(index_from_class(&CLASSES[0]), Some(0));
    assert_eq!(index_from_class(&CLASSES[2]), Some(2));
    assert_eq!(index_from_class(&SEGMENT_STATE), None);
}

#[test]
fn oracle_is_cache_enabled_rows() {
    // cache 0xffffffff and 0xfffffffb, CLSID_DirectMusicBand
    assert!(is_cache_enabled(0xffffffff, &CLASSES[2]));
    assert!(!is_cache_enabled(0xfffffffb, &CLASSES[2]));
}

#[test]
fn oracle_enable_cache_rows() {
    // (cache in, class, enable) -> (cache out, clear_cache, hr)
    let rows = [
        (0xffffffff, CLASSES[0], false, 0x0, true, S_OK),
        (0x0, CLASSES[0], false, 0x0, true, S_FALSE),
        (0x0, CLASSES[2], true, 0x4, false, S_OK),
        (0x4, CLASSES[2], true, 0x4, false, S_FALSE),
        (0x4, CLASSES[2], false, 0x0, true, S_OK),
    ];
    for (cache, class, enable, cache_out, clear, hr) in rows {
        assert_eq!(
            enable_cache(cache, &class, enable),
            CacheToggle {
                cache_class: cache_out,
                clear_cache: clear,
                hr,
            },
            "cache {cache:#010x} enable {enable}"
        );
    }
}

#[test]
fn oracle_get_search_path_rows() {
    // (search_paths, class, output length, output FNV-1a)
    let rows = [
        (paths(&[(0, units("C:\\Music"))]), CLASSES[0], 8, 0xdc9d00b7),
        (paths(&[(0, units("D:\\All"))]), CLASSES[2], 6, 0x4324a734),
        (paths(&[]), SEGMENT_STATE, 0, 0x811c9dc5),
        (
            paths(&[(
                6,
                (0..300).map(|i| u16::from(b'a' + (i % 26) as u8)).collect(),
            )]),
            CLASSES[6],
            259,
            0x8b7e1481,
        ),
        (
            paths(&[(8, vec![0x61, 0x62, 0x00, 0x63, 0x64])]),
            CLASSES[8],
            2,
            0x4d2505ca,
        ),
    ];
    for (search_paths, class, len, hash) in rows {
        let out = get_search_path(&class, &search_paths);
        assert_eq!((out.len(), fnv1a(&out)), (len, hash));
    }
}
