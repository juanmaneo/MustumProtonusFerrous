//! Oracle-derived test vectors for the ported cryptnet functions.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/cryptnet/` and
//! running the harness clean under both sanitizers. Sanitizer-flagged vectors
//! were excluded (none were flagged).

use super::*;

struct OidVector {
    name: &'static str,
    oid: Oid<'static>,
    expected: &'static str,
}

struct EscapeVector {
    name: &'static str,
    src: &'static [u16],
    expected: &'static str,
    expected_len: usize,
}

struct StatusVector {
    name: &'static str,
    status: u32,
    expected: u32,
}

/// url_oid_to_str vectors from the verbatim C oracle: one row per switch arm
/// plus the decimal default arm and a string OID.
const OID_VECTORS: [OidVector; 10] = [
    OidVector {
        name: "int1",
        oid: Oid::Int(1),
        expected: "URL_OID_CERTIFICATE_ISSUER",
    },
    OidVector {
        name: "int2",
        oid: Oid::Int(2),
        expected: "URL_OID_CERTIFICATE_CRL_DIST_POINT",
    },
    OidVector {
        name: "int3",
        oid: Oid::Int(3),
        expected: "URL_OID_CTL_ISSUER",
    },
    OidVector {
        name: "int4",
        oid: Oid::Int(4),
        expected: "URL_OID_CTL_NEXT_UPDATE",
    },
    OidVector {
        name: "int5",
        oid: Oid::Int(5),
        expected: "URL_OID_CRL_ISSUER",
    },
    OidVector {
        name: "int6",
        oid: Oid::Int(6),
        expected: "URL_OID_CERTIFICATE_FRESHEST_CRL",
    },
    OidVector {
        name: "int7",
        oid: Oid::Int(7),
        expected: "URL_OID_CRL_FRESHEST_CRL",
    },
    OidVector {
        name: "int8",
        oid: Oid::Int(8),
        expected: "URL_OID_CROSS_CERT_DIST_POINT",
    },
    OidVector {
        name: "int9",
        oid: Oid::Int(9),
        expected: "9",
    },
    OidVector {
        name: "str_hello",
        oid: Oid::Str("hello"),
        expected: "hello",
    },
];

/// escape_path vectors from the verbatim C oracle.
///
/// dst_len is the total output length (src_len + 2 * number_of_escaped_chars),
/// matching the C behaviour where `*dst_len` starts at `src_len` and +=2 per escaped char.
/// The returned String contains all output WCHARs including non-escaped ones.
/// The oracle's dst_len column confirms every row; its dst column is garbled
/// (`%s` cannot print WCHAR), so the string contents follow from the C code.
const ESCAPE_VECTORS: [EscapeVector; 6] = [
    EscapeVector {
        name: "abc",
        src: &[0x61, 0x62, 0x63],
        expected: "abc",
        expected_len: 3,
    },
    EscapeVector {
        name: "a+b",
        src: &[0x61, 0x2b, 0x62],
        expected: "a%2Bb",
        expected_len: 5,
    },
    EscapeVector {
        name: "a/b",
        src: &[0x61, 0x2f, 0x62],
        expected: "a%2Fb",
        expected_len: 5,
    },
    EscapeVector {
        name: "a=b",
        src: &[0x61, 0x3d, 0x62],
        expected: "a%3Db",
        expected_len: 5,
    },
    EscapeVector {
        name: "a+b/c=d",
        src: &[0x61, 0x2b, 0x62, 0x2f, 0x63, 0x3d, 0x64],
        expected: "a%2Bb%2Fc%3Dd",
        expected_len: 13,
    },
    EscapeVector {
        name: "empty",
        src: &[],
        expected: "",
        expected_len: 0,
    },
];

/// map_ocsp_status vectors from the verbatim C oracle: one row per switch arm
/// plus the default arm (the oracle also confirms 4, 255 and u32::MAX).
const STATUS_VECTORS: [StatusVector; 4] = [
    StatusVector {
        name: "good",
        status: 0,
        expected: 0,
    },
    StatusVector {
        name: "revoked",
        status: 1,
        expected: 0x8009_2010,
    },
    StatusVector {
        name: "unknown",
        status: 2,
        expected: 0x8009_2013,
    },
    StatusVector {
        name: "default_3",
        status: 3,
        expected: 0x8009_2013,
    },
];

#[test]
fn url_oid_to_str_matches_oracle() {
    for v in &OID_VECTORS {
        let got = url_oid_to_str(v.oid.clone());
        assert_eq!(got, v.expected, "{}", v.name);
    }
}

#[test]
fn escape_path_matches_oracle() {
    for v in &ESCAPE_VECTORS {
        let (got, len) = escape_path(v.src);
        assert_eq!(got, v.expected, "{}", v.name);
        assert_eq!(len, v.expected_len, "{}", v.name);
    }
}

#[test]
fn map_ocsp_status_matches_oracle() {
    for v in &STATUS_VECTORS {
        let got = map_ocsp_status(v.status);
        assert_eq!(got, v.expected, "{}", v.name);
    }
}
