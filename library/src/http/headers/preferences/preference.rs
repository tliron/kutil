use super::{selector::*, weight::*};

use std::fmt;

//
// Preference
//

/// Weighted preference.
#[derive(Clone, Debug)]
pub struct Preference<SelectionT> {
    /// Selector.
    pub selector: Selector<SelectionT>,

    /// Weight.
    pub weight: Weight,
}

impl<SelectionT> Preference<SelectionT> {
    /// Constructor.
    pub fn new(selector: Selector<SelectionT>, weight: Weight) -> Self {
        Self { selector, weight }
    }
}

impl<SelectionT> fmt::Display for Preference<SelectionT>
where
    SelectionT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.weight {
            Weight::MAX => fmt::Display::fmt(&self.selector, formatter),
            weight => write!(formatter, "{};{}", self.selector, weight),
        }
    }
}
