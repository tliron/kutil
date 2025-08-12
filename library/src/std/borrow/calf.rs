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
pub enum Calf<'own, BorrowedT> {
    /// Borrowed.
    Borrowed(&'own BorrowedT),

    /// Owned.
    Owned(BorrowedT),
}

impl<'own, BorrowedT> Calf<'own, BorrowedT> {
    /// Are we borrowed?
    pub fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }

    /// Are we owned?
    pub fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }
}

impl<'own, BorrowedT> Borrow<BorrowedT> for Calf<'own, BorrowedT> {
    fn borrow(&self) -> &BorrowedT {
        match self {
            Self::Owned(owned) => owned,
            Self::Borrowed(borrowed) => *borrowed,
        }
    }
}

// Experimental

impl<'own, BorrowedT, OwnedT> Calf<'own, BorrowedT>
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

impl<'own, BorrowedT> BorrowMut<BorrowedT> for Calf<'own, BorrowedT>
where
    &'own BorrowedT: AsMut<&'own mut BorrowedT>,
{
    fn borrow_mut(&mut self) -> &mut BorrowedT {
        match self {
            Self::Owned(owned) => owned,
            Self::Borrowed(borrowed) => borrowed.as_mut(),
        }
    }
}

impl<'own, BorrowedT, OwnedT> ToOwned for Calf<'own, BorrowedT>
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

impl<'own, BorrowedT> From<Cow<'own, BorrowedT>> for Calf<'own, BorrowedT>
where
    BorrowedT: ToOwned<Owned = BorrowedT>,
{
    fn from(cow: Cow<'own, BorrowedT>) -> Self {
        match cow {
            Cow::Owned(owned) => Calf::Owned(owned),
            Cow::Borrowed(borrowed) => Calf::Borrowed(borrowed),
        }
    }
}

impl<'own, BorrowedT> Into<Cow<'own, BorrowedT>> for Calf<'own, BorrowedT>
where
    BorrowedT: ToOwned<Owned = BorrowedT>,
{
    fn into(self) -> Cow<'own, BorrowedT> {
        match self {
            Self::Owned(owned) => Cow::Owned(owned),
            Self::Borrowed(borrowed) => Cow::Borrowed(borrowed),
        }
    }
}
