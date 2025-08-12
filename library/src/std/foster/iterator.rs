use super::{super::iter::*, foster::*};

//
// FosterIterator
//

/// [Foster] [Iterator].
///
/// See [ConvertingIterator].
pub type FosterIterator<ItemT, OwnedItemT, FosteredItemT, OwnedIteratorT, FosteredIteratorT> = Foster<
    ConvertingIterator<ItemT, OwnedIteratorT, OwnedItemT>,
    ConvertingIterator<ItemT, FosteredIteratorT, FosteredItemT>,
>;

impl<ItemT, OwnedItemT, FosteredItemT, OwnedIteratorT, FosteredIteratorT> Iterator
    for FosterIterator<ItemT, OwnedItemT, FosteredItemT, OwnedIteratorT, FosteredIteratorT>
where
    OwnedIteratorT: Iterator<Item = OwnedItemT>,
    FosteredIteratorT: Iterator<Item = FosteredItemT>,
{
    type Item = ItemT;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Owned(iterator) => iterator.next(),
            Self::Fostered(iterator) => iterator.next(),
        }
    }
}
