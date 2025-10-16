use std::any::*;

//
// IntoAnyRef
//

/// Convert into [Any] reference.
pub trait IntoAnyRef {
    /// Convert into [Any] reference.
    fn into_any_ref<AnyT>(&self) -> Option<&AnyT>
    where
        AnyT: Any;
}
