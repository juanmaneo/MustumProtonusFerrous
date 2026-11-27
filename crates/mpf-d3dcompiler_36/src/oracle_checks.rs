//! Oracle-derived test vectors for the ported d3dcompiler_36 logic.
//!
//! Vectors were captured by running the verbatim `d3dcompiler_43` C bodies of `hresult_from_vkd3d_result`,
//! `check_blob_part` and `d3dcompiler_get_blob_part`, the include path of `d3dcompiler_include_from_file_open`,
//! the `D3DDisassemble` source choice and the `D3DPreprocess` gate, plus the verbatim
//! `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The harness is
//! `target/canon/d3dcompiler_36/harness.c`, built with `gcc -fsanitize=address,undefined -g`. It printed
//! 24 rows, and ASan and UBSan flagged none of them, so no vector is excluded.
//!
//! The vectors that `get_blob_part` tests need a parsed container. The harness supplies its sections
//! as fixtures, with the payload `payload{i}` for section `i` (printed as `pay{i}`), and the port takes the same sections.
//! The C results of the include open are E_FAIL, because the harness stops the open after it builds the path.
//! The port returns the built path, and the open failure is outside the port.

use super::*;
use crate::blob::{TAG_ISGN, TAG_OSG5, TAG_OSGN, TAG_PCSG, TAG_SDBG};

/// A 32-byte DXBC header-sized input. Only its length matters to the port.
const DATA: &[u8] = &[0; 32];

#[test]
fn oracle_stubs() {
    // row 01 stub D3DDisassembleEffect -> raise=1 code=0x80000100 flags=0x1 nargs=2 module=d3dcompiler_36.dll function=D3DDisassembleEffect
    assert_eq!(
        d3d_disassemble_effect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_36.dll",
            function: "D3DDisassembleEffect",
        }
    );

    // row 02 stub DebugSetMute -> raise=1 code=0x80000100 flags=0x1 nargs=2 module=d3dcompiler_36.dll function=DebugSetMute
    assert_eq!(
        debug_set_mute(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_36.dll",
            function: "DebugSetMute",
        }
    );
}

#[test]
fn oracle_hresult_from_vkd3d_result_rows() {
    // row 03 vkd3d=0 -> hr=0x00000000
    assert_eq!(hresult_from_vkd3d_result(0), 0x0000_0000);
    // row 04 vkd3d=-1 -> hr=0x80004005
    assert_eq!(hresult_from_vkd3d_result(-1), 0x8000_4005);
    // row 05 vkd3d=-2 -> hr=0x8007000e
    assert_eq!(hresult_from_vkd3d_result(-2), 0x8007_000e);
    // row 06 vkd3d=-3 -> hr=0x80070057
    assert_eq!(hresult_from_vkd3d_result(-3), 0x8007_0057);
    // row 07 vkd3d=-4 -> hr=0x80004005
    assert_eq!(hresult_from_vkd3d_result(-4), 0x8000_4005);
    // row 08 vkd3d=-5 -> hr=0x80004001
    assert_eq!(hresult_from_vkd3d_result(-5), 0x8000_4001);
    // row 09 vkd3d=-6 -> hr=0x80004005
    assert_eq!(hresult_from_vkd3d_result(-6), 0x8000_4005);
}

#[test]
fn oracle_disassemble_source_rows() {
    // row 10 disassemble DXB size=3 -> source=D3D_BYTECODE
    assert_eq!(disassemble_source_type(b"DXB"), SourceType::D3dBytecode);
    // row 11 disassemble DXBC size=4 -> source=DXBC_TPF
    assert_eq!(disassemble_source_type(b"DXBC"), SourceType::DxbcTpf);
}

#[test]
fn oracle_preprocess_gate_rows() {
    // row 12 preprocess data=NULL shader=set -> hr=0x80070057 reached_vkd3d=0
    assert_eq!(check_preprocess_args(None, true), Err(0x8007_0057));
    // row 13 preprocess data=set shader=NULL -> hr=0x80070057 reached_vkd3d=0
    assert_eq!(check_preprocess_args(Some(DATA), false), Err(0x8007_0057));
}

