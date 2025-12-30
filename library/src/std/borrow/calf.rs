use std::borrow::*;

//
// Calf
//

/// A read-only container for either an owned value or a reference to one.
///
/// Similar to [Cow], but does not support [ToOwned] (and does not require your
/// value to support it), nor does it support [BorrowMut].
///
/// It's a baby cow!
///
/// Note that after coming up with the idea for this, I discovered that someone
/// else did, too. Check out [maybe-owned](https://github.com/rustonaut/maybe-owned)
/// for an identical type that comes with a zillion more trait implementations.
pub enum Calf<'borrow, BorrowedT> {
    /// Borrowed.
    Borrowed(&'borrow BorrowedT),

    /// Owned.
    Owned(BorrowedT),
}

impl<'borrow, BorrowedT> Calf<'borrow, BorrowedT> {
    /// Are we borrowed?
    pub fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }

    /// Are we owned?
    pub fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }
}

impl<'borrow, BorrowedT> Borrow<BorrowedT> for Calf<'borrow, BorrowedT> {
    fn borrow(&self) -> &BorrowedT {
        match self {
            Self::Owned(owned) => owned,
            Self::Borrowed(borrowed) => *borrowed,
        }
    }
}

// Experimental

impl<'borrow, BorrowedT, OwnedT> Calf<'borrow, BorrowedT>
where
    BorrowedT: ToOwned<Owned = OwnedT>,
{
    /// Into owned.
    pub fn into_owned(self) -> <BorrowedT as ToOwned>::Owned {
        match self {
            Self::Owned(owned) => owned.to_owned(),
            Self::Borrowed(borrowed) => borrowed.to_owned(),
        }
    }
}

impl<'borrow, BorrowedT> BorrowMut<BorrowedT> for Calf<'borrow, BorrowedT>
where
    &'borrow BorrowedT: AsMut<&'borrow mut BorrowedT>,
{
    fn borrow_mut(&mut self) -> &mut BorrowedT {
        match self {
            Self::Owned(owned) => owned,
            Self::Borrowed(borrowed) => borrowed.as_mut(),
        }
    }
}

impl<'borrow, BorrowedT, OwnedT> ToOwned for Calf<'borrow, BorrowedT>
where
    BorrowedT: ToOwned<Owned = OwnedT>,
    OwnedT: Borrow<Self>,
{
    type Owned = OwnedT;

    fn to_owned(&self) -> OwnedT {
        match self {
            Self::Owned(owned) => owned.to_owned(),
            Self::Borrowed(borrowed) => (*borrowed).to_owned(),
        }
    }
}

// Conversions

impl<'borrow, BorrowedT> From<Cow<'borrow, BorrowedT>> for Calf<'borrow, BorrowedT>
where
    BorrowedT: ToOwned<Owned = BorrowedT>,
{
    fn from(cow: Cow<'borrow, BorrowedT>) -> Self {
        match cow {
            Cow::Owned(owned) => Calf::Owned(owned),
            Cow::Borrowed(borrowed) => Calf::Borrowed(borrowed),
        }
    }
}

impl<'borrow, BorrowedT> Into<Cow<'borrow, BorrowedT>> for Calf<'borrow, BorrowedT>
where
    BorrowedT: ToOwned<Owned = BorrowedT>,
{
    fn into(self) -> Cow<'borrow, BorrowedT> {
        match self {
            Self::Owned(owned) => Cow::Owned(owned),
            Self::Borrowed(borrowed) => Cow::Borrowed(borrowed),
        }
    }
}
