use super::{r#as::*, iter::*, style::*};

use deluxe::*;

//
// VariantAttribute
//

/// Variant-level attribute for `#[derive(Depict)]`.
#[derive(ExtractAttributes)]
#[deluxe(attributes(depict))]
pub struct VariantAttribute {
    /// How to represent the value.
    #[deluxe(default, rename = as)]
    pub value_as: As,

    /// How to represent the key, for `iter(kv)`.
    #[deluxe(default)]
    pub key_as: As,

    /// Value style.
    #[deluxe(default, rename = style)]
    pub value_style: Style,

    /// Key style.
    #[deluxe(default)]
    pub key_style: Style,

    /// Whether it's an [Option].
    #[deluxe(default)]
    pub option: bool,

    /// Whether and how to iterate.
    #[deluxe(default)]
    pub iter: Iter,
}
