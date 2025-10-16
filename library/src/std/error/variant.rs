use super::recipient::*;

use std::marker::*;

//
// VariantErrorRecipient
//

/// An [ErrorRecipient] for an error variant.
#[derive(Debug)]
pub struct VariantErrorRecipient<'inner, InnerT, ErrorT, ErrorVariantT> {
    /// Inner.
    pub inner: &'inner mut InnerT,

    error: PhantomData<ErrorT>,
    error_variant: PhantomData<ErrorVariantT>,
}

impl<'outer, InnerT, ErrorT, ErrorVariantT> ErrorRecipient<ErrorVariantT>
    for VariantErrorRecipient<'outer, InnerT, ErrorT, ErrorVariantT>
where
    InnerT: ErrorRecipient<ErrorT>,
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
// ToVariantErrorRecipient
//

/// Wrap in a [VariantErrorRecipient].
pub trait ToVariantErrorRecipient<'inner, InnerT, ErrorT, ErrorVariantT> {
    /// Wrap in a [VariantErrorRecipient].
    fn to_variant_error_recipient(&'inner mut self) -> VariantErrorRecipient<'inner, InnerT, ErrorT, ErrorVariantT>;
}

impl<'inner, InnerT, ErrorT, ErrorVariantT> ToVariantErrorRecipient<'inner, InnerT, ErrorT, ErrorVariantT> for InnerT
where
    InnerT: ErrorRecipient<ErrorT>,
    ErrorT: TryInto<ErrorVariantT>,
    ErrorVariantT: Into<ErrorT>,
{
    fn to_variant_error_recipient(&'inner mut self) -> VariantErrorRecipient<'inner, InnerT, ErrorT, ErrorVariantT> {
        VariantErrorRecipient { inner: self, error: PhantomData, error_variant: PhantomData }
    }
}
