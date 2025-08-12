#[cfg(feature = "immutable")]
mod byte_string;
#[cfg(feature = "immutable")]
mod byte_string_vector;
mod foster;
mod has_length;
mod iterator;
mod string;
mod string_vector;

#[allow(unused_imports)]
pub use {
    crate::{delegate_newtype_of_foster_string, delegate_newtype_of_foster_string_vector},
    foster::*,
    has_length::*,
    iterator::*,
    string::*,
    string_vector::*,
};

#[cfg(feature = "immutable")]
#[allow(unused_imports)]
pub use {
    crate::{delegate_newtype_of_foster_byte_string, delegate_newtype_of_foster_byte_string_vector},
    byte_string::*,
    byte_string_vector::*,
};
