/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name passed to Wine's stub handler, from `MODULE = d3dx9_27.dll`.
pub const MODULE_NAME: &str = "d3dx9_27.dll";

/// The record raised by `__wine_spec_unimplemented_stub`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Models `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`.
/// The C implementation repeatedly raises this exception and never returns.
pub fn wine_spec_unimplemented_stub(function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module: MODULE_NAME,
        function,
    }
}

/// All exports marked `@ stub` in `dlls/d3dx9_27/d3dx9_27.spec`.
pub fn stub_exports() -> &'static [&'static str] {
    &[
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
    ]
}
