use super::{Local, Mutable, Problem, ProblemConversion, ProblemResultExt, SendSync};

use {
    rootcause::{IntoReport, IntoReportCollection},
    std::fmt,
};

//
// ProblemResultExtension
//

/// Extension for [ProblemResultExt].
pub trait ProblemResultExtension<OkT, ErrorT> {
    /// On error, create a new problem with the provided context. The error will be its child.
    ///
    /// Alias for [ProblemResultExt::context].
    fn in_context<ContextT>(self, context: ContextT) -> Result<OkT, Problem<ContextT, Mutable, SendSync>>
    where
        ErrorT: IntoReportCollection<SendSync>,
        ContextT: fmt::Debug + fmt::Display + Send + Sync;

    /// On error, create a new problem with the provided context. The error will be its child.
    ///
    /// Alias for [ProblemResultExt::local_context].
    fn in_context_local<ContextT>(self, context: ContextT) -> Result<OkT, Problem<ContextT, Mutable, Local>>
    where
        ErrorT: IntoReportCollection<Local>,
        ContextT: fmt::Debug + fmt::Display;

    /// On error, convert into a problem with a different context.
    ///
    /// Depends on an existing implementation of [ProblemConversion].
    ///
    /// Alias for [ProblemResultExt::context_to].
    fn convert_into<ContextT>(self) -> Result<OkT, Problem<ContextT, Mutable, SendSync>>
    where
        ErrorT: IntoReport<SendSync>,
        ContextT: ProblemConversion<ErrorT::Context, ErrorT::Ownership, SendSync>;

    /// On error, convert into a problem with a different context.
    ///
    /// Depends on an existing implementation of [ProblemConversion].
    ///
    /// Alias for [ProblemResultExt::local_context_to].
    fn convert_into_local<ContextT>(self) -> Result<OkT, Problem<ContextT, Mutable, Local>>
    where
        ErrorT: IntoReport<Local>,
        ContextT: ProblemConversion<ErrorT::Context, ErrorT::Ownership, Local>;
}

impl<OkT, ErrorT> ProblemResultExtension<OkT, ErrorT> for Result<OkT, ErrorT> {
    fn in_context<ContextT>(self, context: ContextT) -> Result<OkT, Problem<ContextT, Mutable, SendSync>>
    where
        ErrorT: IntoReportCollection<SendSync>,
        ContextT: fmt::Debug + fmt::Display + Send + Sync,
    {
        self.context(context)
    }

    fn in_context_local<ContextT>(self, context: ContextT) -> Result<OkT, Problem<ContextT, Mutable, Local>>
    where
        ErrorT: IntoReportCollection<Local>,
        ContextT: fmt::Debug + fmt::Display,
    {
        self.local_context(context)
    }

    fn convert_into<ContextT>(self) -> Result<OkT, Problem<ContextT, Mutable, SendSync>>
    where
        ErrorT: IntoReport<SendSync>,
        ContextT: ProblemConversion<ErrorT::Context, ErrorT::Ownership, SendSync>,
    {
        self.context_to()
    }

    fn convert_into_local<ContextT>(self) -> Result<OkT, Problem<ContextT, Mutable, Local>>
    where
        ErrorT: IntoReport<Local>,
        ContextT: ProblemConversion<ErrorT::Context, ErrorT::Ownership, Local>,
    {
        self.local_context_to()
    }
}
