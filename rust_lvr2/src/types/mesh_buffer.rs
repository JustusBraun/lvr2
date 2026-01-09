//! Mesh buffer for storing triangle mesh data
//!
//! A mesh buffer stores vertices, faces (triangles), and associated
//! attributes like normals, colors, and texture coordinates.

use crate::geometry::{BaseVector, Vec3f, BoundingBox};
use super::Channel;

/// A buffer for storing triangle mesh data.
///
/// The mesh buffer stores vertex positions, face indices, and optional
/// attributes like normals, colors, and texture coordinates.
///
/// # Examples
///
/// ```
/// use lvr2::types::MeshBuffer;
/// use lvr2::geometry::Vec3f;
///
/// let vertices = vec![
///     Vec3f::new(0.0, 0.0, 0.0),
///     Vec3f::new(1.0, 0.0, 0.0),
///     Vec3f::new(0.5, 1.0, 0.0),
/// ];
/// let faces = vec![0u32, 1, 2];
///
/// let mut mesh = MeshBuffer::new();
/// mesh.set_vertices(vertices);
/// mesh.set_faces(faces);
///
/// assert_eq!(mesh.num_vertices(), 3);
/// assert_eq!(mesh.num_faces(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct MeshBuffer {
    /// Vertex positions (x, y, z)
    vertices: Channel<f32>,
    /// Face indices (3 indices per face)
    faces: Channel<u32>,
    /// Vertex normals (nx, ny, nz)
    vertex_normals: Option<Channel<f32>>,
    /// Face normals (nx, ny, nz)
    face_normals: Option<Channel<f32>>,
    /// Vertex colors (r, g, b) or (r, g, b, a)
    vertex_colors: Option<Channel<u8>>,
    /// Face colors
    face_colors: Option<Channel<u8>>,
    /// Texture coordinates (u, v)
    texture_coords: Option<Channel<f32>>,
    /// Face material indices
    face_materials: Option<Channel<u32>>,
}

impl MeshBuffer {
    /// Creates an empty mesh buffer.
    pub fn new() -> Self {
        Self {
            vertices: Channel::with_width(3),
            faces: Channel::with_width(3),
            vertex_normals: None,
            face_normals: None,
            vertex_colors: None,
            face_colors: None,
            texture_coords: None,
            face_materials: None,
        }
    }

    /// Returns the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of faces (triangles).
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    /// Returns true if the mesh has vertices.
    pub fn has_vertices(&self) -> bool {
        !self.vertices.is_empty()
    }

    /// Returns true if the mesh has faces.
    pub fn has_faces(&self) -> bool {
        !self.faces.is_empty()
    }

    /// Gets the vertex at the given index.
    pub fn get_vertex(&self, index: usize) -> Option<Vec3f> {
        self.vertices.get(index).map(|v| Vec3f::new(v[0], v[1], v[2]))
    }

