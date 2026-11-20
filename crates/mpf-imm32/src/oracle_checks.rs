//! Replays of the oracle rows O01 to O25 in `target/canon/imm32/oracle.txt`. The oracle compiles
//! the verbatim Wine C with `-fsanitize=address,undefined`, and the sanitizers report nothing for
//! these rows. Each test names its row, and its expectations are the fields of that row. The row
//! fields that are not checked are: `ret`, which the wrappers always return as TRUE; `count`, which
//! is the number of messages listed; `calls`, which counts driver calls; and the `/n` send counts
//! of the `ImmIsUIMessage` probes. Those belong to the caller.

use crate::composition::{
    build_comp_str, cancel_composition, complete_composition, imm_accepts_scs_index, set_status,
    set_string, CompositionString, SCS_CHANGEATTR, SCS_CHANGECLAUSE, SCS_QUERYRECONVERTSTRING,
    SCS_SETRECONVERTSTRING, SCS_SETSTR,
};
use crate::hkl::is_ime_hkl;
use crate::keyboard::{process_key, to_ascii_ex, DriverOutcome, VK_SHIFT};
use crate::messages::{
    is_ime_ui_msg, is_ui_message, MsimeMessages, WM_IME_CHAR, WM_IME_CONTROL,
    WM_IME_ENDCOMPOSITION, WM_IME_KEYDOWN, WM_IME_KEYUP, WM_IME_SETCONTEXT,
    WM_IME_STARTCOMPOSITION, WM_IME_SYSTEM,
};
use crate::property::{
    escape_requires_wa, get_property, is_unicode, ImeInfo, IGP_CONVERSION, IGP_GETIMEVERSION,
    IGP_PROPERTY, IGP_SELECT, IGP_SENTENCE, IGP_SETCOMPSTR, IGP_UI, IME_ESC_GETHELPFILENAME,
    IME_ESC_GET_EUDC_DICTIONARY, IME_ESC_HANJA_MODE, IME_ESC_IME_NAME, IME_ESC_MAX_KEY,
    IME_ESC_SEQUENCE_TO_INTERNAL, IME_ESC_SET_EUDC_DICTIONARY, IME_PROP_UNICODE,
};
use crate::{hex, render};

const HKL_JP: usize = 0x0411_0411;
const HKL_EN: usize = 0x0409_0409;

/// The clause array `[0, 1]` that the harness writes.
const CLAUSE_01: [u8; 8] = [0, 0, 0, 0, 1, 0, 0, 0];

/// The block the driver writes: `header`, with each `(offset, bytes)` part copied in, zero
/// elsewhere. Its length is `header.size`.
fn make_block(header: CompositionString, parts: &[(usize, &[u8])]) -> Vec<u8> {
    let mut block = vec![0; header.size as usize];
    block[..header.to_bytes().len()].copy_from_slice(&header.to_bytes());
    for (offset, bytes) in parts {
        block[*offset..*offset + bytes.len()].copy_from_slice(bytes);
    }
    block
}

#[test]
fn o01_is_ime_hkl_table() {
    // The 20 language IDs that the harness passes, in its order.
    const LANGIDS: [u16; 20] = [
        0x045e, 0x0445, 0x0804, 0x0404, 0x0447, 0x0439, 0x0411, 0x044b, 0x0412, 0x044c, 0x044e,
        0x0461, 0x0448, 0x0446, 0x0449, 0x0849, 0x044a, 0x0473, 0x042a, 0x0478,
    ];

    assert!(is_ime_hkl(HKL_JP));
    assert!(!is_ime_hkl(HKL_EN));
    assert!(is_ime_hkl(0xe001_0409));
    assert!(!is_ime_hkl(0xd001_0409));
    assert!(is_ime_hkl(0x0804_0804));
    for langid in LANGIDS {
        assert!(is_ime_hkl(usize::from(langid) << 16));
    }
}

