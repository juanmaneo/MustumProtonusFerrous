//! Edge-case tests for the ported diasymreader logic.
//!
//! These cover the buffer boundary, the NULL-buffer path, and the UTF-8 encoding at the
//! end of the filename buffer. Expectations marked "not oracle-derived" are worked out
//! by hand or come from std's encoder, because the oracle cannot capture them.

use super::*;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn pdb(units: &[u16]) -> PdbInfo {
    let mut filename = [0u16; MAX_PATH];
    filename[..units.len()].copy_from_slice(units);
    PdbInfo {
        guid: [0; 16],
        age: 1,
        filename,
    }
}

/// Runs `get_debug_info` into a 512-byte buffer prefilled with `0xaa`.
fn record(units: &[u16]) -> (DebugInfo, Vec<u8>) {
    let mut buf = vec![0xaau8; 512];
    let info = get_debug_info(&pdb(units), Some(&mut buf));
    (info, buf)
}

#[test]
fn buffer_size_boundary() {
    let p = pdb(&wide("a.pdb"));

    let mut short = [0xaau8; 29];
    let info = get_debug_info(&p, Some(&mut short));
    assert_eq!(info.status, Err(InsufficientBuffer));
    assert_eq!(info.size, 30);
    assert_eq!(short, [0xaau8; 29]);

    let mut exact = [0xaau8; 30];
    let info = get_debug_info(&p, Some(&mut exact));
    assert_eq!(info.status, Ok(()));
    assert_eq!(&exact[24..], b"a.pdb\0");
}

#[test]
fn none_buffer_reports_size_and_directory_only() {
    let info = get_debug_info(&pdb(&wide("a.pdb")), None);
    assert_eq!(info.status, Ok(()));
    assert_eq!(info.size, 30);
    assert_eq!(
        info.directory,
        ImageDebugDirectory {
            characteristics: 0,
            major_version: 0,
            minor_version: 0,
            kind: IMAGE_DEBUG_TYPE_CODEVIEW,
            size_of_data: 30,
        }
    );
}

#[test]
fn full_buffer_without_nul_is_read_to_the_end() {
    // Not oracle-derived. The C code reads one WCHAR past the array here, so the
    // harness cannot capture it. SymWriter always NUL-terminates pdb_filename, so
    // this only pins down what the port does with an unterminated buffer.
    let units = [0x0061u16; MAX_PATH];
    let (info, buf) = record(&units);
    assert_eq!(info.size, 24 + 260 + 1);
    assert_eq!(&buf[24..284], &[0x61u8; 260][..]);
    assert_eq!(buf[284], 0);
}

#[test]
fn lone_high_surrogate_in_last_slot_is_replaced() {
    // Not oracle-derived, hand-computed. 258 ASCII units, a lone high surrogate at
    // index 258, then the NUL at index 259. U+FFFD takes 3 bytes.
    let mut units = [0x0061u16; MAX_PATH];
    units[258] = 0xd800;
    units[259] = 0;
    let (info, buf) = record(&units);
    assert_eq!(info.size, 24 + 258 + 3 + 1);
    assert_eq!(&buf[24 + 258..24 + 258 + 4], &[0xef, 0xbf, 0xbd, 0x00]);
}

#[test]
fn surrogate_pair_ending_at_last_name_unit() {
    // Not oracle-derived, hand-computed. 257 ASCII units, then U+1F600 as the pair
    // at indices 257 and 258, then the NUL. The pair encodes to 4 bytes.
    let mut units = [0x0061u16; MAX_PATH];
    units[257] = 0xd83d;
    units[258] = 0xde00;
    units[259] = 0;
    let (info, buf) = record(&units);
    assert_eq!(info.size, 24 + 257 + 4 + 1);
    assert_eq!(
        &buf[24 + 257..24 + 257 + 5],
        &[0xf0, 0x9f, 0x98, 0x80, 0x00]
    );
}

#[test]
fn code_point_boundaries_match_std_utf8() {
    // Not oracle-derived. The expected bytes come from std's UTF-8 encoder.
    for ch in [
        '\u{7f}',
        '\u{80}',
        '\u{7ff}',
        '\u{800}',
        '\u{d7ff}',
        '\u{e000}',
        '\u{ffff}',
        '\u{10000}',
        '\u{10ffff}',
    ] {
        let mut units = [0u16; 2];
        let n = ch.encode_utf16(&mut units).len();
        let mut path = units[..n].to_vec();
        path.push(0);
        let (info, buf) = record(&path);

        let mut utf8 = [0u8; 4];
        let want = ch.encode_utf8(&mut utf8);
        assert_eq!(info.size as usize, 24 + want.len() + 1, "{ch:?}");
        assert_eq!(&buf[24..24 + want.len()], want.as_bytes(), "{ch:?}");
        assert_eq!(buf[24 + want.len()], 0, "{ch:?}");
    }
}
