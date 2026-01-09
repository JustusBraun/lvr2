//! PTS/XYZ file format support
//!
//! Provides reading of PTS and XYZ point cloud files.

use crate::types::PointBuffer;
use crate::geometry::Vec3f;
use super::IoError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Loads a PTS or XYZ point cloud file.
///
/// PTS format: Each line contains x y z [intensity] [r g b]
/// XYZ format: Each line contains x y z [r g b]
pub fn load_pts<P: AsRef<Path>>(path: P) -> Result<PointBuffer, IoError> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|_| IoError::FileNotFound(path.display().to_string()))?;
    let reader = BufReader::new(file);
    
    let mut points = Vec::new();
    let mut intensities = Vec::new();
    let mut colors = Vec::new();
    let mut has_intensity = false;
    let mut has_colors = false;
    let mut first_line = true;
    let mut skip_first_line = false;
    
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // First line might be point count
        if first_line {
            first_line = false;
            if parts.len() == 1 {
                if let Ok(_count) = parts[0].parse::<usize>() {
                    skip_first_line = true;
                    continue;
                }
            }
        }
        
        if parts.len() < 3 {
            continue;
        }
        
        // Parse coordinates
        let x: f32 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let y: f32 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let z: f32 = match parts[2].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        
        points.push(Vec3f::new(x, y, z));
        
        // Check for intensity (4th value) and colors (5th-7th or 4th-6th)
        if parts.len() >= 4 {
            // Try to determine if we have intensity or colors
            // Intensity is usually a float, colors are usually integers 0-255
            let val: f32 = parts[3].parse().unwrap_or(0.0);
            
            if parts.len() >= 7 {
                // x y z intensity r g b
                has_intensity = true;
                has_colors = true;
                intensities.push(val);
                colors.push(parts[4].parse().unwrap_or(128));
                colors.push(parts[5].parse().unwrap_or(128));
                colors.push(parts[6].parse().unwrap_or(128));
            } else if parts.len() >= 6 {
                // x y z r g b (no intensity)
                has_colors = true;
                colors.push(parts[3].parse().unwrap_or(128));
                colors.push(parts[4].parse().unwrap_or(128));
                colors.push(parts[5].parse().unwrap_or(128));
            } else if parts.len() == 4 {
                // x y z intensity
                has_intensity = true;
                intensities.push(val);
            }
        }
    }
    
    if points.is_empty() {
        return Err(IoError::ParseError("No valid points found in file".to_string()));
    }
    
    let mut buffer = PointBuffer::from_points(points);
    
    if has_intensity && !intensities.is_empty() {
        buffer.set_intensities(intensities);
    }
    
    if has_colors && !colors.is_empty() {
        buffer.set_colors(colors, 3);
    }
    
    Ok(buffer)
}
