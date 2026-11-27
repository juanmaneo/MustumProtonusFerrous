use std::mem::size_of;

use crate::geometry::{set_rect, Point, PointF, Rect, RectF};
use crate::math::{deg2rad, gdip_round};
use crate::status::Status;

/// `GpMatrix` from `dlls/gdiplus/gdiplus_private.h`: the six `REAL` elements `m11, m12, m21, m22,
/// dx, dy`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    pub elements: [f32; 6],
}

/// The identity matrix used by `GdipCreateMatrix` and `GdipIsMatrixIdentity`.
fn identity_matrix() -> Matrix {
    Matrix {
        elements: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    }
}

/// `GpMatrixOrder` from `include/gdiplus.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MatrixOrder {
    Prepend,
    Append,
}

impl MatrixOrder {
    /// `None` for a value outside the enum, which the C functions reject with `InvalidParameter`.
    fn from_raw(order: i32) -> Option<MatrixOrder> {
        match order {
            0 => Some(MatrixOrder::Prepend),
            1 => Some(MatrixOrder::Append),
            _ => None,
        }
    }
}

/// `malloc` from the C functions: `alloc` is called with the size and reports whether it succeeded.
fn allocate(alloc: impl FnOnce(usize) -> bool, size: usize) -> Result<(), Status> {
    if alloc(size) {
        Ok(())
    } else {
        Err(Status::OutOfMemory)
    }
}

/// `matrix_multiply` from `dlls/gdiplus/matrix.c`, the product `left * right` of two 3x2 matrices.
fn matrix_multiply(left: &[f32; 6], right: &[f32; 6]) -> [f32; 6] {
    [
        left[0] * right[0] + left[1] * right[2],
        left[0] * right[1] + left[1] * right[3],
        left[2] * right[0] + left[3] * right[2],
        left[2] * right[1] + left[3] * right[3],
        left[4] * right[0] + left[5] * right[2] + right[4],
        left[4] * right[1] + left[5] * right[3] + right[5],
    ]
}

/// `matrix_det` from `dlls/gdiplus/matrix.c`.
fn matrix_det(m: &[f32; 6]) -> f32 {
    m[0] * m[3] - m[1] * m[2]
}

/// The `fabs(det) >= 1e-5` test that `GdipInvertMatrix` and `GdipIsMatrixInvertible` make in double
/// precision.
fn is_nonsingular(det: f32) -> bool {
    f64::from(det.abs()) >= 1e-5
}

/// Multiplies `matrix` by `transform` in the given order, as `GdipMultiplyMatrix`,
/// `GdipRotateMatrix` and `GdipShearMatrix` do.
fn apply_transform(matrix: &mut Matrix, transform: [f32; 6], order: i32) -> Status {
    match MatrixOrder::from_raw(order) {
        Some(MatrixOrder::Append) => {
            matrix.elements = matrix_multiply(&matrix.elements, &transform);
        }
        Some(MatrixOrder::Prepend) => {
            matrix.elements = matrix_multiply(&transform, &matrix.elements);
        }
        None => return Status::InvalidParameter,
    }
    Status::Ok
}

/// `GdipCreateMatrix2` from `dlls/gdiplus/matrix.c`.
pub fn gdip_create_matrix2(
    m11: f32,
    m12: f32,
    m21: f32,
    m22: f32,
    dx: f32,
    dy: f32,
    alloc: impl FnOnce(usize) -> bool,
) -> Result<Matrix, Status> {
    allocate(alloc, size_of::<Matrix>())?;
    Ok(Matrix {
        elements: [m11, m12, m21, m22, dx, dy],
    })
}

