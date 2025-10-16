use super::receiver::*;

//
// FailFastErrorReceiver
//

/// [ErrorReceiver] that fails on the first given error.
pub struct FailFastErrorReceiver;

impl<ErrorT> ErrorReceiver<ErrorT> for FailFastErrorReceiver {
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        Err(error)
    }
}
