//! 4x4 Transformation matrix
//!
//! Provides homogeneous transformation matrices for 3D transformations.

use super::BaseVector;
use std::ops::Mul;

/// A 4x4 transformation matrix stored in row-major order.
///
/// Used for representing 3D transformations including translation,
/// rotation, and scaling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4<T> {
    /// Matrix data in row-major order
    pub data: [[T; 4]; 4],
}

impl<T: Copy + Default> Matrix4<T> {
    /// Creates a new zero matrix.
    pub fn new() -> Self
    where
        T: Default,
    {
        Self {
            data: [[T::default(); 4]; 4],
        }
    }
}

// Specialized implementation for f32
impl Matrix4<f32> {
    /// Creates an identity matrix.
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a translation matrix.
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a uniform scaling matrix.
    pub fn scale(s: f32) -> Self {
        Self {
            data: [
                [s, 0.0, 0.0, 0.0],
                [0.0, s, 0.0, 0.0],
                [0.0, 0.0, s, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a non-uniform scaling matrix.
    pub fn scale_xyz(sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            data: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a rotation matrix around the X axis.
    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a rotation matrix around the Y axis.
    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        
        Self {
            data: [
                [c, 0.0, s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a rotation matrix around the Z axis.
    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        
        Self {
            data: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl<T> Matrix4<T>
where
    T: Copy + Mul<Output = T> + std::ops::Add<Output = T>,
{
    /// Transforms a point by this matrix.
    pub fn transform_point(&self, p: &BaseVector<T>) -> BaseVector<T> {
        BaseVector {
            x: self.data[0][0] * p.x + self.data[0][1] * p.y + self.data[0][2] * p.z + self.data[0][3],
            y: self.data[1][0] * p.x + self.data[1][1] * p.y + self.data[1][2] * p.z + self.data[1][3],
            z: self.data[2][0] * p.x + self.data[2][1] * p.y + self.data[2][2] * p.z + self.data[2][3],
        }
    }

    /// Transforms a direction (ignores translation).
    pub fn transform_direction(&self, d: &BaseVector<T>) -> BaseVector<T> {
        BaseVector {
            x: self.data[0][0] * d.x + self.data[0][1] * d.y + self.data[0][2] * d.z,
            y: self.data[1][0] * d.x + self.data[1][1] * d.y + self.data[1][2] * d.z,
            z: self.data[2][0] * d.x + self.data[2][1] * d.y + self.data[2][2] * d.z,
        }
    }

    /// Multiplies two matrices.
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = Self { data: [[self.data[0][0]; 4]; 4] };
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j]
                    + self.data[i][3] * other.data[3][j];
            }
        }
        result
    }
}

impl Default for Matrix4<f32> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T> Mul for Matrix4<T>
where
    T: Copy + Mul<Output = T> + std::ops::Add<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.multiply(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vec3f;

    #[test]
    fn test_identity() {
        let m = Matrix4::<f32>::identity();
        let p = Vec3f::new(1.0, 2.0, 3.0);
        let transformed = m.transform_point(&p);
        assert!((transformed.x - p.x).abs() < 1e-6);
        assert!((transformed.y - p.y).abs() < 1e-6);
        assert!((transformed.z - p.z).abs() < 1e-6);
    }

    #[test]
    fn test_translation() {
        let m = Matrix4::<f32>::translation(1.0, 2.0, 3.0);
        let p = Vec3f::new(0.0, 0.0, 0.0);
        let transformed = m.transform_point(&p);
        assert!((transformed.x - 1.0).abs() < 1e-6);
        assert!((transformed.y - 2.0).abs() < 1e-6);
        assert!((transformed.z - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_scale() {
        let m = Matrix4::<f32>::scale(2.0);
        let p = Vec3f::new(1.0, 2.0, 3.0);
        let transformed = m.transform_point(&p);
        assert!((transformed.x - 2.0).abs() < 1e-6);
        assert!((transformed.y - 4.0).abs() < 1e-6);
        assert!((transformed.z - 6.0).abs() < 1e-6);
    }
}
