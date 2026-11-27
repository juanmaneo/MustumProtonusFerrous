/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = d3dx9_29.dll` in `dlls/d3dx9_29/Makefile.in`.
pub const D3DX9_29_DLL_NAME: &str = "d3dx9_29.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// A call to the `@ stub` export `function` of `d3dx9_29.dll`.
pub fn d3dx9_29_stub(function: &'static str) -> StubException {
    wine_spec_unimplemented_stub(D3DX9_29_DLL_NAME, function)
}

/// The 68 `@ stub` exports of `dlls/d3dx9_29/d3dx9_29.spec`, in spec order.
pub const STUB_EXPORTS: [&str; 68] = [
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
