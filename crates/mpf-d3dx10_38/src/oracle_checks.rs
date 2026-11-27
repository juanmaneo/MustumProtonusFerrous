//! Oracle-derived vectors from `target/canon/d3dx10_38/harness.c`, which compiles the unmodified
//! `dlls/d3dx10_38/d3dx10_38_main.c` and the verbatim stub body from
//! `dlls/ntdll/exception.c` with `gcc -fsanitize=address,undefined -g`. The harness uses
//! Windows header shims and longjmps after recording the stub exception. It emitted 25 version
//! combinations and one stub record; ASan and UBSan reported no findings.

use super::*;

#[test]
fn check_version_matches_all_oracle_vectors() {
    let versions = [0, 28, 29, 30, u32::MAX];
    for d3d_sdk in versions {
        for d3dx_sdk in versions {
            let expected = d3d_sdk == 29 && d3dx_sdk == 38;
            assert_eq!(d3dx10_check_version(d3d_sdk, d3dx_sdk), expected);
        }
    }
}

#[test]
fn create_reduction_stub_matches_oracle_record() {
    assert_eq!(
        d3dx10_create_reduction(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dx10_38.dll",
            function: "D3DX10CreateReduction",
        }
    );
}
