//! Edge-case tests for the ported d3dcompiler_36 logic.
//!
//! These cover every arm of the blob-part tables, the argument gates, the source and include path
//! choices, the stub record and the export table. Not every expectation is oracle-derived; the ones
//! that aren't say so in a comment.

use super::*;
use crate::blob::{
    check_blob_part, section_count_is_valid, TAG_AON9, TAG_ISGN, TAG_OSG5, TAG_OSGN, TAG_PCSG,
    TAG_SDBG, TAG_XNAP, TAG_XNAS,
};
use crate::hresult::VKD3D_OK;

const DATA: &[u8] = &[0; 32];

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/d3dcompiler.h, include/winerror.h, include/d3d9.h,
    // include/windef.h, include/winternl.h, include/winnt.h and dlls/d3dcompiler_43/d3dcompiler_private.h.
    assert_eq!(DXBC_HEADER_SIZE, 32);
    assert_eq!(D3D_BLOB_INPUT_SIGNATURE_BLOB, 0);
    assert_eq!(D3D_BLOB_OUTPUT_SIGNATURE_BLOB, 1);
    assert_eq!(D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB, 2);
    assert_eq!(D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB, 3);
    assert_eq!(D3D_BLOB_ALL_SIGNATURE_BLOB, 4);
    assert_eq!(D3D_BLOB_DEBUG_INFO, 5);
    assert_eq!(D3D_BLOB_LEGACY_SHADER, 6);
    assert_eq!(D3D_BLOB_XNA_PREPASS_SHADER, 7);
    assert_eq!(D3D_BLOB_XNA_SHADER, 8);
    assert_eq!(D3D_BLOB_PDB, 9);
    assert_eq!(D3D_BLOB_DEBUG_NAME, 12);
    assert_eq!(D3D_BLOB_TEST_ALTERNATE_SHADER, 0x8000);
    assert_eq!(D3D_BLOB_TEST_COMPILE_REPORT, 0x8003);
    assert_eq!(TAG_DXBC, u32::from_le_bytes(*b"DXBC"));
    assert_eq!(MAX_PATH, 260);
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_FAIL, 0x8000_4005);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000E);
    assert_eq!(E_INVALIDARG, 0x8007_0057);
    assert_eq!(D3DERR_INVALIDCALL, 0x8876_086C);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn hresult_maps_every_vkd3d_code() {
    // Not oracle-derived: the rows cover -1 to -6 and 0. The value -7 hits the `default` arm of
    // hresult_from_vkd3d_result, and a large positive code does too.
    assert_eq!(hresult_from_vkd3d_result(VKD3D_OK), S_OK);
    assert_eq!(hresult_from_vkd3d_result(-7), E_FAIL);
    assert_eq!(hresult_from_vkd3d_result(i32::MAX), E_FAIL);
    assert_eq!(hresult_from_vkd3d_result(i32::MIN), E_FAIL);
}

#[test]
fn check_blob_part_selects_tags_per_part() {
    // Not oracle-derived: the tag set of each arm of check_blob_part in dlls/d3dcompiler_43/blob.c.
    // Tag order: ISGN, OSGN, OSG5, PCSG, SDBG, Aon9, XNAP, XNAS.
    let tags = [
        TAG_ISGN, TAG_OSGN, TAG_OSG5, TAG_PCSG, TAG_SDBG, TAG_AON9, TAG_XNAP, TAG_XNAS,
    ];
    let cases: [(u32, [bool; 8]); 9] = [
        (
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            [true, false, false, false, false, false, false, false],
        ),
        (
            D3D_BLOB_OUTPUT_SIGNATURE_BLOB,
            [false, true, true, false, false, false, false, false],
        ),
        (
            D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
            [true, true, true, false, false, false, false, false],
        ),
        (
            D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB,
            [false, false, false, true, false, false, false, false],
        ),
        (
            D3D_BLOB_ALL_SIGNATURE_BLOB,
            [true, true, true, true, false, false, false, false],
        ),
        (
            D3D_BLOB_DEBUG_INFO,
            [false, false, false, false, true, false, false, false],
        ),
        (
            D3D_BLOB_LEGACY_SHADER,
            [false, false, false, false, false, true, false, false],
        ),
        (
            D3D_BLOB_XNA_PREPASS_SHADER,
            [false, false, false, false, false, false, true, false],
        ),
        (
            D3D_BLOB_XNA_SHADER,
            [false, false, false, false, false, false, false, true],
        ),
    ];
    for (part, expected) in cases {
        for (tag, want) in tags.iter().zip(expected) {
            assert_eq!(
                check_blob_part(*tag, part),
                want,
                "tag {tag:#x} part {part}"
            );
        }
    }
    // The `_` arm: PDB, the parts between the ranges, and the test parts select no tag.
    for part in [
        D3D_BLOB_PDB,
        D3D_BLOB_DEBUG_NAME,
        D3D_BLOB_TEST_ALTERNATE_SHADER,
        u32::MAX,
    ] {
        for tag in tags {
            assert!(!check_blob_part(tag, part), "tag {tag:#x} part {part}");
        }
    }
    assert!(!check_blob_part(0, D3D_BLOB_INPUT_SIGNATURE_BLOB));
}

