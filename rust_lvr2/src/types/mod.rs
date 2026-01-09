//! Core data types for point clouds and meshes
//!
//! This module provides the fundamental data structures for storing
//! and manipulating point cloud and mesh data.

mod point_buffer;
mod mesh_buffer;
mod channel;

pub use point_buffer::PointBuffer;
pub use mesh_buffer::MeshBuffer;
pub use channel::Channel;
