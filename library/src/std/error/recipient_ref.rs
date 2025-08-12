use super::recipient::*;

use std::{cell::*, sync::*};

//
// ErrorRecipientRef
//

/// Common reference type for [ErrorRecipient].
pub type ErrorRecipientRef<'own, ErrorT> = Arc<RefCell<&'own mut dyn ErrorRecipient<ErrorT>>>;

//
// ErrorRecipientToRef
//

/// To error recipient reference.
pub trait ErrorRecipientToRef<'own, ErrorT, ErrorRecipientT> {
    /// To error recipient reference.
    fn to_ref(&'own mut self) -> ErrorRecipientRef<'own, ErrorT>;
}

impl<'own, ErrorT, ErrorRecipientT> ErrorRecipientToRef<'own, ErrorT, ErrorRecipientT> for ErrorRecipientT
where
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    fn to_ref(&'own mut self) -> ErrorRecipientRef<'own, ErrorT> {
        ErrorRecipientRef::new(RefCell::new(self))
    }
}

//
// ErrorRecipientRefImplementation
//

/// An [ErrorRecipient] implementation for an [ErrorRecipientRef].
pub struct ErrorRecipientRefImplementation<'own, ErrorT> {
    /// Inner.
    pub inner: ErrorRecipientRef<'own, ErrorT>,
}

impl<'own, ErrorT> ErrorRecipient<ErrorT> for ErrorRecipientRefImplementation<'own, ErrorT> {
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        self.inner.borrow_mut().give_error(error)
    }
}

//
// ToErrorRecipient
//

/// Create an [ErrorRecipient] implementation for an [ErrorRecipientRef].
pub trait ToErrorRecipient<'own, ErrorT> {
    /// Create an [ErrorRecipient] implementation for an [ErrorRecipientRef].
    fn to_error_recipient(&self) -> ErrorRecipientRefImplementation<'own, ErrorT>;
}

impl<'own, ErrorT> ToErrorRecipient<'own, ErrorT> for ErrorRecipientRef<'own, ErrorT> {
    fn to_error_recipient(&self) -> ErrorRecipientRefImplementation<'own, ErrorT> {
        ErrorRecipientRefImplementation { inner: self.clone() }
    }
}
