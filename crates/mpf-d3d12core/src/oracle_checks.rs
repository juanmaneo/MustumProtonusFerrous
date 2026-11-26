//! Oracle-derived test vectors for the ported d3d12core stub model.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/d3d12core/`, and running the harness. It printed 18 rows. Rows 01-02 call the two
//! `@ stub` exports the way winebuild emits them. Rows 03-18 call the verbatim helper directly with
//! other (module, function) pairs. ASan and UBSan flagged no row, so none is excluded.

use super::*;

/// The record the C harness printed for every row: code 0x80000100, flags 1, two parameters.
fn raised(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module,
        function,
    }
}

#[test]
fn oracle_export_rows() {
    // row 01 call=export -> module=[d3d12core.dll] function=[D3D12GetInterface]
    assert_eq!(
        d3d12_get_interface(),
        raised("d3d12core.dll", "D3D12GetInterface")
    );
    // row 02 call=export -> module=[d3d12core.dll] function=[D3D12SDKVersion]
    assert_eq!(
        d3d12_sdk_version(),
        raised("d3d12core.dll", "D3D12SDKVersion")
    );
}

#[test]
fn oracle_direct_rows() {
    // Rows 03-18, call=direct: the verbatim helper called with each (module, function) pair.
    let rows: [(&'static str, &'static str); 16] = [
        ("d3d12core.dll", ""),                   // row 03
        ("", "D3D12GetInterface"),               // row 04
        ("", ""),                                // row 05
        ("other.dll", "Foo"),                    // row 06
        ("d3d12core.dll", "d3d12getinterface"),  // row 07
        ("D3D12CORE.DLL", "D3D12SDKVERSION"),    // row 08
        ("d3d12core.dll", "D3D12GetInterface "), // row 09
        ("d3d12core.dll", " D3D12GetInterface"), // row 10
        ("d3d12core.dll", "17"),                 // row 11
        ("d3d12.dll", "D3D12GetInterface"),      // row 12
        ("d3d12core.dll", "D3D12CreateDevice"),  // row 13
        ("kernel32.dll", "GetProcAddress"),      // row 14
        ("ntdll.dll", "RtlRaiseException"),      // row 15
        (
            "d3d12core.dll",
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        ), // row 16
        ("d3d12core\u{e9}.dll", "\u{e9}"),       // row 17
        ("d3d12core.dll", " "),                  // row 18
    ];
    for (module, function) in rows {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            raised(module, function)
        );
    }
}
