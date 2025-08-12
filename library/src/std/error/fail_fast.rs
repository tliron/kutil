use super::recipient::*;

//
// FailFastErrorRecipient
//

/// [ErrorRecipient] that fails on the first given error.
pub struct FailFastErrorRecipient;

impl<ErrorT> ErrorRecipient<ErrorT> for FailFastErrorRecipient {
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        Err(error)
    }
}