/// `GdipCreateMatrix3` from `dlls/gdiplus/matrix.c`: the matrix that maps the corners of `rect`
/// onto the three points `pt`.
pub fn gdip_create_matrix3(
    rect: Option<&RectF>,
    pt: Option<&[PointF; 3]>,
    alloc: impl FnOnce(usize) -> bool,
) -> Result<Matrix, Status> {
    let (Some(rect), Some(pt)) = (rect, pt) else {
        return Err(Status::InvalidParameter);
    };

    let m11 = (pt[1].x - pt[0].x) / rect.width;
    let m21 = (pt[2].x - pt[0].x) / rect.height;
    let dx = pt[0].x - m11 * rect.x - m21 * rect.y;
    let m12 = (pt[1].y - pt[0].y) / rect.width;
    let m22 = (pt[2].y - pt[0].y) / rect.height;
    let dy = pt[0].y - m12 * rect.x - m22 * rect.y;

    gdip_create_matrix2(m11, m12, m21, m22, dx, dy, alloc)
}

/// `GdipCreateMatrix3I` from `dlls/gdiplus/matrix.c`: the integer version of
/// [`gdip_create_matrix3`].
pub fn gdip_create_matrix3_i(
    rect: Option<&Rect>,
    pt: Option<&[Point; 3]>,
    alloc: impl FnOnce(usize) -> bool,
) -> Result<Matrix, Status> {
    let (Some(rect), Some(pt)) = (rect, pt) else {
        return Err(Status::InvalidParameter);
    };

    let rect_f = set_rect(
        rect.x as f32,
        rect.y as f32,
        rect.width as f32,
        rect.height as f32,
    );
    let pt_f = [0, 1, 2].map(|i| PointF {
        x: pt[i].x as f32,
        y: pt[i].y as f32,
    });
    gdip_create_matrix3(Some(&rect_f), Some(&pt_f), alloc)
}

/// `GdipCloneMatrix` from `dlls/gdiplus/matrix.c`.
pub fn gdip_clone_matrix(
    matrix: Option<&Matrix>,
    alloc: impl FnOnce(usize) -> bool,
) -> Result<Matrix, Status> {
    let Some(matrix) = matrix else {
        return Err(Status::InvalidParameter);
    };
    allocate(alloc, size_of::<Matrix>())?;
    Ok(*matrix)
}

/// `GdipCreateMatrix` from `dlls/gdiplus/matrix.c`: the identity matrix.
pub fn gdip_create_matrix(alloc: impl FnOnce(usize) -> bool) -> Result<Matrix, Status> {
    allocate(alloc, size_of::<Matrix>())?;
    Ok(identity_matrix())
}

/// `GdipDeleteMatrix` from `dlls/gdiplus/matrix.c`. The value is owned by the caller, so the
/// port only reports the NULL argument.
pub fn gdip_delete_matrix(matrix: Option<Matrix>) -> Status {
    if matrix.is_some() {
        Status::Ok
    } else {
        Status::InvalidParameter
    }
}

/// `GdipGetMatrixElements` from `dlls/gdiplus/matrix.c`.
pub fn gdip_get_matrix_elements(matrix: Option<&Matrix>) -> Result<[f32; 6], Status> {
    matrix.map(|m| m.elements).ok_or(Status::InvalidParameter)
}

/// `GdipInvertMatrix` from `dlls/gdiplus/matrix.c`.
pub fn gdip_invert_matrix(matrix: Option<&mut Matrix>) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };

    if matrix.elements[1] == 0.0 && matrix.elements[2] == 0.0 {
        let m = &mut matrix.elements;
        if m[0] != 0.0 && m[3] != 0.0 {
            m[4] = -m[4] / m[0];
            m[5] = -m[5] / m[3];
            m[0] = 1.0 / m[0];
            m[3] = 1.0 / m[3];
            return Status::Ok;
        }
        return Status::InvalidParameter;
    }

    let det = matrix_det(&matrix.elements);
    if !is_nonsingular(det) {
        return Status::InvalidParameter;
    }

    let det = 1.0 / det;
    let c = matrix.elements;
    matrix.elements = [
        c[3] * det,
        -c[1] * det,
        -c[2] * det,
        c[0] * det,
        (c[2] * c[5] - c[3] * c[4]) * det,
        -(c[0] * c[5] - c[1] * c[4]) * det,
    ];
    Status::Ok
}

