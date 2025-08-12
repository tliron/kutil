use deluxe::*;

//
// EnumAttribute
//

/// Enum-level attribute for `#[derive(Depict)]`.
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(depict))]
pub struct EnumAttribute {
    /// Whether to include the variant name.
    #[deluxe(default = true)]
    pub variant: bool,
}
