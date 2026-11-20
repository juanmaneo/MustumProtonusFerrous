//! Rows from the Wine oracle: the verbatim `isDigit` and `parse_runtime_version`
//! from `dlls/mscoree/metahost.c`, built with `gcc -fsanitize=address,undefined`
//! and run on the inputs below. The oracle ran clean under both sanitizers for
//! all 22 inputs, so no vectors were excluded. For a `0` result, the C driver also
//! prints partial components that the Rust API does not return, so only the
//! accept or reject decision is compared.

use crate::{parse_runtime_version, RuntimeVersion};

/// `(input, expected)`. `None` is the oracle's `0` result.
type Row = (&'static str, Option<(u32, u32, u32)>);
const ROWS: &[Row] = &[
    ("v1", Some((1, 0, 0))),
    ("V1", Some((1, 0, 0))),
    ("v0", Some((0, 0, 0))),
    ("v12.34", Some((12, 34, 0))),
    ("V12.34", Some((12, 34, 0))),
    ("v1.2.3", Some((1, 2, 3))),
    ("V1.2.3", Some((1, 2, 3))),
    ("v0007.0008.0009", Some((7, 8, 9))),
    ("v4294967295", Some((4_294_967_295, 0, 0))),
    ("v4294967296", Some((0, 0, 0))),
    ("v1.4294967296.3", Some((1, 0, 3))),
    ("v1.2.4294967296", Some((1, 2, 0))),
    ("", None),
    ("1.2.3", None),
    ("x1.2.3", None),
    ("v", None),
    ("v.1", None),
    ("v1.", None),
    ("v1..2", None),
    ("v1.2.3.4", None),
    ("v1.2", Some((1, 2, 0))),
    ("v1x", None),
];

fn wide(input: &str) -> Vec<u16> {
    input.encode_utf16().chain(std::iter::once(0)).collect()
}

#[test]
fn matches_wine_oracle_rows() {
    assert_eq!(ROWS.len(), 22);
    for &(input, expected) in ROWS {
        let actual = parse_runtime_version(&wide(input));
        let expected = expected.map(|(major, minor, build)| RuntimeVersion {
            major,
            minor,
            build,
        });
        assert_eq!(actual, expected, "input {input:?}");
    }
}
