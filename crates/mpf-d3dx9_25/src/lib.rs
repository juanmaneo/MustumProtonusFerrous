//! Port of the portable logic in Wine's `dlls/d3dx9_25/d3dx9_25.spec`.
//!
//! Ported:
//! - All 64 `@ stub` exports, which winebuild routes through
//!   `__wine_spec_unimplemented_stub`: [`wine_spec_unimplemented_stub`], [`StubException`],
//!   and the per-export functions in [`stub`].
//!
//! Skipped:
//! - The other 259 exports. `dlls/d3dx9_25/Makefile.in` sets `PARENTSRC = ../d3dx9_36`, so their
//!   C bodies live in `dlls/d3dx9_36` and are outside this component's local portable logic.
//! - The stub's exception address is not represented as a process-specific function pointer;
//!   the record's code, flags, parameter count, module and export name are modelled instead.

mod stub;

pub use stub::{
    d3dx_compute_tangent, d3dx_compute_tangent_frame, d3dx_concatenate_meshes,
    d3dx_convert_mesh_subset_to_strips, d3dx_create_compressed_animation_set,
    d3dx_create_cube_texture_from_resource_a, d3dx_create_cube_texture_from_resource_ex_a,
    d3dx_create_cube_texture_from_resource_ex_w, d3dx_create_cube_texture_from_resource_w,
    d3dx_create_npatch_mesh, d3dx_create_patch_mesh, d3dx_create_pmesh_from_stream,
    d3dx_create_prtbuffer, d3dx_create_prtbuffer_tex, d3dx_create_prtcomp_buffer,
    d3dx_create_prtengine, d3dx_create_skin_info_from_blended_mesh, d3dx_create_spmesh,
    d3dx_create_texture_gutter_helper, d3dx_create_volume_texture_from_resource_a,
    d3dx_create_volume_texture_from_resource_ex_a, d3dx_create_volume_texture_from_resource_ex_w,
    d3dx_create_volume_texture_from_resource_w, d3dx_frame_append_child,
    d3dx_frame_calculate_bounding_sphere, d3dx_frame_num_named_matrices,
    d3dx_frame_register_named_matrices, d3dx_gather_fragments, d3dx_gather_fragments_from_file_a,
    d3dx_gather_fragments_from_file_w, d3dx_gather_fragments_from_resource_a,
    d3dx_gather_fragments_from_resource_w, d3dx_generate_output_decl, d3dx_generate_pmesh,
    d3dx_get_target_desc_by_name, d3dx_get_target_desc_by_version, d3dx_load_mesh_from_xof,
    d3dx_load_patch_mesh_from_xof, d3dx_load_prtbuffer_from_file_a,
    d3dx_load_prtbuffer_from_file_w, d3dx_load_prtcomp_buffer_from_file_a,
    d3dx_load_prtcomp_buffer_from_file_w, d3dx_load_volume_from_resource_a,
    d3dx_load_volume_from_resource_w, d3dx_rect_patch_size, d3dx_save_mesh_hierarchy_to_file_a,
    d3dx_save_mesh_hierarchy_to_file_w, d3dx_save_mesh_to_xa, d3dx_save_mesh_to_xw,
    d3dx_save_prtbuffer_to_file_a, d3dx_save_prtbuffer_to_file_w,
    d3dx_save_prtcomp_buffer_to_file_a, d3dx_save_prtcomp_buffer_to_file_w,
    d3dx_shprtcomp_split_mesh_sc, d3dx_shprtcomp_super_cluster, d3dx_simplify_mesh,
    d3dx_split_mesh, d3dx_tessellate_rect_patch, d3dx_tessellate_tri_patch, d3dx_tri_patch_size,
    d3dx_uvatlas_create, d3dx_uvatlas_pack, d3dx_uvatlas_partition, d3dx_valid_patch_mesh,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
