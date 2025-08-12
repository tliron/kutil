use {deluxe::*, proc_macro2::*, quote::*};

//
// Style
//

#[derive(Default, ParseMetaItem)]
pub enum Style {
    #[default]
    None,
    Symbol,
    Number,
    String,
    Name,
    Meta,
    Error,
    Delimiter,
    Heading,
}

impl Style {
    /// Write it with style.
    pub fn style(&self, value: TokenStream) -> TokenStream {
        match self {
            Self::None => value,
            Self::Symbol => quote! { context.theme.symbol(#value) },
            Self::Number => quote! { context.theme.number(#value) },
            Self::String => quote! { context.theme.string(#value) },
            Self::Name => quote! { context.theme.name(#value) },
            Self::Meta => quote! { context.theme.meta(#value) },
            Self::Error => quote! { context.theme.error(#value) },
            Self::Delimiter => quote! { context.theme.delimiter(#value) },
            Self::Heading => quote! { context.theme.heading(#value) },
        }
    }
}
