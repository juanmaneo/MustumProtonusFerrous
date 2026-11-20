//! Headless subset of Wine's `dlls/d3drm`, the Direct3D Retained Mode helpers.
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`math`]: the colour constructors and getters, quaternion multiply, matrix from quaternion,
//!   quaternion from rotation and slerp, and the vector helpers of `math.c`.
//! - [`transform`]: the affine matrix product, the rotation matrix and the affine vector transform
//!   of `frame.c`.
//! - [`animation`]: the key lookup, the index search and the range query of `frame.c`.
//! - [`texture`]: the palettise and 32-bit and 8-bit load helpers of `texture.c`.
//!
//! Skipped: the COM objects and their reference counts, `D3DRMVectorRandom` (it calls `rand`),
//! `d3drm_array_reserve` (a `Vec` replaces it), `d3drm_validate_image`, key insertion and deletion,
//! the viewport normalisation, and the DLL entry points.
//!
//! Where Wine's C is undefined or reads past its input, the port defines the result instead. The
//! details are on [`math::color_rgba`], [`animation::index_max`], [`texture::palettise`] and
//! [`texture::ImageError::ShortSource`].

pub mod animation;
pub mod math;
pub mod texture;
pub mod transform;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A `D3DVECTOR`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// The vector `(x, y, z)`.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// A `D3DRMQUATERNION`: the scalar part `s` and the vector part `v`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    pub s: f32,
    pub v: Vector,
}

impl Quaternion {
    /// The quaternion with scalar part `s` and vector part `v`.
    pub const fn new(s: f32, v: Vector) -> Self {
        Self { s, v }
    }
}

/// A `D3DRMMATRIX4D`, indexed `[row][column]`. A point is a row vector, so
/// [`transform::vector_transform_affine`] computes `v * m`.
pub type Matrix = [[f32; 4]; 4];

/// A `D3DCOLOR`, packed as `0xAARRGGBB`.
pub type Color = u32;
