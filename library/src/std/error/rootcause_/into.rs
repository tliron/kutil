use super::{Problem, ProblemCollection};

use rootcause::{IntoReport, IntoReportCollection};

//
// IntoProblem
//

/// Alias for [IntoReport].
pub trait IntoProblem<ThreadSafetyT> {
    /// The context type of the resulting problem.
    type Context: ?Sized + 'static;

    /// The ownership marker of the resulting problem.
    type Ownership: 'static;

    /// Alias for [IntoReport::into_report].
    fn into_problem(self) -> Problem<Self::Context, Self::Ownership, ThreadSafetyT>;
}

impl<IntoReportT, ThreadSafetyT> IntoProblem<ThreadSafetyT> for IntoReportT
where
    IntoReportT: IntoReport<ThreadSafetyT>,
{
    type Context = IntoReportT::Context;
    type Ownership = IntoReportT::Ownership;

    fn into_problem(self) -> Problem<Self::Context, Self::Ownership, ThreadSafetyT> {
        self.into_report()
    }
}

//
// IntoProblemCollection
//

/// Alias for [IntoReportCollection].
pub trait IntoProblemCollection<ThreadSafetyT> {
    /// The context type of the resulting problem collection.
    type Context: ?Sized + 'static;

    /// Alias for [IntoReportCollection::into_report_collection].
    fn into_problem_collection(self) -> ProblemCollection<Self::Context, ThreadSafetyT>;
}

impl<IntoReportCollectionT, ThreadSafetyT> IntoProblemCollection<ThreadSafetyT> for IntoReportCollectionT
where
    IntoReportCollectionT: IntoReportCollection<ThreadSafetyT>,
{
    type Context = IntoReportCollectionT::Context;

    fn into_problem_collection(self) -> ProblemCollection<Self::Context, ThreadSafetyT> {
        self.into_report_collection()
    }
}
