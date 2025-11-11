use super::{
    super::{borrow::*, iter::*},
    foster::*,
    has_length::*,
    iterator::*,
};

use std::{cmp::*, hash::*, slice::*};

/// [Foster] for [Vec]\<[String]\>.
///
/// Supports [IntoOwned], [HasLength], [Eq]/[PartialEq], [Ord]/[PartialOrd], [Hash], and
/// [IntoIterator].
pub type FosterStringVector = Foster<Vec<String>, &'static [&'static str]>;

impl IntoOwned for FosterStringVector {
    fn into_owned(self) -> Self {
        match self {
            Self::Owned(_) => self,
            Self::Fostered(strings) => Self::new_owned(strings.iter().map(|string| (*string).into()).collect()),
        }
    }
}

impl HasLength for FosterStringVector {
    fn len(&self) -> usize {
        match self {
            Self::Owned(strings) => strings.len(),
            Self::Fostered(strings) => strings.len(),
        }
    }
}

impl PartialEq for FosterStringVector {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Owned(left), Self::Owned(right)) => left == right,

            (Self::Owned(left), Self::Fostered(right)) => {
                if left.len() == right.len() {
                    let mut equal = true;
                    for (index, left) in left.iter().enumerate() {
                        if left != right[index] {
                            equal = false;
                            break;
                        }
                    }
                    equal
                } else {
                    false
                }
            }

            (Self::Fostered(left), Self::Owned(right)) => {
                if left.len() == right.len() {
                    let mut equal = true;
                    for (index, left) in left.iter().enumerate() {
                        if left != &right[index] {
                            equal = false;
                            break;
                        }
                    }
                    equal
                } else {
                    false
                }
            }

            (Self::Fostered(left), Self::Fostered(right)) => left == right,
        }
    }
}

impl Eq for FosterStringVector {}

impl PartialOrd for FosterStringVector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // See: core::slice::cmp::SlicePartialOrd

        match (self, other) {
            (Self::Owned(left), Self::Owned(right)) => left.partial_cmp(right),

            (Self::Owned(left), Self::Fostered(right)) => {
                let left_length = left.len();
                let right_length = right.len();
                let length = min(left_length, right_length);

                // enable compiler bound check elimination
                let left_bounded = &left[..length];
                let right_bounded = &right[..length];

                for index in 0..length {
                    match left_bounded[index].as_str().partial_cmp(right_bounded[index]) {
                        Some(Ordering::Equal) => {}
                        not_equal => return not_equal,
                    }
                }

                left_length.partial_cmp(&right_length)
            }

            (Self::Fostered(left), Self::Owned(right)) => {
                let left_length = left.len();
                let right_length = right.len();
                let length = min(left_length, right_length);

                // enable compiler bound check elimination
                let left_bounded = &left[..length];
                let right_bounded = &right[..length];

                for index in 0..length {
                    match left_bounded[index].partial_cmp(right_bounded[index].as_str()) {
                        Some(Ordering::Equal) => {}
                        not_equal => return not_equal,
                    }
                }

                left_length.partial_cmp(&right_length)
            }

            (Self::Fostered(left), Self::Fostered(right)) => left.partial_cmp(right),
        }
    }
}

impl Ord for FosterStringVector {
    fn cmp(&self, other: &Self) -> Ordering {
        // See: core::slice::cmp::SliceOrd

        match (self, other) {
            (Self::Owned(left), Self::Owned(right)) => left.cmp(right),

            (Self::Owned(left), Self::Fostered(right)) => {
                let left_length = left.len();
                let right_length = right.len();
                let length = min(left_length, right_length);

                // enable compiler bound check elimination
                let left_bounded = &left[..length];
                let right_bounded = &right[..length];

                for index in 0..length {
                    match left_bounded[index].as_str().cmp(right_bounded[index]) {
                        Ordering::Equal => {}
                        not_equal => return not_equal,
                    }
                }

                left_length.cmp(&right_length)
            }

            (Self::Fostered(left), Self::Owned(right)) => {
                let left_length = left.len();
                let right_length = right.len();
                let length = min(left_length, right_length);

                // enable compiler bound check elimination
                let left_bounded = &left[..length];
                let right_bounded = &right[..length];

                for index in 0..length {
                    match left_bounded[index].cmp(right_bounded[index].as_str()) {
                        Ordering::Equal => {}
                        not_equal => return not_equal,
                    }
                }

                left_length.cmp(&right_length)
            }

            (Self::Fostered(left), Self::Fostered(right)) => left.cmp(right),
        }
    }
}

impl Hash for FosterStringVector {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Owned(strings) => {
                for string in strings {
                    state.write(string.as_bytes());
                }
            }

            Self::Fostered(strings) => {
                for string in strings.iter() {
                    state.write(string.as_bytes());
                }
            }
        }
    }
}

impl<'own> IntoIterator for &'own FosterStringVector {
    type Item = &'own str;
    type IntoIter =
        FosterIterator<&'own str, &'own String, &'own &'static str, Iter<'own, String>, Iter<'own, &'static str>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Foster::Owned(strings) => {
                Foster::new_owned(ConvertingIterator::new(strings.iter(), |string| Some(&string)))
            }
            Foster::Fostered(strings) => {
                Foster::new_fostered(ConvertingIterator::new(strings.iter(), |string| Some(string)))
            }
        }
    }
}

/// Delegates traits to a [FosterStringVector] newtype.
///
/// Example:
///
/// ```
/// #[derive(Clone, Debug)]
/// pub struct MyType(FosterStringVector);
///
/// delegate_newtype_of_foster_string_vector!(MyType);
/// ```
#[macro_export]
macro_rules! delegate_newtype_of_foster_string_vector {
    ( $type:ty $(,)? ) => {
        impl $type {
            /// Constructor.
            pub fn new_owned(strings: ::std::vec::Vec<::std::string::String>) -> Self {
                Self(::kutil_std::foster::Foster::new_owned(strings))
            }

            /// Constructor.
            pub const fn new_fostered(strings: &'static [&'static str]) -> Self {
                Self(::kutil_std::foster::Foster::new_fostered(strings))
            }
        }

        impl ::kutil_std::borrow::IntoOwned for $type {
            fn into_owned(self) -> Self {
                match self.0 {
                    ::kutil_std::foster::Foster::Owned(_) => self,
                    ::kutil_std::foster::Foster::Fostered(_) => Self(self.0.into_owned()),
                }
            }
        }

        impl ::kutil_std::foster::HasLength for $type {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for $type {
            fn from(strings: Vec<::std::string::String>) -> Self {
                strings.into()
            }
        }

        impl ::std::cmp::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl ::std::cmp::Eq for $type {}

        impl ::std::cmp::PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl ::std::cmp::Ord for $type {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl ::std::hash::Hash for $type {
            fn hash<HasherT>(&self, state: &mut HasherT)
            where
                HasherT: ::std::hash::Hasher,
            {
                self.0.hash(state)
            }
        }

        impl<'own> ::std::iter::IntoIterator for &'own $type {
            type Item = &'own str;
            type IntoIter = ::kutil_std::foster::FosterIterator<
                &'own str,
                &'own ::std::string::String,
                &'own &'static str,
                ::std::slice::Iter<'own, ::std::string::String>,
                ::std::slice::Iter<'own, &'static str>,
            >;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

#[allow(unused_imports)]
pub use delegate_newtype_of_foster_string_vector;