/// `GdipIsMatrixInvertible` from `dlls/gdiplus/matrix.c`.
pub fn gdip_is_matrix_invertible(matrix: Option<&Matrix>) -> Result<bool, Status> {
    let Some(matrix) = matrix else {
        return Err(Status::InvalidParameter);
    };
    let m = &matrix.elements;
    if m[1] == 0.0 && m[2] == 0.0 {
        Ok(m[0] != 0.0 && m[3] != 0.0)
    } else {
        Ok(is_nonsingular(matrix_det(m)))
    }
}

/// `GdipMultiplyMatrix` from `dlls/gdiplus/matrix.c`.
pub fn gdip_multiply_matrix(
    matrix: Option<&mut Matrix>,
    matrix2: Option<&Matrix>,
    order: i32,
) -> Status {
    match (matrix, matrix2) {
        (Some(matrix), Some(matrix2)) => apply_transform(matrix, matrix2.elements, order),
        _ => Status::InvalidParameter,
    }
}

/// `GdipRotateMatrix` from `dlls/gdiplus/matrix.c`. The angle is in degrees.
pub fn gdip_rotate_matrix(matrix: Option<&mut Matrix>, angle: f32, order: i32) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };

    let rad = f64::from(deg2rad(angle));
    let cos_theta = rad.cos() as f32;
    let sin_theta = rad.sin() as f32;
    let rotate = [cos_theta, sin_theta, -sin_theta, cos_theta, 0.0, 0.0];
    apply_transform(matrix, rotate, order)
}

/// `GdipScaleMatrix` from `dlls/gdiplus/matrix.c`. The two orders scale different elements, as
/// the C code does.
pub fn gdip_scale_matrix(
    matrix: Option<&mut Matrix>,
    scale_x: f32,
    scale_y: f32,
    order: i32,
) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };

    let m = &mut matrix.elements;
    match MatrixOrder::from_raw(order) {
        Some(MatrixOrder::Append) => {
            m[0] *= scale_x;
            m[1] *= scale_y;
            m[2] *= scale_x;
            m[3] *= scale_y;
            m[4] *= scale_x;
            m[5] *= scale_y;
        }
        Some(MatrixOrder::Prepend) => {
            m[0] *= scale_x;
            m[1] *= scale_x;
            m[2] *= scale_y;
            m[3] *= scale_y;
        }
        None => return Status::InvalidParameter,
    }
    Status::Ok
}

/// `GdipSetMatrixElements` from `dlls/gdiplus/matrix.c`. `elements` is `m11, m12, m21, m22, dx, dy`.
pub fn gdip_set_matrix_elements(matrix: Option<&mut Matrix>, elements: [f32; 6]) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };
    matrix.elements = elements;
    Status::Ok
}

/// `GdipShearMatrix` from `dlls/gdiplus/matrix.c`.
pub fn gdip_shear_matrix(
    matrix: Option<&mut Matrix>,
    shear_x: f32,
    shear_y: f32,
    order: i32,
) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };
    apply_transform(matrix, [1.0, shear_y, shear_x, 1.0, 0.0, 0.0], order)
}

/// `GdipTransformMatrixPoints` from `dlls/gdiplus/matrix.c`: applies the matrix, translation
/// included, to every point.
pub fn gdip_transform_matrix_points(matrix: Option<&Matrix>, pts: &mut [PointF]) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };
    if pts.is_empty() {
        return Status::InvalidParameter;
    }

    let m = matrix.elements;
    for p in pts.iter_mut() {
        let (x, y) = (p.x, p.y);
        p.x = x * m[0] + y * m[2] + m[4];
        p.y = x * m[1] + y * m[3] + m[5];
    }
    Status::Ok
}

/// `GdipTransformMatrixPointsI` from `dlls/gdiplus/matrix.c`.
pub fn gdip_transform_matrix_points_i(
    matrix: Option<&Matrix>,
    pts: &mut [Point],
    alloc: impl FnOnce(usize) -> bool,
) -> Status {
    transform_int_points(matrix, pts, alloc, gdip_transform_matrix_points)
}

