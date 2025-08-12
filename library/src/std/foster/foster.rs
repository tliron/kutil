//
// Foster
//

/// Fostering allows for *subjective* equivalence between owned values and differently-typed
/// fostered values. Equivalence can mean comparison, iteration, hashing, representation, etc.
///
/// An example use case:
///
/// You create [Vec]\<[String]\> values dynamically in your program, but you also want to allow for
/// constant values, which *cannot* be a [Vec]. A reasonable constant equivalent would be
/// `&'static [&'static str]`, which is a very different type.
///
/// Although you could convert `&'static [&'static str]` to [Vec]\<[String]\>, it would require
/// allocation, which is unnecessary and wasteful in situations in which you don't actually need or
/// care about ownership, e.g. comparisons. (Although we *can* allow for efficient conversion via
/// the [IntoOwned](super::super::borrow::IntoOwned) trait.)
///
/// "Fostering" means creating a single type on top of both types, and then implementing necessary
/// traits for that type, e.g. [PartialEq].
///
/// This type simply provides a unified mechanism for fostering. Furthermore, this module contains
/// commonly useful implementations, such as [FosterString](super::string::FosterString) and
/// [FosterStringVector](super::string_vector::FosterStringVector), as well as macros for
/// delegating the implemented traits to newtypes.
///
/// Note that [Cow](std::borrow::Cow) also counts as fostering, but it's more specific in that the
/// fostered type is always a reference of the owned type, which allows for generalized conversion
/// to ownership when necessary via [ToOwned].
#[derive(Clone, Debug)]
pub enum Foster<OwnedT, FosteredT> {
    /// Owned.
    Owned(OwnedT),

    /// Fostered.
    Fostered(FosteredT),
}

impl<OwnedT, FosteredT> Foster<OwnedT, FosteredT> {
    /// Constructor.
    pub const fn new_owned(value: OwnedT) -> Self {
        Self::Owned(value)
    }

    /// Constructor.
    pub const fn new_fostered(value: FosteredT) -> Self {
        Self::Fostered(value)
    }

    /// True if owned.
    pub const fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }
}

impl<OwnedT, FosteredT> From<OwnedT> for Foster<OwnedT, FosteredT> {
    fn from(value: OwnedT) -> Self {
        Self::Owned(value)
    }
}
