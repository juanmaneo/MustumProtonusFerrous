use crate::{
    apply_prosody,
    ProsodyAttribute::{Pitch, Rate, Volume},
};

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// Rows from `target/canon/sapi/output.txt`, the verbatim Wine `xml.c`
/// prosody conversion built with `gcc -fsanitize=address,undefined -g`.
/// Clean under both sanitizers; no sanitizer-flagged vectors were excluded.
#[test]
fn matches_wine_oracle_rows() {
    let rows = [
        (Rate, "x-slow", 0, -9),
        (Rate, "slow", 0, -4),
        (Rate, "medium", 0, 0),
        (Rate, "fast", 0, 4),
        (Rate, "x-fast", 0, 9),
        (Rate, "50%", 0, 4),
        (Rate, "+50%", 0, 4),
        (Rate, "6", 0, 16),
        (Rate, "0.01000001", 0, -42),
        (Rate, "0.01", 0, -10),
        (Rate, "0", 0, -10),
        (Rate, "-1.0", 0, 0),
        (Volume, "silent", 100, 0),
        (Volume, "50%", 100, 100),
        (Volume, "-50%", 100, 50),
        (Volume, "10", 60, 10),
        (Volume, "+10", 60, 70),
        (Volume, "-10.1", 100, 90),
        (Pitch, "300Hz", 0, 0),
        (Pitch, "41.4%", 0, 12),
        (Pitch, "-50%", 0, -24),
        (Pitch, "-98.99999%", 0, -159),
        (Pitch, "-99%", 0, -10),
        (Pitch, "-101%", 0, -10),
    ];
    for (attribute, text, current, expected) in rows {
        assert_eq!(
            apply_prosody(attribute, &wide(text), current),
            Ok(expected),
            "{text}"
        );
    }
}