    /// Returns an iterator over all vertices.
    pub fn vertices(&self) -> impl Iterator<Item = Vec3f> + '_ {
        self.vertices.into_iter().map(|v| Vec3f::new(v[0], v[1], v[2]))
    }

    /// Returns a reference to the raw vertex data.
    pub fn vertex_data(&self) -> &[f32] {
        self.vertices.data()
    }

    /// Sets the vertex array.
    pub fn set_vertices(&mut self, vertices: Vec<Vec3f>) {
        let mut data = Vec::with_capacity(vertices.len() * 3);
        for v in vertices {
            data.push(v.x);
            data.push(v.y);
            data.push(v.z);
        }
        self.vertices = Channel::new(data, 3);
    }

    /// Sets the vertex array from raw data.
    pub fn set_vertices_raw(&mut self, data: Vec<f32>) {
        self.vertices = Channel::new(data, 3);
    }

    /// Gets the face indices at the given index.
    pub fn get_face(&self, index: usize) -> Option<[u32; 3]> {
        self.faces.get(index).map(|f| [f[0], f[1], f[2]])
    }

    /// Returns an iterator over all faces.
    pub fn faces(&self) -> impl Iterator<Item = [u32; 3]> + '_ {
        self.faces.into_iter().map(|f| [f[0], f[1], f[2]])
    }

    /// Returns a reference to the raw face index data.
    pub fn face_data(&self) -> &[u32] {
        self.faces.data()
    }

    /// Sets the face index array.
    pub fn set_faces(&mut self, faces: Vec<u32>) {
        self.faces = Channel::new(faces, 3);
    }

    /// Returns true if the mesh has vertex normals.
    pub fn has_vertex_normals(&self) -> bool {
        self.vertex_normals.is_some()
    }

    /// Gets the vertex normal at the given index.
    pub fn get_vertex_normal(&self, index: usize) -> Option<Vec3f> {
        self.vertex_normals
            .as_ref()
            .and_then(|n| n.get(index))
            .map(|n| Vec3f::new(n[0], n[1], n[2]))
    }

    /// Returns a reference to the raw vertex normal data.
    pub fn vertex_normal_data(&self) -> Option<&[f32]> {
        self.vertex_normals.as_ref().map(|n| n.data())
    }

    /// Sets the vertex normal array.
    pub fn set_vertex_normals(&mut self, normals: Vec<Vec3f>) {
        assert_eq!(normals.len(), self.num_vertices(), "Normal count must match vertex count");
        let mut data = Vec::with_capacity(normals.len() * 3);
        for n in normals {
            data.push(n.x);
            data.push(n.y);
            data.push(n.z);
        }
        self.vertex_normals = Some(Channel::new(data, 3));
    }

    /// Sets the vertex normal array from raw data.
    pub fn set_vertex_normals_raw(&mut self, data: Vec<f32>) {
        self.vertex_normals = Some(Channel::new(data, 3));
    }

    /// Returns true if the mesh has face normals.
    pub fn has_face_normals(&self) -> bool {
        self.face_normals.is_some()
    }

    /// Gets the face normal at the given index.
    pub fn get_face_normal(&self, index: usize) -> Option<Vec3f> {
        self.face_normals
            .as_ref()
            .and_then(|n| n.get(index))
            .map(|n| Vec3f::new(n[0], n[1], n[2]))
    }

    /// Sets the face normal array from raw data.
    pub fn set_face_normals_raw(&mut self, data: Vec<f32>) {
        self.face_normals = Some(Channel::new(data, 3));
    }

    /// Returns true if the mesh has vertex colors.
    pub fn has_vertex_colors(&self) -> bool {
        self.vertex_colors.is_some()
    }

    /// Gets the vertex color at the given index.
    pub fn get_vertex_color(&self, index: usize) -> Option<&[u8]> {
        self.vertex_colors.as_ref().and_then(|c| c.get(index))
    }

    /// Returns the vertex color width (3 for RGB, 4 for RGBA).
    pub fn vertex_color_width(&self) -> Option<usize> {
        self.vertex_colors.as_ref().map(|c| c.width())
    }

    /// Sets the vertex color array.
    pub fn set_vertex_colors(&mut self, data: Vec<u8>, width: usize) {
        assert!(width == 3 || width == 4, "Color width must be 3 (RGB) or 4 (RGBA)");
        assert_eq!(data.len() / width, self.num_vertices(), "Color count must match vertex count");
        self.vertex_colors = Some(Channel::new(data, width));
    }

    /// Returns true if the mesh has face colors.
    pub fn has_face_colors(&self) -> bool {
        self.face_colors.is_some()
    }

    /// Gets the face color at the given index.
    pub fn get_face_color(&self, index: usize) -> Option<&[u8]> {
        self.face_colors.as_ref().and_then(|c| c.get(index))
    }

    /// Sets the face color array.
    pub fn set_face_colors(&mut self, data: Vec<u8>, width: usize) {
        assert!(width == 3 || width == 4, "Color width must be 3 (RGB) or 4 (RGBA)");
        assert_eq!(data.len() / width, self.num_faces(), "Color count must match face count");
        self.face_colors = Some(Channel::new(data, width));
    }

    /// Returns true if the mesh has texture coordinates.
    pub fn has_texture_coords(&self) -> bool {
        self.texture_coords.is_some()
    }

    /// Gets the texture coordinates at the given vertex index.
    pub fn get_texture_coord(&self, index: usize) -> Option<(f32, f32)> {
        self.texture_coords
            .as_ref()
            .and_then(|t| t.get(index))
            .map(|t| (t[0], t[1]))
    }

    /// Sets the texture coordinate array.
    pub fn set_texture_coords(&mut self, data: Vec<f32>) {
        assert_eq!(data.len() / 2, self.num_vertices(), "Texture coord count must match vertex count");
        self.texture_coords = Some(Channel::new(data, 2));
    }

    /// Returns true if the mesh has face material indices.
    pub fn has_face_materials(&self) -> bool {
        self.face_materials.is_some()
    }

    /// Gets the material index for the face at the given index.
    pub fn get_face_material(&self, index: usize) -> Option<u32> {
        self.face_materials
            .as_ref()
            .and_then(|m| m.get(index))
            .map(|m| m[0])
    }

    /// Sets the face material index array.
    pub fn set_face_materials(&mut self, data: Vec<u32>) {
        assert_eq!(data.len(), self.num_faces(), "Material count must match face count");
        self.face_materials = Some(Channel::new(data, 1));
    }

    /// Computes the bounding box of all vertices.
    pub fn bounding_box(&self) -> BoundingBox<f32> {
        self.vertices().collect()
    }

    /// Computes face normals from vertex positions.
    pub fn compute_face_normals(&mut self) {
        let mut normals = Vec::with_capacity(self.num_faces() * 3);
        
        for face in self.faces() {
            let v0 = self.get_vertex(face[0] as usize).unwrap();
            let v1 = self.get_vertex(face[1] as usize).unwrap();
            let v2 = self.get_vertex(face[2] as usize).unwrap();
            
            let e1 = v1 - v0;
            let e2 = v2 - v0;
            let n = e1.cross(&e2).normalized();
            
            normals.push(n.x);
            normals.push(n.y);
            normals.push(n.z);
        }
        
        self.face_normals = Some(Channel::new(normals, 3));
    }

    /// Computes vertex normals by averaging face normals.
    pub fn compute_vertex_normals(&mut self) {
        // First ensure we have face normals
        if self.face_normals.is_none() {
            self.compute_face_normals();
        }
        
        let mut normals = vec![Vec3f::default(); self.num_vertices()];
        let mut counts = vec![0usize; self.num_vertices()];
        
        for (face_idx, face) in self.faces().enumerate() {
            let fn_ = self.get_face_normal(face_idx).unwrap();
            for &vi in &face {
                let vi = vi as usize;
                normals[vi] += fn_;
                counts[vi] += 1;
            }
        }
        
        let mut data = Vec::with_capacity(self.num_vertices() * 3);
        for (n, c) in normals.iter().zip(counts.iter()) {
            if *c > 0 {
                let avg = *n / (*c as f32);
                let norm = avg.normalized();
                data.push(norm.x);
                data.push(norm.y);
                data.push(norm.z);
            } else {
                data.push(0.0);
                data.push(0.0);
                data.push(1.0);
            }
        }
        
        self.vertex_normals = Some(Channel::new(data, 3));
    }
}

