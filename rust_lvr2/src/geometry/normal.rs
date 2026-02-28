//! Normal vector type
//!
//! Represents a normalized 3D direction vector.

use super::vector::BaseVector;
use std::fmt;
use std::ops::Neg;

/// A normalized 3D direction vector.
///
/// Unlike `BaseVector`, a `Normal` is guaranteed to have unit length
/// (within floating-point precision). This type is used to represent
/// surface normals and other directional quantities.
///
/// # Examples
///
/// ```
/// use lvr2::geometry::{Normal, Vec3f};
///
/// let n = Normal::new(0.0, 0.0, 1.0);
/// assert!((n.length() - 1.0).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Normal<T> {
    /// X component of the normal
    pub x: T,
    /// Y component of the normal
    pub y: T,
    /// Z component of the normal
    pub z: T,
}

// Specialized implementation for f32
impl Normal<f32> {
    /// Creates a new normal from the given components.
    ///
    /// The input is automatically normalized to unit length.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let len = (x * x + y * y + z * z).sqrt();
        if len > 1e-10 {
            Self {
                x: x / len,
                y: y / len,
                z: z / len,
            }
        } else {
            Self { x: 0.0, y: 0.0, z: 1.0 }
        }
    }

    /// Returns the length of this normal (should be ~1.0).
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculates the dot product with another normal.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the dot product with a vector.
    #[inline]
    pub fn dot_vec(&self, other: &BaseVector<f32>) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Converts this normal to a regular vector.
    #[inline]
    pub fn to_vector(&self) -> BaseVector<f32> {
        BaseVector::new(self.x, self.y, self.z)
    }

    /// Calculates the cross product with another normal.
    #[inline]
    pub fn cross(&self, other: &Self) -> BaseVector<f32> {
        BaseVector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl From<BaseVector<f32>> for Normal<f32> {
    fn from(v: BaseVector<f32>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<T: Copy + Neg<Output = T>> Neg for Normal<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Normal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Normal: [{} {} {}]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let n = Normal::<f32>::new(3.0, 4.0, 0.0);
        assert!((n.length() - 1.0).abs() < 1e-6);
        assert!((n.x - 0.6).abs() < 1e-6);
        assert!((n.y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_dot() {
        let n1 = Normal::<f32>::new(1.0, 0.0, 0.0);
        let n2 = Normal::<f32>::new(0.0, 1.0, 0.0);
        assert!((n1.dot(&n2)).abs() < 1e-6);
    }

    #[test]
    fn test_neg() {
        let n = Normal::<f32>::new(1.0, 0.0, 0.0);
        let neg = -n;
        assert!((neg.x + 1.0).abs() < 1e-6);
    }
}
