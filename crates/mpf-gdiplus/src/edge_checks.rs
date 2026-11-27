//! Boundary and branch tests for the ported gdiplus logic.
//!
//! Each expectation says where it comes from. "C source" means a branch, constant or argument
//! check in `dlls/gdiplus`. "Hand-computed" means worked out from the C formula. "UB" means the C
//! code has undefined behaviour at that input; the value is the port's own result, not oracle
//! output. None of these expectations is oracle-derived.

use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use super::*;

fn identity() -> Matrix {
    Matrix {
        elements: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    }
}

fn approx(actual: f32, expected: f32) -> bool {
    (actual - expected).abs() <= 1e-6
}

fn alloc_fail(_size: usize) -> bool {
    false
}

fn alloc_for_two_points(size: usize) -> bool {
    size == 2 * std::mem::size_of::<PointF>()
}

fn alloc_ok(_size: usize) -> bool {
    true
}

#[test]
fn export_table_matches_gdiplus_spec() {
    // C source: gdiplus.spec has 630 entries, ordinals 1..=630 with no gaps, 12 `@ stub`
    // entries and 618 `@ stdcall` entries.
    assert_eq!(EXPORTS.len(), 630);
    for (i, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), i + 1);
    }
    let stubs = EXPORTS
        .iter()
        .filter(|e| e.kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, 12);
    let stdcall = EXPORTS
        .iter()
        .filter(|e| matches!(e.kind, ExportKind::Stdcall(_)))
        .count();
    assert_eq!(stdcall, 618);
}

#[test]
fn export_names_are_unique_and_stdcall_sizes_are_word_multiples() {
    let mut names = std::collections::HashSet::new();
    for export in &EXPORTS {
        assert!(names.insert(export.name), "duplicate {}", export.name);
        if let Some(bytes) = export.stdcall_bytes() {
            assert_eq!(bytes % 4, 0, "{}", export.name);
        }
    }
}

#[test]
fn export_lookup_is_exact_and_case_sensitive() {
    // Ordinal 98 is `98 stdcall GdipCreateMatrix2` in gdiplus.spec.
    assert_eq!(export("GdipCreateMatrix2").map(|e| e.ordinal), Some(98));
    assert_eq!(export("gdipcreatematrix2"), None);
    assert_eq!(export("GdipCreateMatri"), None);
    assert_eq!(export(""), None);
    assert_eq!(
        export("GdipSetPenUnit").map(|e| e.kind),
        Some(ExportKind::Stub)
    );
}

#[test]
fn stdcall_bytes_follow_the_spec_signatures() {
    // C source: the i386 stack bytes of each signature in gdiplus.spec, int128 taking 16 bytes.
    let bytes = |name: &str| export(name).and_then(|e| e.stdcall_bytes());
    assert_eq!(bytes("GdipCreateMatrix2"), Some(28)); // six floats and a pointer
    assert_eq!(bytes("GdiplusStartup"), Some(12)); // three pointers
    assert_eq!(bytes("GdipCreateEffect"), Some(20)); // int128 and a pointer
    assert_eq!(bytes("GdipConvertToEmfPlus"), Some(24)); // ptr ptr ptr long wstr ptr
    assert_eq!(bytes("GdipSetPenUnit"), None);
}

#[test]
fn gdip_round_rounds_half_up() {
    // C source: (INT) floorf(x + 0.5).
    assert_eq!(gdip_round(0.5), 1);
    assert_eq!(gdip_round(-0.5), 0);
    assert_eq!(gdip_round(-0.75), -1);
    // Hand-computed: the largest f32 below 2^31 and its negative are in range.
    assert_eq!(gdip_round(2147483520.0), 2147483520);
    assert_eq!(gdip_round(-2147483520.0), -2147483520);
}

#[test]
fn gdip_round_maps_nan_infinity_and_out_of_range_to_i32_min() {
    // UB in C: the conversion to INT is out of range. The port returns i32::MIN, the cvttss2si
    // result on x86-64. Not oracle-derived.
    for x in [
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        2147483648.0,
        3e9,
        -3e9,
    ] {
        assert_eq!(gdip_round(x), i32::MIN);
    }
}

