//! Axis-aligned bounding box
//!
//! Provides a 3D bounding box for spatial queries and bounds checking.

use super::vector::BaseVector;
use std::fmt;

/// An axis-aligned bounding box (AABB) in 3D space.
///
/// The bounding box is defined by its minimum and maximum corners.
/// It provides methods for testing point containment, computing
/// intersections, and expanding to include new points.
///
/// # Examples
///
/// ```
/// use lvr2::geometry::{BoundingBox, Vec3f};
///
/// let mut bb = BoundingBox::<f32>::new();
/// bb.expand(Vec3f::new(0.0, 0.0, 0.0));
/// bb.expand(Vec3f::new(1.0, 1.0, 1.0));
///
/// assert!(bb.contains(&Vec3f::new(0.5, 0.5, 0.5)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox<T> {
    /// Minimum corner of the bounding box
    pub min: BaseVector<T>,
    /// Maximum corner of the bounding box
    pub max: BaseVector<T>,
    /// Whether the bounding box has been initialized
    valid: bool,
}

impl<T: Copy + PartialOrd + Default> BoundingBox<T> {
    /// Creates a new empty bounding box.
    pub fn new() -> Self {
        Self {
            min: BaseVector::default(),
            max: BaseVector::default(),
            valid: false,
        }
    }

    /// Creates a bounding box from min and max points.
    pub fn from_points(min: BaseVector<T>, max: BaseVector<T>) -> Self {
        Self {
            min,
            max,
            valid: true,
        }
    }

    /// Returns whether the bounding box is valid (has at least one point).
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Expands the bounding box to include the given point.
    pub fn expand(&mut self, point: BaseVector<T>) {
        if !self.valid {
            self.min = point;
            self.max = point;
            self.valid = true;
        } else {
            if point.x < self.min.x {
                self.min.x = point.x;
            }
            if point.y < self.min.y {
                self.min.y = point.y;
            }
            if point.z < self.min.z {
                self.min.z = point.z;
            }
            if point.x > self.max.x {
                self.max.x = point.x;
            }
            if point.y > self.max.y {
                self.max.y = point.y;
            }
            if point.z > self.max.z {
                self.max.z = point.z;
            }
        }
    }

    /// Tests if a point is inside the bounding box.
    pub fn contains(&self, point: &BaseVector<T>) -> bool {
        if !self.valid {
            return false;
        }
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }
}

impl<T> BoundingBox<T>
where
    T: Copy + PartialOrd + Default + std::ops::Sub<Output = T>,
{
    /// Returns the extent (size) of the bounding box along each axis.
    pub fn extent(&self) -> BaseVector<T> {
        BaseVector {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
            z: self.max.z - self.min.z,
        }
    }
}

// Specialized implementation for f32
impl BoundingBox<f32> {
    /// Returns the center point of the bounding box.
    pub fn center(&self) -> BaseVector<f32> {
        BaseVector {
            x: (self.min.x + self.max.x) / 2.0,
            y: (self.min.y + self.max.y) / 2.0,
            z: (self.min.z + self.max.z) / 2.0,
        }
    }

    /// Returns the longest axis (0=x, 1=y, 2=z).
    pub fn longest_axis(&self) -> usize {
        let ext = self.extent();
        if ext.x >= ext.y && ext.x >= ext.z {
            0
        } else if ext.y >= ext.z {
            1
        } else {
            2
        }
    }
}

impl<T: Copy + PartialOrd + Default> Default for BoundingBox<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for BoundingBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BoundingBox: [{}, {}, {}] - [{}, {}, {}]",
            self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z
        )
    }
}

impl<T: Copy + PartialOrd + Default> FromIterator<BaseVector<T>> for BoundingBox<T> {
    fn from_iter<I: IntoIterator<Item = BaseVector<T>>>(iter: I) -> Self {
        let mut bb = BoundingBox::new();
        for point in iter {
            bb.expand(point);
        }
        bb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vec3f;

    #[test]
    fn test_new_is_invalid() {
        let bb = BoundingBox::<f32>::new();
        assert!(!bb.is_valid());
    }

    #[test]
    fn test_expand() {
        let mut bb = BoundingBox::<f32>::new();
        bb.expand(Vec3f::new(0.0, 0.0, 0.0));
        bb.expand(Vec3f::new(1.0, 2.0, 3.0));

        assert!(bb.is_valid());
        assert_eq!(bb.min, Vec3f::new(0.0, 0.0, 0.0));
        assert_eq!(bb.max, Vec3f::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_contains() {
        let bb = BoundingBox::from_points(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(1.0, 1.0, 1.0));

        assert!(bb.contains(&Vec3f::new(0.5, 0.5, 0.5)));
        assert!(!bb.contains(&Vec3f::new(2.0, 0.5, 0.5)));
    }

    #[test]
    fn test_center() {
        let bb = BoundingBox::from_points(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(2.0, 4.0, 6.0));
        let center = bb.center();

        assert!((center.x - 1.0).abs() < 1e-6);
        assert!((center.y - 2.0).abs() < 1e-6);
        assert!((center.z - 3.0).abs() < 1e-6);
    }
}
