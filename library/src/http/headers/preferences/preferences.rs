use super::{super::super::super::std::collections::*, preference::*, selector::*, weight::*};

use std::{hash::*, iter::*, str::*};

//
// Preferences
//

/// List of [Preference].
#[derive(Clone, Debug)]
pub struct Preferences<SelectionT>(pub Vec<Preference<SelectionT>>);

impl<SelectionT> Preferences<SelectionT> {
    /// Sort by weight (highest to lowest).
    pub fn sorted(mut self) -> Self {
        self.0.sort_by(|a, b| b.weight.cmp(&a.weight));
        self
    }

    /// Parse, combine, and sort.
    pub fn parse(representations: &Vec<&str>) -> Self
    where
        SelectionT: Clone + Eq + FromStr,
    {
        let preferences: Vec<_> = representations
            .iter()
            .flat_map(|representation| representation.split(","))
            .filter_map(move |format| {
                let mut split = format.splitn(2, ';');

                let selector = (split.next().expect("splitn not empty").trim()).parse().ok()?;

                let weight = match split.next() {
                    Some(weight) => Weight::parse(weight.trim())?,
                    None => Weight::MAX,
                };

                Some(Preference::new(selector, weight))
            })
            .collect();

        Self(preferences).sorted()
    }

    /// Select the most preferred allowance.
    ///
    /// If there is a tie, we will go by the order of allowances.
    pub fn best<'own>(&'own self, allowances: &'own [SelectionT]) -> Option<&'own SelectionT>
    where
        SelectionT: Hash + Eq,
    {
        if self.0.is_empty() {
            None
        } else {
            let mut candidates = FastHashSet::<&SelectionT>::with_capacity(allowances.len());

            for (index, preference) in self.0.iter().enumerate() {
                let selections = preference.selector.select(allowances);

                if !selections.is_empty() {
                    candidates.extend(selections);

                    if preference.selector.is_specific() {
                        // There might still be more candidates if there are other preferences
                        // of equal weight (and they would be right after us)
                        candidates.extend(self.select_tied(index, preference.weight, allowances));
                    }

                    break;
                }
            }

            if candidates.len() == 1 {
                // No contest
                return Some(candidates.into_iter().next().expect("iter not empty"));
            } else if !candidates.is_empty() {
                // Break the tie
                for selection in allowances {
                    if candidates.contains(&selection) {
                        return Some(selection);
                    }
                }
            }

            None
        }
    }

    /// Select the most preferred allowance.
    ///
    /// If there is a tie, we will go by the order of allowances.
    ///
    /// If no best preference can be found we'll return the first allowance. `allowances` must not
    /// be empty!
    pub fn best_or_first<'own>(&'own self, allowances: &'own [SelectionT]) -> &'own SelectionT
    where
        SelectionT: Hash + Eq,
    {
        assert!(allowances.len() > 0);
        self.best(allowances).unwrap_or_else(|| &allowances[0])
    }

    // Selections with equal weight.
    fn select_tied<'own>(
        &'own self,
        index: usize,
        weight: Weight,
        allowances: &'own [SelectionT],
    ) -> FastHashSet<&'own SelectionT>
    where
        SelectionT: Hash + Eq,
    {
        let mut tied = FastHashSet::with_capacity(self.0.len() - index);

        for preference in &self.0[index + 1..] {
            if preference.weight != weight {
                break;
            }

            tied.extend(preference.selector.select(allowances));
        }

        tied
    }
}
