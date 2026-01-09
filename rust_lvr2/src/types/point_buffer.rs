//! Point buffer for storing point cloud data
//!
//! A point buffer stores 3D point coordinates along with associated
//! attributes like normals, colors, and intensity values.

use crate::geometry::{BaseVector, Vec3f, BoundingBox};
use super::Channel;
use std::collections::HashMap;

/// A buffer for storing point cloud data with arbitrary attributes.
///
/// The point buffer stores 3D coordinates as well as optional attributes
/// like normals, colors, and intensity values. All attributes must have
/// the same number of elements as the point array.
///
/// # Examples
///
/// ```
/// use lvr2::types::PointBuffer;
/// use lvr2::geometry::Vec3f;
///
/// let points = vec![
///     Vec3f::new(0.0, 0.0, 0.0),
///     Vec3f::new(1.0, 0.0, 0.0),
///     Vec3f::new(0.0, 1.0, 0.0),
/// ];
///
/// let mut buffer = PointBuffer::from_points(points);
/// assert_eq!(buffer.num_points(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct PointBuffer {
    /// Point coordinates (x, y, z)
    points: Channel<f32>,
    /// Point normals (nx, ny, nz)
    normals: Option<Channel<f32>>,
    /// Point colors (r, g, b) or (r, g, b, a)
    colors: Option<Channel<u8>>,
    /// Point intensities
    intensities: Option<Channel<f32>>,
    /// Custom float channels
    float_channels: HashMap<String, Channel<f32>>,
    /// Custom unsigned char channels
    uchar_channels: HashMap<String, Channel<u8>>,
}

impl PointBuffer {
    /// Creates an empty point buffer.
    pub fn new() -> Self {
        Self {
            points: Channel::with_width(3),
            normals: None,
            colors: None,
            intensities: None,
            float_channels: HashMap::new(),
            uchar_channels: HashMap::new(),
        }
    }

    /// Creates a point buffer from a vector of 3D points.
    pub fn from_points(points: Vec<Vec3f>) -> Self {
        let n = points.len();
        let mut data = Vec::with_capacity(n * 3);
        for p in points {
            data.push(p.x);
            data.push(p.y);
            data.push(p.z);
        }
        Self {
            points: Channel::new(data, 3),
            normals: None,
            colors: None,
            intensities: None,
            float_channels: HashMap::new(),
            uchar_channels: HashMap::new(),
        }
    }

    /// Creates a point buffer from raw coordinate data.
    pub fn from_raw(data: Vec<f32>) -> Self {
        Self {
            points: Channel::new(data, 3),
            normals: None,
            colors: None,
            intensities: None,
            float_channels: HashMap::new(),
            uchar_channels: HashMap::new(),
        }
    }

    /// Returns the number of points in the buffer.
    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Gets the point at the given index.
    pub fn get_point(&self, index: usize) -> Option<Vec3f> {
        self.points.get(index).map(|p| Vec3f::new(p[0], p[1], p[2]))
    }