#[test]
fn o02_escape_requires_wa() {
    assert!(escape_requires_wa(IME_ESC_GET_EUDC_DICTIONARY));
    assert!(escape_requires_wa(IME_ESC_SET_EUDC_DICTIONARY));
    assert!(escape_requires_wa(IME_ESC_IME_NAME));
    assert!(escape_requires_wa(IME_ESC_GETHELPFILENAME));
    assert!(!escape_requires_wa(IME_ESC_SEQUENCE_TO_INTERNAL));
    assert!(!escape_requires_wa(IME_ESC_MAX_KEY));
    assert!(!escape_requires_wa(IME_ESC_HANJA_MODE));
}

#[test]
fn o03_ui_message_send_filter() {
    // `a_null=1`: with no window the call still returns TRUE. The caller skips the send.
    assert!(is_ui_message(WM_IME_STARTCOMPOSITION));
    assert!(is_ui_message(WM_IME_SYSTEM));
    assert!(!is_ui_message(0x0111));
    assert!(!is_ui_message(WM_IME_KEYDOWN));
}

#[test]
fn o04_ui_quirks_and_scs_gate() {
    assert!(is_ui_message(WM_IME_ENDCOMPOSITION));
    assert!(!is_ui_message(0x0110));
    assert!(is_ui_message(WM_IME_SETCONTEXT));
    assert!(!is_ui_message(WM_IME_CONTROL));
    assert!(!is_ui_message(WM_IME_CHAR));
    assert!(is_ui_message(WM_IME_SYSTEM));
    assert!(!is_ui_message(WM_IME_KEYDOWN));
    assert!(!is_ui_message(WM_IME_KEYUP));

    assert!(imm_accepts_scs_index(SCS_SETSTR));
    assert!(imm_accepts_scs_index(SCS_CHANGEATTR));
    assert!(imm_accepts_scs_index(SCS_CHANGECLAUSE));
    assert!(imm_accepts_scs_index(SCS_SETRECONVERTSTRING));
    assert!(imm_accepts_scs_index(SCS_QUERYRECONVERTSTRING));
    assert!(!imm_accepts_scs_index(1));
    assert!(!imm_accepts_scs_index(0));
}

#[test]
fn o05_ime_ui_msg() {
    let msime = MsimeMessages {
        service: 0xc100,
        reconvert_options: 0xc101,
        mouse: 0xc102,
        reconvert_request: 0xc103,
        reconvert: 0xc104,
        query_position: 0xc105,
        document_feed: 0xc106,
    };

    assert!(is_ime_ui_msg(WM_IME_STARTCOMPOSITION, &msime));
    assert!(!is_ime_ui_msg(0x0111, &msime));
    assert!(is_ime_ui_msg(WM_IME_CHAR, &msime));
    assert!(is_ime_ui_msg(WM_IME_KEYDOWN, &msime));
    assert!(is_ime_ui_msg(WM_IME_KEYUP, &msime));
    assert!(!is_ime_ui_msg(0x0287, &msime));
    for msg in 0xc100..=0xc106 {
        assert!(is_ime_ui_msg(msg, &msime));
    }
    assert!(!is_ime_ui_msg(0xc107, &msime));
}

#[test]
fn o06_get_property() {
    let info = ImeInfo {
        property: 0x1234,
        conversion_caps: 5,
        sentence_caps: 6,
        scs_caps: 8,
        select_caps: 9,
    };

    assert_eq!(get_property(&info, IGP_PROPERTY), 4660);
    assert_eq!(get_property(&info, IGP_CONVERSION), 5);
    assert_eq!(get_property(&info, IGP_SENTENCE), 6);
    assert_eq!(get_property(&info, IGP_UI), 0);
    assert_eq!(get_property(&info, IGP_SETCOMPSTR), 8);
    assert_eq!(get_property(&info, IGP_SELECT), 9);
    assert_eq!(get_property(&info, IGP_GETIMEVERSION), 262_144);
    assert_eq!(get_property(&info, 0x99), 0);
}

#[test]
fn o07_is_unicode() {
    assert!(is_unicode(IME_PROP_UNICODE));
    assert!(!is_unicode(0x4));
    assert!(is_unicode(0x0008_0004));
}