#[test]
fn gdiplus_atan2_handles_axes_and_the_origin() {
    // C source: dx == 0 gives +/- pi/2 by the sign of dy; the origin gives 0.
    assert!(approx(gdiplus_atan2(1.0, 0.0), FRAC_PI_2));
    assert!(approx(gdiplus_atan2(-1.0, 0.0), -FRAC_PI_2));
    assert!(approx(gdiplus_atan2(0.0, 1.0), 0.0));
    assert!(approx(gdiplus_atan2(0.0, -1.0), PI));
    // Hand-computed: the diagonals of the quadrants that the branch does not special-case.
    assert!(approx(gdiplus_atan2(1.0, -1.0), 3.0 * FRAC_PI_4));
    assert!(approx(gdiplus_atan2(-1.0, -1.0), -3.0 * FRAC_PI_4));
    assert_eq!(gdiplus_atan2(0.0, 0.0), 0.0);
}

#[test]
fn unknown_units_give_zero_in_every_conversion() {
    // C source: the default branch of each unit switch returns 0.
    for unit in [-1, 7, 100] {
        assert_eq!(units_to_pixels(1.0, unit, 96.0, false), 0.0);
        assert_eq!(units_to_pixels(1.0, unit, 96.0, true), 0.0);
        assert_eq!(pixels_to_units(1.0, unit, 96.0, false), 0.0);
        assert_eq!(pixels_to_units(1.0, unit, 96.0, true), 0.0);
        assert_eq!(units_scale(unit, 2, 96.0, false), 0.0);
        assert_eq!(units_scale(2, unit, 96.0, false), 0.0);
    }
}

#[test]
fn printer_display_converts_in_hundredths_of_an_inch() {
    // C source: the Display branch multiplies by 0.01 when printer_display is set.
    // Hand-computed: 96 pixels at 96 dpi is one inch, which is 100 units of 0.01 inch.
    assert_eq!(pixels_to_units(96.0, 1, 96.0, true), 100.0);
    assert_eq!(pixels_to_units(96.0, 1, 96.0, false), 96.0);
}

#[test]
fn color_over_early_returns_for_opaque_and_transparent_foreground() {
    // C source: fg alpha 0xff returns fg, fg alpha 0 returns bg, and bg alpha 0 returns fg.
    assert_eq!(color_over(0x1234_5678, 0xff00_ff00), 0xff00_ff00);
    assert_eq!(color_over(0x1234_5678, 0x00aa_bbcc), 0x1234_5678);
    assert_eq!(color_over(0x00ff_ffff, 0x80ff_0000), 0x80ff_0000);
}

#[test]
fn color_over_blends_below_alpha_0x80() {
    // Hand-computed from the C formula: bg alpha 0x40 scales to 47, fg alpha 0x40 is 64, and
    // the sum is 111, below 0x80, so the C shift is defined.
    assert_eq!(color_over(0x4000_00ff, 0x40ff_0000), 0x6f93_006b);
    assert_eq!(color_over_fgpremult(0x4000_00ff, 0x4010_0000), 0x6f24_006b);
}

#[test]
fn color_over_values_at_alpha_0x80_and_above_are_gcc_output() {
    // UB in C: `a << 24` with a >= 0x80 overflows int. The expected values are the GCC -O0 results
    // recorded in target/canon/gdiplus/rows.txt. Not oracle-derived.
    assert_eq!(color_over(0xff33_6699, 0x80ff_0000), 0xff99_324c);
    assert_eq!(color_over(0xff33_6699, 0x4010_2030), 0xff2a_547e);
    assert_eq!(color_over_fgpremult(0x0000_0000, 0x8040_2010), 0x807f_3f1f);
}

#[test]
fn color_over_at_full_alpha_sum_is_hand_computed() {
    // UB in C: the alpha sum is 0xff. Hand-computed from the formula, not checked against GCC.
    assert_eq!(color_over(0xff00_00ff, 0x80ff_0000), 0xff80_007f);
    assert_eq!(color_over_fgpremult(0xff00_00ff, 0x80ff_0000), 0xffff_007f);
}

#[test]
fn premultiply_with_zero_size_leaves_destination_untouched() {
    // C source: the row and column loops run zero times when width or height is zero.
    let mut dst = [0xaa_u8; 8];
    convert_32bpp_argb_to_32bpp_pargb(0, 2, &mut dst, 4, &[], 4);
    convert_32bpp_argb_to_32bpp_pargb(2, 0, &mut dst, 8, &[], 8);
    assert_eq!(dst, [0xaa; 8]);
}