    /// Returns an iterator over all points.
    pub fn points(&self) -> impl Iterator<Item = Vec3f> + '_ {
        self.points.into_iter().map(|p| Vec3f::new(p[0], p[1], p[2]))
    }

    /// Returns a reference to the raw point data.
    pub fn point_data(&self) -> &[f32] {
        self.points.data()
    }

    /// Sets the point array.
    pub fn set_points(&mut self, points: Vec<Vec3f>) {
        let mut data = Vec::with_capacity(points.len() * 3);
        for p in points {
            data.push(p.x);
            data.push(p.y);
            data.push(p.z);
        }
        self.points = Channel::new(data, 3);
    }

    /// Returns true if the buffer has normals.
    pub fn has_normals(&self) -> bool {
        self.normals.is_some()
    }

    /// Gets the normal at the given index.
    pub fn get_normal(&self, index: usize) -> Option<Vec3f> {
        self.normals
            .as_ref()
            .and_then(|n| n.get(index))
            .map(|n| Vec3f::new(n[0], n[1], n[2]))
    }

    /// Returns an iterator over all normals.
    pub fn normals(&self) -> Option<impl Iterator<Item = Vec3f> + '_> {
        self.normals
            .as_ref()
            .map(|n| n.into_iter().map(|n| Vec3f::new(n[0], n[1], n[2])))
    }

    /// Returns a reference to the raw normal data.
    pub fn normal_data(&self) -> Option<&[f32]> {
        self.normals.as_ref().map(|n| n.data())
    }

    /// Sets the normal array.
    pub fn set_normals(&mut self, normals: Vec<Vec3f>) {
        assert_eq!(normals.len(), self.num_points(), "Normals count must match point count");
        let mut data = Vec::with_capacity(normals.len() * 3);
        for n in normals {
            data.push(n.x);
            data.push(n.y);
            data.push(n.z);
        }
        self.normals = Some(Channel::new(data, 3));
    }

    /// Sets the normal array from raw data.
    pub fn set_normals_raw(&mut self, data: Vec<f32>) {
        assert_eq!(data.len() / 3, self.num_points(), "Normals count must match point count");
        self.normals = Some(Channel::new(data, 3));
    }

    /// Returns true if the buffer has colors.
    pub fn has_colors(&self) -> bool {
        self.colors.is_some()
    }

    /// Gets the color at the given index.
    pub fn get_color(&self, index: usize) -> Option<&[u8]> {
        self.colors.as_ref().and_then(|c| c.get(index))
    }

    /// Returns the color channel width (3 for RGB, 4 for RGBA).
    pub fn color_width(&self) -> Option<usize> {
        self.colors.as_ref().map(|c| c.width())
    }

    /// Sets the color array.
    pub fn set_colors(&mut self, data: Vec<u8>, width: usize) {
        assert!(width == 3 || width == 4, "Color width must be 3 (RGB) or 4 (RGBA)");
        assert_eq!(data.len() / width, self.num_points(), "Colors count must match point count");
        self.colors = Some(Channel::new(data, width));
    }

    /// Returns true if the buffer has intensity values.
    pub fn has_intensities(&self) -> bool {
        self.intensities.is_some()
    }

    /// Gets the intensity at the given index.
    pub fn get_intensity(&self, index: usize) -> Option<f32> {
        self.intensities
            .as_ref()
            .and_then(|i| i.get(index))
            .map(|i| i[0])
    }

    /// Sets the intensity array.
    pub fn set_intensities(&mut self, data: Vec<f32>) {
        assert_eq!(data.len(), self.num_points(), "Intensities count must match point count");
        self.intensities = Some(Channel::new(data, 1));
    }

    /// Computes the bounding box of all points.
    pub fn bounding_box(&self) -> BoundingBox<f32> {
        self.points().collect()
    }

    /// Adds a custom float channel.
    pub fn add_float_channel(&mut self, name: &str, data: Vec<f32>, width: usize) {
        assert_eq!(data.len() / width, self.num_points(), "Channel length must match point count");
        self.float_channels.insert(name.to_string(), Channel::new(data, width));
    }

    /// Gets a custom float channel.
    pub fn get_float_channel(&self, name: &str) -> Option<&Channel<f32>> {
        self.float_channels.get(name)
    }

    /// Adds a custom unsigned char channel.
    pub fn add_uchar_channel(&mut self, name: &str, data: Vec<u8>, width: usize) {
        assert_eq!(data.len() / width, self.num_points(), "Channel length must match point count");
        self.uchar_channels.insert(name.to_string(), Channel::new(data, width));
    }

    /// Gets a custom unsigned char channel.
    pub fn get_uchar_channel(&self, name: &str) -> Option<&Channel<u8>> {
        self.uchar_channels.get(name)
    }

    /// Creates a clone of this buffer.
    pub fn clone_buffer(&self) -> Self {
        self.clone()
    }
}

impl Default for PointBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_points() {
        let points = vec![
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
        ];
        let buffer = PointBuffer::from_points(points);
        assert_eq!(buffer.num_points(), 3);
    }

    #[test]
    fn test_get_point() {
        let points = vec![
            Vec3f::new(1.0, 2.0, 3.0),
            Vec3f::new(4.0, 5.0, 6.0),
        ];
        let buffer = PointBuffer::from_points(points);
        
        let p = buffer.get_point(0).unwrap();
        assert!((p.x - 1.0).abs() < 1e-6);
        assert!((p.y - 2.0).abs() < 1e-6);
        assert!((p.z - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_normals() {
        let points = vec![Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(1.0, 0.0, 0.0)];
        let normals = vec![Vec3f::new(0.0, 0.0, 1.0), Vec3f::new(0.0, 0.0, 1.0)];
        
        let mut buffer = PointBuffer::from_points(points);
        buffer.set_normals(normals);
        
        assert!(buffer.has_normals());
        let n = buffer.get_normal(0).unwrap();
        assert!((n.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 2.0, 3.0),
        ];
        let buffer = PointBuffer::from_points(points);
        let bb = buffer.bounding_box();
        
        assert!(bb.is_valid());
        assert!((bb.min.x).abs() < 1e-6);
        assert!((bb.max.x - 1.0).abs() < 1e-6);
    }
}