#[test]
fn o08_composition_status_messages() {
    let mut in_comp = false;
    let start = set_status(&mut in_comp, true).map(|m| m.message);
    let again = set_status(&mut in_comp, true);
    let end = set_status(&mut in_comp, false).map(|m| m.message);
    let end_again = set_status(&mut in_comp, false);

    assert_eq!((start, end), (Some(0x10d), Some(0x10e)));
    assert!(again.is_none());
    assert!(end_again.is_none());
    assert!(!in_comp);
}

#[test]
fn o09_build_two_units() {
    let block = build_comp_str(&[0x61, 0x62]);

    assert_eq!(block.len(), 114);
    assert_eq!(hex(&block), "720000000000000000000000000000000000000000000000000000000200000070000000080000006800000002000000640000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006100620000000000020000000000");
}

#[test]
fn o10_build_empty() {
    let block = build_comp_str(&[]);

    assert_eq!(block.len(), 100);
    assert_eq!(hex(&block), "64000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
}

#[test]
fn o11_build_surrogate_pair() {
    let block = build_comp_str(&[0xd83d, 0xde00]);

    assert_eq!(block.len(), 114);
    assert_eq!(hex(&block), "720000000000000000000000000000000000000000000000000000000200000070000000080000006800000002000000640000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003dd800de00000000020000000000");
}

#[test]
fn o12_set_string_two_units() {
    // The string is `'a', 'b', 0, 0`, so the trailing NULs are dropped.
    let mut in_comp = false;
    let update = set_string(&mut in_comp, &[0x61, 0, 0x62, 0, 0, 0, 0, 0]);

    assert_eq!(render(&update.messages), "10d:0:0 10f:61:1b8");
    assert_eq!(update.block.len(), 114);
    assert!(in_comp);
}

#[test]
fn o13_set_string_empty_ends_composition() {
    let mut in_comp = true;
    let update = set_string(&mut in_comp, &[]);

    assert_eq!(render(&update.messages), "10e:0:0 10f:0:1b8");
    assert_eq!(update.block.len(), 100);
    assert!(!in_comp);
}

#[test]
fn o14_set_string_zeros() {
    let mut in_comp = false;
    let update = set_string(&mut in_comp, &[0, 0, 0, 0]);

    assert_eq!(render(&update.messages), "10f:0:1b8");
    assert_eq!(update.block.len(), 100);
    assert!(!in_comp);
}

#[test]
fn o15_complete_moves_composition_to_result() {
    let block = build_comp_str(&[0x61, 0x62]);
    let mut in_comp = true;
    let update = complete_composition(&mut in_comp, &block).unwrap();

    assert_eq!(render(&update.messages), "10f:61:1800 10e:0:0");
    assert_eq!(update.block.len(), 100);
    assert!(!in_comp);
}

#[test]
fn o16_cancel_ends_composition() {
    let mut in_comp = true;
    let update = cancel_composition(&mut in_comp);

    assert_eq!(render(&update.messages), "10e:0:0");
    assert_eq!(update.block.len(), 100);
    assert!(!in_comp);
}

#[test]
fn o17_complete_empty_block_emits_nothing() {
    let mut in_comp = false;
    let update = complete_composition(&mut in_comp, &build_comp_str(&[])).unwrap();

    assert_eq!(render(&update.messages), "-");
    assert_eq!(update.block.len(), 100);
    assert!(!in_comp);
}

#[test]
fn o18_not_implemented_key_is_forwarded() {
    let mut in_comp = false;
    let down = to_ascii_ex(
        &mut in_comp,
        HKL_JP,
        0x41,
        0x1e,
        DriverOutcome::NotImplemented,
    )
    .unwrap();
    let up = to_ascii_ex(
        &mut in_comp,
        HKL_JP,
        0x41,
        0x801e,
        DriverOutcome::NotImplemented,
    )
    .unwrap();
    let non_ime = to_ascii_ex(
        &mut in_comp,
        HKL_EN,
        0x41,
        0x1e,
        DriverOutcome::NotImplemented,
    )
    .unwrap();

    assert_eq!(render(&down), "100:41:1e0001");
    assert_eq!(render(&up), "101:41:801e0001");
    assert_eq!(render(&non_ime), "-");
    assert!(!in_comp);
}

#[test]
fn o19_result_and_composition_with_clauses() {
    let block = make_block(
        CompositionString {
            size: 121,
            result_str_len: 1,
            result_str_offset: 100,
            result_clause_len: 8,
            result_clause_offset: 102,
            comp_str_len: 1,
            comp_str_offset: 110,
            comp_clause_len: 8,
            comp_clause_offset: 112,
            comp_attr_len: 1,
            comp_attr_offset: 120,
            ..Default::default()
        },
        &[
            (100, &0x78u16.to_le_bytes()),
            (102, &CLAUSE_01),
            (110, &0x79u16.to_le_bytes()),
            (112, &CLAUSE_01),
        ],
    );
    let mut in_comp = false;
    let msgs = to_ascii_ex(
        &mut in_comp,
        HKL_JP,
        0x41,
        0x1e,
        DriverOutcome::Filled {
            block,
            key_consumed: true,
        },
    )
    .unwrap();

    assert_eq!(render(&msgs), "10d:0:0 10f:78:1800 10f:79:1b8");
    assert!(in_comp);
}

#[test]
fn o20_composition_only_key_not_consumed() {
    let block = make_block(
        CompositionString {
            size: 102,
            comp_str_len: 1,
            comp_str_offset: 100,
            ..Default::default()
        },
        &[(100, &0x79u16.to_le_bytes())],
    );
    let mut in_comp = true;
    let msgs = to_ascii_ex(
        &mut in_comp,
        HKL_JP,
        0x41,
        0x1e,
        DriverOutcome::Filled {
            block,
            key_consumed: false,
        },
    )
    .unwrap();

    assert_eq!(render(&msgs), "10f:79:6188 290:41:1e0001");
    assert!(in_comp);
}

#[test]
fn o21_driver_failed_reports_nothing() {
    let mut in_comp = true;
    let msgs = to_ascii_ex(&mut in_comp, HKL_JP, 0x41, 0x1e, DriverOutcome::Failed).unwrap();

    assert_eq!(render(&msgs), "-");
    assert!(in_comp);
}

#[test]
fn o22_empty_block_reports_only_the_key() {
    let block = make_block(
        CompositionString {
            size: 100,
            ..Default::default()
        },
        &[],
    );
    let mut in_comp = false;
    let msgs = to_ascii_ex(
        &mut in_comp,
        HKL_JP,
        0x41,
        0x1e,
        DriverOutcome::Filled {
            block,
            key_consumed: false,
        },
    )
    .unwrap();

    assert_eq!(render(&msgs), "290:41:1e0001");
    assert!(!in_comp);
}

#[test]
fn o23_process_key() {
    assert!(!process_key(HKL_JP, VK_SHIFT, true));
    assert!(!process_key(HKL_JP, 0x0001_0010, true));
    assert!(process_key(HKL_JP, 0x41, true));
    assert!(!process_key(HKL_EN, 0x41, true));
    assert!(!process_key(HKL_JP, 0x41, false));
}

#[test]
fn o24_result_only_ends_composition() {
    let block = make_block(
        CompositionString {
            size: 102,
            result_str_len: 1,
            result_str_offset: 100,
            ..Default::default()
        },
        &[(100, &0x78u16.to_le_bytes())],
    );
    let mut in_comp = true;
    let msgs = to_ascii_ex(
        &mut in_comp,
        HKL_JP,
        0x41,
        0x1e,
        DriverOutcome::Filled {
            block,
            key_consumed: true,
        },
    )
    .unwrap();

    assert_eq!(render(&msgs), "10f:78:800 10e:0:0");
    assert!(!in_comp);
}

#[test]
fn o25_complete_composition_only_block() {
    let block = make_block(
        CompositionString {
            size: 102,
            comp_str_len: 1,
            comp_str_offset: 100,
            ..Default::default()
        },
        &[(100, &0x79u16.to_le_bytes())],
    );
    let mut in_comp = true;
    let update = complete_composition(&mut in_comp, &block).unwrap();

    assert_eq!(render(&update.messages), "10f:79:800 10e:0:0");
    assert_eq!(update.block.len(), 100);
    assert!(!in_comp);
}
