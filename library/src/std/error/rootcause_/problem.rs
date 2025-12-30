use super::{Mutable, ObjectMarkerFor, PreformattedContext, Problem, ProblemConversion, ProblemOwnershipMarker};

use std::{fmt, iter::*};

//
// UnsizedProblemExtension
//

/// Extension for [Problem] (unsized contexts).
pub trait UnsizedProblemExtension<ContextT, OwnershipT, ThreadSafetyT>
where
    ContextT: 'static + ?Sized,
    OwnershipT: 'static,
    ThreadSafetyT: 'static,
{
    /// Create a new problem with the provided context. This problem will be its child.
    ///
    /// Alias for [Problem::context].
    fn in_context<ParentContextT>(self, context: ParentContextT) -> Problem<ParentContextT, Mutable, ThreadSafetyT>
    where
        ParentContextT: fmt::Display + fmt::Debug + ObjectMarkerFor<ThreadSafetyT>;

    /// Convert into a problem with a different context.
    ///
    /// Depends on an existing implementation of [ProblemConversion].
    ///
    /// Alias for [Problem::context_to].
    fn convert_into<NewContextT>(self) -> Problem<NewContextT, Mutable, ThreadSafetyT>
    where
        NewContextT: ProblemConversion<ContextT, OwnershipT, ThreadSafetyT>;

    /// Whether this problem or any of its children has a certain context.
    fn has_context<FindContextT>(&self) -> bool
    where
        OwnershipT: ProblemOwnershipMarker,
        FindContextT: 'static;
}

impl<ContextT, OwnershipT, ThreadSafetyT> UnsizedProblemExtension<ContextT, OwnershipT, ThreadSafetyT>
    for Problem<ContextT, OwnershipT, ThreadSafetyT>
where
    ContextT: ?Sized,
{
    fn in_context<ParentContextT>(self, context: ParentContextT) -> Problem<ParentContextT, Mutable, ThreadSafetyT>
    where
        ParentContextT: fmt::Display + fmt::Debug + ObjectMarkerFor<ThreadSafetyT>,
    {
        self.context(context)
    }

    fn convert_into<NewContextT>(self) -> Problem<NewContextT, Mutable, ThreadSafetyT>
    where
        NewContextT: ProblemConversion<ContextT, OwnershipT, ThreadSafetyT>,
    {
        self.context_to()
    }

    fn has_context<FindContextT>(&self) -> bool
    where
        OwnershipT: ProblemOwnershipMarker,
        FindContextT: 'static,
    {
        self.iter_reports().filter_map(|report| report.downcast_current_context::<FindContextT>()).next().is_some()
    }
}

//
// SizedProblemExtension
//

/// Extension for [Problem] (sized contexts).
pub trait SizedProblemExtension<ContextT, ThreadSafetyT>
where
    ContextT: Sized,
{
    /// Replaces the context with the value returned from a function.
    ///
    /// Alias for [Problem::context_transform].
    fn replace_context<F, NewContextT>(self, f: F) -> Problem<NewContextT, Mutable, ThreadSafetyT>
    where
        F: FnOnce(ContextT) -> NewContextT,
        NewContextT: fmt::Display + fmt::Debug + ObjectMarkerFor<ThreadSafetyT>;

    /// Replaces the context with the value returned from a function.
    ///
    /// The original context is preserved as a type-less child.
    ///
    /// Alias for [Problem::context_transform].
    fn replace_and_preserve_context<F, NewContextT>(self, f: F) -> Problem<NewContextT, Mutable, ThreadSafetyT>
    where
        F: FnOnce(ContextT) -> NewContextT,
        NewContextT: fmt::Display + fmt::Debug + ObjectMarkerFor<ThreadSafetyT>,
        PreformattedContext: ObjectMarkerFor<ThreadSafetyT>;
}

impl<ContextT, ThreadSafetyT> SizedProblemExtension<ContextT, ThreadSafetyT>
    for Problem<ContextT, Mutable, ThreadSafetyT>
where
    ContextT: Sized,
{
    fn replace_context<F, NewContextT>(self, f: F) -> Problem<NewContextT, Mutable, ThreadSafetyT>
    where
        F: FnOnce(ContextT) -> NewContextT,
        NewContextT: fmt::Display + fmt::Debug + ObjectMarkerFor<ThreadSafetyT>,
    {
        self.context_transform(f)
    }

    fn replace_and_preserve_context<F, NewContextT>(self, f: F) -> Problem<NewContextT, Mutable, ThreadSafetyT>
    where
        F: FnOnce(ContextT) -> NewContextT,
        NewContextT: fmt::Display + fmt::Debug + ObjectMarkerFor<ThreadSafetyT>,
        PreformattedContext: ObjectMarkerFor<ThreadSafetyT>,
    {
        self.context_transform_nested(f)
    }
}
