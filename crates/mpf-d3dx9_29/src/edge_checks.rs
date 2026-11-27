//! Edge-case checks for the stub record constructor.

use super::*;

#[test]
fn general_stub_constructor_preserves_both_names() {
    let record = wine_spec_unimplemented_stub("other-module.dll", "ExampleExport");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);
    assert_eq!(record.module, "other-module.dll");
    assert_eq!(record.function, "ExampleExport");
}

#[test]
fn module_name_and_stub_table_are_consistent() {
    assert_eq!(D3DX9_29_DLL_NAME, "d3dx9_29.dll");
    assert_eq!(STUB_EXPORTS.len(), 68);
    let record = d3dx9_29_stub("D3DXValidPatchMesh");
    assert_eq!(record.module, D3DX9_29_DLL_NAME);
    assert_eq!(record.function, "D3DXValidPatchMesh");
}
