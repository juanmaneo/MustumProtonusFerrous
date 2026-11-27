/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name from `MODULE = d3dx9_25.dll` in `dlls/d3dx9_25/Makefile.in`.
const MODULE_NAME: &str = "d3dx9_25.dll";

/// The record raised by `__wine_spec_unimplemented_stub`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Model of `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`.
/// The C function raises this record in a non-returning loop; the port returns it.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

/// `D3DXComputeTangent`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_compute_tangent() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXComputeTangent")
}

/// `D3DXComputeTangentFrame`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_compute_tangent_frame() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXComputeTangentFrame")
}

/// `D3DXConcatenateMeshes`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_concatenate_meshes() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXConcatenateMeshes")
}

/// `D3DXConvertMeshSubsetToStrips`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_convert_mesh_subset_to_strips() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXConvertMeshSubsetToStrips")
}

/// `D3DXCreateCompressedAnimationSet`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_compressed_animation_set() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateCompressedAnimationSet")
}

/// `D3DXCreateCubeTextureFromResourceA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_cube_texture_from_resource_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateCubeTextureFromResourceA")
}

/// `D3DXCreateCubeTextureFromResourceExA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_cube_texture_from_resource_ex_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateCubeTextureFromResourceExA")
}

/// `D3DXCreateCubeTextureFromResourceExW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_cube_texture_from_resource_ex_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateCubeTextureFromResourceExW")
}

/// `D3DXCreateCubeTextureFromResourceW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_cube_texture_from_resource_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateCubeTextureFromResourceW")
}

/// `D3DXCreateNPatchMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_npatch_mesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateNPatchMesh")
}

/// `D3DXCreatePMeshFromStream`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_pmesh_from_stream() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreatePMeshFromStream")
}

/// `D3DXCreatePRTBuffer`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_prtbuffer() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreatePRTBuffer")
}

/// `D3DXCreatePRTBufferTex`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_prtbuffer_tex() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreatePRTBufferTex")
}

/// `D3DXCreatePRTCompBuffer`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_prtcomp_buffer() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreatePRTCompBuffer")
}

/// `D3DXCreatePRTEngine`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_prtengine() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreatePRTEngine")
}

/// `D3DXCreatePatchMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_patch_mesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreatePatchMesh")
}

/// `D3DXCreateSPMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_spmesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateSPMesh")
}

/// `D3DXCreateSkinInfoFromBlendedMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_skin_info_from_blended_mesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateSkinInfoFromBlendedMesh")
}

/// `D3DXCreateTextureGutterHelper`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_texture_gutter_helper() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateTextureGutterHelper")
}

/// `D3DXCreateVolumeTextureFromResourceA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_volume_texture_from_resource_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateVolumeTextureFromResourceA")
}

/// `D3DXCreateVolumeTextureFromResourceExA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_volume_texture_from_resource_ex_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateVolumeTextureFromResourceExA")
}

/// `D3DXCreateVolumeTextureFromResourceExW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_volume_texture_from_resource_ex_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateVolumeTextureFromResourceExW")
}

/// `D3DXCreateVolumeTextureFromResourceW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_create_volume_texture_from_resource_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXCreateVolumeTextureFromResourceW")
}

/// `D3DXFrameAppendChild`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_frame_append_child() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXFrameAppendChild")
}

/// `D3DXFrameCalculateBoundingSphere`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_frame_calculate_bounding_sphere() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXFrameCalculateBoundingSphere")
}

/// `D3DXFrameNumNamedMatrices`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_frame_num_named_matrices() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXFrameNumNamedMatrices")
}

/// `D3DXFrameRegisterNamedMatrices`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_frame_register_named_matrices() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXFrameRegisterNamedMatrices")
}

/// `D3DXGatherFragments`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_gather_fragments() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGatherFragments")
}

/// `D3DXGatherFragmentsFromFileA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_gather_fragments_from_file_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGatherFragmentsFromFileA")
}

/// `D3DXGatherFragmentsFromFileW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_gather_fragments_from_file_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGatherFragmentsFromFileW")
}

