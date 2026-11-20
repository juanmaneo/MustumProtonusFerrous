//! Rust-only edge cases that the oracle rows do not reach: blocks that are too short or whose
//! strings lie outside them, the SCS index gate of `ime.c`, the `MAKELANGID` packing, and the
//! non-IME path. The one-unit and three-unit blocks, and the odd trailing byte of `set_string`,
//! are not oracle rows. Their clause array sits at byte 102 or 106, which is not 4-byte aligned,
//! so UBSan flags the verbatim C there and the vectors were excluded. These tests pin the layout
//! the Rust code writes, and they are not checked against the C.

use crate::composition::{
    build_comp_str, complete_composition, ime_accepts_scs_index, imm_accepts_scs_index, set_string,
    unit_at, CompositionString, HEADER_LEN, SCS_CHANGEATTR, SCS_CHANGECLAUSE,
    SCS_QUERYRECONVERTSTRING, SCS_SETRECONVERTSTRING, SCS_SETSTR,
};
use crate::hkl::makelangid;
use crate::keyboard::{process_key, to_ascii_ex, DriverOutcome, VK_CAPITAL, VK_CONTROL, VK_MENU};
use crate::render;

const HKL_JP: usize = 0x0411_0411;
const HKL_EN: usize = 0x0409_0409;

/// A header followed by zeros, `len` bytes in all.
fn header_block(header: CompositionString, len: usize) -> Vec<u8> {
    let mut block = header.to_bytes();
    block.resize(len, 0);
    block
}

#[test]
fn short_block_is_rejected_and_changes_nothing() {
    let short = vec![0; HEADER_LEN - 1];
    assert!(CompositionString::read(&short).is_none());

    let mut in_comp = true;
    assert!(complete_composition(&mut in_comp, &short).is_none());
    assert!(in_comp);

    let outcome = DriverOutcome::Filled {
        block: short,
        key_consumed: true,
    };
    assert!(to_ascii_ex(&mut in_comp, HKL_JP, 0x41, 0x1e, outcome).is_none());
    assert!(in_comp);
}

#[test]
fn unit_at_checks_bounds() {
    let block = [0x61, 0, 0x62, 0];

    assert_eq!(unit_at(&block, 0), Some(0x61));
    assert_eq!(unit_at(&block, 2), Some(0x62));
    assert_eq!(unit_at(&block, 3), None);
    assert_eq!(unit_at(&block, 4), None);
    assert_eq!(unit_at(&block, u32::MAX), None);
}

#[test]
fn complete_rejects_comp_string_past_the_block() {
    let block = CompositionString {
        size: HEADER_LEN as u32,
        comp_str_len: 1,
        comp_str_offset: HEADER_LEN as u32 - 1,
        ..Default::default()
    }
    .to_bytes();

    let mut in_comp = true;
    assert!(complete_composition(&mut in_comp, &block).is_none());
    assert!(in_comp);
}

#[test]
fn to_ascii_rejects_strings_past_the_block() {
    let result_past_end = header_block(
        CompositionString {
            size: 102,
            result_str_len: 1,
            result_str_offset: 101,
            ..Default::default()
        },
        102,
    );
    let comp_past_end = header_block(
        CompositionString {
            size: 102,
            comp_str_len: 1,
            comp_str_offset: 102,
            ..Default::default()
        },
        102,
    );

    for block in [result_past_end, comp_past_end] {
        let mut in_comp = true;
        let outcome = DriverOutcome::Filled {
            block,
            key_consumed: true,
        };
        assert!(to_ascii_ex(&mut in_comp, HKL_JP, 0x41, 0x1e, outcome).is_none());
        assert!(in_comp);
    }
}

#[test]
fn one_unit_block_layout() {
    let block = build_comp_str(&[0x61]);
    let header = CompositionString::read(&block).unwrap();

    assert_eq!(block.len(), 111);
    assert_eq!(header.size, 111);
    assert_eq!(header.comp_clause_offset, 102);
    assert_eq!(header.comp_attr_offset, 110);
    assert_eq!(&block[102..110], &[0, 0, 0, 0, 1, 0, 0, 0]);
}

#[test]
fn three_unit_block_layout() {
    let block = build_comp_str(&[0x61, 0x62, 0x63]);
    let header = CompositionString::read(&block).unwrap();

    assert_eq!(block.len(), 117);
    assert_eq!(header.size, 117);
    assert_eq!(header.comp_clause_offset, 106);
    assert_eq!(header.comp_attr_offset, 114);
    assert_eq!(&block[106..114], &[0, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn set_string_ignores_odd_trailing_byte() {
    let mut in_comp = false;
    let update = set_string(&mut in_comp, &[0x61, 0, 0x62]);

    assert_eq!(render(&update.messages), "10d:0:0 10f:61:1b8");
    assert_eq!(update.block.len(), 111);
    assert!(in_comp);
}

#[test]
fn set_string_single_byte_is_empty() {
    let mut in_comp = false;
    let update = set_string(&mut in_comp, &[0x61]);

    assert_eq!(render(&update.messages), "10f:0:1b8");
    assert_eq!(update.block.len(), 100);
    assert!(!in_comp);
}

#[test]
fn header_fields_follow_immdev_order() {
    let bytes: Vec<u8> = (1..=25u32).flat_map(u32::to_le_bytes).collect();
    let header = CompositionString::read(&bytes).unwrap();

    assert_eq!(header.size, 1);
    assert_eq!(header.comp_str_offset, 13);
    assert_eq!(header.result_str_offset, 23);
    assert_eq!(header.private_offset, 25);
    assert_eq!(header.to_bytes(), bytes);
}

#[test]
fn process_key_skips_control_caps_and_menu() {
    for vkey in [VK_CONTROL, VK_CAPITAL, VK_MENU] {
        assert!(!process_key(HKL_JP, vkey, true));
    }
    assert!(!process_key(HKL_JP, 0x0001_0011, true));
    assert!(process_key(HKL_JP, 0x0001_0041, true));
}

#[test]
fn scs_index_gates() {
    // `ime_accepts_scs_index` is `ime.c:688`, and `imm_accepts_scs_index` is `imm.c:2543`.
    let table = [
        (SCS_SETSTR, true, true),
        (SCS_CHANGEATTR, true, true),
        (SCS_CHANGECLAUSE, true, true),
        (SCS_SETRECONVERTSTRING, false, true),
        (SCS_QUERYRECONVERTSTRING, false, true),
        (1, false, false),
        (0, false, false),
    ];
    for (index, ime, imm) in table {
        assert_eq!(ime_accepts_scs_index(index), ime, "ime index {index:#x}");
        assert_eq!(imm_accepts_scs_index(index), imm, "imm index {index:#x}");
    }
}

#[test]
fn makelangid_packs_sublanguage_above_primary() {
    assert_eq!(makelangid(0x11, 0x01), 0x0411);
    assert_eq!(makelangid(0x04, 0x02), 0x0804);
    assert_eq!(makelangid(0x49, 0x02), 0x0849);
}

#[test]
fn non_ime_layout_ignores_driver_block() {
    let mut in_comp = true;
    let outcome = DriverOutcome::Filled {
        block: vec![0; 3],
        key_consumed: false,
    };
    let msgs = to_ascii_ex(&mut in_comp, HKL_EN, 0x41, 0x1e, outcome).unwrap();

    assert_eq!(render(&msgs), "-");
    assert!(in_comp);
}
