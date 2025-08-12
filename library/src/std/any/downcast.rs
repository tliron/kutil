use std::any::*;

/// Downcast to an [Any] reference.
pub trait DowncastRef {
    /// Downcast to an [Any] reference.
    fn downcast_ref<AnyT>(&self) -> Option<&AnyT>
    where
        AnyT: Any;
}
