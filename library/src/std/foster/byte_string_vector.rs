use super::{
    super::{borrow::*, immutable::*, iter::*},
    foster::*,
    has_length::*,
    iterator::*,
};

use std::{cmp::*, hash::*, slice::*};

/// [Foster] for [Vec]\<[ByteString]\>.
///
/// Supports [IntoOwned], [HasLength], [Eq]/[PartialEq], [Ord]/[PartialOrd], [Hash], and
/// [IntoIterator].
pub type FosterByteStringVector = Foster<Vec<ByteString>, &'static [&'static str]>;

impl IntoOwned for FosterByteStringVector {
    fn into_owned(self) -> Self {
        match self {
            Self::Owned(_) => self,
            Self::Fostered(strings) => Self::new_owned(strings.iter().map(|string| (*string).into()).collect()),
        }
    }
}

impl HasLength for FosterByteStringVector {
    fn len(&self) -> usize {
        match self {
            Self::Owned(strings) => strings.len(),
            Self::Fostered(strings) => strings.len(),
        }
    }
}

impl PartialEq for FosterByteStringVector {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Owned(strings), Self::Owned(other_strings)) => strings == other_strings,

            (Self::Owned(strings), Self::Fostered(other_strings)) => {
                if strings.len() == other_strings.len() {
                    let mut equal = true;
                    for (index, string) in strings.iter().enumerate() {
                        if string != other_strings[index] {
                            equal = false;
                            break;
                        }
                    }
                    equal
                } else {
                    false
                }
            }

            (Self::Fostered(strings), Self::Owned(other_strings)) => {
                if strings.len() == other_strings.len() {
                    let mut equal = true;
                    for (index, string) in strings.iter().enumerate() {
                        let other_string: &str = &other_strings[index];
                        if *string != other_string {
                            equal = false;
                            break;
                        }
                    }
                    equal
                } else {
                    false
                }
            }

            (Self::Fostered(strings), Self::Fostered(other_strings)) => strings == other_strings,
        }
    }
}

impl Eq for FosterByteStringVector {}

impl PartialOrd for FosterByteStringVector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // See: core::slice::cmp::SlicePartialOrd

        match (self, other) {
            (Self::Owned(strings), Self::Owned(other_strings)) => strings.partial_cmp(other_strings),

            (Self::Owned(strings), Self::Fostered(other_strings)) => {
                let strings_length = strings.len();
                let other_strings_length = other_strings.len();
                let length = min(strings_length, other_strings_length);

                // enable compiler bound check elimination
                let strings_bounded = &strings[..length];
                let other_strings_bounded = &other_strings[..length];

                for index in 0..length {
                    match (*strings_bounded[index]).partial_cmp(other_strings_bounded[index]) {
                        Some(Ordering::Equal) => {}
                        not_equal => return not_equal,
                    }
                }

                strings_length.partial_cmp(&other_strings_length)
            }

            (Self::Fostered(strings), Self::Owned(other_strings)) => {
                let strings_length = strings.len();
                let other_strings_length = other_strings.len();
                let length = min(strings_length, other_strings_length);

                // enable compiler bound check elimination
                let strings_bounded = &strings[..length];
                let other_strings_bounded = &other_strings[..length];

                for index in 0..length {
                    let other_string_bounded: &str = &other_strings_bounded[index];
                    match strings_bounded[index].partial_cmp(other_string_bounded) {
                        Some(Ordering::Equal) => {}
                        not_equal => return not_equal,
                    }
                }

                strings_length.partial_cmp(&other_strings_length)
            }

            (Self::Fostered(strings), Self::Fostered(other_strings)) => strings.partial_cmp(other_strings),
        }
    }
}

impl Ord for FosterByteStringVector {
    fn cmp(&self, other: &Self) -> Ordering {
        // See: core::slice::cmp::SliceOrd

        match (self, other) {
            (Self::Owned(strings), Self::Owned(other_strings)) => strings.cmp(other_strings),

            (Self::Owned(strings), Self::Fostered(other_strings)) => {
                let strings_length = strings.len();
                let other_strings_length = other_strings.len();
                let length = min(strings_length, other_strings_length);

                // enable compiler bound check elimination
                let strings_bounded = &strings[..length];
                let other_strings_bounded = &other_strings[..length];

                for index in 0..length {
                    match (*strings_bounded[index]).cmp(other_strings_bounded[index]) {
                        Ordering::Equal => {}
                        not_equal => return not_equal,
                    }
                }

                strings_length.cmp(&other_strings_length)
            }

            (Self::Fostered(strings), Self::Owned(other_strings)) => {
                let strings_length = strings.len();
                let other_strings_length = other_strings.len();
                let length = min(strings_length, other_strings_length);

                // enable compiler bound check elimination
                let strings_bounded = &strings[..length];
                let other_strings_bounded = &other_strings[..length];

                for index in 0..length {
                    let other_string_bounded: &str = &other_strings_bounded[index];
                    match strings_bounded[index].cmp(other_string_bounded) {
                        Ordering::Equal => {}
                        not_equal => return not_equal,
                    }
                }

                strings_length.cmp(&other_strings_length)
            }

            (Self::Fostered(strings), Self::Fostered(other_strings)) => strings.cmp(other_strings),
        }
    }
}

impl Hash for FosterByteStringVector {
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

impl<'own> IntoIterator for &'own FosterByteStringVector {
    type Item = &'own str;
    type IntoIter = FosterIterator<
        &'own str,
        &'own ByteString,
        &'own &'static str,
        Iter<'own, ByteString>,
        Iter<'own, &'static str>,
    >;

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

/// Delegates traits to a [FosterByteStringVector] newtype.
///
/// Example:
///
/// ```
/// #[derive(Clone, Debug)]
/// pub struct MyType(FosterByteStringVector);
///
/// delegate_newtype_of_foster_byte_string_vector!(MyType);
/// ```
#[macro_export]
macro_rules! delegate_newtype_of_foster_byte_string_vector {
    ( $type:ty $(,)? ) => {
        impl $type {
            /// Constructor.
            pub fn new_owned(strings: ::std::vec::Vec<$crate::std::immutable::ByteString>) -> Self {
                Self($crate::std::foster::Foster::new_owned(strings))
            }

            /// Constructor.
            pub const fn new_fostered(strings: &'static [&'static str]) -> Self {
                Self($crate::std::foster::Foster::new_fostered(strings))
            }
        }

        impl $crate::std::borrow::IntoOwned for $type {
            fn into_owned(self) -> Self {
                match self.0 {
                    $crate::std::foster::Foster::Owned(_) => self,
                    $crate::std::foster::Foster::Fostered(_) => Self(self.0.into_owned()),
                }
            }
        }

        impl $crate::std::foster::HasLength for $type {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl ::std::convert::From<::std::vec::Vec<$crate::std::immutable::ByteString>> for $type {
            fn from(strings: Vec<$crate::std::immutable::ByteString>) -> Self {
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
            type IntoIter = $crate::std::foster::FosterIterator<
                &'own str,
                &'own $crate::std::immutable::ByteString,
                &'own &'static str,
                ::std::slice::Iter<'own, $crate::std::immutable::ByteString>,
                ::std::slice::Iter<'own, &'static str>,
            >;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

#[allow(unused_imports)]
pub use delegate_newtype_of_foster_byte_string_vector;
