use std::any::*;

//
// IntoAny
//

/// Convert into a boxed [Any].
pub trait IntoAny {
    /// Convert into a boxed [Any].
    fn into_any(&mut self) -> Box<dyn Any>;
}

//
// IntoConcrete
//

/// Convert into a boxed concrete type.
pub trait IntoConcrete {
    /// Convert into a boxed concrete type.
    fn into_concrete<AnyT>(&mut self) -> Result<Box<AnyT>, Box<dyn Any>>
    where
        AnyT: Any;
}

impl<IntoAnyT> IntoConcrete for IntoAnyT
where
    IntoAnyT: IntoAny + ?Sized,
{
    fn into_concrete<AnyT>(&mut self) -> Result<Box<AnyT>, Box<dyn Any>>
    where
        AnyT: Any,
    {
        self.into_any().downcast()
    }
}
