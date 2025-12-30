use std::any::*;

//
// AsAnyRef
//

/// As an [Any] reference.
pub trait AsAnyRef {
    /// As an [Any] reference.
    ///
    /// Implementations that do not support it should return [None].
    fn as_any_ref(&self) -> Option<&dyn Any>;
}

/// Implement [AsAnyRef].
#[macro_export]
macro_rules! impl_as_any_ref {
    ( $type:ty $(,)? ) => {
        impl ::kutil::std::any::AsAnyRef for $type {
            fn as_any_ref(&self) -> Option<&dyn ::std::any::Any> {
                Some(self)
            }
        }
    };
}

//
// DowncastRef
//

/// As a concrete type reference.
pub trait DowncastRef {
    /// As a concrete type reference.
    fn downcast_ref<AnyT>(&self) -> Option<&AnyT>
    where
        AnyT: 'static;
}

impl<AsAnyRefT> DowncastRef for AsAnyRefT
where
    AsAnyRefT: AsAnyRef,
{
    fn downcast_ref<AnyT>(&self) -> Option<&AnyT>
    where
        AnyT: 'static,
    {
        self.as_any_ref().and_then(|any| any.downcast_ref())
    }
}

#[allow(unused_imports)]
pub use impl_as_any_ref;
