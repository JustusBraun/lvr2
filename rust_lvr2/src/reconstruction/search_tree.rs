//! K-D tree based spatial search
//!
//! Provides efficient nearest neighbor queries on point clouds.

use crate::types::PointBuffer;
use crate::geometry::Vec3f;
use kiddo::{KdTree, SquaredEuclidean};

/// A spatial search tree for efficient nearest neighbor queries.
pub struct SearchTree {
    tree: KdTree<f32, 3>,
    points: Vec<Vec3f>,
}

impl SearchTree {
    /// Creates a new search tree from a point buffer.
    pub fn new(buffer: &PointBuffer) -> Self {
        let points: Vec<Vec3f> = buffer.points().collect();
        let mut tree = KdTree::new();
        
        for (i, p) in points.iter().enumerate() {
            tree.add(&[p.x, p.y, p.z], i as u64);
        }
        
        Self { tree, points }
    }
    
    /// Creates a new search tree from a vector of points.
    pub fn from_points(points: Vec<Vec3f>) -> Self {
        let mut tree = KdTree::new();
        
        for (i, p) in points.iter().enumerate() {
            tree.add(&[p.x, p.y, p.z], i as u64);
        }
        
        Self { tree, points }
    }
    
    /// Finds the k nearest neighbors to the query point.
    pub fn knn(&self, query: &Vec3f, k: usize) -> Vec<Vec3f> {
        let results = self.tree.nearest_n::<SquaredEuclidean>(&[query.x, query.y, query.z], k);
        
        results
            .into_iter()
            .map(|r| self.points[r.item as usize])
            .collect()
    }
    
    /// Finds the k nearest neighbor indices to the query point.
    pub fn knn_indices(&self, query: &Vec3f, k: usize) -> Vec<usize> {
        let results = self.tree.nearest_n::<SquaredEuclidean>(&[query.x, query.y, query.z], k);
        
        results
            .into_iter()
            .map(|r| r.item as usize)
            .collect()
    }
    
    /// Finds all points within a given radius of the query point.
    pub fn radius_search(&self, query: &Vec3f, radius: f32) -> Vec<Vec3f> {
        let results = self.tree.within::<SquaredEuclidean>(&[query.x, query.y, query.z], radius * radius);
        
        results
            .into_iter()
            .map(|r| self.points[r.item as usize])
            .collect()
    }
    
    /// Returns the number of points in the tree.
    pub fn size(&self) -> usize {
        self.points.len()
    }
    
    /// Gets the point at the given index.
    pub fn get_point(&self, index: usize) -> Option<Vec3f> {
        self.points.get(index).copied()
    }
}
