//! Mesh smoothing algorithms
//!
//! Provides Laplacian and other smoothing methods for meshes.

use crate::types::MeshBuffer;
use crate::geometry::Vec3f;
use std::collections::HashMap;

/// Smooths a mesh using Laplacian smoothing.
///
/// # Arguments
///
/// * `mesh` - The mesh to smooth (modified in place)
/// * `iterations` - Number of smoothing iterations
/// * `lambda` - Smoothing factor (0.0 to 1.0)
pub fn smooth_mesh(mesh: &mut MeshBuffer, iterations: usize, lambda: f32) {
    log::info!("Smoothing mesh with {} iterations, lambda={}", iterations, lambda);
    
    // Build adjacency information
    let mut adjacency: HashMap<u32, Vec<u32>> = HashMap::new();
    
    for face in mesh.faces() {
        for i in 0..3 {
            let v = face[i];
            let next = face[(i + 1) % 3];
            let prev = face[(i + 2) % 3];
            
            adjacency.entry(v).or_default().push(next);
            adjacency.entry(v).or_default().push(prev);
        }
    }
    
    // Remove duplicates from adjacency lists
    for neighbors in adjacency.values_mut() {
        neighbors.sort();
        neighbors.dedup();
    }
    
    // Perform smoothing iterations
    for _ in 0..iterations {
        let mut new_positions = Vec::with_capacity(mesh.num_vertices());
        
        for i in 0..mesh.num_vertices() {
            let v = mesh.get_vertex(i).unwrap();
            
            if let Some(neighbors) = adjacency.get(&(i as u32)) {
                if !neighbors.is_empty() {
                    // Compute centroid of neighbors
                    let mut centroid = Vec3f::default();
                    for &n in neighbors {
                        centroid += mesh.get_vertex(n as usize).unwrap();
                    }
                    centroid = centroid / neighbors.len() as f32;
                    
                    // Move vertex towards centroid
                    let new_pos = v + (centroid - v) * lambda;
                    new_positions.push(new_pos);
                } else {
                    new_positions.push(v);
                }
            } else {
                new_positions.push(v);
            }
        }
        
        mesh.set_vertices(new_positions);
    }
    
    // Recompute normals after smoothing
    mesh.compute_vertex_normals();
}
