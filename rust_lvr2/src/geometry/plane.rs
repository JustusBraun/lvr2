//! 3D Plane representation
//!
//! Provides a plane defined by a point and normal vector.

use super::{BaseVector, Normal, Vec3f};

/// A plane in 3D space defined by a point and a normal vector.
///
/// The plane equation is: n·(p - p0) = 0, where n is the normal
/// and p0 is the reference point.
#[derive(Debug, Clone, Copy)]
pub struct Plane<T> {
    /// A point on the plane
    pub point: BaseVector<T>,
    /// The normal vector of the plane
    pub normal: Normal<T>,
}

impl Plane<f32> {
    /// Creates a new plane from a point and normal.
    pub fn new(point: Vec3f, normal: Normal<f32>) -> Self {
        Self { point, normal }
    }

    /// Creates a plane from three points.
    ///
    /// The normal is computed as (p2-p1) × (p3-p1), normalized.
    pub fn from_points(p1: Vec3f, p2: Vec3f, p3: Vec3f) -> Self {
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let n = v1.cross(&v2).normalized();
        Self {
            point: p1,
            normal: Normal::from(n),
        }
    }

    /// Computes the signed distance from a point to the plane.
    ///
    /// Positive values indicate the point is on the side of the normal,
    /// negative values indicate the opposite side.
    pub fn signed_distance(&self, point: &Vec3f) -> f32 {
        let diff = *point - self.point;
        self.normal.x * diff.x + self.normal.y * diff.y + self.normal.z * diff.z
    }

    /// Projects a point onto the plane.
    pub fn project(&self, point: &Vec3f) -> Vec3f {
        let dist = self.signed_distance(point);
        Vec3f {
            x: point.x - self.normal.x * dist,
            y: point.y - self.normal.y * dist,
            z: point.z - self.normal.z * dist,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vec3f;

    #[test]
    fn test_signed_distance() {
        let plane = Plane::new(
            Vec3f::new(0.0, 0.0, 0.0),
            Normal::new(0.0, 0.0, 1.0),
        );
        
        assert!((plane.signed_distance(&Vec3f::new(0.0, 0.0, 5.0)) - 5.0).abs() < 1e-6);
        assert!((plane.signed_distance(&Vec3f::new(0.0, 0.0, -3.0)) + 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_project() {
        let plane = Plane::new(
            Vec3f::new(0.0, 0.0, 0.0),
            Normal::new(0.0, 0.0, 1.0),
        );
        
        let projected = plane.project(&Vec3f::new(1.0, 2.0, 5.0));
        assert!((projected.x - 1.0).abs() < 1e-6);
        assert!((projected.y - 2.0).abs() < 1e-6);
        assert!((projected.z).abs() < 1e-6);
    }
}
