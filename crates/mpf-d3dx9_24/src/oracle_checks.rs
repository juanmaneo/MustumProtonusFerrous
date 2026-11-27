//! Oracle vectors for the `@ stub` exports of `dlls/d3dx9_24/d3dx9_24.spec`.
//!
//! Source: `target/canon/d3dx9_24/harness.c`, built with `gcc -fsanitize=address,undefined -g`
//! and run once. It compiles `__wine_spec_unimplemented_stub` verbatim from
//! `dlls/ntdll/exception.c` (lines 504-516) and records the `EXCEPTION_RECORD` that each call
//! raises. The module name is `MODULE = d3dx9_24.dll` from `dlls/d3dx9_24/Makefile.in`. The
//! names are the `@ stub` lines of the spec. ASan and UBSan reported nothing, and every call
//! raised exactly one record with a NULL `ExceptionRecord`. The rows below are the output of
//! the harness (`vectors.txt`), in spec order.

use crate::{stub_export, StubException, STUB_EXPORTS};

/// (function, ExceptionCode, ExceptionFlags, NumberParameters, module)
const VECTORS: [(&str, u32, u32, u32, &str); 61] = [
    ("D3DXComputeTangent", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXComputeTangentFrame",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXConcatenateMeshes", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXConvertMeshSubsetToStrips",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateCompressedAnimationSet",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateCubeTextureFromResourceA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateCubeTextureFromResourceExA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateCubeTextureFromResourceExW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateCubeTextureFromResourceW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXCreateNPatchMesh", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXCreatePMeshFromStream",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXCreatePRTBuffer", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXCreatePRTBufferTex",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreatePRTCompBuffer",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXCreatePRTEngine", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    ("D3DXCreatePatchMesh", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    ("D3DXCreateSPMesh", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXCreateSkinInfoFromBlendedMesh",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateTextureGutterHelper",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateVolumeTextureFromResourceA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateVolumeTextureFromResourceExA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateVolumeTextureFromResourceExW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXCreateVolumeTextureFromResourceW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXFrameAppendChild", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXFrameCalculateBoundingSphere",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXFrameNumNamedMatrices",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXFrameRegisterNamedMatrices",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXGatherFragments", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXGatherFragmentsFromFileA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXGatherFragmentsFromFileW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXGatherFragmentsFromResourceA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXGatherFragmentsFromResourceW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXGenerateOutputDecl",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXGeneratePMesh", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXGetTargetDescByName",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXGetTargetDescByVersion",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXLoadMeshFromXof", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXLoadPRTBufferFromFileA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXLoadPRTBufferFromFileW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXLoadPRTCompBufferFromFileA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXLoadPRTCompBufferFromFileW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXLoadPatchMeshFromXof",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXLoadVolumeFromResourceA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXLoadVolumeFromResourceW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXRectPatchSize", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXSHPRTCompSplitMeshSC",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXSHPRTCompSuperCluster",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXSaveMeshHierarchyToFileA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXSaveMeshHierarchyToFileW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXSaveMeshToXA", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    ("D3DXSaveMeshToXW", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXSavePRTBufferToFileA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXSavePRTBufferToFileW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXSavePRTCompBufferToFileA",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXSavePRTCompBufferToFileW",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXSimplifyMesh", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    ("D3DXSplitMesh", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    (
        "D3DXTessellateRectPatch",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    (
        "D3DXTessellateTriPatch",
        0x80000100,
        0x01,
        2,
        "d3dx9_24.dll",
    ),
    ("D3DXTriPatchSize", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
    ("D3DXValidPatchMesh", 0x80000100, 0x01, 2, "d3dx9_24.dll"),
];

#[test]
fn stub_records_match_the_oracle() {
    assert_eq!(VECTORS.len(), STUB_EXPORTS.len());
    for (&(function, code, flags, number_parameters, module), &name) in
        VECTORS.iter().zip(STUB_EXPORTS.iter())
    {
        assert_eq!(function, name);
        assert_eq!(
            stub_export(function),
            Some(StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            })
        );
    }
}
