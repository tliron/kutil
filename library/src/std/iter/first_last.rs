use std::iter::*;

//
//  IterateWithFirstLast
//

/// [Iterator] providing first and last flags.
#[derive(Clone, Debug)]
pub struct IterateWithFirstLast<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    inner: Peekable<InnerT::IntoIter>,
    first: bool,
}

impl<ItemT, InnerT> IterateWithFirstLast<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner: inner.into_iter().peekable(), first: true }
    }
}

impl<ItemT, InnerT> Iterator for IterateWithFirstLast<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    type Item = (ItemT, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let first = if self.first {
            self.first = false;
            true
        } else {
            false
        };

        self.inner.next().map(|item| (item, first, self.inner.peek().is_none()))
    }
}

//
//  IterateWithFirst
//

/// [Iterator] providing first flag.
#[derive(Clone, Debug)]
pub struct IterateWithFirst<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    inner: InnerT::IntoIter,
    first: bool,
}

impl<ItemT, InnerT> IterateWithFirst<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner: inner.into_iter(), first: true }
    }
}

impl<ItemT, InnerT> Iterator for IterateWithFirst<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    type Item = (ItemT, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let first = if self.first {
            self.first = false;
            true
        } else {
            false
        };

        self.inner.next().map(|item| (item, first))
    }
}

//
//  IterateWithLast
//

/// [Iterator] providing last flag.
#[derive(Clone, Debug)]
pub struct IterateWithLast<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    inner: Peekable<InnerT::IntoIter>,
}

impl<ItemT, InnerT> IterateWithLast<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner: inner.into_iter().peekable() }
    }
}

impl<ItemT, InnerT> Iterator for IterateWithLast<ItemT, InnerT>
where
    InnerT: IntoIterator<Item = ItemT>,
{
    type Item = (ItemT, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|item| (item, self.inner.peek().is_none()))
    }
}
