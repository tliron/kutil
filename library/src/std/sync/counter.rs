use std::sync::atomic::*;

/// Thread-safe counter.
#[derive(Debug)]
pub struct Counter(AtomicUsize);

impl Counter {
    /// Constructor.
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    /// Next value.
    pub fn next(&self) -> usize {
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
