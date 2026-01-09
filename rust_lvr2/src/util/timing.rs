//! Timing utilities

use std::time::{Duration, Instant};

/// A simple timer for measuring elapsed time.
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    /// Creates and starts a new timer.
    pub fn new(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }
    
    /// Returns the elapsed time.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    /// Returns the elapsed time in seconds.
    pub fn elapsed_secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }
    
    /// Returns the elapsed time in milliseconds.
    pub fn elapsed_ms(&self) -> u128 {
        self.elapsed().as_millis()
    }
    
    /// Stops the timer and logs the elapsed time.
    pub fn stop(&self) {
        log::info!("{}: {:.3}s", self.name, self.elapsed_secs());
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        // Optionally log on drop
    }
}

/// Measures the execution time of a closure.
pub fn measure_time<F, T>(name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let timer = Timer::new(name);
    let result = f();
    timer.stop();
    result
}
