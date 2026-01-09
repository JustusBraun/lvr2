//! Surface reconstruction algorithms
//!
//! This module provides algorithms for reconstructing surfaces from
//! point cloud data, including marching cubes and related methods.

mod marching_cubes;
mod hash_grid;
mod search_tree;
mod normals;

pub use marching_cubes::{MarchingCubes, MCTable};
pub use hash_grid::HashGrid;
pub use search_tree::SearchTree;
pub use normals::estimate_normals;

use crate::types::{PointBuffer, MeshBuffer};
use thiserror::Error;

/// Errors that can occur during reconstruction
#[derive(Error, Debug)]
pub enum ReconstructionError {
    #[error("Not enough points for reconstruction: {0}")]
    NotEnoughPoints(usize),
    
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    
    #[error("Algorithm error: {0}")]
    AlgorithmError(String),
}

/// Options for surface reconstruction
#[derive(Debug, Clone)]
pub struct ReconstructionOptions {
    /// Voxel size for the reconstruction grid
    pub voxel_size: f32,
    /// Number of neighbors for normal estimation
    pub kn: usize,
    /// Number of neighbors for normal interpolation
    pub ki: usize,
    /// Number of neighbors for distance function
    pub kd: usize,
    /// Fill holes up to this size (0 = disabled)
    pub fill_holes: usize,
    /// Remove small regions below this threshold
    pub small_region_threshold: usize,
    /// Number of threads to use (0 = auto)
    pub num_threads: usize,
}

impl Default for ReconstructionOptions {
    fn default() -> Self {
        Self {
            voxel_size: 10.0,
            kn: 10,
            ki: 10,
            kd: 5,
            fill_holes: 0,
            small_region_threshold: 10,
            num_threads: 0,
        }
    }
}

/// Reconstructs a surface from a point cloud.
///
/// This is the main entry point for surface reconstruction. It uses
/// the marching cubes algorithm to create a triangle mesh from the
/// input point cloud.
///
/// # Arguments
///
/// * `points` - The input point cloud
/// * `options` - Reconstruction options
///
/// # Returns
///
/// A mesh buffer containing the reconstructed surface.
///
/// # Example
///
/// ```rust,no_run
/// use lvr2::types::PointBuffer;
/// use lvr2::reconstruction::{ReconstructionOptions, reconstruct};
/// use lvr2::geometry::Vec3f;
///
/// let points = PointBuffer::from_points(vec![
///     Vec3f::new(0.0, 0.0, 0.0),
///     // ... more points
/// ]);
///
/// let options = ReconstructionOptions::default();
/// let mesh = reconstruct(&points, &options).unwrap();
/// ```
pub fn reconstruct(points: &PointBuffer, options: &ReconstructionOptions) -> Result<MeshBuffer, ReconstructionError> {
    if points.num_points() < 10 {
        return Err(ReconstructionError::NotEnoughPoints(points.num_points()));
    }
    
    log::info!("Starting reconstruction with {} points", points.num_points());
    log::info!("Voxel size: {}", options.voxel_size);
    
    // Step 1: Estimate normals if not present
    let points_with_normals = if points.has_normals() {
        points.clone()
    } else {
        log::info!("Estimating normals...");
        let mut pb = points.clone();
        let normals = estimate_normals(points, options.kn)?;
        pb.set_normals(normals);
        pb
    };
    
    // Step 2: Create hash grid
    log::info!("Creating hash grid...");
    let grid = HashGrid::new(&points_with_normals, options.voxel_size);
    log::info!("Grid cells: {}", grid.num_cells());
    
    // Step 3: Run marching cubes
    log::info!("Running marching cubes...");
    let mut mc = MarchingCubes::new(&grid, &points_with_normals, options.kd);
    let mesh = mc.reconstruct()?;
    
    log::info!("Reconstruction complete: {} vertices, {} faces",
               mesh.num_vertices(), mesh.num_faces());
    
    Ok(mesh)
}
