//! Oracle-derived test vectors for the ported d3dcompiler_41 stub records.
//!
//! Vectors were captured from `target/canon/d3dcompiler_41/harness.c`, which includes the
//! verbatim `__wine_spec_unimplemented_stub` body (`stub_verbatim.inc`, lines 504-516 of
//! `dlls/ntdll/exception.c`) and runs it under `gcc -fsanitize=address,undefined -g`. Each
//! stub raised one record, and the RtlRaiseException shim longjmp'd out. ASan and UBSan
//! reported nothing, so no vector is excluded.
//!
//! The C bodies of the stdcall exports are in `dlls/d3dcompiler_43` (PARENTSRC), so there
//! are no oracle vectors for them here.

use super::*;

/// Captured stub output: `(function, code, flags, number_parameters, module, function)`.
const ORACLE: [(&str, u32, u32, u32, &str, &str); 3] = [
    (
        "DebugSetMute",
        0x8000_0100,
        1,
        2,
        "d3dcompiler_41.dll",
        "DebugSetMute",
    ),
    (
        "D3DDisassemble10Effect",
        0x8000_0100,
        1,
        2,
        "d3dcompiler_41.dll",
        "D3DDisassemble10Effect",
    ),
    (
        "D3DReturnFailure1",
        0x8000_0100,
        1,
        2,
        "d3dcompiler_41.dll",
        "D3DReturnFailure1",
    ),
];

fn record(function: &str) -> StubException {
    stub_exception(function).expect("oracle stub")
}

#[test]
fn stub_records_match_the_harness() {
    for (function, code, flags, params, module, name) in ORACLE {
        let got = record(function);
        assert_eq!(got.code, code, "{function} code");
        assert_eq!(got.flags, flags, "{function} flags");
        assert_eq!(got.number_parameters, params, "{function} parameters");
        assert_eq!(got.module, module, "{function} module");
        assert_eq!(got.function, name, "{function} function name");
    }
}

#[test]
fn stub_functions_match_the_harness() {
    assert_eq!(debug_set_mute(), record("DebugSetMute"));
    assert_eq!(d3d_disassemble10_effect(), record("D3DDisassemble10Effect"));
    assert_eq!(d3d_return_failure1(), record("D3DReturnFailure1"));
}

#[test]
fn spec_stubs_match_the_harness() {
    for (function, ..) in ORACLE {
        let entry = export(function).expect("spec entry");
        assert_eq!(entry.kind, Kind::Stub, "{function} must be a @ stub");
    }
}