#[test]
fn arc2polybezier_counts_points_without_an_output_buffer() {
    // Hand-traced through the loop in arc2polybezier: a 90 degree sweep takes one 90 degree
    // step (four points) and a zero sweep takes no step.
    let bounds = set_rect(0.0, 0.0, 100.0, 100.0);
    assert_eq!(arc2polybezier(None, bounds, 0.0, 90.0), 4);
    assert_eq!(arc2polybezier(None, bounds, 0.0, 0.0), 0);
}

#[test]
fn null_matrix_arguments_return_invalid_parameter() {
    // C source: each function checks its pointer arguments before it does any work. The
    // allocating functions get alloc_fail, so the check is shown to come before the allocation.
    let m = identity();
    let rect = set_rect(0.0, 0.0, 1.0, 1.0);
    let rect_i = Rect {
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    };
    let pts = [PointF { x: 0.0, y: 0.0 }; 3];
    let pts_i = [Point { x: 0, y: 0 }; 3];
    let mut work = pts;
    let mut work_i = pts_i;

    assert_eq!(
        gdip_create_matrix3(None, Some(&pts), alloc_fail),
        Err(Status::InvalidParameter)
    );
    assert_eq!(
        gdip_create_matrix3(Some(&rect), None, alloc_fail),
        Err(Status::InvalidParameter)
    );
    assert_eq!(
        gdip_create_matrix3_i(None, Some(&pts_i), alloc_fail),
        Err(Status::InvalidParameter)
    );
    assert_eq!(
        gdip_create_matrix3_i(Some(&rect_i), None, alloc_fail),
        Err(Status::InvalidParameter)
    );
    assert_eq!(
        gdip_clone_matrix(None, alloc_fail),
        Err(Status::InvalidParameter)
    );
    assert_eq!(
        gdip_clone_matrix(Some(&m), alloc_fail),
        Err(Status::OutOfMemory)
    );
    assert_eq!(gdip_delete_matrix(None), Status::InvalidParameter);
    assert_eq!(
        gdip_get_matrix_elements(None),
        Err(Status::InvalidParameter)
    );
    assert_eq!(gdip_invert_matrix(None), Status::InvalidParameter);
    assert_eq!(
        gdip_is_matrix_invertible(None),
        Err(Status::InvalidParameter)
    );
    assert_eq!(
        gdip_multiply_matrix(None, Some(&m), 0),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_multiply_matrix(Some(&mut identity()), None, 0),
        Status::InvalidParameter
    );
    assert_eq!(gdip_rotate_matrix(None, 30.0, 0), Status::InvalidParameter);
    assert_eq!(
        gdip_scale_matrix(None, 2.0, 2.0, 0),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_set_matrix_elements(None, [0.0; 6]),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_shear_matrix(None, 1.0, 1.0, 0),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_translate_matrix(None, 1.0, 2.0, 0),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_transform_matrix_points(None, &mut work),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_vector_transform_matrix_points(None, &mut work),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_is_matrix_equal(None, Some(&m)),
        Err(Status::InvalidParameter)
    );
    assert_eq!(
        gdip_is_matrix_equal(Some(&m), None),
        Err(Status::InvalidParameter)
    );
    assert_eq!(gdip_is_matrix_identity(None), Err(Status::InvalidParameter));
    assert_eq!(
        gdip_transform_matrix_points_i(None, &mut work_i, alloc_ok),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_vector_transform_matrix_points_i(None, &mut work_i, alloc_ok),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_transform_matrix_points_i(Some(&m), &mut work_i, alloc_ok),
        Status::Ok
    );
    assert_eq!(
        gdip_vector_transform_matrix_points_i(Some(&m), &mut work_i, alloc_ok),
        Status::Ok
    );
}

#[test]
fn integer_point_functions_allocate_before_checking_the_matrix() {
    // C source: GdipTransformMatrixPointsI and GdipVectorTransformMatrixPointsI malloc the float
    // copy before GdipTransformMatrixPoints rejects a NULL matrix. This is bug-compatible.
    let mut work_i = [Point { x: 0, y: 0 }; 1];
    assert_eq!(
        gdip_transform_matrix_points_i(None, &mut work_i, alloc_for_two_points),
        Status::OutOfMemory
    );
    assert_eq!(
        gdip_vector_transform_matrix_points_i(None, &mut work_i, alloc_for_two_points),
        Status::OutOfMemory
    );
}

