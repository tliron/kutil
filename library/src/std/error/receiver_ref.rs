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
// ErrorReceiverAsRef
//

/// As error receiver reference.
pub trait ErrorReceiverAsRef<'own, ErrorT, ErrorReceiverT> {
    /// As error receiver reference.
    fn as_ref(&'own mut self) -> ErrorReceiverRef<'own, ErrorT>;
}

impl<'own, ErrorT, ErrorReceiverT> ErrorReceiverAsRef<'own, ErrorT, ErrorReceiverT> for ErrorReceiverT
where
    ErrorReceiverT: ErrorReceiver<ErrorT>,
{
    fn as_ref(&'own mut self) -> ErrorReceiverRef<'own, ErrorT> {
        ErrorReceiverRef::new(RefCell::new(self))
    }
}