#[test]
fn section_count_rules_per_part() {
    // Not oracle-derived: the count rules of section_count_is_valid in dlls/d3dcompiler_43/blob.c.
    for part in [
        D3D_BLOB_INPUT_SIGNATURE_BLOB,
        D3D_BLOB_OUTPUT_SIGNATURE_BLOB,
        D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB,
        D3D_BLOB_DEBUG_INFO,
        D3D_BLOB_LEGACY_SHADER,
        D3D_BLOB_XNA_PREPASS_SHADER,
        D3D_BLOB_XNA_SHADER,
    ] {
        assert!(!section_count_is_valid(part, 0));
        assert!(section_count_is_valid(part, 1));
        assert!(!section_count_is_valid(part, 2));
    }
    assert!(!section_count_is_valid(
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
        1
    ));
    assert!(section_count_is_valid(
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
        2
    ));
    assert!(!section_count_is_valid(
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
        3
    ));
    assert!(!section_count_is_valid(D3D_BLOB_ALL_SIGNATURE_BLOB, 2));
    assert!(section_count_is_valid(D3D_BLOB_ALL_SIGNATURE_BLOB, 3));
    assert!(!section_count_is_valid(D3D_BLOB_ALL_SIGNATURE_BLOB, 4));
    // The `_` arm accepts every count for the parts that select no section.
    for part in [
        D3D_BLOB_PDB,
        D3D_BLOB_DEBUG_NAME,
        D3D_BLOB_TEST_ALTERNATE_SHADER,
        u32::MAX,
    ] {
        for count in 0..=4 {
            assert!(section_count_is_valid(part, count));
        }
    }
}

#[test]
fn get_blob_part_rejects_bad_arguments() {
    let isgn = Section {
        tag: TAG_ISGN,
        data: b"pay0",
    };
    // Not oracle-derived: each gate of get_blob_part on its own. The oracle covers the NULL data,
    // the short data and the gap part 13.
    assert_eq!(
        get_blob_part(None, 0, D3D_BLOB_INPUT_SIGNATURE_BLOB, true, Some(&[isgn])),
        Err(D3DERR_INVALIDCALL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            1,
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            true,
            Some(&[isgn])
        ),
        Err(D3DERR_INVALIDCALL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            false,
            Some(&[isgn])
        ),
        Err(D3DERR_INVALIDCALL)
    );
    assert_eq!(
        get_blob_part(
            Some(&DATA[..31]),
            0,
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            true,
            Some(&[isgn])
        ),
        Err(D3DERR_INVALIDCALL)
    );
    assert_eq!(
        get_blob_part(Some(DATA), 0, 0x7FFF, true, Some(&[isgn])),
        Err(D3DERR_INVALIDCALL)
    );
    assert_eq!(
        get_blob_part(Some(DATA), 0, 0x8004, true, Some(&[isgn])),
        Err(D3DERR_INVALIDCALL)
    );
    assert_eq!(
        get_blob_part(Some(DATA), 0, u32::MAX, true, Some(&[isgn])),
        Err(D3DERR_INVALIDCALL)
    );
}

