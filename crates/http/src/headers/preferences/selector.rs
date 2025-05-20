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
            Self::Any => {
                let selections: Vec<_> = candidates.iter().collect();
                selections
            }

            Self::Specific(selector) => {
                if candidates.contains(&selector) {
                    vec![selector]
                } else {
                    Vec::new()
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
        if let Self::Specific(selector) = self {
            size += selector.cache_weight();
        }
        size
    }
}

impl<SelectionT> PartialEq for Selector<SelectionT>
where
    SelectionT: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Any => matches!(other, Self::Any),
            Self::Specific(selector) => match other {
                Self::Any => false,
                Self::Specific(other_selector) => selector.eq(other_selector),
            },
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

            Self::Specific(selector) => {
                state.write_u8(1);
                selector.hash(state);
            }
        }
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
