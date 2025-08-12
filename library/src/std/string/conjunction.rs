use super::super::iter::*;

/// Join with conjunction.
pub trait JoinConjunction<'own> {
    /// Join iterated strings as human-readable in English with a conjunction
    /// and an Oxford comma.
    ///
    /// Examples:
    ///
    /// * `one`
    /// * `one or two`
    /// * `one, two, or three`
    /// * `one, two, three, or four`
    fn join_conjunction(&'own self, conjunction: &str) -> String;
}

impl<'own, ItemT, IterableT> JoinConjunction<'own> for IterableT
where
    ItemT: 'own + AsRef<str>,
    &'own IterableT: 'own + IntoIterator<Item = ItemT>,
{
    fn join_conjunction(&'own self, conjunction: &str) -> String {
        let mut options = String::default();

        let mut has_at_least_two = false;
        for (item, first, last) in IterateWithFirstLast::new(self) {
            if !first {
                if last {
                    if has_at_least_two {
                        options.push_str(", ");
                    } else {
                        options.push(' ');
                    }
                    options.push_str(conjunction);
                    options.push(' ');
                } else {
                    options.push_str(", ");
                    has_at_least_two = true;
                }
            }

            options.push_str(item.as_ref());
        }

        options
    }
}
