//! Progress reporting utilities

use std::io::{self, Write};

/// A simple progress bar for console output.
pub struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
    message: String,
}

impl ProgressBar {
    /// Creates a new progress bar.
    pub fn new(total: usize, message: &str) -> Self {
        Self {
            total,
            current: 0,
            width: 50,
            message: message.to_string(),
        }
    }
    
    /// Updates the progress.
    pub fn set(&mut self, current: usize) {
        self.current = current.min(self.total);
        self.draw();
    }
    
    /// Increments the progress by one.
    pub fn inc(&mut self) {
        self.current = (self.current + 1).min(self.total);
        self.draw();
    }
    
    /// Finishes the progress bar.
    pub fn finish(&self) {
        println!();
    }
    
    fn draw(&self) {
        let progress = if self.total > 0 {
            self.current as f64 / self.total as f64
        } else {
            0.0
        };
        
        let filled = (progress * self.width as f64) as usize;
        let empty = self.width - filled;
        
        print!(
            "\r{}: [{}{}] {:.1}%",
            self.message,
            "=".repeat(filled),
            " ".repeat(empty),
            progress * 100.0
        );
        
        io::stdout().flush().ok();
    }
}
