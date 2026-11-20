//! Tests against Wine's oracle vectors.
//!
//! The vectors are replies of Wine's verbatim `cap20wxx.c` logic, built with ASan and UBSan by
//! `target/canon/capi2032/oracle.c`. The C logic is:
//!
//! ```c
//! if (!strncmp (SzBuffer, "AVM", 3)) {
//!     strcpy (SzBuffer, "AVM-GmbH");
//! }
//! ```

use crate::normalize_manufacturer;

#[rustfmt::skip]
const VECTORS: &[(&str, &[u8], bool, &[u8])] = &[
    // (name, before, expected_replaced, after)
    ("avm_short", b"AVM", false, b"AVM"),
    ("avm_nul", b"AVM\0", false, b"AVM\0"),
    ("avm_plus", b"AVM Xtra\0", true, b"AVM-GmbH\0"),
    ("avm_full", b"AVM-GmbH\0", true, b"AVM-GmbH\0"),
    ("avm_partial", b"AVMX", false, b"AVMX"),
    ("not_avm", b"BVM", false, b"BVM"),
    ("empty", b"", false, b""),
    ("av", b"AV", false, b"AV"),
    ("a", b"A", false, b"A"),
    ("avm_lowercase", b"avm", false, b"avm"),
    ("avm_with_nul_trailing", b"AVM\0\0\0\0\0\0\0\0\0", true, b"AVM-GmbH\0\0\0\0"),
    ("long_avm", b"AVM-GmbH-extr", true, b"AVM-GmbH\0extr"),
];

#[test]
fn oracle_vectors_match_wine() {
    for (name, before, expected_replaced, after) in VECTORS {
        let mut buffer = before.to_vec();
        let replaced = normalize_manufacturer(&mut buffer);
        assert_eq!(replaced, *expected_replaced, "case={name}: replaced");
        assert_eq!(&buffer[..], *after, "case={name}: after");
    }
}
