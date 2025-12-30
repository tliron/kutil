use super::{Mutable, ObjectMarkerFor, Problem};

use std::fmt;

//
// MaybeAttach
//

/// Maybe attach.
pub trait MaybeAttach<ThreadSafetyT> {
    /// Maybe attach.
    fn maybe_attach<AttachmentT>(self, attachment: Option<AttachmentT>) -> Self
    where
        AttachmentT: fmt::Debug + fmt::Display + ObjectMarkerFor<ThreadSafetyT>;
}

impl<ContextT, ThreadSafetyT> MaybeAttach<ThreadSafetyT> for Problem<ContextT, Mutable, ThreadSafetyT> {
    fn maybe_attach<AttachmentT>(self, attachment: Option<AttachmentT>) -> Self
    where
        AttachmentT: fmt::Debug + fmt::Display + ObjectMarkerFor<ThreadSafetyT>,
    {
        match attachment {
            Some(attachment) => self.attach(attachment),
            None => self,
        }
    }
}

impl<OkT, ContextT, ThreadSafetyT> MaybeAttach<ThreadSafetyT>
    for Result<OkT, Problem<ContextT, Mutable, ThreadSafetyT>>
{
    fn maybe_attach<AttachmentT>(self, attachment: Option<AttachmentT>) -> Self
    where
        AttachmentT: fmt::Debug + fmt::Display + ObjectMarkerFor<ThreadSafetyT>,
    {
        self.map_err(|report| report.maybe_attach(attachment))
    }
}
