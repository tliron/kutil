use deluxe::*;

/// Prefix branch style.
#[derive(Default, ParseMetaItem)]
pub enum Branch {
    #[default]
    Thin,
    Thick,
    Double,
}
