//! Hash grid for spatial hashing
//!
//! Provides a sparse voxel grid using spatial hashing for efficient
//! lookup of grid cells during reconstruction.

use crate::types::PointBuffer;
use crate::geometry::{Vec3f, BoundingBox};
use std::collections::HashMap;

/// A cell in the hash grid containing indices of points.
#[derive(Debug, Clone, Default)]
pub struct GridCell {
    /// Indices of points in this cell
    pub point_indices: Vec<usize>,
    /// Distance value at this cell (for marching cubes)
    pub distance: f32,
    /// Whether this cell has been processed
    pub processed: bool,
}

/// A sparse voxel grid using spatial hashing.
///
/// The hash grid divides space into uniform voxels and provides
/// efficient lookup for finding which points fall into each voxel.
pub struct HashGrid {
    /// Map from cell coordinates to cell data
    cells: HashMap<(i32, i32, i32), GridCell>,
    /// Voxel size
    voxel_size: f32,
    /// Bounding box of the grid
    bounding_box: BoundingBox<f32>,
    /// Grid dimensions
    dims: (usize, usize, usize),
    /// Origin of the grid
    origin: Vec3f,
}

impl HashGrid {
    /// Creates a new hash grid from a point buffer.
    pub fn new(points: &PointBuffer, voxel_size: f32) -> Self {
        let bb = points.bounding_box();
        
        // Extend bounding box slightly
        let extent = bb.extent();
        let padding = voxel_size;
        let min = Vec3f::new(
            bb.min.x - padding,
            bb.min.y - padding,
            bb.min.z - padding,
        );
        let max = Vec3f::new(
            bb.max.x + padding,
            bb.max.y + padding,
            bb.max.z + padding,
        );
        let extended_bb = BoundingBox::from_points(min, max);
        
        // Calculate grid dimensions
        let dims = (
            ((max.x - min.x) / voxel_size).ceil() as usize + 1,
            ((max.y - min.y) / voxel_size).ceil() as usize + 1,
            ((max.z - min.z) / voxel_size).ceil() as usize + 1,
        );
        
        let mut grid = Self {
            cells: HashMap::new(),
            voxel_size,
            bounding_box: extended_bb,
            dims,
            origin: min,
        };
        
        // Insert points into grid
        for i in 0..points.num_points() {
            let p = points.get_point(i).unwrap();
            let cell_coords = grid.point_to_cell(&p);
            
            grid.cells
                .entry(cell_coords)
                .or_default()
                .point_indices
                .push(i);
        }
        
        grid
    }
    
    /// Converts a point to cell coordinates.
    pub fn point_to_cell(&self, point: &Vec3f) -> (i32, i32, i32) {
        let x = ((point.x - self.origin.x) / self.voxel_size).floor() as i32;
        let y = ((point.y - self.origin.y) / self.voxel_size).floor() as i32;
        let z = ((point.z - self.origin.z) / self.voxel_size).floor() as i32;
        (x, y, z)
    }
    
    /// Converts cell coordinates to world position (cell center).
    pub fn cell_to_point(&self, cell: (i32, i32, i32)) -> Vec3f {
        Vec3f::new(
            self.origin.x + (cell.0 as f32 + 0.5) * self.voxel_size,
            self.origin.y + (cell.1 as f32 + 0.5) * self.voxel_size,
            self.origin.z + (cell.2 as f32 + 0.5) * self.voxel_size,
        )
    }
    
    /// Converts cell coordinates to the corner position.
    pub fn cell_corner(&self, cell: (i32, i32, i32)) -> Vec3f {
        Vec3f::new(
            self.origin.x + cell.0 as f32 * self.voxel_size,
            self.origin.y + cell.1 as f32 * self.voxel_size,
            self.origin.z + cell.2 as f32 * self.voxel_size,
        )
    }
    
    /// Gets a cell by its coordinates.
    pub fn get_cell(&self, coords: (i32, i32, i32)) -> Option<&GridCell> {
        self.cells.get(&coords)
    }
    
    /// Gets a mutable cell by its coordinates.
    pub fn get_cell_mut(&mut self, coords: (i32, i32, i32)) -> Option<&mut GridCell> {
        self.cells.get_mut(&coords)
    }
    
    /// Inserts or updates a cell.
    pub fn set_cell(&mut self, coords: (i32, i32, i32), cell: GridCell) {
        self.cells.insert(coords, cell);
    }
    
    /// Returns the number of non-empty cells.
    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }
    
    /// Returns the voxel size.
    pub fn voxel_size(&self) -> f32 {
        self.voxel_size
    }
    
    /// Returns the grid dimensions.
    pub fn dims(&self) -> (usize, usize, usize) {
        self.dims
    }
    
    /// Returns the grid origin.
    pub fn origin(&self) -> Vec3f {
        self.origin
    }
    
    /// Returns the bounding box.
    pub fn bounding_box(&self) -> &BoundingBox<f32> {
        &self.bounding_box
    }
    
    /// Returns an iterator over all cell coordinates.
    pub fn cell_coords(&self) -> impl Iterator<Item = &(i32, i32, i32)> {
        self.cells.keys()
    }
    
    /// Returns an iterator over all cells.
    pub fn cells(&self) -> impl Iterator<Item = (&(i32, i32, i32), &GridCell)> {
        self.cells.iter()
    }
    
    /// Gets the 8 corner coordinates of a cell.
    pub fn cell_corners(&self, cell: (i32, i32, i32)) -> [(i32, i32, i32); 8] {
        [
            (cell.0, cell.1, cell.2),
            (cell.0 + 1, cell.1, cell.2),
            (cell.0 + 1, cell.1 + 1, cell.2),
            (cell.0, cell.1 + 1, cell.2),
            (cell.0, cell.1, cell.2 + 1),
            (cell.0 + 1, cell.1, cell.2 + 1),
            (cell.0 + 1, cell.1 + 1, cell.2 + 1),
            (cell.0, cell.1 + 1, cell.2 + 1),
        ]
    }
}
