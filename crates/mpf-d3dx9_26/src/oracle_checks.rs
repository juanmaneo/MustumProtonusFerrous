//! Oracle-derived test vectors for the ported d3dx9_26 stub records.
//!
//! Vectors were captured from `target/canon/d3dx9_26/harness.c`, which compiles the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` (lines 504-516) with
//! `gcc -fsanitize=address,undefined -g`. `RtlRaiseException` is shimmed to record the record and
//! `longjmp` out. The harness ran once for each of the 68 `@ stub` names in
//! `dlls/d3dx9_26/d3dx9_26.spec`, with the module name `d3dx9_26.dll` from `MODULE` in
//! `Makefile.in`. Each run raised exactly one record, and ASan and UBSan reported nothing, so no
//! vector is excluded. The `@ stdcall` bodies are not compiled, because they live in `dlls/d3dx9_36`.

use super::*;

/// Rows of `(export, ExceptionCode, ExceptionFlags, NumberParameters)` from the harness.
const ORACLE_STUB_ROWS: &[(&str, u32, u32, u32)] = &[
    (
        "D3DXComputeIMTFromPerTexelSignal",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXComputeIMTFromPerVertexSignal",
        0x80000100,
        0x00000001,
        2,
    ),
    ("D3DXComputeIMTFromSignal", 0x80000100, 0x00000001, 2),
    ("D3DXComputeIMTFromTexture", 0x80000100, 0x00000001, 2),
    ("D3DXComputeTangent", 0x80000100, 0x00000001, 2),
    ("D3DXComputeTangentFrame", 0x80000100, 0x00000001, 2),
    ("D3DXConcatenateMeshes", 0x80000100, 0x00000001, 2),
    ("D3DXConvertMeshSubsetToStrips", 0x80000100, 0x00000001, 2),
    (
        "D3DXCreateCompressedAnimationSet",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXCreateCubeTextureFromResourceA",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXCreateCubeTextureFromResourceExA",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXCreateCubeTextureFromResourceExW",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXCreateCubeTextureFromResourceW",
        0x80000100,
        0x00000001,
        2,
    ),
    ("D3DXCreateNPatchMesh", 0x80000100, 0x00000001, 2),
    ("D3DXCreatePMeshFromStream", 0x80000100, 0x00000001, 2),
    ("D3DXCreatePRTBuffer", 0x80000100, 0x00000001, 2),
    ("D3DXCreatePRTBufferTex", 0x80000100, 0x00000001, 2),
    ("D3DXCreatePRTCompBuffer", 0x80000100, 0x00000001, 2),
    ("D3DXCreatePRTEngine", 0x80000100, 0x00000001, 2),
    ("D3DXCreatePatchMesh", 0x80000100, 0x00000001, 2),
    ("D3DXCreateSPMesh", 0x80000100, 0x00000001, 2),
    (
        "D3DXCreateSkinInfoFromBlendedMesh",
        0x80000100,
        0x00000001,
        2,
    ),
    ("D3DXCreateTextureGutterHelper", 0x80000100, 0x00000001, 2),
    (
        "D3DXCreateVolumeTextureFromResourceA",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXCreateVolumeTextureFromResourceExA",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXCreateVolumeTextureFromResourceExW",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXCreateVolumeTextureFromResourceW",
        0x80000100,
        0x00000001,
        2,
    ),
    ("D3DXFrameAppendChild", 0x80000100, 0x00000001, 2),
    (
        "D3DXFrameCalculateBoundingSphere",
        0x80000100,
        0x00000001,
        2,
    ),
    ("D3DXFrameNumNamedMatrices", 0x80000100, 0x00000001, 2),
    ("D3DXFrameRegisterNamedMatrices", 0x80000100, 0x00000001, 2),
    ("D3DXGatherFragments", 0x80000100, 0x00000001, 2),
    ("D3DXGatherFragmentsFromFileA", 0x80000100, 0x00000001, 2),
    ("D3DXGatherFragmentsFromFileW", 0x80000100, 0x00000001, 2),
    (
        "D3DXGatherFragmentsFromResourceA",
        0x80000100,
        0x00000001,
        2,
    ),
    (
        "D3DXGatherFragmentsFromResourceW",
        0x80000100,
        0x00000001,
        2,
    ),
    ("D3DXGenerateOutputDecl", 0x80000100, 0x00000001, 2),
    ("D3DXGeneratePMesh", 0x80000100, 0x00000001, 2),
    ("D3DXGetTargetDescByName", 0x80000100, 0x00000001, 2),
    ("D3DXGetTargetDescByVersion", 0x80000100, 0x00000001, 2),
    ("D3DXLoadMeshFromXof", 0x80000100, 0x00000001, 2),
    ("D3DXLoadPRTBufferFromFileA", 0x80000100, 0x00000001, 2),
    ("D3DXLoadPRTBufferFromFileW", 0x80000100, 0x00000001, 2),
    ("D3DXLoadPRTCompBufferFromFileA", 0x80000100, 0x00000001, 2),
    ("D3DXLoadPRTCompBufferFromFileW", 0x80000100, 0x00000001, 2),
    ("D3DXLoadPatchMeshFromXof", 0x80000100, 0x00000001, 2),
    ("D3DXLoadVolumeFromResourceA", 0x80000100, 0x00000001, 2),
    ("D3DXLoadVolumeFromResourceW", 0x80000100, 0x00000001, 2),
    ("D3DXRectPatchSize", 0x80000100, 0x00000001, 2),
    ("D3DXSHPRTCompSplitMeshSC", 0x80000100, 0x00000001, 2),
    ("D3DXSHPRTCompSuperCluster", 0x80000100, 0x00000001, 2),
    ("D3DXSaveMeshHierarchyToFileA", 0x80000100, 0x00000001, 2),
    ("D3DXSaveMeshHierarchyToFileW", 0x80000100, 0x00000001, 2),
    ("D3DXSaveMeshToXA", 0x80000100, 0x00000001, 2),
    ("D3DXSaveMeshToXW", 0x80000100, 0x00000001, 2),
    ("D3DXSavePRTBufferToFileA", 0x80000100, 0x00000001, 2),
    ("D3DXSavePRTBufferToFileW", 0x80000100, 0x00000001, 2),
    ("D3DXSavePRTCompBufferToFileA", 0x80000100, 0x00000001, 2),
    ("D3DXSavePRTCompBufferToFileW", 0x80000100, 0x00000001, 2),
    ("D3DXSimplifyMesh", 0x80000100, 0x00000001, 2),
    ("D3DXSplitMesh", 0x80000100, 0x00000001, 2),
    ("D3DXTessellateRectPatch", 0x80000100, 0x00000001, 2),
    ("D3DXTessellateTriPatch", 0x80000100, 0x00000001, 2),
    ("D3DXTriPatchSize", 0x80000100, 0x00000001, 2),
    ("D3DXUVAtlasCreate", 0x80000100, 0x00000001, 2),
    ("D3DXUVAtlasPack", 0x80000100, 0x00000001, 2),
    ("D3DXUVAtlasPartition", 0x80000100, 0x00000001, 2),
    ("D3DXValidPatchMesh", 0x80000100, 0x00000001, 2),
];

#[test]
fn stub_records_match_oracle() {
    for &(name, code, flags, number_parameters) in ORACLE_STUB_ROWS {
        let record = stub_exception(name).expect("every oracle row is an @ stub export");
        assert_eq!(record.code, code, "{name}");
        assert_eq!(record.flags, flags, "{name}");
        assert_eq!(record.number_parameters, number_parameters, "{name}");
        assert_eq!(record.module, "d3dx9_26.dll", "{name}");
        assert_eq!(record.function, name);
    }
}

#[test]
fn oracle_covers_every_stub_export() {
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let rows: Vec<&str> = ORACLE_STUB_ROWS.iter().map(|row| row.0).collect();
    assert_eq!(rows, stubs);
}
