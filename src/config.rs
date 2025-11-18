// Config handling
// Simple placeholder for future config handling.
// For the v1 starter we keep defaults in code.

pub struct Config {
    pub bpm: u32,
    pub steps: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { bpm: 120, steps: 8 }
    }
}