#[test]
fn get_blob_part_fails_without_sections() {
    let isgn = Section {
        tag: TAG_ISGN,
        data: b"pay0",
    };
    let sdbg = Section {
        tag: TAG_SDBG,
        data: b"pay0",
    };
    // Not oracle-derived: a failed parse (sections None), and empty or wrong-count selections.
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_INPUT_SIGNATURE_BLOB, true, None),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            true,
            Some(&[])
        ),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            true,
            Some(&[isgn, isgn])
        ),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_DEBUG_INFO,
            true,
            Some(&[sdbg, sdbg])
        ),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_ALL_SIGNATURE_BLOB,
            true,
            Some(&[isgn, isgn])
        ),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_ALL_SIGNATURE_BLOB, true, Some(&[])),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_ALL_SIGNATURE_BLOB,
            true,
            Some(&[isgn, isgn, isgn, isgn])
        ),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_PDB, true, Some(&[sdbg])),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_DEBUG_NAME, true, Some(&[])),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_TEST_ALTERNATE_SHADER,
            true,
            Some(&[])
        ),
        Err(E_FAIL)
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_TEST_COMPILE_REPORT,
            true,
            Some(&[isgn])
        ),
        Err(E_FAIL)
    );
}

#[test]
fn get_blob_part_returns_verbatim_for_single_section_parts() {
    let aon9 = Section {
        tag: TAG_AON9,
        data: b"legacy",
    };
    let xnap = Section {
        tag: TAG_XNAP,
        data: b"prepass",
    };
    let xnas = Section {
        tag: TAG_XNAS,
        data: b"xna",
    };
    // Not oracle-derived: the Verbatim arm for the three parts beyond DEBUG_INFO. The oracle covers
    // DEBUG_INFO itself.
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_LEGACY_SHADER, true, Some(&[aon9])),
        Ok(BlobPart::Verbatim(b"legacy"))
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_XNA_PREPASS_SHADER,
            true,
            Some(&[xnap])
        ),
        Ok(BlobPart::Verbatim(b"prepass"))
    );
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_XNA_SHADER, true, Some(&[xnas])),
        Ok(BlobPart::Verbatim(b"xna"))
    );
}

#[test]
fn get_blob_part_returns_container_with_matching_sections_in_order() {
    let isgn = Section {
        tag: TAG_ISGN,
        data: b"in",
    };
    let osgn = Section {
        tag: TAG_OSGN,
        data: b"out",
    };
    let osg5 = Section {
        tag: TAG_OSG5,
        data: b"out5",
    };
    let pcsg = Section {
        tag: TAG_PCSG,
        data: b"patch",
    };
    let sdbg = Section {
        tag: TAG_SDBG,
        data: b"dbg",
    };
    // Not oracle-derived: the Container arm for the patch-constant and input-and-output parts,
    // and the filtering that drops unrelated sections while keeping the input order.
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB,
            true,
            Some(&[sdbg, pcsg])
        ),
        Ok(BlobPart::Container(vec![pcsg]))
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
            true,
            Some(&[osgn, sdbg, isgn])
        ),
        Ok(BlobPart::Container(vec![osgn, isgn]))
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
            true,
            Some(&[isgn, osg5])
        ),
        Ok(BlobPart::Container(vec![isgn, osg5]))
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_ALL_SIGNATURE_BLOB,
            true,
            Some(&[pcsg, sdbg, osg5, isgn])
        ),
        Ok(BlobPart::Container(vec![pcsg, osg5, isgn]))
    );
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_OUTPUT_SIGNATURE_BLOB,
            true,
            Some(&[isgn, osg5, sdbg])
        ),
        Ok(BlobPart::Container(vec![osg5]))
    );
}

#[test]
fn disassemble_source_type_reads_the_first_four_bytes() {
    // Not oracle-derived: the empty, short and longer-than-four inputs of the source choice.
    assert_eq!(disassemble_source_type(b""), SourceType::D3dBytecode);
    assert_eq!(disassemble_source_type(b"D"), SourceType::D3dBytecode);
    assert_eq!(disassemble_source_type(b"DXBD"), SourceType::D3dBytecode);
    assert_eq!(disassemble_source_type(b"DXBC"), SourceType::DxbcTpf);
    assert_eq!(disassemble_source_type(b"DXBCextra"), SourceType::DxbcTpf);
}

#[test]
fn check_preprocess_args_accepts_both_pointers() {
    // Not oracle-derived: the accepted case. The oracle covers the two rejected cases.
    assert_eq!(check_preprocess_args(Some(DATA), true), Ok(()));
    assert_eq!(check_preprocess_args(None, false), Err(E_INVALIDARG));
}

