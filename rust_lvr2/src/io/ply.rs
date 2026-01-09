//! PLY file format support
//!
//! Provides reading and writing of PLY (Polygon File Format) files.

use crate::types::{PointBuffer, MeshBuffer};
use crate::geometry::Vec3f;
use super::IoError;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::path::Path;

/// Loads a PLY file, returning point buffer and optionally mesh buffer.
pub fn load_ply<P: AsRef<Path>>(path: P) -> Result<(PointBuffer, Option<MeshBuffer>), IoError> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|_| IoError::FileNotFound(path.display().to_string()))?;
    let reader = BufReader::new(file);
    
    let mut lines = reader.lines();
    
    // Parse header
    let mut num_vertices = 0usize;
    let mut num_faces = 0usize;
    let mut has_normals = false;
    let mut has_colors = false;
    let mut in_header = true;
    let mut format_binary = false;
    
    while in_header {
        let line = lines.next()
            .ok_or_else(|| IoError::ParseError("Unexpected end of file in header".to_string()))??;
        let line = line.trim();
        
        if line == "end_header" {
            in_header = false;
        } else if line.starts_with("format") {
            if line.contains("binary") {
                format_binary = true;
            }
        } else if line.starts_with("element vertex") {
            num_vertices = line.split_whitespace()
                .nth(2)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| IoError::ParseError("Invalid vertex count".to_string()))?;
        } else if line.starts_with("element face") {
            num_faces = line.split_whitespace()
                .nth(2)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| IoError::ParseError("Invalid face count".to_string()))?;
        } else if line.starts_with("property") && line.contains("nx") {
            has_normals = true;
        } else if line.starts_with("property") && (line.contains("red") || line.contains("r ")) {
            has_colors = true;
        }
    }
    
    if format_binary {
        return Err(IoError::ParseError("Binary PLY not yet supported".to_string()));
    }
    
    // Parse vertices
    let mut points = Vec::with_capacity(num_vertices);
    let mut normals = if has_normals { Some(Vec::with_capacity(num_vertices)) } else { None };
    let mut colors = if has_colors { Some(Vec::with_capacity(num_vertices * 3)) } else { None };
    
    for _ in 0..num_vertices {
        let line = lines.next()
            .ok_or_else(|| IoError::ParseError("Unexpected end of file in vertices".to_string()))??;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() < 3 {
            return Err(IoError::ParseError("Invalid vertex line".to_string()));
        }
        
        let x: f32 = parts[0].parse().map_err(|_| IoError::ParseError("Invalid x coordinate".to_string()))?;
        let y: f32 = parts[1].parse().map_err(|_| IoError::ParseError("Invalid y coordinate".to_string()))?;
        let z: f32 = parts[2].parse().map_err(|_| IoError::ParseError("Invalid z coordinate".to_string()))?;
        
        points.push(Vec3f::new(x, y, z));
        
        if let Some(ref mut n) = normals {
            if parts.len() >= 6 {
                let nx: f32 = parts[3].parse().unwrap_or(0.0);
                let ny: f32 = parts[4].parse().unwrap_or(0.0);
                let nz: f32 = parts[5].parse().unwrap_or(1.0);
                n.push(Vec3f::new(nx, ny, nz));
            }
        }
        
        if let Some(ref mut c) = colors {
            let offset = if has_normals { 6 } else { 3 };
            if parts.len() >= offset + 3 {
                let r: u8 = parts[offset].parse().unwrap_or(128);
                let g: u8 = parts[offset + 1].parse().unwrap_or(128);
                let b: u8 = parts[offset + 2].parse().unwrap_or(128);
                c.push(r);
                c.push(g);
                c.push(b);
            }
        }
    }
    
    let mut point_buffer = PointBuffer::from_points(points);
    if let Some(n) = normals {
        if !n.is_empty() {
            point_buffer.set_normals(n);
        }
    }
    if let Some(c) = colors {
        if !c.is_empty() {
            point_buffer.set_colors(c, 3);
        }
    }
    
    // Parse faces
    let mesh_buffer = if num_faces > 0 {
        let mut vertices = Vec::with_capacity(num_vertices);
        for i in 0..num_vertices {
            vertices.push(point_buffer.get_point(i).unwrap());
        }
        
        let mut faces = Vec::with_capacity(num_faces * 3);
        for _ in 0..num_faces {
            let line = lines.next()
                .ok_or_else(|| IoError::ParseError("Unexpected end of file in faces".to_string()))??;
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() < 4 {
                return Err(IoError::ParseError("Invalid face line".to_string()));
            }
            
            let _n: usize = parts[0].parse().unwrap_or(3);
            let i0: u32 = parts[1].parse().map_err(|_| IoError::ParseError("Invalid face index".to_string()))?;
            let i1: u32 = parts[2].parse().map_err(|_| IoError::ParseError("Invalid face index".to_string()))?;
            let i2: u32 = parts[3].parse().map_err(|_| IoError::ParseError("Invalid face index".to_string()))?;
            
            faces.push(i0);
            faces.push(i1);
            faces.push(i2);
        }
        
        let mut mesh = MeshBuffer::new();
        mesh.set_vertices(vertices);
        mesh.set_faces(faces);
        Some(mesh)
    } else {
        None
    };
    
    Ok((point_buffer, mesh_buffer))
}

/// Saves a mesh to a PLY file.
pub fn save_ply<P: AsRef<Path>>(path: P, mesh: &MeshBuffer) -> Result<(), IoError> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    // Write header
    writeln!(writer, "ply")?;
    writeln!(writer, "format ascii 1.0")?;
    writeln!(writer, "element vertex {}", mesh.num_vertices())?;
    writeln!(writer, "property float x")?;
    writeln!(writer, "property float y")?;
    writeln!(writer, "property float z")?;
    
    if mesh.has_vertex_normals() {
        writeln!(writer, "property float nx")?;
        writeln!(writer, "property float ny")?;
        writeln!(writer, "property float nz")?;
    }
    
    if mesh.has_vertex_colors() {
        writeln!(writer, "property uchar red")?;
        writeln!(writer, "property uchar green")?;
        writeln!(writer, "property uchar blue")?;
    }
    
    writeln!(writer, "element face {}", mesh.num_faces())?;
    writeln!(writer, "property list uchar int vertex_indices")?;
    writeln!(writer, "end_header")?;
    
    // Write vertices
    for i in 0..mesh.num_vertices() {
        let v = mesh.get_vertex(i).unwrap();
        write!(writer, "{} {} {}", v.x, v.y, v.z)?;
        
        if mesh.has_vertex_normals() {
            let n = mesh.get_vertex_normal(i).unwrap();
            write!(writer, " {} {} {}", n.x, n.y, n.z)?;
        }
        
        if mesh.has_vertex_colors() {
            let c = mesh.get_vertex_color(i).unwrap();
            write!(writer, " {} {} {}", c[0], c[1], c[2])?;
        }
        
        writeln!(writer)?;
    }
    
    // Write faces
    for face in mesh.faces() {
        writeln!(writer, "3 {} {} {}", face[0], face[1], face[2])?;
    }
    
    Ok(())
}
