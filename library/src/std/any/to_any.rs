use std::any::*;

//
// ToAny
//

/// To a boxed [Any].
pub trait ToAny {
    /// To a boxed [Any].
    ///
    /// Implementations that do not support it should return [None].
    fn to_any(&mut self) -> Option<Box<dyn Any>>;
}

//
// Downcast
//

/// To a boxed concrete type.
pub trait Downcast {
    /// To a boxed concrete type.
    fn downcast<AnyT>(&mut self) -> Option<Box<AnyT>>
    where
        AnyT: Any;
}

impl<IntoAnyT> Downcast for IntoAnyT
where
    IntoAnyT: ToAny + ?Sized,
{
    fn downcast<AnyT>(&mut self) -> Option<Box<AnyT>>
    where
        AnyT: Any,
    {
        self.to_any().and_then(|any| any.downcast().ok())
    }
}
