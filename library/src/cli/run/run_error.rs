//
// RunError
//

/// Error for [run](super::run::run).
pub trait RunError {
    /// Handle the error.
    ///
    /// If we return false will print the error message in red.
    fn handle(&self) -> (bool, u8) {
        (false, 1)
    }
}