#[test]
fn oracle_include_path_rows() {
    // row 14 include initial=C:\dir\main.hlsl cwd_ret=0 name=inc.h -> opened=C:\dir\inc.h
    // (the C open then fails with E_FAIL; the port returns the path only)
    assert_eq!(
        include_full_path("C:\\dir\\main.hlsl", "inc.h", None),
        Ok("C:\\dir\\inc.h".to_owned())
    );
    // row 15 include initial=main.hlsl cwd_ret=7 name=inc.h -> opened=C:\work\inc.h
    assert_eq!(
        include_full_path("main.hlsl", "inc.h", Some("C:\\work")),
        Ok("C:\\work\\inc.h".to_owned())
    );
    // row 16 include initial=main.hlsl cwd_ret=0 name=inc.h -> hr=0x80004005 opened=<none>
    assert_eq!(
        include_full_path("main.hlsl", "inc.h", None),
        Err(0x8000_4005)
    );
}

#[test]
fn oracle_blob_argument_rows() {
    // row 17 part=INPUT_SIG data=NULL -> hr=0x8876086c path=none
    assert_eq!(
        get_blob_part(None, 0, D3D_BLOB_INPUT_SIGNATURE_BLOB, true, Some(&[])),
        Err(0x8876_086c)
    );
    // row 18 part=INPUT_SIG size=31 -> hr=0x8876086c path=none
    assert_eq!(
        get_blob_part(
            Some(&DATA[..31]),
            0,
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            true,
            Some(&[])
        ),
        Err(0x8876_086c)
    );
    // row 19 part=13 (gap) -> hr=0x8876086c path=none
    assert_eq!(
        get_blob_part(Some(DATA), 0, 13, true, Some(&[])),
        Err(0x8876_086c)
    );
}

#[test]
fn oracle_blob_section_rows() {
    let isgn = Section {
        tag: TAG_ISGN,
        data: b"pay0",
    };
    let osgn = Section {
        tag: TAG_OSGN,
        data: b"pay1",
    };
    let osg5 = Section {
        tag: TAG_OSG5,
        data: b"pay1",
    };
    let pcsg = Section {
        tag: TAG_PCSG,
        data: b"pay2",
    };
    let sdbg = Section {
        tag: TAG_SDBG,
        data: b"pay0",
    };

    // row 20 part=INPUT_SIG tags=ISGN -> hr=0x00000000 path=container sections=ISGN
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_INPUT_SIGNATURE_BLOB,
            true,
            Some(&[isgn])
        ),
        Ok(BlobPart::Container(vec![isgn]))
    );

    // row 21 part=ALL tags=ISGN,OSGN,PCSG -> hr=0x00000000 path=container sections=ISGN,OSGN,PCSG
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_ALL_SIGNATURE_BLOB,
            true,
            Some(&[isgn, osgn, pcsg])
        ),
        Ok(BlobPart::Container(vec![isgn, osgn, pcsg]))
    );

    // row 22 part=DEBUG_INFO tags=SDBG -> hr=0x00000000 path=verbatim blob=4:pay0
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_DEBUG_INFO, true, Some(&[sdbg])),
        Ok(BlobPart::Verbatim(b"pay0"))
    );

    // row 23 part=OUTPUT_SIG tags=OSGN,OSG5 -> hr=0x80004005 path=none
    assert_eq!(
        get_blob_part(
            Some(DATA),
            0,
            D3D_BLOB_OUTPUT_SIGNATURE_BLOB,
            true,
            Some(&[osgn, osg5])
        ),
        Err(0x8000_4005)
    );

    // row 24 part=PDB tags=SDBG -> hr=0x80004005 path=none
    assert_eq!(
        get_blob_part(Some(DATA), 0, D3D_BLOB_PDB, true, Some(&[sdbg])),
        Err(0x8000_4005)
    );
}
