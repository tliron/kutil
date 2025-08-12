use super::{super::borrow::*, foster::*, has_length::*};

use std::{cmp::*, fmt, hash::*};

/// [Foster] for [String].
///
/// Supports [IntoOwned], [HasLength], [AsRef]\<str\>, [Eq]/[PartialEq], [Ord]/[PartialOrd],
/// [Hash], and [Display](fmt::Display).
pub type FosterString = Foster<String, &'static str>;

impl IntoOwned for FosterString {
    /// Into owned.
    fn into_owned(self) -> Self {
        match self {
            Self::Owned(_) => self,
            Self::Fostered(string) => Self::Owned(string.into()),
        }
    }
}

impl HasLength for FosterString {
    fn len(&self) -> usize {
        match self {
            Self::Owned(string) => string.len(),
            Self::Fostered(string) => string.len(),
        }
    }
}

impl From<&str> for FosterString {
    fn from(string: &str) -> Self {
        Self::Owned(string.into())
    }
}

impl AsRef<str> for FosterString {
    fn as_ref(&self) -> &str {
        match self {
            Self::Owned(string) => string,
            Self::Fostered(string) => string,
        }
    }
}

impl PartialEq for FosterString {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Owned(string), Self::Owned(other_string)) => string == other_string,
            (Self::Owned(string), Self::Fostered(other_string)) => string == other_string,
            (Self::Fostered(string), Self::Owned(other_string)) => string == other_string,
            (Self::Fostered(string), Self::Fostered(other_string)) => string == other_string,
        }
    }
}

impl Eq for FosterString {}

impl PartialOrd for FosterString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Owned(string), Self::Owned(other_string)) => string.partial_cmp(other_string),
            (Self::Owned(string), Self::Fostered(other_string)) => string.as_str().partial_cmp(*other_string),
            (Self::Fostered(string), Self::Owned(other_string)) => (*string).partial_cmp(other_string.as_str()),
            (Self::Fostered(string), Self::Fostered(other_string)) => string.partial_cmp(other_string),
        }
    }
}

impl Ord for FosterString {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Owned(string), Self::Owned(other_string)) => string.cmp(other_string),
            (Self::Owned(string), Self::Fostered(other_string)) => string.as_str().cmp(*other_string),
            (Self::Fostered(string), Self::Owned(other_string)) => (*string).cmp(other_string.as_str()),
            (Self::Fostered(string), Self::Fostered(other_string)) => string.cmp(other_string),
        }
    }
}

impl Hash for FosterString {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Owned(string) => state.write(string.as_bytes()),
            Self::Fostered(string) => state.write(string.as_bytes()),
        }
    }
}

impl fmt::Display for FosterString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owned(string) => fmt::Display::fmt(string, formatter),
            Self::Fostered(string) => fmt::Display::fmt(string, formatter),
        }
    }
}

/// Delegates traits to a [FosterString] newtype.
///
/// Example:
///
/// ```
/// #[derive(Clone, Debug)]
/// pub struct MyType(FosterString);
///
/// delegate_newtype_of_foster_string!(MyType);
/// ```
///
#[macro_export]
macro_rules! delegate_newtype_of_foster_string {
    ( $type:ty $(,)? ) => {
        impl $type {
            /// Constructor.
            pub const fn new_owned(string: ::std::string::String) -> Self {
                Self(::kutil_std::foster::Foster::new_owned(string))
            }

            /// Constructor.
            pub const fn new_fostered(string: &'static str) -> Self {
                Self(::kutil_std::foster::Foster::new_fostered(string))
            }
        }

        impl ::kutil_std::borrow::IntoOwned for $type {
            fn into_owned(self) -> Self {
                match self.0 {
                    ::kutil_std::foster::Foster::Owned(_) => self,
                    ::kutil_std::foster::Foster::Fostered(string) => Self::new_owned(string.into()),
                }
            }
        }

        impl ::kutil_std::foster::HasLength for $type {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl ::std::convert::From<::std::string::String> for $type {
            fn from(string: ::std::string::String) -> Self {
                string.into()
            }
        }

        impl ::std::convert::From<&str> for $type {
            fn from(string: &str) -> Self {
                Self(string.into())
            }
        }

        impl ::std::convert::AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl ::std::cmp::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl ::std::cmp::Eq for $type {}

        impl ::std::cmp::PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
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

        impl ::std::fmt::Display for $type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, formatter)
            }
        }
    };
}
