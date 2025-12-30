/// Enable conversion of a problem into a new problem with a different context by making the
/// current problem the child of the new problem.
///
/// See [ProblemExtension::convert_into](super::ProblemExtension::convert_into) and
/// [ProblemResultExtension::convert_into](super::ProblemResultExtension::convert_into).
#[macro_export]
macro_rules! convert_problem_into_child {
    ( $error:ty, $context:ident $(,)? ) => {
        impl<OwnershipT, ThreadSafetyT> $crate::std::error::ProblemConversion<$error, OwnershipT, ThreadSafetyT>
            for $context
        where
            Self: $crate::std::error::ObjectMarkerFor<ThreadSafetyT>,
        {
            fn convert_report(
                problem: $crate::std::error::Problem<$error, OwnershipT, ThreadSafetyT>,
            ) -> $crate::std::error::Problem<Self, $crate::std::error::Mutable, ThreadSafetyT> {
                $crate::std::error::UnsizedProblemExtension::in_context(problem, $context)
            }
        }
    };

    ( $error:ty, $context:ident, $context_variant:ident $(,)? ) => {
        impl<OwnershipT, ThreadSafetyT> $crate::std::error::ProblemConversion<$error, OwnershipT, ThreadSafetyT>
            for $context
        where
            Self: $crate::std::error::ObjectMarkerFor<ThreadSafetyT>,
        {
            fn convert_report(
                problem: $crate::std::error::Problem<$error, OwnershipT, ThreadSafetyT>,
            ) -> $crate::std::error::Problem<Self, $crate::std::error::Mutable, ThreadSafetyT> {
                $crate::std::error::UnsizedProblemExtension::in_context(problem, $context::$context_variant)
            }
        }
    };
}

/// Enable conversion of a problem into a new problem with a different context by converting
/// the current problem's context into a string.
///
/// See [ProblemExtension::convert_into](super::ProblemExtension::convert_into) and
/// [ProblemResultExtension::convert_into](super::ProblemResultExtension::convert_into).
#[macro_export]
macro_rules! convert_problem_context_to_string {
    ( $error:ty, $context:ident $(,)? ) => {
        impl<OwnershipT, ThreadSafetyT> $crate::std::error::ProblemConversion<$error, OwnershipT, ThreadSafetyT>
            for $context
        where
            Self: $crate::std::error::ObjectMarkerFor<ThreadSafetyT>,
        {
            fn convert_report(
                problem: $crate::std::error::Problem<$error, OwnershipT, ThreadSafetyT>,
            ) -> $crate::std::error::Problem<Self, $crate::std::error::Mutable, ThreadSafetyT> {
                $context(problem.current_context().to_string()).into()
            }
        }
    };

    ( $error:ty, $context:ident, $context_variant:ident $(,)? ) => {
        impl<OwnershipT, ThreadSafetyT> $crate::std::error::ProblemConversion<$error, OwnershipT, ThreadSafetyT>
            for $context
        where
            Self: $crate::std::error::ObjectMarkerFor<ThreadSafetyT>,
        {
            fn convert_report(
                problem: $crate::std::error::Problem<$error, OwnershipT, ThreadSafetyT>,
            ) -> $crate::std::error::Problem<Self, $crate::std::error::Mutable, ThreadSafetyT> {
                $context::$context_variant(problem.current_context().to_string()).into()
            }
        }
    };
}

///
#[macro_export]
macro_rules! convert_problem_generic_context_to_string {
    ( $error:ty, $context:ident $(,)? ) => {
        impl<OwnershipT, ThreadSafetyT> $crate::std::error::ProblemConversion<$error, OwnershipT, ThreadSafetyT>
            for $context
        where
            Self: $crate::std::error::ObjectMarkerFor<ThreadSafetyT>,
        {
            fn convert_report(
                problem: $crate::std::error::Problem<$error, OwnershipT, ThreadSafetyT>,
            ) -> $crate::std::error::Problem<Self, $crate::std::error::Mutable, ThreadSafetyT> {
                $context(problem.current_context().to_string()).into()
            }
        }
    };

    ( $error:tt, $context:ident, $context_variant:ident $(,)? ) => {
        impl<T, OwnershipT, ThreadSafetyT> $crate::std::error::ProblemConversion<$error<T>, OwnershipT, ThreadSafetyT>
            for $context
        where
            Self: $crate::std::error::ObjectMarkerFor<ThreadSafetyT>,
        {
            fn convert_report(
                problem: $crate::std::error::Problem<$error<T>, OwnershipT, ThreadSafetyT>,
            ) -> $crate::std::error::Problem<Self, $crate::std::error::Mutable, ThreadSafetyT> {
                $context::$context_variant(problem.current_context().to_string()).into()
            }
        }
    };
}

#[allow(unused_imports)]
pub use {convert_problem_context_to_string, convert_problem_generic_context_to_string, convert_problem_into_child};
