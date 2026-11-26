//! Oracle-derived test vectors for the ported atlthunk logic.
//!
//! Vectors were captured by compiling the verbatim `alloc_thunk_pool()` loop
//! bodies from `dlls/atlthunk/atlthunk.c` with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/atlthunk/` (harness.c, run once). The harness allocates
//! real pool structs and prints each thunk's address, the addresses of its
//! `arg` and `proc` fields, and the bytes it wrote. 18 rows were printed (6 per
//! architecture); ASan and UBSan flagged none, so 0 rows were excluded.
//!
//! The `pool sizes` line of the same run is checked by `pool_sizes_match_oracle`.

use super::*;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn pool_sizes_match_oracle() {
    // pool sizes x86_64=315 i386=273 aarch64=256
    assert_eq!(pool_capacity(13), 315);
    assert_eq!(pool_capacity(15), 273);
    assert_eq!(pool_capacity(16), 256);
}

#[test]
fn oracle_x86_64_rows() {
    let rows: [(u64, u64, u64, &str); 6] = [
        (
            0x7dca_aa9e_0100,
            0x7dca_aa9e_1118,
            0x7dca_aa9e_1110,
            "488b0d11100000ff2503100000",
        ),
        (
            0x7dca_aa9e_010d,
            0x7dca_aa9e_1130,
            0x7dca_aa9e_1128,
            "488b0d1c100000ff250e100000",
        ),
        (
            0x7dca_aa9e_011a,
            0x7dca_aa9e_1148,
            0x7dca_aa9e_1140,
            "488b0d27100000ff2519100000",
        ),
        (
            0x7dca_aa9e_015b,
            0x7dca_aa9e_11c0,
            0x7dca_aa9e_11b8,
            "488b0d5e100000ff2550100000",
        ),
        (
            0x7dca_aa9e_0614,
            0x7dca_aa9e_1a78,
            0x7dca_aa9e_1a70,
            "488b0d5d140000ff254f140000",
        ),
        (
            0x7dca_aa9e_10f2,
            0x7dca_aa9e_2e88,
            0x7dca_aa9e_2e80,
            "488b0d8f1d0000ff25811d0000",
        ),
    ];
    for (thunk, arg, proc_, expected) in rows {
        assert_eq!(hex(&encode_x86_64(thunk, arg, proc_)), expected);
    }
}

#[test]
fn oracle_i386_rows() {
    let rows: [(u64, u64, &str); 6] = [
        (
            0x7dca_aa9e_4118,
            0x7dca_aa9e_4110,
            "a118419eaa89442404ff2510419eaa",
        ),
        (
            0x7dca_aa9e_4130,
            0x7dca_aa9e_4128,
            "a130419eaa89442404ff2528419eaa",
        ),
        (
            0x7dca_aa9e_4148,
            0x7dca_aa9e_4140,
            "a148419eaa89442404ff2540419eaa",
        ),
        (
            0x7dca_aa9e_41c0,
            0x7dca_aa9e_41b8,
            "a1c0419eaa89442404ff25b8419eaa",
        ),
        (
            0x7dca_aa9e_4a78,
            0x7dca_aa9e_4a70,
            "a1784a9eaa89442404ff25704a9eaa",
        ),
        (
            0x7dca_aa9e_5a98,
            0x7dca_aa9e_5a90,
            "a1985a9eaa89442404ff25905a9eaa",
        ),
    ];
    for (arg, proc_, expected) in rows {
        assert_eq!(hex(&encode_i386(arg, proc_)), expected);
    }
}

#[test]
fn oracle_aarch64_rows() {
    let rows: [(u64, u64, u64, &str); 6] = [
        (
            0x7dca_aa9e_6100,
            0x7dca_aa9e_7118,
            0x7dca_aa9e_7110,
            "c08000587080005800021fd600000000",
        ),
        (
            0x7dca_aa9e_6110,
            0x7dca_aa9e_7130,
            0x7dca_aa9e_7128,
            "00810058b080005800021fd600000000",
        ),
        (
            0x7dca_aa9e_6120,
            0x7dca_aa9e_7148,
            0x7dca_aa9e_7140,
            "40810058f080005800021fd600000000",
        ),
        (
            0x7dca_aa9e_6170,
            0x7dca_aa9e_71c0,
            0x7dca_aa9e_71b8,
            "808200583082005800021fd600000000",
        ),
        (
            0x7dca_aa9e_6740,
            0x7dca_aa9e_7a78,
            0x7dca_aa9e_7a70,
            "c09900587099005800021fd600000000",
        ),
        (
            0x7dca_aa9e_70f0,
            0x7dca_aa9e_8900,
            0x7dca_aa9e_88f8,
            "80c0005830c0005800021fd600000000",
        ),
    ];
    for (thunk, arg, proc_, expected) in rows {
        assert_eq!(hex(&encode_aarch64(thunk, arg, proc_)), expected);
    }
}
