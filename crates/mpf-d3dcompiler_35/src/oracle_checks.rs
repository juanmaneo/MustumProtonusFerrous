//! Oracle-derived test vectors for the ported d3dcompiler_35 logic.
//!
//! Vectors were captured by compiling the verbatim Wine bodies of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`), the `D3DDisassemble` size gate (`dlls/d3dcompiler_43/compiler.c`)
//! and the reflection block (`dlls/d3dcompiler_43/reflection.c`) with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/d3dcompiler_35/`, once each for `D3D_COMPILER_VERSION` 35, 46 and 0. The
//! 21 printed rows are all covered here. ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

/// The row 03 to row 10 input sizes. The version-46 build prints the same sizes.
const GATE_SIZES: [usize; 8] = [0, 1, 2, 3, 4, 16, 65536, usize::MAX];

#[test]
fn oracle_stub_rows() {
    // row 01 stub export=D3DDisassembleEffect -> raised=1 code=0x80000100 flags=0x1 nparams=2 p0=d3dcompiler_35.dll p1=D3DDisassembleEffect
    assert_eq!(
        d3d_disassemble_effect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_35.dll",
            function: "D3DDisassembleEffect",
        }
    );

    // row 02 stub export=DebugSetMute -> raised=1 code=0x80000100 flags=0x1 nparams=2 p0=d3dcompiler_35.dll p1=DebugSetMute
    assert_eq!(
        debug_set_mute(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_35.dll",
            function: "DebugSetMute",
        }
    );
}

#[test]
fn oracle_gate_rows_version_35() {
    // rows 03 to 10: ver=35 gate size=0,1,2,3,4,16,65536,SIZE_MAX -> fallthrough
    for size in GATE_SIZES {
        assert_eq!(disassemble_precheck(35, size), None, "size {size}");
    }
}

#[test]
fn oracle_gate_rows_version_46() {
    // row 01 ver=46 gate size=0 -> E_INVALIDARG
    assert_eq!(disassemble_precheck(46, 0), Some(0x8007_0057));

    // rows 02 to 08: ver=46 gate size=1,2,3,4,16,65536,SIZE_MAX -> fallthrough
    for size in &GATE_SIZES[1..] {
        assert_eq!(disassemble_precheck(46, *size), None, "size {size}");
    }
}

#[test]
fn oracle_reflect_rows() {
    // row 11 ver=35 reflect entry=D3DReflect vkd3d_version=35
    assert_eq!(
        reflect_forward(35),
        ReflectForward {
            entry: "D3DReflect",
            vkd3d_version: 35,
        }
    );

    // row 09 ver=46 reflect entry=D3DReflect vkd3d_version=46
    assert_eq!(
        reflect_forward(46),
        ReflectForward {
            entry: "D3DReflect",
            vkd3d_version: 46,
        }
    );

    // row 01 ver=0 reflect entry=D3D10ReflectShader vkd3d_version=0
    assert_eq!(
        reflect_forward(0),
        ReflectForward {
            entry: "D3D10ReflectShader",
            vkd3d_version: 0,
        }
    );
}
