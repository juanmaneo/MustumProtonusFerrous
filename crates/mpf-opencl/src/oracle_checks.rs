//! Replays the Wine oracle (`target/canon/opencl`) against this crate. The rows are the
//! verbatim output of Wine's `filter_extensions`, `extension_is_supported` and the
//! `CL_EXEC_NATIVE_KERNEL` mask in `dlls/opencl`, compiled with ASan and UBSan.

use super::*;

struct FilterRow {
    input: &'static [u8],
    /// `None` is a size query with a NULL buffer.
    buffer: Option<usize>,
    status: i32,
    size: usize,
    output: &'static [u8],
}

const FILTER: &[FilterRow] = &[
    // empty_query
    FilterRow {
        input: b"",
        buffer: None,
        status: CL_SUCCESS,
        size: 0,
        output: b"",
    },
    // empty_buffer_1
    FilterRow {
        input: b"",
        buffer: Some(1),
        status: CL_SUCCESS,
        size: 0,
        output: b"",
    },
    // all_blocked
    FilterRow {
        input: b"cl_khr_gl_sharing",
        buffer: Some(64),
        status: CL_SUCCESS,
        size: 0,
        output: b"",
    },
    // one_query
    FilterRow {
        input: b"cl_khr_fp64",
        buffer: None,
        status: CL_SUCCESS,
        size: 12,
        output: b"",
    },
    // one_short
    FilterRow {
        input: b"cl_khr_fp64",
        buffer: Some(11),
        status: CL_INVALID_VALUE,
        size: 12,
        output: b"",
    },
    // one_exact
    FilterRow {
        input: b"cl_khr_fp64",
        buffer: Some(12),
        status: CL_SUCCESS,
        size: 12,
        output: b"cl_khr_fp64",
    },
    // two_output_len
    FilterRow {
        input: b"cl_khr_fp64 cl_khr_fp16",
        buffer: Some(24),
        status: CL_INVALID_VALUE,
        size: 36,
        output: b"",
    },
    // two_exact
    FilterRow {
        input: b"cl_khr_fp64 cl_khr_fp16",
        buffer: Some(36),
        status: CL_SUCCESS,
        size: 36,
        output: b"cl_khr_fp64 cl_khr_fp16",
    },
    // mixed
    FilterRow {
        input: b"cl_khr_fp64 cl_khr_gl_sharing cl_khr_int64_base_atomics",
        buffer: Some(82),
        status: CL_SUCCESS,
        size: 82,
        output: b"cl_khr_fp64 cl_khr_int64_base_atomics",
    },
    // double_space
    FilterRow {
        input: b"cl_khr_fp64  cl_khr_fp16",
        buffer: Some(37),
        status: CL_SUCCESS,
        size: 37,
        output: b"cl_khr_fp64 cl_khr_fp16",
    },
    // edge_spaces
    FilterRow {
        input: b" cl_khr_fp16 ",
        buffer: Some(13),
        status: CL_SUCCESS,
        size: 13,
        output: b"cl_khr_fp16",
    },
    // case_and_prefix
    FilterRow {
        input: b"CL_KHR_GL cl_khr_fp16 cl_nv_d3d9_sharing",
        buffer: Some(31),
        status: CL_SUCCESS,
        size: 31,
        output: b"cl_khr_fp16",
    },
    // tab_is_not_separator
    FilterRow {
        input: b"cl_khr_fp64\x09cl_khr_gl_sharing",
        buffer: Some(30),
        status: CL_SUCCESS,
        size: 30,
        output: b"cl_khr_fp64\x09cl_khr_gl_sharing",
    },
    // longer_than_blocked
    FilterRow {
        input: b"cl_khr_gl_sharingx",
        buffer: Some(64),
        status: CL_SUCCESS,
        size: 19,
        output: b"cl_khr_gl_sharingx",
    },
];

const SUPPORT: &[(&[u8], bool)] = &[
    // empty
    (b"", false),
    // icd
    (b"cl_khr_icd", false),
    // icd_prefix
    (b"cl_khr_ic", false),
    // icd_longer
    (b"cl_khr_icd2", true),
    // mixed_case
    (b"Cl_Nv_D3d9_Sharing", false),
    // fp64
    (b"cl_khr_fp64", true),
];

const EXEC_CAPS: &[(u64, u64)] = &[
    // native_and_default
    (0x7, 0x5),
    // native_only
    (0x2, 0x0),
    // none
    (0x0, 0x0),
    // all_bits
    (0xffffffffffffffff, 0xfffffffffffffffd),
];

#[test]
fn filter_extensions_matches_oracle() {
    for row in FILTER {
        let (status, size) = match row.buffer {
            None => filter_extensions(row.input, None),
            Some(len) => {
                let mut buf = vec![0xAA; len];
                let result = filter_extensions(row.input, Some(&mut buf[..]));
                if result.0 == CL_SUCCESS {
                    let end = row.output.len();
                    assert_eq!(&buf[..end], row.output, "{:?}", row.input);
                    assert_eq!(buf[end], 0, "{:?}", row.input);
                }
                result
            }
        };
        assert_eq!((status, size), (row.status, row.size), "{:?}", row.input);
    }
}

#[test]
fn extension_is_supported_matches_oracle() {
    for (name, supported) in SUPPORT {
        assert_eq!(extension_is_supported(name), *supported, "{name:?}");
    }
}

#[test]
fn exec_caps_match_oracle() {
    for &(caps, expected) in EXEC_CAPS {
        assert_eq!(filter_exec_capabilities(caps), expected);
    }
}
