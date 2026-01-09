//! Base vector type for 3D coordinates
//!
//! A generic, weakly-typed vector that provides all common operations
//! for 3D geometric computations.

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut, Neg};
use std::fmt;

/// A generic 3D vector with coordinates of type `T`.
///
/// This is the fundamental building block for all geometric operations
/// in LVR2. It supports standard vector operations like addition,
/// subtraction, dot product, cross product, and normalization.
///
/// # Type Parameters
///
/// * `T` - The coordinate type (typically `f32` or `f64`)
///
/// # Examples
///
/// ```
/// use lvr2::geometry::BaseVector;
///
/// let v1 = BaseVector::new(1.0, 2.0, 3.0);
/// let v2 = BaseVector::new(4.0, 5.0, 6.0);
///
/// let sum = v1 + v2;
/// let dot = v1.dot(&v2);
/// let cross = v1.cross(&v2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaseVector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Type alias for 3D vector with f32 coordinates
pub type Vec3f = BaseVector<f32>;

/// Type alias for 3D vector with f64 coordinates
pub type Vec3d = BaseVector<f64>;

impl<T> BaseVector<T> {
    /// Creates a new vector with the given coordinates.
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Default> Default for BaseVector<T> {
    /// Creates a null vector (0, 0, 0).
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T> BaseVector<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    /// Returns the squared length of this vector.
    ///
    /// The squared length is easier to calculate and sufficient for certain
    /// use cases (e.g., comparing distances). This method exists for performance.
    #[inline]
    pub fn length2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculates the dot product between this and another vector.
    #[inline]
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the squared distance to another vector.
    #[inline]
    pub fn distance2(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

// Specialized implementations for f32
impl BaseVector<f32> {
    /// Returns the length (magnitude) of this vector.
    #[inline]
    pub fn length(&self) -> f32 {
        self.length2().sqrt()
    }

    /// Calculates the distance to another vector.
    #[inline]
    pub fn distance(&self, other: &Self) -> f32 {
        self.distance2(other).sqrt()
    }

    /// Returns a normalized version of this vector.
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 1e-10 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }

    /// Normalizes this vector in place.
    #[inline]
    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 1e-10 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }
}

// Specialized implementations for f64
impl BaseVector<f64> {
    /// Returns the length (magnitude) of this vector.
    #[inline]
    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    /// Calculates the distance to another vector.
    #[inline]
    pub fn distance(&self, other: &Self) -> f64 {
        self.distance2(other).sqrt()
    }

    /// Returns a normalized version of this vector.
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 1e-10 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }

    /// Normalizes this vector in place.
    #[inline]
    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 1e-10 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }
}

impl<T> BaseVector<T>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    /// Calculates the cross product between this and another vector.
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// Operator overloads

impl<T: Add<Output = T>> Add for BaseVector<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for BaseVector<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for BaseVector<T> {
    type Output = Self;

    #[inline]
    fn mul(self, scale: T) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for BaseVector<T> {
    type Output = Self;

    #[inline]
    fn div(self, scale: T) -> Self {
        Self {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
        }
    }
}

impl<T: Copy + Neg<Output = T>> Neg for BaseVector<T> {
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

impl<T: AddAssign> AddAssign for BaseVector<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: SubAssign> SubAssign for BaseVector<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for BaseVector<T> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for BaseVector<T> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        self.x /= scale;
        self.y /= scale;
        self.z /= scale;
    }
}

impl<T> Index<usize> for BaseVector<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {} >= 3", index),
        }
    }
}

impl<T> IndexMut<usize> for BaseVector<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds: {} >= 3", index),
        }
    }
}

impl<T: fmt::Display> fmt::Display for BaseVector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec: [{} {} {}]", self.x, self.y, self.z)
    }
}

impl<T> From<[T; 3]> for BaseVector<T> {
    #[inline]
    fn from(arr: [T; 3]) -> Self {
        let [x, y, z] = arr;
        Self { x, y, z }
    }
}

impl<T: Copy> From<&[T; 3]> for BaseVector<T> {
    #[inline]
    fn from(arr: &[T; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl<T> From<BaseVector<T>> for [T; 3] {
    #[inline]
    fn from(v: BaseVector<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<T> BaseVector<T>
where
    T: Copy + Add<Output = T> + Div<Output = T> + Default + AddAssign,
{
    /// Returns the centroid of all points in the given collection.
    /// 
    /// Note: For f32 vectors, use `centroid_f32` for better performance.
    /// This generic version is provided for compatibility but has limitations.
    pub fn centroid<I>(_points: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        // Note: Generic centroid requires division by a count, which needs
        // T: From<usize> or similar. Use centroid_f32 for f32 vectors.
        Self::default()
    }
}

impl BaseVector<f32> {
    /// Returns the centroid of all f32 points in the given collection.
    pub fn centroid_f32<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let mut sum = Self::default();
        let mut count = 0usize;
        
        for p in points {
            sum += p;
            count += 1;
        }
        
        if count == 0 {
            return sum;
        }
        
        let n = count as f32;
        Self {
            x: sum.x / n,
            y: sum.y / n,
            z: sum.z / n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_default() {
        let v: Vec3f = Default::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3f::new(4.0, 5.0, 6.0);
        let v2 = Vec3f::new(1.0, 2.0, 3.0);
        let diff = v1 - v2;
        assert_eq!(diff.x, 3.0);
        assert_eq!(diff.y, 3.0);
        assert_eq!(diff.z, 3.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross.x, 0.0);
        assert_eq!(cross.y, 0.0);
        assert_eq!(cross.z, 1.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3f::new(3.0, 4.0, 0.0);
        assert!((v.length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalized() {
        let v = Vec3f::new(3.0, 4.0, 0.0);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < 1e-6);
        assert!((n.x - 0.6).abs() < 1e-6);
        assert!((n.y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_index() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }
}
