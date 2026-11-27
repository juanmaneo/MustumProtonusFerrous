//! Oracle-derived checks for the `@ stub` exports.
//!
//! Vectors were captured from `target/canon/d3dx9_27/harness.c`, which models the record
//! setup and non-returning raise loop from `dlls/ntdll/exception.c`, with the longjmp escape
//! required by the brief. The harness was built and run with
//! `gcc -fsanitize=address,undefined -g`; it emitted 68 records.

use super::*;

const ORACLE_EXPORTS: &[&str] = &[
    "D3DXComputeIMTFromPerTexelSignal",
    "D3DXComputeIMTFromPerVertexSignal",
    "D3DXComputeIMTFromSignal",
    "D3DXComputeIMTFromTexture",
    "D3DXComputeTangent",
    "D3DXComputeTangentFrame",
    "D3DXConcatenateMeshes",
    "D3DXConvertMeshSubsetToStrips",
    "D3DXCreateCompressedAnimationSet",
    "D3DXCreateCubeTextureFromResourceA",
    "D3DXCreateCubeTextureFromResourceExA",
    "D3DXCreateCubeTextureFromResourceExW",
    "D3DXCreateCubeTextureFromResourceW",
    "D3DXCreateNPatchMesh",
    "D3DXCreatePMeshFromStream",
    "D3DXCreatePRTBuffer",
    "D3DXCreatePRTBufferTex",
    "D3DXCreatePRTCompBuffer",
    "D3DXCreatePRTEngine",
    "D3DXCreatePatchMesh",
    "D3DXCreateSPMesh",
    "D3DXCreateSkinInfoFromBlendedMesh",
    "D3DXCreateTextureGutterHelper",
    "D3DXCreateVolumeTextureFromResourceA",
    "D3DXCreateVolumeTextureFromResourceExA",
    "D3DXCreateVolumeTextureFromResourceExW",
    "D3DXCreateVolumeTextureFromResourceW",
    "D3DXFrameAppendChild",
    "D3DXFrameCalculateBoundingSphere",
    "D3DXFrameNumNamedMatrices",
    "D3DXFrameRegisterNamedMatrices",
    "D3DXGatherFragments",
    "D3DXGatherFragmentsFromFileA",
    "D3DXGatherFragmentsFromFileW",
    "D3DXGatherFragmentsFromResourceA",
    "D3DXGatherFragmentsFromResourceW",
    "D3DXGenerateOutputDecl",
    "D3DXGeneratePMesh",
    "D3DXGetTargetDescByName",
    "D3DXGetTargetDescByVersion",
    "D3DXLoadMeshFromXof",
    "D3DXLoadPRTBufferFromFileA",
    "D3DXLoadPRTBufferFromFileW",
    "D3DXLoadPRTCompBufferFromFileA",
    "D3DXLoadPRTCompBufferFromFileW",
    "D3DXLoadPatchMeshFromXof",
    "D3DXLoadVolumeFromResourceA",
    "D3DXLoadVolumeFromResourceW",
    "D3DXRectPatchSize",
    "D3DXSHPRTCompSplitMeshSC",
    "D3DXSHPRTCompSuperCluster",
    "D3DXSaveMeshHierarchyToFileA",
    "D3DXSaveMeshHierarchyToFileW",
    "D3DXSaveMeshToXA",
    "D3DXSaveMeshToXW",
    "D3DXSavePRTBufferToFileA",
    "D3DXSavePRTBufferToFileW",
    "D3DXSavePRTCompBufferToFileA",
    "D3DXSavePRTCompBufferToFileW",
    "D3DXSimplifyMesh",
    "D3DXSplitMesh",
    "D3DXTessellateRectPatch",
    "D3DXTessellateTriPatch",
    "D3DXTriPatchSize",
    "D3DXUVAtlasCreate",
    "D3DXUVAtlasPack",
    "D3DXUVAtlasPartition",
    "D3DXValidPatchMesh",
];

#[test]
fn all_stub_records_match_the_harness_vectors() {
    assert_eq!(stub_exports(), ORACLE_EXPORTS);
    for &function in ORACLE_EXPORTS {
        assert_eq!(
            wine_spec_unimplemented_stub(function),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "d3dx9_27.dll",
                function,
            }
        );
    }
}
