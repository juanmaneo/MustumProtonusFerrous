//! Oracle-derived test vectors for the `d3dx9_25` stub records.
//!
//! Captured by `target/canon/d3dx9_25/harness.c`, which compiles the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` with Windows
//! type shims and a `RtlRaiseException` shim that records the exception and `longjmp`s.
//! The harness compiled and ran with `gcc -fsanitize=address,undefined -g`; all 64 stub
//! rows matched, with no sanitizer findings.

use super::*;

type OracleRow = (&'static str, fn() -> StubException);

const ORACLE_ROWS: [OracleRow; 64] = [
    (
        "D3DXComputeTangent",
        d3dx_compute_tangent as fn() -> StubException,
    ),
    (
        "D3DXComputeTangentFrame",
        d3dx_compute_tangent_frame as fn() -> StubException,
    ),
    (
        "D3DXConcatenateMeshes",
        d3dx_concatenate_meshes as fn() -> StubException,
    ),
    (
        "D3DXConvertMeshSubsetToStrips",
        d3dx_convert_mesh_subset_to_strips as fn() -> StubException,
    ),
    (
        "D3DXCreateCompressedAnimationSet",
        d3dx_create_compressed_animation_set as fn() -> StubException,
    ),
    (
        "D3DXCreateCubeTextureFromResourceA",
        d3dx_create_cube_texture_from_resource_a as fn() -> StubException,
    ),
    (
        "D3DXCreateCubeTextureFromResourceExA",
        d3dx_create_cube_texture_from_resource_ex_a as fn() -> StubException,
    ),
    (
        "D3DXCreateCubeTextureFromResourceExW",
        d3dx_create_cube_texture_from_resource_ex_w as fn() -> StubException,
    ),
    (
        "D3DXCreateCubeTextureFromResourceW",
        d3dx_create_cube_texture_from_resource_w as fn() -> StubException,
    ),
    (
        "D3DXCreateNPatchMesh",
        d3dx_create_npatch_mesh as fn() -> StubException,
    ),
    (
        "D3DXCreatePMeshFromStream",
        d3dx_create_pmesh_from_stream as fn() -> StubException,
    ),
    (
        "D3DXCreatePRTBuffer",
        d3dx_create_prtbuffer as fn() -> StubException,
    ),
    (
        "D3DXCreatePRTBufferTex",
        d3dx_create_prtbuffer_tex as fn() -> StubException,
    ),
    (
        "D3DXCreatePRTCompBuffer",
        d3dx_create_prtcomp_buffer as fn() -> StubException,
    ),
    (
        "D3DXCreatePRTEngine",
        d3dx_create_prtengine as fn() -> StubException,
    ),
    (
        "D3DXCreatePatchMesh",
        d3dx_create_patch_mesh as fn() -> StubException,
    ),
    (
        "D3DXCreateSPMesh",
        d3dx_create_spmesh as fn() -> StubException,
    ),
    (
        "D3DXCreateSkinInfoFromBlendedMesh",
        d3dx_create_skin_info_from_blended_mesh as fn() -> StubException,
    ),
    (
        "D3DXCreateTextureGutterHelper",
        d3dx_create_texture_gutter_helper as fn() -> StubException,
    ),
    (
        "D3DXCreateVolumeTextureFromResourceA",
        d3dx_create_volume_texture_from_resource_a as fn() -> StubException,
    ),
    (
        "D3DXCreateVolumeTextureFromResourceExA",
        d3dx_create_volume_texture_from_resource_ex_a as fn() -> StubException,
    ),
    (
        "D3DXCreateVolumeTextureFromResourceExW",
        d3dx_create_volume_texture_from_resource_ex_w as fn() -> StubException,
    ),
    (
        "D3DXCreateVolumeTextureFromResourceW",
        d3dx_create_volume_texture_from_resource_w as fn() -> StubException,
    ),
    (
        "D3DXFrameAppendChild",
        d3dx_frame_append_child as fn() -> StubException,
    ),
    (
        "D3DXFrameCalculateBoundingSphere",
        d3dx_frame_calculate_bounding_sphere as fn() -> StubException,
    ),
    (
        "D3DXFrameNumNamedMatrices",
        d3dx_frame_num_named_matrices as fn() -> StubException,
    ),
    (
        "D3DXFrameRegisterNamedMatrices",
        d3dx_frame_register_named_matrices as fn() -> StubException,
    ),
    (
        "D3DXGatherFragments",
        d3dx_gather_fragments as fn() -> StubException,
    ),
    (
        "D3DXGatherFragmentsFromFileA",
        d3dx_gather_fragments_from_file_a as fn() -> StubException,
    ),
    (
        "D3DXGatherFragmentsFromFileW",
        d3dx_gather_fragments_from_file_w as fn() -> StubException,
    ),
    (
        "D3DXGatherFragmentsFromResourceA",
        d3dx_gather_fragments_from_resource_a as fn() -> StubException,
    ),
    (
        "D3DXGatherFragmentsFromResourceW",
        d3dx_gather_fragments_from_resource_w as fn() -> StubException,
    ),
    (
        "D3DXGenerateOutputDecl",
        d3dx_generate_output_decl as fn() -> StubException,
    ),
    (
        "D3DXGeneratePMesh",
        d3dx_generate_pmesh as fn() -> StubException,
    ),
    (
        "D3DXGetTargetDescByName",
        d3dx_get_target_desc_by_name as fn() -> StubException,
    ),
    (
        "D3DXGetTargetDescByVersion",
        d3dx_get_target_desc_by_version as fn() -> StubException,
    ),
    (
        "D3DXLoadMeshFromXof",
        d3dx_load_mesh_from_xof as fn() -> StubException,
    ),
    (
        "D3DXLoadPRTBufferFromFileA",
        d3dx_load_prtbuffer_from_file_a as fn() -> StubException,
    ),
    (
        "D3DXLoadPRTBufferFromFileW",
        d3dx_load_prtbuffer_from_file_w as fn() -> StubException,
    ),
    (
        "D3DXLoadPRTCompBufferFromFileA",
        d3dx_load_prtcomp_buffer_from_file_a as fn() -> StubException,
    ),
    (
        "D3DXLoadPRTCompBufferFromFileW",
        d3dx_load_prtcomp_buffer_from_file_w as fn() -> StubException,
    ),
    (
        "D3DXLoadPatchMeshFromXof",
        d3dx_load_patch_mesh_from_xof as fn() -> StubException,
    ),
    (
        "D3DXLoadVolumeFromResourceA",
        d3dx_load_volume_from_resource_a as fn() -> StubException,
    ),
    (
        "D3DXLoadVolumeFromResourceW",
        d3dx_load_volume_from_resource_w as fn() -> StubException,
    ),
    (
        "D3DXRectPatchSize",
        d3dx_rect_patch_size as fn() -> StubException,
    ),
    (
        "D3DXSHPRTCompSplitMeshSC",
        d3dx_shprtcomp_split_mesh_sc as fn() -> StubException,
    ),
    (
        "D3DXSHPRTCompSuperCluster",
        d3dx_shprtcomp_super_cluster as fn() -> StubException,
    ),
    (
        "D3DXSaveMeshHierarchyToFileA",
        d3dx_save_mesh_hierarchy_to_file_a as fn() -> StubException,
    ),
    (
        "D3DXSaveMeshHierarchyToFileW",
        d3dx_save_mesh_hierarchy_to_file_w as fn() -> StubException,
    ),
    (
        "D3DXSaveMeshToXA",
        d3dx_save_mesh_to_xa as fn() -> StubException,
    ),
    (
        "D3DXSaveMeshToXW",
        d3dx_save_mesh_to_xw as fn() -> StubException,
    ),
    (
        "D3DXSavePRTBufferToFileA",
        d3dx_save_prtbuffer_to_file_a as fn() -> StubException,
    ),
    (
        "D3DXSavePRTBufferToFileW",
        d3dx_save_prtbuffer_to_file_w as fn() -> StubException,
    ),
    (
        "D3DXSavePRTCompBufferToFileA",
        d3dx_save_prtcomp_buffer_to_file_a as fn() -> StubException,
    ),
    (
        "D3DXSavePRTCompBufferToFileW",
        d3dx_save_prtcomp_buffer_to_file_w as fn() -> StubException,
    ),
    (
        "D3DXSimplifyMesh",
        d3dx_simplify_mesh as fn() -> StubException,
    ),
    ("D3DXSplitMesh", d3dx_split_mesh as fn() -> StubException),
    (
        "D3DXTessellateRectPatch",
        d3dx_tessellate_rect_patch as fn() -> StubException,
    ),
    (
        "D3DXTessellateTriPatch",
        d3dx_tessellate_tri_patch as fn() -> StubException,
    ),
    (
        "D3DXTriPatchSize",
        d3dx_tri_patch_size as fn() -> StubException,
    ),
    (
        "D3DXUVAtlasCreate",
        d3dx_uvatlas_create as fn() -> StubException,
    ),
    (
        "D3DXUVAtlasPack",
        d3dx_uvatlas_pack as fn() -> StubException,
    ),
    (
        "D3DXUVAtlasPartition",
        d3dx_uvatlas_partition as fn() -> StubException,
    ),
    (
        "D3DXValidPatchMesh",
        d3dx_valid_patch_mesh as fn() -> StubException,
    ),
];

#[test]
fn oracle_stub_rows_match_the_c_harness() {
    for (function, stub) in ORACLE_ROWS {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "d3dx9_25.dll",
                function,
            },
            "{function}",
        );
    }
}
