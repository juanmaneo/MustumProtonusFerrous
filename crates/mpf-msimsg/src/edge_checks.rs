//! Edge-case tests for the ported msimsg resource.
//!
//! These cover the `MAKELANGID` bit packing at its limits and the shape of the RCDATA payload.
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn make_lang_id_packs_primary_and_sublang() {
    // Not oracle-derived: the MAKELANGID bit layout from include/winnt.h, at the limits of the
    // 10-bit primary and the 6-bit sublanguage. The oracle only covers LANG_ENGLISH/SUBLANG_DEFAULT.
    assert_eq!(make_lang_id(0x3ff, 0), 0x03ff);
    assert_eq!(make_lang_id(0, 0x3f), 0xfc00);
    assert_eq!(make_lang_id(0x3ff, 0x3f), 0xffff);
    assert_eq!(make_lang_id(0, 0), 0);
}

#[test]
fn make_lang_id_truncates_like_the_word_macro() {
    // Not oracle-derived: the C macro computes in WORD, so a sublanguage above 6 bits loses its
    // high bits and a primary above 10 bits is not masked. The port matches that with u16.
    assert_eq!(make_lang_id(0, 0x40), 0);
    assert_eq!(make_lang_id(0x400, 0), 0x0400);
    assert_eq!(make_lang_id(0xffff, 0xffff), 0xffff);
}

#[test]
fn reboot_prompt_payload_is_text_plus_two_nuls() {
    // Not oracle-derived: the structure of the `\0\0` literal, checked against the oracle's
    // text_len and data_size rows.
    let data = reboot_prompt_rcdata();
    assert_eq!(data.len(), REBOOT_PROMPT.len() + 2);
    assert_eq!(&data[..REBOOT_PROMPT.len()], REBOOT_PROMPT.as_bytes());
    assert_eq!(&data[REBOOT_PROMPT.len()..], &[0, 0]);
}

#[test]
fn reboot_prompt_text_has_no_embedded_nul() {
    // Not oracle-derived: the first NUL of the payload is the text's end, so the text itself
    // must not hold one (the oracle's text_len row finds the first NUL at 132).
    assert!(!REBOOT_PROMPT.contains('\0'));
    assert_eq!(REBOOT_PROMPT.matches("[2]").count(), 1);
}

#[test]
fn reboot_prompt_resource_carries_the_constants() {
    // Not oracle-derived: the entry fields are the constants above, and the data is the payload.
    assert_eq!(
        reboot_prompt_resource(),
        ResourceEntry {
            type_id: 10,
            name_id: 1703,
            lang_id: 0x0409,
            data: reboot_prompt_rcdata(),
        }
    );
    assert_eq!(
        REBOOT_PROMPT_LANG_ID,
        make_lang_id(LANG_ENGLISH, SUBLANG_DEFAULT)
    );
}
