use super::branch::*;

use deluxe::*;

//
// StructAttribute
//

/// Struct-level attribute for `#[derive(Depict)]`.
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(depict))]
pub struct StructAttribute {
    /// Branch.
    #[deluxe(default)]
    pub branch: Branch,

    /// Optional tag.
    #[deluxe(default)]
    pub tag: Option<syn::Expr>,
}