impl Default for MeshBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mesh = MeshBuffer::new();
        assert_eq!(mesh.num_vertices(), 0);
        assert_eq!(mesh.num_faces(), 0);
    }

    #[test]
    fn test_single_triangle() {
        let mut mesh = MeshBuffer::new();
        
        let vertices = vec![
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 0.0, 0.0),
            Vec3f::new(0.5, 1.0, 0.0),
        ];
        mesh.set_vertices(vertices);
        mesh.set_faces(vec![0, 1, 2]);

        assert_eq!(mesh.num_vertices(), 3);
        assert_eq!(mesh.num_faces(), 1);
        
        let face = mesh.get_face(0).unwrap();
        assert_eq!(face, [0, 1, 2]);
    }

    #[test]
    fn test_compute_face_normals() {
        let mut mesh = MeshBuffer::new();
        
        // Triangle in XY plane, normal should point in +Z
        let vertices = vec![
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
        ];
        mesh.set_vertices(vertices);
        mesh.set_faces(vec![0, 1, 2]);
        
        mesh.compute_face_normals();
        
        let n = mesh.get_face_normal(0).unwrap();
        assert!((n.x).abs() < 1e-6);
        assert!((n.y).abs() < 1e-6);
        assert!((n.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_bounding_box() {
        let mut mesh = MeshBuffer::new();
        
        let vertices = vec![
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 2.0, 3.0),
            Vec3f::new(-1.0, -2.0, -3.0),
        ];
        mesh.set_vertices(vertices);
        
        let bb = mesh.bounding_box();
        assert!((bb.min.x + 1.0).abs() < 1e-6);
        assert!((bb.max.x - 1.0).abs() < 1e-6);
    }
}
