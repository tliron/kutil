use super::super::super::cache::*;

use std::{cmp::*, fmt, hash::*, str::*};

//
// IsSpecific
//

/// Is specific
pub trait IsSpecific {
    /// True if specific.
    fn is_specific(&self) -> bool;
}

//
// Selector
//

/// Selector.
#[derive(Clone, Debug, Eq)]
pub enum Selector<SelectionT> {
    /// Any.
    Any,

    /// Specific.
    Specific(SelectionT),
}

impl<SelectionT> Selector<SelectionT> {
    /// Select from candidates.
    ///
    /// If we are [Any](Selector::Any) we will select all candidates. Otherwise we will select
    /// either one or none of the candidates.
    pub fn select<'own>(&'own self, candidates: &'own [SelectionT]) -> Vec<&'own SelectionT>
    where
        SelectionT: Eq,
    {
        match self {
            Self::Any => candidates.iter().collect(),

            Self::Specific(selection) => {
                if candidates.contains(&selection) {
                    vec![selection]
                } else {
                    Default::default()
                }
            }
        }
    }
}

impl<SelectionT> IsSpecific for Selector<SelectionT> {
    fn is_specific(&self) -> bool {
        matches!(self, Self::Specific(_))
    }
}

impl<SelectionT> CacheWeight for Selector<SelectionT>
where
    SelectionT: CacheWeight,
{
    fn cache_weight(&self) -> usize {
        let mut size = size_of::<Self>();
        if let Self::Specific(selection) = self {
            size += selection.cache_weight();
        }
        size
    }
}

impl<SelectionT> PartialEq for Selector<SelectionT>
where
    SelectionT: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, Self::Any) => true,
            (Self::Specific(selection), Self::Specific(other_selection)) => selection.eq(other_selection),
            _ => false,
        }
    }
}

impl<SelectionT> PartialEq<SelectionT> for Selector<SelectionT>
where
    SelectionT: PartialEq,
{
    fn eq(&self, other: &SelectionT) -> bool {
        match self {
            Self::Any => false,
            Self::Specific(selection) => selection.eq(other),
        }
    }
}

impl<SelectionT> Hash for Selector<SelectionT>
where
    SelectionT: Hash,
{
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Any => {
                state.write_u8(0);
            }

            Self::Specific(selection) => {
                state.write_u8(1);
                selection.hash(state);
            }
        }
    }
}

impl<SelectionT> From<SelectionT> for Selector<SelectionT> {
    fn from(selection: SelectionT) -> Self {
        Self::Specific(selection)
    }
}

impl<SelectionT> FromStr for Selector<SelectionT>
where
    SelectionT: FromStr,
{
    type Err = SelectionT::Err;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Ok(if representation == "*" { Self::Any } else { Self::Specific(representation.parse()?) })
    }
}

impl<SelectionT> fmt::Display for Selector<SelectionT>
where
    SelectionT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => fmt::Display::fmt("*", formatter),
            Self::Specific(selector) => fmt::Display::fmt(selector, formatter),
        }
    }
}
