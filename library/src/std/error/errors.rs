use super::recipient::*;

use std::{error::*, fmt, iter::*, slice, vec};

//
// Errors
//

/// An [Error] that contains zero or more errors.
///
/// Implements [ErrorRecipient] for accumulating errors.
#[derive(Clone, Debug)]
pub struct Errors<ErrorT> {
    /// The errors.
    pub errors: Vec<ErrorT>,
}

impl<ErrorT> Errors<ErrorT> {
    /// Constructor.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { errors: Vec::with_capacity(capacity) }
    }

    /// True if there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Fails with self if there are errors.
    pub fn check(&self) -> Result<(), &Self> {
        if self.is_empty() { Ok(()) } else { Err(self) }
    }
}

impl<ErrorT> ErrorRecipient<ErrorT> for Errors<ErrorT> {
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        self.errors.push(error.into());
        Ok(())
    }
}

impl<ErrorT> Default for Errors<ErrorT> {
    fn default() -> Self {
        Self { errors: Default::default() }
    }
}

impl<ErrorT> fmt::Display for Errors<ErrorT>
where
    ErrorT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iterator = self.errors.iter().peekable();
        while let Some(error) = iterator.next() {
            fmt::Display::fmt(error, formatter)?;
            if iterator.peek().is_some() {
                writeln!(formatter)?;
            }
        }
        Ok(())
    }
}

impl<ErrorT> Error for Errors<ErrorT> where ErrorT: Error {}

// Delegated

impl<ErrorT> IntoIterator for Errors<ErrorT> {
    type Item = ErrorT;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'own, ErrorT> IntoIterator for &'own Errors<ErrorT> {
    type Item = &'own ErrorT;
    type IntoIter = slice::Iter<'own, ErrorT>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

impl<'own, ErrorT> IntoIterator for &'own mut Errors<ErrorT> {
    type Item = &'own mut ErrorT;
    type IntoIter = slice::IterMut<'own, ErrorT>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter_mut()
    }
}

// Conversions

impl<ErrorT> From<ErrorT> for Errors<ErrorT> {
    fn from(value: ErrorT) -> Self {
        let mut errors = Errors::with_capacity(1);
        errors.errors.push(value);
        errors
    }
}

impl<ErrorT> Into<Vec<ErrorT>> for Errors<ErrorT> {
    fn into(self) -> Vec<ErrorT> {
        self.errors
    }
}

//
// ErrorsResult
//

/// [Result].
pub trait ErrorsResult<OkT, ErrorT> {
    /// Convert to a [Result] with [Errors].
    fn as_errors(self) -> Result<OkT, Errors<ErrorT>>;
}

impl<OkT, ErrorT> ErrorsResult<OkT, ErrorT> for Result<OkT, ErrorT> {
    fn as_errors(self) -> Result<OkT, Errors<ErrorT>> {
        Ok(self?)
    }
}