#[test]
fn include_full_path_uses_the_last_backslash() {
    // Not oracle-derived: the last backslash decides the directory, and current_dir is ignored
    // once the initial filename has one.
    assert_eq!(
        include_full_path("C:\\x\\y\\main.hlsl", "inc.h", Some("C:\\cwd")),
        Ok("C:\\x\\y\\inc.h".to_owned())
    );
    assert_eq!(
        include_full_path("dir\\", "inc.h", Some("")),
        Ok("dir\\inc.h".to_owned())
    );
    assert_eq!(
        include_full_path("\\main.hlsl", "inc.h", None),
        Ok("\\inc.h".to_owned())
    );
}

#[test]
fn include_full_path_checks_the_current_directory_length() {
    // Not oracle-derived: the empty and MAX_PATH boundaries of the current directory, which the
    // C code checks before it builds the path.
    assert_eq!(
        include_full_path("main.hlsl", "inc.h", Some("")),
        Err(E_FAIL)
    );

    let longest = "a".repeat(MAX_PATH - 1);
    assert_eq!(
        include_full_path("main.hlsl", "inc.h", Some(&longest)),
        Ok(format!("{longest}\\inc.h"))
    );
    let too_long = "a".repeat(MAX_PATH);
    assert_eq!(
        include_full_path("main.hlsl", "inc.h", Some(&too_long)),
        Err(E_FAIL)
    );
}

#[test]
fn stub_record_carries_the_given_names() {
    // Not oracle-derived: wine_spec_unimplemented_stub copies its arguments into the record. The
    // oracle covers the two stubs of the spec.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Ordinal"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Ordinal",
        }
    );
}

#[test]
fn export_table_matches_the_spec() {
    // Not oracle-derived: the ten lines of dlls/d3dcompiler_36/d3dcompiler_36.spec, in spec order.
    assert_eq!(
        EXPORTS,
        [
            ("D3DCompileFromMemory", ExportKind::Stdcall("D3DCompile")),
            ("D3DDisassembleCode", ExportKind::Stdcall("D3DDisassemble")),
            ("D3DDisassembleEffect", ExportKind::Stub),
            (
                "D3DGetCodeDebugInfo",
                ExportKind::Stdcall("D3DGetDebugInfo")
            ),
            (
                "D3DGetInputAndOutputSignatureBlob",
                ExportKind::Stdcall("D3DGetInputAndOutputSignatureBlob")
            ),
            (
                "D3DGetInputSignatureBlob",
                ExportKind::Stdcall("D3DGetInputSignatureBlob")
            ),
            (
                "D3DGetOutputSignatureBlob",
                ExportKind::Stdcall("D3DGetOutputSignatureBlob")
            ),
            (
                "D3DPreprocessFromMemory",
                ExportKind::Stdcall("D3DPreprocess")
            ),
            ("D3DReflectCode", ExportKind::Stdcall("D3DReflect")),
            ("DebugSetMute", ExportKind::Stub),
        ]
    );
}

#[test]
fn export_kind_resolves_each_name() {
    // Not oracle-derived: one lookup per spec line, plus names the spec does not export.
    assert_eq!(
        export_kind("D3DCompileFromMemory"),
        Some(ExportKind::Stdcall("D3DCompile"))
    );
    assert_eq!(
        export_kind("D3DDisassembleCode"),
        Some(ExportKind::Stdcall("D3DDisassemble"))
    );
    assert_eq!(export_kind("D3DDisassembleEffect"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("D3DGetCodeDebugInfo"),
        Some(ExportKind::Stdcall("D3DGetDebugInfo"))
    );
    assert_eq!(
        export_kind("D3DGetInputAndOutputSignatureBlob"),
        Some(ExportKind::Stdcall("D3DGetInputAndOutputSignatureBlob"))
    );
    assert_eq!(
        export_kind("D3DGetInputSignatureBlob"),
        Some(ExportKind::Stdcall("D3DGetInputSignatureBlob"))
    );
    assert_eq!(
        export_kind("D3DGetOutputSignatureBlob"),
        Some(ExportKind::Stdcall("D3DGetOutputSignatureBlob"))
    );
    assert_eq!(
        export_kind("D3DPreprocessFromMemory"),
        Some(ExportKind::Stdcall("D3DPreprocess"))
    );
    assert_eq!(
        export_kind("D3DReflectCode"),
        Some(ExportKind::Stdcall("D3DReflect"))
    );
    assert_eq!(export_kind("DebugSetMute"), Some(ExportKind::Stub));
    assert_eq!(export_kind("D3DCompile"), None);
    assert_eq!(export_kind("debugsetmute"), None);
    assert_eq!(export_kind(""), None);
}
