use super::receiver::*;

use std::marker::*;

//
// VariantErrorReceiver
//

/// An [ErrorReceiver] for an error variant.
#[derive(Debug)]
pub struct VariantErrorReceiver<'inner, InnerT, ErrorT, ErrorVariantT> {
    /// Inner.
    pub inner: &'inner mut InnerT,

    error: PhantomData<ErrorT>,
    error_variant: PhantomData<ErrorVariantT>,
}

impl<'outer, InnerT, ErrorT, ErrorVariantT> ErrorReceiver<ErrorVariantT>
    for VariantErrorReceiver<'outer, InnerT, ErrorT, ErrorVariantT>
where
    InnerT: ErrorReceiver<ErrorT>,
    ErrorT: TryInto<ErrorVariantT>,
    ErrorVariantT: Into<ErrorT>,
{
    fn give_error(&mut self, error: ErrorVariantT) -> Result<(), ErrorVariantT> {
        self.inner.give_error(error.into()).or_else(|error| match error.try_into() {
            Ok(error) => Err(error),
            Err(_) => Ok(()), // this should never happen
        })
    }
}

//
// ToVariantErrorReceiver
//

/// Wrap in a [VariantErrorReceiver].
pub trait ToVariantErrorReceiver<'inner, InnerT, ErrorT, ErrorVariantT> {
    /// Wrap in a [VariantErrorReceiver].
    fn to_variant_error_receiver(&'inner mut self) -> VariantErrorReceiver<'inner, InnerT, ErrorT, ErrorVariantT>;
}

impl<'inner, InnerT, ErrorT, ErrorVariantT> ToVariantErrorReceiver<'inner, InnerT, ErrorT, ErrorVariantT> for InnerT
where
    InnerT: ErrorReceiver<ErrorT>,
    ErrorT: TryInto<ErrorVariantT>,
    ErrorVariantT: Into<ErrorT>,
{
    fn to_variant_error_receiver(&'inner mut self) -> VariantErrorReceiver<'inner, InnerT, ErrorT, ErrorVariantT> {
        VariantErrorReceiver { inner: self, error: PhantomData, error_variant: PhantomData }
    }
}
