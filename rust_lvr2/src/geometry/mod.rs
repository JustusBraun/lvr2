//! Geometry primitives and operations
//!
//! This module provides fundamental geometric types like vectors, normals,
//! bounding boxes, and transformation matrices.

mod vector;
mod normal;
mod bounding_box;
mod plane;
mod matrix;

pub use vector::{BaseVector, Vec3f, Vec3d};
pub use normal::Normal;
pub use bounding_box::BoundingBox;
pub use plane::Plane;
pub use matrix::Matrix4;