/// `GdipTranslateMatrix` from `dlls/gdiplus/matrix.c`.
pub fn gdip_translate_matrix(
    matrix: Option<&mut Matrix>,
    offset_x: f32,
    offset_y: f32,
    order: i32,
) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };

    let m = &mut matrix.elements;
    match MatrixOrder::from_raw(order) {
        Some(MatrixOrder::Append) => {
            m[4] += offset_x;
            m[5] += offset_y;
        }
        Some(MatrixOrder::Prepend) => {
            m[4] += offset_x * m[0] + offset_y * m[2];
            m[5] += offset_x * m[1] + offset_y * m[3];
        }
        None => return Status::InvalidParameter,
    }
    Status::Ok
}

/// `GdipVectorTransformMatrixPoints` from `dlls/gdiplus/matrix.c`: applies the linear part of the
/// matrix only.
pub fn gdip_vector_transform_matrix_points(matrix: Option<&Matrix>, pts: &mut [PointF]) -> Status {
    let Some(matrix) = matrix else {
        return Status::InvalidParameter;
    };
    if pts.is_empty() {
        return Status::InvalidParameter;
    }

    let m = matrix.elements;
    for p in pts.iter_mut() {
        let (x, y) = (p.x, p.y);
        p.x = x * m[0] + y * m[2];
        p.y = x * m[1] + y * m[3];
    }
    Status::Ok
}

/// `GdipVectorTransformMatrixPointsI` from `dlls/gdiplus/matrix.c`.
pub fn gdip_vector_transform_matrix_points_i(
    matrix: Option<&Matrix>,
    pts: &mut [Point],
    alloc: impl FnOnce(usize) -> bool,
) -> Status {
    transform_int_points(matrix, pts, alloc, gdip_vector_transform_matrix_points)
}

/// The shared body of the two `_I` point functions. The C code checks `count` before it allocates
/// the float copy, and only copies the result back when the transform succeeds.
fn transform_int_points(
    matrix: Option<&Matrix>,
    pts: &mut [Point],
    alloc: impl FnOnce(usize) -> bool,
    transform: fn(Option<&Matrix>, &mut [PointF]) -> Status,
) -> Status {
    if pts.is_empty() {
        return Status::InvalidParameter;
    }
    if !alloc(pts.len() * size_of::<PointF>()) {
        return Status::OutOfMemory;
    }

    let mut pts_f: Vec<PointF> = pts
        .iter()
        .map(|p| PointF {
            x: p.x as f32,
            y: p.y as f32,
        })
        .collect();
    let ret = transform(matrix, &mut pts_f);
    if ret == Status::Ok {
        copy_int_points(pts, &pts_f);
    }
    ret
}

fn copy_int_points(pts: &mut [Point], pts_f: &[PointF]) {
    for (dst, src) in pts.iter_mut().zip(pts_f) {
        dst.x = gdip_round(src.x);
        dst.y = gdip_round(src.y);
    }
}

/// `GdipIsMatrixEqual` from `dlls/gdiplus/matrix.c`. The C code compares the bytes with `memcmp`,
/// so the comparison is on the bit patterns.
pub fn gdip_is_matrix_equal(
    matrix: Option<&Matrix>,
    matrix2: Option<&Matrix>,
) -> Result<bool, Status> {
    let (Some(matrix), Some(matrix2)) = (matrix, matrix2) else {
        return Err(Status::InvalidParameter);
    };
    Ok(matrix.elements.map(f32::to_bits) == matrix2.elements.map(f32::to_bits))
}

/// `GdipIsMatrixIdentity` from `dlls/gdiplus/matrix.c`.
pub fn gdip_is_matrix_identity(matrix: Option<&Matrix>) -> Result<bool, Status> {
    gdip_is_matrix_equal(matrix, Some(&identity_matrix()))
}