#[test]
fn allocation_failure_returns_out_of_memory() {
    let m = identity();
    let rect = set_rect(0.0, 0.0, 1.0, 1.0);
    let rect_i = Rect {
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    };
    let pts = [PointF { x: 0.0, y: 0.0 }; 3];
    let pts_i = [Point { x: 0, y: 0 }; 3];
    let mut work_i = [Point { x: 0, y: 0 }; 1];

    assert_eq!(
        gdip_create_matrix2(1.0, 0.0, 0.0, 1.0, 0.0, 0.0, alloc_fail),
        Err(Status::OutOfMemory)
    );
    assert_eq!(
        gdip_create_matrix3(Some(&rect), Some(&pts), alloc_fail),
        Err(Status::OutOfMemory)
    );
    assert_eq!(
        gdip_create_matrix3_i(Some(&rect_i), Some(&pts_i), alloc_fail),
        Err(Status::OutOfMemory)
    );
    assert_eq!(
        gdip_clone_matrix(Some(&m), alloc_fail),
        Err(Status::OutOfMemory)
    );
    assert_eq!(gdip_create_matrix(alloc_fail), Err(Status::OutOfMemory));
    assert_eq!(
        gdip_transform_matrix_points_i(Some(&m), &mut work_i, alloc_for_two_points),
        Status::OutOfMemory
    );
    assert_eq!(
        gdip_vector_transform_matrix_points_i(Some(&m), &mut work_i, alloc_for_two_points),
        Status::OutOfMemory
    );
}

#[test]
fn all_allocators_succeed_with_valid_sizes() {
    // Ensure all monomorphizations of allocate() and transform_int_points execute their
    // success paths. This covers the full table of test/oracle combinations.
    let m = identity();
    let rect = set_rect(0.0, 0.0, 1.0, 1.0);
    let rect_i = Rect {
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    };
    let pts = [PointF { x: 1.0, y: 2.0 }; 3];
    let pts_i = [Point { x: 1, y: 2 }; 3];

    assert!(gdip_create_matrix2(1.0, 0.0, 0.0, 1.0, 0.0, 0.0, alloc_ok).is_ok());
    assert!(gdip_create_matrix3(Some(&rect), Some(&pts), alloc_ok).is_ok());
    assert!(gdip_create_matrix3_i(Some(&rect_i), Some(&pts_i), alloc_ok).is_ok());
    assert!(gdip_clone_matrix(Some(&m), alloc_ok).is_ok());
    assert!(gdip_create_matrix(alloc_ok).is_ok());

    let mut work_i = [Point { x: 1, y: 2 }; 2];
    assert_eq!(
        gdip_transform_matrix_points_i(Some(&m), &mut work_i, alloc_ok),
        Status::Ok
    );
    assert_eq!(
        gdip_vector_transform_matrix_points_i(Some(&m), &mut work_i, alloc_ok),
        Status::Ok
    );
}

#[test]
fn allocation_requests_match_the_c_malloc_sizes() {
    // C source: malloc(sizeof(GpMatrix)) is 24 bytes, and malloc(sizeof(GpPointF) * count) is
    // count * 8 bytes.
    assert!(gdip_create_matrix(|size| size == 24).is_ok());

    let mut pts = [
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];
    let status = gdip_transform_matrix_points_i(Some(&identity()), &mut pts, |size| size == 3 * 8);
    assert_eq!(status, Status::Ok);
}

#[test]
fn empty_point_slices_are_invalid_before_any_allocation() {
    // C source: each function rejects count <= 0 with InvalidParameter. The `_I` functions do so
    // before the malloc. This test allocator rejects a one-point request and accepts two points,
    // covering both malloc outcomes for the same allocator type.
    let m = identity();
    let mut work = [PointF { x: 0.0, y: 0.0 }; 1];
    let mut int_work = [Point { x: 7, y: 9 }];
    let mut two_int_work = [Point { x: 7, y: 9 }, Point { x: -2, y: 4 }];
    assert_eq!(
        gdip_transform_matrix_points(Some(&m), &mut []),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_vector_transform_matrix_points(Some(&m), &mut []),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_transform_matrix_points_i(Some(&m), &mut [], alloc_for_two_points),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_vector_transform_matrix_points_i(Some(&m), &mut [], alloc_for_two_points),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_transform_matrix_points_i(Some(&m), &mut int_work, alloc_for_two_points),
        Status::OutOfMemory
    );
    assert_eq!(
        gdip_vector_transform_matrix_points_i(Some(&m), &mut int_work, alloc_for_two_points),
        Status::OutOfMemory
    );
    assert_eq!(int_work, [Point { x: 7, y: 9 }]);
    assert_eq!(
        gdip_transform_matrix_points_i(Some(&m), &mut two_int_work, alloc_for_two_points),
        Status::Ok
    );
    assert_eq!(two_int_work, [Point { x: 7, y: 9 }, Point { x: -2, y: 4 }]);
    assert_eq!(
        gdip_transform_matrix_points(Some(&m), &mut work),
        Status::Ok
    );
}

