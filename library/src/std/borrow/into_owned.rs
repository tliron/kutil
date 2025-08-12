//
// IntoOwned
//

/// Into owned.
///
/// Note the difference between this and [ToOwned]: here `self` is moved rather than referenced.
/// This allows implementations to avoid allocation when unnecessary. For example, if we are
/// already owned then we can return `self`.
pub trait IntoOwned {
    /// Into owned.
    fn into_owned(self) -> Self;
}
