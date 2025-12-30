use std::{collections::*, hash::*};

//
// IterateByKeyOrder
//

/// [Iterator] for [HashMap] in the sort order of the keys.
#[derive(Clone, Debug)]
pub struct IterateByKeyOrder<'this, KeyT, ValueT, HasherT> {
    inner: &'this HashMap<KeyT, ValueT, HasherT>,
    keys: Vec<&'this KeyT>,
    size: usize,
    index: usize,
}

impl<'this, KeyT, ValueT, HasherT> IterateByKeyOrder<'this, KeyT, ValueT, HasherT> {
    /// Constructor.
    pub fn new(inner: &'this HashMap<KeyT, ValueT, HasherT>) -> Self
    where
        KeyT: Clone + Ord,
    {
        let mut keys: Vec<_> = inner.keys().collect();
        keys.sort();
        let size = keys.len();
        Self { inner, keys, size, index: 0 }
    }
}

impl<'this, KeyT, ValueT, HasherT> Iterator for IterateByKeyOrder<'this, KeyT, ValueT, HasherT>
where
    KeyT: Eq + Hash,
    HasherT: BuildHasher,
{
    type Item = (&'this KeyT, &'this ValueT);

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
