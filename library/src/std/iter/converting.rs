//
// ConvertingIterator
//

/// [Iterator] wrapper that converts its items.
#[derive(Clone, Debug)]
pub struct ConvertingIterator<ItemT, InnerIteratorT, InnerItemT> {
    inner: InnerIteratorT,
    convert: fn(InnerItemT) -> Option<ItemT>,
}

impl<ItemT, InnerIteratorT, InnerItemT> ConvertingIterator<ItemT, InnerIteratorT, InnerItemT> {
    /// Constructor.
    pub fn new(inner: InnerIteratorT, convert: fn(InnerItemT) -> Option<ItemT>) -> Self {
        Self { inner, convert }
    }
}

impl<ItemT, InnerIteratorT, InnerItemT> Iterator for ConvertingIterator<ItemT, InnerIteratorT, InnerItemT>
where
    InnerIteratorT: Iterator<Item = InnerItemT>,
{
    type Item = ItemT;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|inner_item| (self.convert)(inner_item))
    }
}
