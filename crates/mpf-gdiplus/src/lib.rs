//! Port of the portable logic in Wine's `dlls/gdiplus`: `gdiplus.c`, `matrix.c`,
//! `gdiplus_private.h`, the `NotImplemented` functions of `graphics.c`, `graphicspath.c`,
//! `image.c` and `metafile.c`, and the export table of `gdiplus.spec`.
//!
//! Ported:
//! - `GpMatrix` functions from `matrix.c`: [`Matrix`] and the `gdip_*matrix*` functions
//! - Arc, spline and unit helpers from `gdiplus.c`: [`arc2polybezier`], [`calc_curve_bezier`],
//!   [`calc_curve_bezier_endp`], [`units_to_pixels`], [`pixels_to_units`], [`units_scale`]
//! - Colour and rounding helpers from `gdiplus.c` and `gdiplus_private.h`: [`argb2colorref`],
//!   [`color_over`], [`color_over_fgpremult`], [`convert_32bpp_argb_to_32bpp_pargb`],
//!   [`gdip_round`], [`deg2rad`], [`gdiplus_atan2`], [`hresult_to_status`]
//! - Explicit functions that return `NotImplemented`: [`gdip_comment`],
//!   [`gdip_winding_mode_outline`], [`gdip_bitmap_convert_format`], [`gdip_emf_to_wmf_bits`],
//!   [`gdip_record_metafile_file_name_i`], [`gdip_convert_to_emf_plus_to_file`]
//! - `gdiplus.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`export`]
//! - `@ stub` exports: [`StubException`] and [`wine_spec_unimplemented_stub`], with the twelve
//!   per-export helpers such as [`gdip_set_pen_unit`]
//!
//! Skipped:
//! - The 8 `.c` files that are not ported: `brush.c`, `customlinecap.c`, `font.c`,
//!   `imageattributes.c`, `pathiterator.c`, `pen.c`, `region.c` and `stringformat.c`. Their
//!   functions operate on GDI+ object state (pens, brushes, fonts, regions and so on) that this
//!   port does not model.
//! - The unported parts of `graphics.c`, `graphicspath.c`, `image.c` and `metafile.c`: the
//!   functions that work on GDI+ object state, file and stream I/O, or Wine's driver interfaces.
//! - `ARGB2BMP`, which builds a GDI device-independent bitmap.
//! - `TRACE` and `FIXME` output and the `debugstr_*` formatting of arguments.
//! - The NULL checks on out parameters (`GpMatrix **`, `BOOL *`, `REAL *`): the port returns the
//!   value instead of writing through a pointer.
//! - The NULL checks on point arrays (`pts`): Rust slices cannot be NULL, so the C `!pts` checks
//!   are not representable. An empty slice takes the `count <= 0` path instead.
//! - The heap: `malloc` is modelled by the `alloc` closure of each function that allocates, and
//!   `free` is the owner dropping the value.
//! - The link-name column of `gdiplus.spec` (one line, `GdiplusShutdown_wrapper`).

mod arc;
mod color;
mod exports;
mod geometry;
mod math;
mod matrix;
mod not_implemented;
mod spline;
mod status;
mod stub;
mod units;

pub use arc::{arc2polybezier, MAX_ARC_PTS};
pub use color::{
    argb2colorref, color_over, color_over_fgpremult, convert_32bpp_argb_to_32bpp_pargb,
};
pub use exports::{export, ArgKind, Export, ExportKind, EXPORTS};
pub use geometry::{set_rect, Point, PointF, Rect, RectF};
pub use math::{deg2rad, gdip_round, gdiplus_atan2};
pub use matrix::{
    gdip_clone_matrix, gdip_create_matrix, gdip_create_matrix2, gdip_create_matrix3,
    gdip_create_matrix3_i, gdip_delete_matrix, gdip_get_matrix_elements, gdip_invert_matrix,
    gdip_is_matrix_equal, gdip_is_matrix_identity, gdip_is_matrix_invertible, gdip_multiply_matrix,
    gdip_rotate_matrix, gdip_scale_matrix, gdip_set_matrix_elements, gdip_shear_matrix,
    gdip_transform_matrix_points, gdip_transform_matrix_points_i, gdip_translate_matrix,
    gdip_vector_transform_matrix_points, gdip_vector_transform_matrix_points_i, Matrix,
};
pub use not_implemented::{
    gdip_bitmap_convert_format, gdip_comment, gdip_convert_to_emf_plus_to_file,
    gdip_emf_to_wmf_bits, gdip_record_metafile_file_name_i, gdip_winding_mode_outline,
};
pub use spline::{calc_curve_bezier, calc_curve_bezier_endp};
pub use status::{hresult_to_status, Status};
pub use stub::{
    gdip_convert_to_emf_plus_to_stream, gdip_create_bitmap_from_direct_draw_surface,
    gdip_enumerate_metafile_dest_points, gdip_enumerate_metafile_dest_points_i,
    gdip_enumerate_metafile_src_rect_dest_point, gdip_enumerate_metafile_src_rect_dest_point_i,
    gdip_enumerate_metafile_src_rect_dest_points_i, gdip_find_next_image_item,
    gdip_get_custom_line_cap_stroke_caps, gdip_get_encoder_parameter_list,
    gdip_play_ts_client_record, gdip_set_pen_unit, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use units::{pixels_to_units, units_scale, units_to_pixels};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
