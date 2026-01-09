//! Generic channel for storing attribute data
//!
//! Channels are used to store per-point or per-vertex attributes
//! like colors, normals, or custom data.

use std::any::Any;

/// A typed channel storing attribute data.
///
/// Each element in the channel corresponds to a point or vertex.
/// The channel width determines how many values make up a single element.
///
/// # Examples
///
/// ```
/// use lvr2::types::Channel;
///
/// // Create a channel for RGB colors (3 values per element)
/// let colors = Channel::new(vec![255u8, 0, 0, 0, 255, 0], 3);
/// assert_eq!(colors.len(), 2);  // 2 color elements
/// ```
#[derive(Debug, Clone)]
pub struct Channel<T> {
    /// Raw data storage
    data: Vec<T>,
    /// Number of values per element
    width: usize,
}

impl<T: Clone> Channel<T> {
    /// Creates a new channel with the given data and width.
    pub fn new(data: Vec<T>, width: usize) -> Self {
        assert!(width > 0, "Channel width must be positive");
        assert!(data.len() % width == 0, "Data length must be divisible by width");
        Self { data, width }
    }

    /// Creates an empty channel with the given width.
    pub fn with_width(width: usize) -> Self {
        assert!(width > 0, "Channel width must be positive");
        Self {
            data: Vec::new(),
            width,
        }
    }

    /// Returns the number of elements in the channel.
    pub fn len(&self) -> usize {
        self.data.len() / self.width
    }

    /// Returns true if the channel is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the width (values per element) of the channel.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns a reference to the raw data.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable reference to the raw data.
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Gets a slice of values for the element at the given index.
    pub fn get(&self, index: usize) -> Option<&[T]> {
        if index >= self.len() {
            return None;
        }
        let start = index * self.width;
        Some(&self.data[start..start + self.width])
    }

    /// Gets a mutable slice of values for the element at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut [T]> {
        if index >= self.len() {
            return None;
        }
        let start = index * self.width;
        Some(&mut self.data[start..start + self.width])
    }

    /// Pushes a new element to the channel.
    pub fn push(&mut self, values: &[T]) {
        assert_eq!(values.len(), self.width, "Values length must match channel width");
        self.data.extend_from_slice(values);
    }

    /// Reserves capacity for additional elements.
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional * self.width);
    }

    /// Clears the channel.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Consumes the channel and returns the underlying data.
    pub fn into_data(self) -> Vec<T> {
        self.data
    }
}

impl<T: Clone + Default> Channel<T> {
    /// Creates a channel with the given number of elements, initialized to default values.
    pub fn with_size(len: usize, width: usize) -> Self {
        Self {
            data: vec![T::default(); len * width],
            width,
        }
    }
}

/// Iterator over channel elements.
pub struct ChannelIter<'a, T> {
    channel: &'a Channel<T>,
    index: usize,
}

impl<'a, T: Clone> Iterator for ChannelIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.channel.get(self.index);
        self.index += 1;
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.channel.len().saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a, T: Clone> IntoIterator for &'a Channel<T> {
    type Item = &'a [T];
    type IntoIter = ChannelIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ChannelIter {
            channel: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ch = Channel::new(vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0], 3);
        assert_eq!(ch.len(), 2);
        assert_eq!(ch.width(), 3);
    }

    #[test]
    fn test_get() {
        let ch = Channel::new(vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0], 3);
        assert_eq!(ch.get(0), Some([1.0, 2.0, 3.0].as_slice()));
        assert_eq!(ch.get(1), Some([4.0, 5.0, 6.0].as_slice()));
        assert_eq!(ch.get(2), None);
    }

    #[test]
    fn test_push() {
        let mut ch = Channel::with_width(3);
        ch.push(&[1.0f32, 2.0, 3.0]);
        ch.push(&[4.0, 5.0, 6.0]);
        assert_eq!(ch.len(), 2);
    }

    #[test]
    fn test_iterator() {
        let ch = Channel::new(vec![1, 2, 3, 4], 2);
        let elements: Vec<_> = ch.into_iter().collect();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], &[1, 2]);
        assert_eq!(elements[1], &[3, 4]);
    }
}
