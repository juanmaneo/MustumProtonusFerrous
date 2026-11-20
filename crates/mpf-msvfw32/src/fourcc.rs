//! FOURCC comparison: `compare_fourcc` from `msvideo_main.c`.

/// `compare_fourcc` reduced to its equality test.
///
/// Wine splits the FOURCC into its bytes, low byte first, and compares them with msvcrt
/// `_strnicmp`. That lowercases only ASCII `A`-`Z`, and it stops at the first position where both
/// bytes are NUL, which counts as equal there. Bytes after that position are not compared.
pub fn fourcc_eq_ignore_case(a: u32, b: u32) -> bool {
    let (a, b) = (a.to_le_bytes(), b.to_le_bytes());
    for (x, y) in a.into_iter().zip(b) {
        let (x, y) = (x.to_ascii_lowercase(), y.to_ascii_lowercase());
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
    }
    true
}
