//! Oracle-derived test vectors for the `d3dcompiler_46` stub exports.
//!
//! Captured from `target/canon/d3dcompiler_46/harness.c`, which exercises the unmodified
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` through the nine `@ stub`
//! export records in `dlls/d3dcompiler_46/d3dcompiler_46.spec`. The harness uses Windows record
//! shims and `longjmp` to escape the stub's exception loop, and was run with
//! `gcc -fsanitize=address,undefined -g`. All nine rows recorded code `0x80000100`, flags `1`,
//! and two parameters; ASan and UBSan reported no findings.

use super::*;

#[test]
fn oracle_stub_exports_match_record_rows() {
    let cases = [
        (debug_set_mute(), "DebugSetMute"),
        (d3d_compress_shaders(), "D3DCompressShaders"),
        (d3d_decompress_shaders(), "D3DDecompressShaders"),
        (d3d_disassemble_10_effect(), "D3DDisassemble10Effect"),
        (d3d_disassemble_11_trace(), "D3DDisassemble11Trace"),
        (d3d_disassemble_region(), "D3DDisassembleRegion"),
        (
            d3d_get_trace_instruction_offsets(),
            "D3DGetTraceInstructionOffsets",
        ),
        (d3d_return_failure_1(), "D3DReturnFailure1"),
        (d3d_set_blob_part(), "D3DSetBlobPart"),
    ];

    for (record, function) in cases {
        assert_eq!(
            record,
            StubException {
                code: 0x8000_0100,
                flags: 1,
                number_parameters: 2,
                module: "d3dcompiler_46.dll",
                function,
            }
        );
    }
}
