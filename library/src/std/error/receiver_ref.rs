use super::receiver::*;

use std::{cell::*, sync::*};

//
// ErrorReceiverRef
//

/// Common reference type for [ErrorReceiver].
pub type ErrorReceiverRef<'own, ErrorT> = Arc<RefCell<&'own mut dyn ErrorReceiver<ErrorT>>>;

impl<'own, ErrorT> ErrorReceiver<ErrorT> for ErrorReceiverRef<'own, ErrorT> {
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        self.borrow_mut().give_error(error)
    }
}

//
// ErrorReceiverToRef
//

/// To error receiver reference.
pub trait ErrorReceiverToRef<'own, ErrorT, ErrorReceiverT> {
    /// To error receiver reference.
    fn to_ref(&'own mut self) -> ErrorReceiverRef<'own, ErrorT>;
}

impl<'own, ErrorT, ErrorReceiverT> ErrorReceiverToRef<'own, ErrorT, ErrorReceiverT> for ErrorReceiverT
where
    ErrorReceiverT: ErrorReceiver<ErrorT>,
{
    fn to_ref(&'own mut self) -> ErrorReceiverRef<'own, ErrorT> {
        ErrorReceiverRef::new(RefCell::new(self))
    }
}

// TODO: do we need this?

//
// ErrorReceiverRefImplementation
//

/// An [ErrorReceiver] implementation for an [ErrorReceiverRef].
pub struct ErrorReceiverRefImplementation<'own, ErrorT> {
    /// Inner.
    pub inner: ErrorReceiverRef<'own, ErrorT>,
}

impl<'own, ErrorT> ErrorReceiver<ErrorT> for ErrorReceiverRefImplementation<'own, ErrorT> {
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        self.inner.borrow_mut().give_error(error)
    }
}

//
// ToErrorReceiver
//

/// Create an [ErrorReceiver] implementation for an [ErrorReceiverRef].
pub trait ToErrorReceiver<'own, ErrorT> {
    /// Create an [ErrorReceiver] implementation for an [ErrorReceiverRef].
    fn to_error_receiver(&self) -> ErrorReceiverRefImplementation<'own, ErrorT>;
}

impl<'own, ErrorT> ToErrorReceiver<'own, ErrorT> for ErrorReceiverRef<'own, ErrorT> {
    fn to_error_receiver(&self) -> ErrorReceiverRefImplementation<'own, ErrorT> {
        ErrorReceiverRefImplementation { inner: self.clone() }
    }
}
