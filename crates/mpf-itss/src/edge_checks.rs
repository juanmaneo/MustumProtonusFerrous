//! Edge tests for the LZX port: the paths the oracle rows do not reach. Expected values are
//! worked out from lzx.c, or come from the harness run that checked the two rows below.

use crate::bits::Bits;
use crate::e8::undo_e8;
use crate::huffman::{read_lengths, Table};
use crate::lzx::copy_match;
use crate::{DecrError, Lzx};

/// Turns a string of bits into the little-endian 16-bit words that LZX reads, most significant
/// bit first. Characters other than 0 and 1 are skipped, and the last word is padded with zeros.
fn stream(bits: &str) -> Vec<u8> {
    let mut digits: Vec<u16> = bits
        .chars()
        .filter_map(|c| c.to_digit(2))
        .map(|d| d as u16)
        .collect();
    digits.resize(digits.len().next_multiple_of(16), 0);
    digits
        .chunks(16)
        .flat_map(|word| {
            word.iter()
                .fold(0u16, |acc, &bit| (acc << 1) | bit)
                .to_le_bytes()
        })
        .collect()
}

fn r_values(r: [u32; 3]) -> Vec<u8> {
    r.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A table of `symbols` symbols and `bits` bits, with the code lengths `lens` built into it.
fn built(symbols: usize, bits: u32, lens: &[u8]) -> Result<Table, DecrError> {
    let mut table = Table::new(symbols, bits);
    table.lens[..lens.len()].copy_from_slice(lens);
    table.build()?;
    Ok(table)
}

/// Runs undo_e8 on a copy of `out` and returns the copy with the new position.
fn undo(out: &[u8], mut curpos: i32, filesize: i32, started: bool) -> (Vec<u8>, i32) {
    let mut out = out.to_vec();
    undo_e8(&mut out, &mut curpos, filesize, started);
    (out, curpos)
}

#[test]
fn window_must_be_a_power_of_two_in_range() {
    for window in [0, 1 << 14, (1 << 15) + 1, 1 << 22] {
        assert!(Lzx::new(window).is_none(), "window {window}");
    }
    for window in [1 << 15, 1 << 21] {
        assert!(Lzx::new(window).is_some(), "window {window}");
    }
}

#[test]
fn error_codes_and_traits() {
    let e = DecrError::DataFormat;
    assert_eq!(e.code(), 1);
    assert_eq!(DecrError::IllegalData.code(), 2);
    assert_eq!(Clone::clone(&e), e);
    assert_ne!(e, DecrError::IllegalData);
    assert_eq!(format!("{e:?}"), "DataFormat");
}

#[test]
fn outlen_past_the_window_is_illegal() {
    let mut lzx = Lzx::new(1 << 15).unwrap();
    assert_eq!(
        lzx.decompress(&[], (1 << 15) + 1),
        Err(DecrError::IllegalData)
    );
}

#[test]
fn input_cut_short_is_illegal() {
    // A verbatim block of one byte, with its trees missing from the input.
    let mut lzx = Lzx::new(1 << 15).unwrap();
    let block_len = 1;
    let input = stream(&format!("0 001 {block_len:024b}"));
    assert_eq!(lzx.decompress(&input, 1), Err(DecrError::IllegalData));
}

#[test]
fn unknown_block_type_stays_illegal() {
    // The bad block type leaves the block length set, as in lzx.c, so the next call fails in
    // the decode loop rather than in the header.
    let mut lzx = Lzx::new(1 << 15).unwrap();
    let block_len = 1;
    let input = stream(&format!("0 000 {block_len:024b}"));
    assert_eq!(lzx.decompress(&input, 1), Err(DecrError::IllegalData));
    assert_eq!(lzx.decompress(&[0; 4], 1), Err(DecrError::IllegalData));
}

#[test]
fn uncompressed_block_past_the_window_end_is_a_data_error() {
    let mut lzx = Lzx::new(1 << 15).unwrap();
    let stored = |prefix: &str, len: usize| {
        let mut input = stream(&format!("{prefix}011 {len:024b} 0000"));
        input.extend(r_values([1, 1, 1]));
        input.extend(std::iter::repeat_n(0x5A, len));
        input
    };
    assert_eq!(
        lzx.decompress(&stored("0 ", 32766), 32766),
        Ok(vec![0x5A; 32766])
    );
    assert_eq!(
        lzx.decompress(&stored("", 10), 10),
        Err(DecrError::DataFormat)
    );
}

#[test]
fn e8_header_sets_the_translation_size() {
    // The C run of this stream is flagged by UBSan (see the commit body), so it is checked here
    // and not in the oracle table. The E8 at offset 1 has a relative call of 0x10 - 1 = 0x0F.
    // The E8 at offset 10 is in the last 10 bytes of the frame, so it is left alone.
    let mut lzx = Lzx::new(1 << 15).unwrap();
    let (hi, lo, len) = (0, 0x100, 16);
    let mut input = stream(&format!("1 {hi:016b} {lo:016b} 011 {len:024b} 0000"));
    input.extend(r_values([1, 1, 1]));
    input.extend([
        0x90, 0xE8, 0x10, 0, 0, 0, 0x90, 0x90, 0x90, 0x90, 0xE8, 0x20, 0, 0, 0, 0x90,
    ]);
    assert_eq!(
        lzx.decompress(&input, 16),
        Ok(vec![
            0x90, 0xE8, 0x0F, 0, 0, 0, 0x90, 0x90, 0x90, 0x90, 0xE8, 0x20, 0, 0, 0, 0x90
        ])
    );
}

#[test]
fn e8_translation_stops_after_32768_frames() {
    let data: [u8; 16] = [
        0x90, 0xE8, 0x10, 0, 0, 0, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    ];
    let frame = |prefix: &str| {
        let len = 16;
        let mut input = stream(&format!("{prefix}011 {len:024b} 0000"));
        input.extend(r_values([1, 1, 1]));
        input.extend(data);
        input
    };
    let first = frame("1 0000000000000000 0000000100000000 ");
    let later = frame("");
    let mut lzx = Lzx::new(1 << 15).unwrap();
    assert_ne!(lzx.decompress(&first, 16).unwrap(), data);
    for _ in 1..32767 {
        lzx.decompress(&later, 16).unwrap();
    }
    assert_ne!(lzx.decompress(&later, 16).unwrap(), data);
    assert_eq!(lzx.decompress(&later, 16).unwrap(), data);
}

#[test]
fn undo_e8_turns_absolute_calls_back_into_relative() {
    assert_eq!(
        undo(&[0, 0xE8, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0], 100, 256, true),
        (vec![0, 0xE8, 0xBB, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0], 112)
    );
    assert_eq!(
        undo(
            &[0xE8, 0xF0, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0],
            100,
            256,
            true
        ),
        (vec![0xE8, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 111)
    );
}

#[test]
fn undo_e8_leaves_other_operands_alone() {
    // Below -curpos, and at or above the file size.
    let below = [0xE8, 0x38, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0];
    assert_eq!(undo(&below, 100, 256, true), (below.to_vec(), 111));
    let above = [0xE8, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(undo(&above, 100, 256, true), (above.to_vec(), 111));
    // A frame of six bytes or fewer, and a frame before the first E8 header.
    let short = [0xE8, 0x10, 0, 0, 0, 0];
    assert_eq!(undo(&short, 100, 256, true), (short.to_vec(), 106));
    let unstarted = [0xE8, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(undo(&unstarted, 100, 256, false), (unstarted.to_vec(), 111));
    // An E8 in the last 10 bytes of the frame.
    let tail = [0, 0, 0xE8, 0x20, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(undo(&tail, 0, 256, true), (tail.to_vec(), 12));
}

#[test]
fn undo_e8_translates_each_call_in_a_frame() {
    let multi = [0xE8, 0x10, 0, 0, 0, 0xE8, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(
        undo(&multi, 10, 256, true),
        (
            vec![0xE8, 6, 0, 0, 0, 0xE8, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            26
        )
    );
}

#[test]
fn copy_match_repeats_its_own_output_and_wraps() {
    let mut window = [9, 0, 0, 0, 0, 0, 0, 0];
    copy_match(&mut window, 1, 1, 3).unwrap();
    assert_eq!(window, [9, 9, 9, 9, 0, 0, 0, 0]);

    let mut window = [0, 1, 2, 3, 4, 5, 6, 7];
    copy_match(&mut window, 2, 5, 4).unwrap();
    assert_eq!(window, [0, 1, 5, 6, 7, 0, 6, 7]);

    let mut window = [0, 1, 2, 3, 4, 5, 6, 7];
    copy_match(&mut window, 0, 1, 2).unwrap();
    assert_eq!(window, [7, 7, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn copy_match_rejects_offsets_and_runs_past_the_window() {
    assert_eq!(copy_match(&mut [0; 8], 0, 9, 1), Err(DecrError::DataFormat));
    assert_eq!(copy_match(&mut [0; 8], 6, 1, 3), Err(DecrError::DataFormat));
}

#[test]
fn tree_overfilling_the_direct_table_is_illegal() {
    assert_eq!(built(3, 12, &[1, 1, 1]).err(), Some(DecrError::IllegalData));
}

#[test]
fn tree_overfilling_the_long_codes_is_illegal() {
    assert_eq!(
        built(4, 1, &[1, 2, 2, 2]).err(),
        Some(DecrError::IllegalData)
    );
}

#[test]
fn tree_needing_nodes_past_its_table_is_illegal() {
    assert_eq!(built(2, 1, &[16, 16]).err(), Some(DecrError::IllegalData));
}

#[test]
fn incomplete_tree_is_illegal() {
    assert_eq!(built(1, 6, &[1]).err(), Some(DecrError::IllegalData));
    assert_eq!(built(2, 6, &[1, 2]).err(), Some(DecrError::IllegalData));
}

#[test]
fn complete_and_empty_trees_are_accepted() {
    assert!(built(2, 6, &[1, 1]).is_ok());
    assert!(built(2, 6, &[0, 0]).is_ok());
}

#[test]
fn length_runs_and_negative_changes() {
    // The pretree gives symbol 0 a 1-bit code and symbols 5 and 19 2-bit codes. The codes read
    // symbol 19 with a run of 4, then symbol 5 (a change of -5, so 12), then symbol 0.
    let input = stream(&format!(
        "0001 0000 0000 0000 0000 0010 {}0010 11 0 10 0",
        "0000 ".repeat(13)
    ));
    let mut lens = [0u8; 5];
    read_lengths(
        &mut Bits::new(&input),
        &mut Table::new(20, 6),
        &mut lens,
        0,
        5,
    )
    .unwrap();
    assert_eq!(lens, [12, 12, 12, 12, 0]);
}

#[test]
fn stored_data_starts_on_a_byte_boundary() {
    let mut bits = Bits::new(&[1, 2, 3, 4]);
    bits.align_word();
    assert_eq!(bits.stored(2), Ok(&[3, 4][..]));

    let mut bits = Bits::new(&[1, 2, 3, 4]);
    bits.read(1);
    bits.align_word();
    assert_eq!(bits.stored(2), Ok(&[3, 4][..]));

    assert_eq!(Bits::new(&[1]).stored(2), Err(DecrError::IllegalData));
}

#[test]
fn long_huffman_codes_row_matches_the_harness() {
    // Checked against the harness run, outside the oracle table.
    let input = unhex(
        "0010840244444444454455555f55b57febcdba3876983254fdf778ff34bc44444444454455555f55df7ffdf741ff11111111111155555755f7dffffdd97ffeff7bb7dfefefdffffbf77fbffffffefffdf7f77fbff77ffff56ef7f7ffff7dff7dfd7ff77fafff00e0",
    );
    let out = Lzx::new(1 << 15).unwrap().decompress(&input, 40).unwrap();
    assert_eq!(
        hex(&out),
        "e84142434445464748494a4b4c4d4e4f4e484547484c48424e4443444d44464a464a4e424c4d4248"
    );
}

#[test]
fn slot_three_row_matches_the_harness() {
    // Checked against the harness run, outside the oracle table.
    let input = unhex(
        "0010840144444444454455555f553933ebe99ca50680dfb34bcf8c3d79f9fdf71dff11111111111155555755cfc53d6ffb3cfffee8bf22a222222222aa2aaaaafefbbffffbef0a23a62998b7bfef00e8",
    );
    let out = Lzx::new(1 << 15).unwrap().decompress(&input, 24).unwrap();
    assert_eq!(
        hex(&out),
        "297e3520213551305e314a36305e315e314a314a36363636"
    );
}

#[test]
fn uncompressed_block_longer_than_its_input_is_illegal() {
    let mut lzx = Lzx::new(1 << 15).unwrap();
    let block_len = 10;
    let mut input = stream(&format!("0 011 {block_len:024b} 0000"));
    input.extend(r_values([1, 1, 1]));
    input.extend([0x5A; 5]);
    assert_eq!(
        lzx.decompress(&input, block_len),
        Err(DecrError::IllegalData)
    );
}

#[test]
fn overfull_aligned_tree_is_illegal() {
    let mut lzx = Lzx::new(1 << 15).unwrap();
    let block_len = 1;
    let lens = "001 ".repeat(8);
    let input = stream(&format!("0 010 {block_len:024b} {lens}"));
    assert_eq!(
        lzx.decompress(&input, block_len),
        Err(DecrError::IllegalData)
    );
}

/// The pretree that lzx_read_lens reads before each run of code lengths: sixteen zero lengths,
/// then symbols 16 to 19 with two-bit codes, which the runs of `lens` use.
fn lens_part(lens: &str) -> String {
    format!("{}{}{lens}", "0000 ".repeat(16), "0010 ".repeat(4))
}

#[test]
fn overfull_pretree_is_illegal_in_each_length_read() {
    let zeros = "0000 ".repeat(20);
    let invalid = "0001 ".repeat(20);
    let block_len = 1;
    for before in [String::new(), zeros.clone(), zeros.repeat(2)] {
        let mut lzx = Lzx::new(1 << 15).unwrap();
        let input = stream(&format!("0 001 {block_len:024b} {before}{invalid}"));
        assert_eq!(
            lzx.decompress(&input, block_len),
            Err(DecrError::IllegalData)
        );
    }
}

#[test]
fn overfull_main_tree_is_illegal() {
    // Three symbols with one-bit codes overfill the main tree.
    let main_a = format!("00 00 00 {}10 11101", "10 11111 ".repeat(4));
    let main_b = format!("{}10 10000", "10 11111 ".repeat(4));
    let block_len = 1;
    let input = stream(&format!(
        "0 001 {block_len:024b} {}{}",
        lens_part(&main_a),
        lens_part(&main_b),
    ));
    let mut lzx = Lzx::new(1 << 15).unwrap();
    assert_eq!(
        lzx.decompress(&input, block_len),
        Err(DecrError::IllegalData)
    );
}

#[test]
fn overfull_length_tree_is_illegal() {
    // Three symbols with one-bit codes overfill the length tree. The main tree is all zero.
    let main_a = format!("{}10 01101 01 1111", "10 11111 ".repeat(4));
    let main_b = format!("{}10 10000", "10 11111 ".repeat(4));
    let length = format!("00 00 00 {}10 10110", "10 11111 ".repeat(4));
    let block_len = 1;
    let input = stream(&format!(
        "0 001 {block_len:024b} {}{}{}",
        lens_part(&main_a),
        lens_part(&main_b),
        lens_part(&length),
    ));
    let mut lzx = Lzx::new(1 << 15).unwrap();
    assert_eq!(
        lzx.decompress(&input, block_len),
        Err(DecrError::IllegalData)
    );
}

#[test]
fn match_past_the_window_end_is_a_data_error() {
    let mut lzx = Lzx::new(1 << 15).unwrap();
    let stored_len = 32766;
    let mut input = stream(&format!("0 011 {stored_len:024b} 0000"));
    input.extend(r_values([1, 1, 1]));
    input.extend(std::iter::repeat_n(0x5A, stored_len));
    assert_eq!(
        lzx.decompress(&input, stored_len),
        Ok(vec![0x5A; stored_len])
    );

    // The lengths of symbols 0 to 255 are zero, symbols 256 and 257 get one-bit codes, and the
    // length tree is zero. The bit "1" then decodes symbol 257: a match of length 3 at offset 1,
    // which runs past the window end.
    let main_a = format!("{}10 01101 01 1111", "10 11111 ".repeat(4));
    let main_b = format!("00 00 {}10 01110", "10 11111 ".repeat(4));
    let length = format!("{}10 11001", "10 11111 ".repeat(4));
    let block_len = 2;
    let input = stream(&format!(
        "001 {block_len:024b} {}{}{} 1",
        lens_part(&main_a),
        lens_part(&main_b),
        lens_part(&length),
    ));
    assert_eq!(
        lzx.decompress(&input, block_len),
        Err(DecrError::DataFormat)
    );
}

#[test]
fn wrapped_code_length_is_illegal_in_read_lengths() {
    // A run of 19 changes the length by the pretree symbol that follows it. A change of 18 or 19
    // can take a length below zero, which lzx.c stores wrapped to 254 or 255.
    for (old, change, legal) in [
        (0, "10", false),
        (0, "11", false),
        (1, "11", false),
        (1, "10", true),
        (2, "11", true),
    ] {
        let input = stream(&format!("{}11 0 {change}", lens_part("")));
        let mut lens = [old, 0, 0, 0];
        let result = read_lengths(
            &mut Bits::new(&input),
            &mut Table::new(20, 6),
            &mut lens,
            0,
            4,
        );
        if legal {
            assert_eq!(result, Ok(()), "old {old}, change {change}");
            assert_eq!(lens, [0; 4]);
        } else {
            assert_eq!(
                result,
                Err(DecrError::IllegalData),
                "old {old}, change {change}"
            );
        }
    }
}

#[test]
fn stale_decode_after_a_wrapped_length_does_not_panic() {
    // The first call stops at the wrapped length, before the main tree is built. The second call
    // then decodes from the zero table, a literal 0 with no bits consumed.
    let main_a = format!("11 0 11 {}", "10 11111 ".repeat(5));
    let main_b = format!("{}10 10000", "10 11111 ".repeat(4));
    let block_len = 1;
    let input = stream(&format!(
        "0 001 {block_len:024b} {}{}",
        lens_part(&main_a),
        lens_part(&main_b),
    ));
    let mut lzx = Lzx::new(1 << 15).unwrap();
    assert_eq!(
        lzx.decompress(&input, block_len),
        Err(DecrError::IllegalData)
    );
    assert_eq!(lzx.decompress(&[], block_len), Ok(vec![0]));
}

#[test]
fn random_streams_do_not_panic() {
    // Most random streams fail early. Each call after a failure decodes from the state the
    // failure left, so this also covers stale tables. A xorshift generator keeps the test
    // repeatable. The AND of two draws makes the bytes sparse, so more trees build.
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for _ in 0..2000 {
        let mut lzx = Lzx::new(1 << 15).unwrap();
        for _ in 0..4 {
            let input: Vec<u8> = (0..64).map(|_| (next() & next()) as u8).collect();
            let outlen = (next() % 300) as usize;
            let _ = lzx.decompress(&input, outlen);
        }
    }
}
