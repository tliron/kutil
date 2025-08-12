use super::super::super::std::iter::*;

use {std::io, utf8_chars::*};

//
// CharIterable
//

/// Can iterate chars.
pub trait CharIterable {
    /// [Iterator] of char. Will end iteration if there is an error.
    fn chars(&mut self) -> impl Iterator<Item = char>;
}

impl<BufReadT> CharIterable for BufReadT
where
    BufReadT: io::BufRead,
{
    fn chars(&mut self) -> impl Iterator<Item = char> {
        ConvertingIterator::new(self.chars_raw(), |result: Result<_, _>| result.ok())
    }
}
