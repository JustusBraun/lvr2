//! # LVR2 - Las Vegas Reconstruction 2
//!
//! A Rust library for surface reconstruction from point cloud data.
//! This library provides tools to build surface reconstructions from point clouds
//! and classify found surfaces into predefined categories.
//!
//! ## Features
//!
//! - Point cloud loading and processing
//! - Surface normal estimation
//! - Marching cubes surface reconstruction
//! - Mesh optimization and hole filling
//! - PLY file I/O
//!
//! ## Example
//!
//! ```rust,no_run
//! use lvr2::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load point cloud
//!     let point_buffer = lvr2::io::load_points("scan.pts")?;
//!     
//!     // Reconstruct surface
//!     let options = ReconstructionOptions::default();
//!     let mesh = lvr2::reconstruction::reconstruct(&point_buffer, &options)?;
//!     
//!     // Save result
//!     lvr2::io::save_mesh("output.ply", &mesh)?;
//!     
//!     Ok(())
//! }
//! ```

pub mod geometry;
pub mod types;
pub mod reconstruction;
pub mod io;
pub mod algorithm;
pub mod util;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::geometry::{BaseVector, Vec3f, Vec3d, Normal, BoundingBox};
    pub use crate::types::{PointBuffer, MeshBuffer};
    pub use crate::reconstruction::{ReconstructionOptions, reconstruct};
}

/// Print the LVR2 ASCII banner
pub fn print_banner() {
    println!(r"         /\");
    println!(r"        /  \               ##          ##      ##    #######         ######");
    println!(r"       /    \              ##          ##      ##    ##     ##     ##      ##");
    println!(r"      /      \             ##           ##    ##     ##      ##            ##");
    println!(r"     /________\            ##           ##    ##     ##     ##            ##");
    println!(r"    /\        /\           ##            ##  ##      #######             ##");
    println!(r"   /  \      /  \          ##            ##  ##      ##    ##          ##");
    println!(r"  /    \    /    \         ##             ####       ##     ##       ##");
    println!(r" /      \  /      \        ##########      ##        ##      ##    ##########");
    println!(r"/________\/________\");
    println!();
}
