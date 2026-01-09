//! File I/O for point clouds and meshes
//!
//! This module provides functions for loading and saving point clouds
//! and meshes in various formats (PLY, PTS, XYZ, OBJ).

mod ply;
mod pts;

pub use ply::{load_ply, save_ply};
pub use pts::load_pts;

use crate::types::{PointBuffer, MeshBuffer};
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during file I/O
#[derive(Error, Debug)]
pub enum IoError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Loads points from a file, auto-detecting the format.
pub fn load_points<P: AsRef<Path>>(path: P) -> Result<PointBuffer, IoError> {
    let path = path.as_ref();
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    
    match extension.as_str() {
        "pts" | "xyz" => load_pts(path),
        "ply" => {
            let (points, _) = load_ply(path)?;
            Ok(points)
        }
        _ => Err(IoError::UnsupportedFormat(extension)),
    }
}

/// Saves a mesh to a file, auto-detecting the format.
pub fn save_mesh<P: AsRef<Path>>(path: P, mesh: &MeshBuffer) -> Result<(), IoError> {
    let path = path.as_ref();
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    
    match extension.as_str() {
        "ply" => save_ply(path, mesh),
        _ => Err(IoError::UnsupportedFormat(extension)),
    }
}