#[test]
fn bad_order_leaves_the_matrix_unchanged() {
    // C source: the order must be MatrixOrderPrepend (0) or MatrixOrderAppend (1). Any other value
    // returns InvalidParameter before the matrix is written.
    let before = Matrix {
        elements: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    };
    let mut m = before;
    assert_eq!(
        gdip_rotate_matrix(Some(&mut m), 30.0, 2),
        Status::InvalidParameter
    );
    assert_eq!(
        gdip_translate_matrix(Some(&mut m), 1.0, 2.0, -1),
        Status::InvalidParameter
    );
    assert_eq!(m, before);
}

#[test]
fn invertibility_threshold_is_an_absolute_determinant_of_1e_minus_5() {
    // Hand-computed: det = 1 * (1 + 2^-17) - 0.5 * 2 = 2^-17, about 7.6e-6, below 1e-5. With
    // 1 + 2^-16 the determinant is 2^-16, about 1.5e-5, above 1e-5. The C code tests
    // fabs(det) >= 1e-5 in double precision.
    let mut singular = Matrix {
        elements: [1.0, 0.5, 2.0, 1.0 + 2f32.powi(-17), 0.0, 0.0],
    };
    assert_eq!(gdip_is_matrix_invertible(Some(&singular)), Ok(false));
    assert_eq!(
        gdip_invert_matrix(Some(&mut singular)),
        Status::InvalidParameter
    );

    let mut invertible = Matrix {
        elements: [1.0, 0.5, 2.0, 1.0 + 2f32.powi(-16), 0.0, 0.0],
    };
    assert_eq!(gdip_is_matrix_invertible(Some(&invertible)), Ok(true));
    assert_eq!(gdip_invert_matrix(Some(&mut invertible)), Status::Ok);
}

#[test]
fn diagonal_matrices_skip_the_determinant_threshold() {
    // C source: when m[1] and m[2] are zero, the invert and invertible checks test m[0] and m[3]
    // for zero and never compute the determinant. So a tiny but non-zero diagonal is invertible.
    let mut tiny = Matrix {
        elements: [1e-6, 0.0, 0.0, 1.0, 0.0, 0.0],
    };
    assert_eq!(gdip_is_matrix_invertible(Some(&tiny)), Ok(true));
    assert_eq!(gdip_invert_matrix(Some(&mut tiny)), Status::Ok);

    let zero_diag = Matrix {
        elements: [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    };
    assert_eq!(gdip_is_matrix_invertible(Some(&zero_diag)), Ok(false));

    let mut zero_y_scale = Matrix {
        elements: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };
    assert_eq!(gdip_is_matrix_invertible(Some(&zero_y_scale)), Ok(false));
    assert_eq!(
        gdip_invert_matrix(Some(&mut zero_y_scale)),
        Status::InvalidParameter
    );
}

#[test]
fn matrix_equality_compares_bit_patterns() {
    // C source: GdipIsMatrixEqual compares the elements with memcmp, so 0.0 and -0.0 differ.
    let pos = Matrix { elements: [0.0; 6] };
    let neg = Matrix {
        elements: [-0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };
    assert_eq!(gdip_is_matrix_equal(Some(&pos), Some(&neg)), Ok(false));
    assert_eq!(gdip_is_matrix_equal(Some(&pos), Some(&pos)), Ok(true));
    assert_eq!(gdip_is_matrix_identity(Some(&identity())), Ok(true));
    assert_eq!(gdip_is_matrix_identity(Some(&neg)), Ok(false));
}

#[test]
fn hresult_values_map_to_status_codes() {
    // C source: the switch in hresult_to_status, with GenericError as the default.
    assert_eq!(hresult_to_status(0), Status::Ok);
    assert_eq!(
        hresult_to_status(0x8007_000e_u32 as i32),
        Status::OutOfMemory
    );
    assert_eq!(
        hresult_to_status(0x8007_0057_u32 as i32),
        Status::InvalidParameter
    );
    assert_eq!(
        hresult_to_status(0x8000_4005_u32 as i32),
        Status::GenericError
    );
}