/// `D3DXGatherFragmentsFromResourceA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_gather_fragments_from_resource_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGatherFragmentsFromResourceA")
}

/// `D3DXGatherFragmentsFromResourceW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_gather_fragments_from_resource_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGatherFragmentsFromResourceW")
}

/// `D3DXGenerateOutputDecl`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_generate_output_decl() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGenerateOutputDecl")
}

/// `D3DXGeneratePMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_generate_pmesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGeneratePMesh")
}

/// `D3DXGetTargetDescByName`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_get_target_desc_by_name() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGetTargetDescByName")
}

/// `D3DXGetTargetDescByVersion`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_get_target_desc_by_version() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXGetTargetDescByVersion")
}

/// `D3DXLoadMeshFromXof`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_mesh_from_xof() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadMeshFromXof")
}

/// `D3DXLoadPRTBufferFromFileA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_prtbuffer_from_file_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadPRTBufferFromFileA")
}

/// `D3DXLoadPRTBufferFromFileW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_prtbuffer_from_file_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadPRTBufferFromFileW")
}

/// `D3DXLoadPRTCompBufferFromFileA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_prtcomp_buffer_from_file_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadPRTCompBufferFromFileA")
}

/// `D3DXLoadPRTCompBufferFromFileW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_prtcomp_buffer_from_file_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadPRTCompBufferFromFileW")
}

/// `D3DXLoadPatchMeshFromXof`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_patch_mesh_from_xof() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadPatchMeshFromXof")
}

/// `D3DXLoadVolumeFromResourceA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_volume_from_resource_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadVolumeFromResourceA")
}

/// `D3DXLoadVolumeFromResourceW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_load_volume_from_resource_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXLoadVolumeFromResourceW")
}

/// `D3DXRectPatchSize`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_rect_patch_size() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXRectPatchSize")
}

/// `D3DXSHPRTCompSplitMeshSC`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_shprtcomp_split_mesh_sc() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSHPRTCompSplitMeshSC")
}

/// `D3DXSHPRTCompSuperCluster`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_shprtcomp_super_cluster() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSHPRTCompSuperCluster")
}

/// `D3DXSaveMeshHierarchyToFileA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_mesh_hierarchy_to_file_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSaveMeshHierarchyToFileA")
}

/// `D3DXSaveMeshHierarchyToFileW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_mesh_hierarchy_to_file_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSaveMeshHierarchyToFileW")
}

/// `D3DXSaveMeshToXA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_mesh_to_xa() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSaveMeshToXA")
}

/// `D3DXSaveMeshToXW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_mesh_to_xw() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSaveMeshToXW")
}

/// `D3DXSavePRTBufferToFileA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_prtbuffer_to_file_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSavePRTBufferToFileA")
}

/// `D3DXSavePRTBufferToFileW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_prtbuffer_to_file_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSavePRTBufferToFileW")
}

/// `D3DXSavePRTCompBufferToFileA`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_prtcomp_buffer_to_file_a() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSavePRTCompBufferToFileA")
}

/// `D3DXSavePRTCompBufferToFileW`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_save_prtcomp_buffer_to_file_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSavePRTCompBufferToFileW")
}

/// `D3DXSimplifyMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_simplify_mesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSimplifyMesh")
}

/// `D3DXSplitMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_split_mesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXSplitMesh")
}

/// `D3DXTessellateRectPatch`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_tessellate_rect_patch() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXTessellateRectPatch")
}

/// `D3DXTessellateTriPatch`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_tessellate_tri_patch() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXTessellateTriPatch")
}

/// `D3DXTriPatchSize`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_tri_patch_size() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXTriPatchSize")
}

/// `D3DXUVAtlasCreate`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_uvatlas_create() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXUVAtlasCreate")
}

/// `D3DXUVAtlasPack`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_uvatlas_pack() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXUVAtlasPack")
}

/// `D3DXUVAtlasPartition`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_uvatlas_partition() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXUVAtlasPartition")
}

/// `D3DXValidPatchMesh`, exported as `@ stub` in `dlls/d3dx9_25/d3dx9_25.spec`.
pub fn d3dx_valid_patch_mesh() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DXValidPatchMesh")
}
