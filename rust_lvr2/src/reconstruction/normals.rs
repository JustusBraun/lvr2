//! Normal estimation for point clouds
//!
//! Provides methods for estimating surface normals from point cloud data.

use crate::types::PointBuffer;
use crate::geometry::Vec3f;
use super::ReconstructionError;
use super::SearchTree;
use rayon::prelude::*;

/// Estimates surface normals for all points in a point buffer.
///
/// Uses PCA (Principal Component Analysis) on local neighborhoods
/// to estimate the surface normal at each point.
///
/// # Arguments
///
/// * `points` - The input point cloud
/// * `k` - Number of neighbors to use for estimation
///
/// # Returns
///
/// A vector of normal vectors, one for each input point.
pub fn estimate_normals(points: &PointBuffer, k: usize) -> Result<Vec<Vec3f>, ReconstructionError> {
    if points.num_points() < k {
        return Err(ReconstructionError::NotEnoughPoints(points.num_points()));
    }
    
    // Build search tree
    let tree = SearchTree::new(points);
    
    // Estimate normals in parallel
    let normals: Vec<Vec3f> = (0..points.num_points())
        .into_par_iter()
        .map(|i| {
            let p = points.get_point(i).unwrap();
            let neighbors = tree.knn(&p, k + 1); // +1 because point itself is included
            
            estimate_normal_pca(&neighbors)
        })
        .collect();
    
    Ok(normals)
}

/// Estimates a normal using PCA on a set of neighbor points.
fn estimate_normal_pca(points: &[Vec3f]) -> Vec3f {
    if points.is_empty() {
        return Vec3f::new(0.0, 0.0, 1.0);
    }
    
    // Compute centroid
    let n = points.len() as f32;
    let centroid = points.iter()
        .fold(Vec3f::default(), |acc, p| acc + *p) / n;
    
    // Build covariance matrix
    let mut cov = [[0.0f32; 3]; 3];
    for p in points {
        let d = *p - centroid;
        cov[0][0] += d.x * d.x;
        cov[0][1] += d.x * d.y;
        cov[0][2] += d.x * d.z;
        cov[1][1] += d.y * d.y;
        cov[1][2] += d.y * d.z;
        cov[2][2] += d.z * d.z;
    }
    cov[1][0] = cov[0][1];
    cov[2][0] = cov[0][2];
    cov[2][1] = cov[1][2];
    
    // Find eigenvector with smallest eigenvalue using power iteration
    // (simplified approach - finds the normal direction)
    let normal = smallest_eigenvector(&cov);
    
    normal
}

/// Finds the eigenvector corresponding to the smallest eigenvalue.
/// Uses a simplified approach based on the cross product of the two
/// largest eigenvectors.
fn smallest_eigenvector(cov: &[[f32; 3]; 3]) -> Vec3f {
    // Use power iteration to find the dominant eigenvector
    let mut v = Vec3f::new(1.0, 0.0, 0.0);
    
    for _ in 0..20 {
        let new_v = Vec3f::new(
            cov[0][0] * v.x + cov[0][1] * v.y + cov[0][2] * v.z,
            cov[1][0] * v.x + cov[1][1] * v.y + cov[1][2] * v.z,
            cov[2][0] * v.x + cov[2][1] * v.y + cov[2][2] * v.z,
        );
        let len = new_v.length();
        if len > 1e-10 {
            v = new_v / len;
        }
    }
    
    // Find second eigenvector by deflation
    let lambda1 = cov[0][0] * v.x * v.x + cov[1][1] * v.y * v.y + cov[2][2] * v.z * v.z
        + 2.0 * (cov[0][1] * v.x * v.y + cov[0][2] * v.x * v.z + cov[1][2] * v.y * v.z);
    
    let mut cov2 = *cov;
    cov2[0][0] -= lambda1 * v.x * v.x;
    cov2[0][1] -= lambda1 * v.x * v.y;
    cov2[0][2] -= lambda1 * v.x * v.z;
    cov2[1][0] = cov2[0][1];
    cov2[1][1] -= lambda1 * v.y * v.y;
    cov2[1][2] -= lambda1 * v.y * v.z;
    cov2[2][0] = cov2[0][2];
    cov2[2][1] = cov2[1][2];
    cov2[2][2] -= lambda1 * v.z * v.z;
    
    let mut v2 = if v.x.abs() < 0.9 {
        Vec3f::new(1.0, 0.0, 0.0)
    } else {
        Vec3f::new(0.0, 1.0, 0.0)
    };
    
    for _ in 0..20 {
        let new_v = Vec3f::new(
            cov2[0][0] * v2.x + cov2[0][1] * v2.y + cov2[0][2] * v2.z,
            cov2[1][0] * v2.x + cov2[1][1] * v2.y + cov2[1][2] * v2.z,
            cov2[2][0] * v2.x + cov2[2][1] * v2.y + cov2[2][2] * v2.z,
        );
        let len = new_v.length();
        if len > 1e-10 {
            v2 = new_v / len;
        }
    }
    
    // Normal is cross product of two largest eigenvectors
    let normal = v.cross(&v2);
    let len = normal.length();
    if len > 1e-10 {
        normal / len
    } else {
        Vec3f::new(0.0, 0.0, 1.0)
    }
}
