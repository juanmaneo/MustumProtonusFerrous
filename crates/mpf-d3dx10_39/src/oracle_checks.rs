//! Oracle vectors captured by compiling Wine's unmodified
//! `dlls/d3dx10_39/d3dx10_39_main.c` with Windows scalar shims, plus the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c`, using
//! `gcc -fsanitize=address,undefined -g` in `target/canon/d3dx10_39/harness.c`.
//! The harness used `D3D10_SDK_VERSION = 29` and reported no ASan or UBSan findings.

use super::*;

#[test]
fn oracle_version_check_rows() {
    assert!(d3dx10_check_version(29, 39));
    assert!(!d3dx10_check_version(28, 39));
    assert!(!d3dx10_check_version(29, 38));
}

#[test]
fn oracle_stub_rows() {
    let cases = [
        (d3dx10_create_reduction(), "D3DX10CreateReduction"),
        (d3dx10_disassemble_effect(), "D3DX10DisassembleEffect"),
        (d3dx10_disassemble_shader(), "D3DX10DisassembleShader"),
        (d3dx10_reflect_shader(), "D3DX10ReflectShader"),
    ];
    for (record, function) in cases {
        assert_eq!(
            record,
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "d3dx10_39.dll",
                function,
            }
        );
    }
}
