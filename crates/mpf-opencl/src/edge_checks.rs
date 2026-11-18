//! Edge cases the Wine oracle vectors do not reach: NUL bytes, non-ASCII input, empty buffers and
//! the prefix rule across the whole blocked table. The NUL and prefix expectations were also
//! checked against Wine's C through the same oracle harness.

use super::*;

#[test]
fn name_is_compared_up_to_its_first_nul() {
    assert!(!extension_is_supported(b"cl_khr_icd\0zz"));
    assert!(extension_is_supported(b"cl_khr_fp64\0"));
}

#[test]
fn non_ascii_bytes_are_not_case_folded() {
    assert!(extension_is_supported(b"\xc3\x81cl"));
}

#[test]
fn blocked_names_block_every_prefix_and_case_variant() {
    for name in UNSUPPORTED {
        let upper = name.to_ascii_uppercase();
        for len in 0..=name.len() {
            assert!(
                !extension_is_supported(&name.as_bytes()[..len]),
                "{name}[..{len}]"
            );
            assert!(
                !extension_is_supported(&upper.as_bytes()[..len]),
                "{upper}[..{len}]"
            );
        }
        let longer = format!("{name}x");
        assert!(extension_is_supported(longer.as_bytes()), "{longer}");
    }
}

#[test]
fn filter_reads_input_up_to_its_first_nul() {
    let input = b"cl_khr_fp64\0cl_khr_gl_sharing";
    assert_eq!(filter_extensions(input, None), (CL_SUCCESS, 12));
    let mut buf = [0xAA; 12];
    assert_eq!(filter_extensions(input, Some(&mut buf)), (CL_SUCCESS, 12));
    assert_eq!(&buf[..11], b"cl_khr_fp64");
    assert_eq!(buf[11], 0);
    assert_eq!(filter_extensions(b"\0cl_khr_fp64", None), (CL_SUCCESS, 0));
}

#[test]
fn blocked_tokens_do_not_contribute_to_the_size() {
    assert_eq!(
        filter_extensions(b"cl_khr_gl_sharing cl_khr_fp64", None),
        (CL_SUCCESS, 12)
    );
}

#[test]
fn separators_only_input_leaves_an_empty_string() {
    let mut buf = [0xAA; 1];
    assert_eq!(filter_extensions(b"  ", Some(&mut buf)), (CL_SUCCESS, 0));
    assert_eq!(buf, [0]);
    let mut buf = [0xAA; 1];
    assert_eq!(
        filter_extensions(b"  cl_khr_gl", Some(&mut buf)),
        (CL_SUCCESS, 0)
    );
    assert_eq!(buf, [0]);
}

#[test]
fn empty_buffer_cannot_hold_the_terminator() {
    let empty: &mut [u8] = &mut [];
    assert_eq!(
        filter_extensions(b"cl_khr_gl_sharing", Some(empty)),
        (CL_INVALID_VALUE, 0)
    );
    let empty: &mut [u8] = &mut [];
    assert_eq!(
        filter_extensions(b"cl_khr_fp64", Some(empty)),
        (CL_INVALID_VALUE, 12)
    );
}

#[test]
fn exec_capability_mask_keeps_other_bits() {
    assert_eq!(filter_exec_capabilities(0x102), 0x100);
}
