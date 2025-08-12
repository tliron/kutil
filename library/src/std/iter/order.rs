use std::{collections::*, hash::*};

//
// IterateByKeyOrder
//

/// [Iterator] for [HashMap] in the sort order of the keys.
#[derive(Clone, Debug)]
pub struct IterateByKeyOrder<'own, KeyT, ValueT, HasherT> {
    inner: &'own HashMap<KeyT, ValueT, HasherT>,
    keys: Vec<&'own KeyT>,
    size: usize,
    index: usize,
}

impl<'own, KeyT, ValueT, HasherT> IterateByKeyOrder<'own, KeyT, ValueT, HasherT> {
    /// Constructor.
    pub fn new(inner: &'own HashMap<KeyT, ValueT, HasherT>) -> Self
    where
        KeyT: Clone + Ord,
    {
        let mut keys: Vec<_> = inner.keys().collect();
        keys.sort();
        let size = keys.len();
        Self { inner, keys, size, index: 0 }
    }
}

impl<'own, KeyT, ValueT, HasherT> Iterator for IterateByKeyOrder<'own, KeyT, ValueT, HasherT>
where
    KeyT: Eq + Hash,
    HasherT: BuildHasher,
{
    type Item = (&'own KeyT, &'own ValueT);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size {
            return None;
        }

        let key = self.keys[self.index];
        let value = self.inner.get(key).expect("value");
        self.index += 1;

        Some((key, value))
    }
}
