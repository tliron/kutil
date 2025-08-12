use super::style::*;

use {deluxe::*, proc_macro2::*, quote::*};

//
// As
//

#[derive(Default, ParseMetaItem)]
pub enum As {
    #[default]
    Display,
    Debug,
    #[deluxe(rename = debug_alt)]
    DebugAlt,
    Depict,
    #[deluxe(rename = dyn_depict)]
    DynDepict,
    Custom(syn::Expr),
}

impl As {
    /// Write it.
    pub fn generate_write_value(&self, style: &Style) -> TokenStream {
        match self {
            Self::Display => {
                let value = style.style(quote! { format!("{}", value) });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            Self::Debug => {
                let value = style.style(quote! { format!("{:?}", value) });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            Self::DebugAlt => {
                let value = style.style(quote! { format!("{:#?}", value) });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            Self::Depict => quote! {
                ::kutil::cli::depict::Depict::depict(
                    value,
                    writer,
                    child_context,
                )?;
            },

            Self::DynDepict => quote! {
                ::kutil::cli::depict::DynDepict::dyn_depict(
                    value.as_ref(),
                    ::std::boxed::Box::new(writer),
                    child_context,
                )?;
            },

            Self::Custom(custom) => {
                let value = style.style(quote! { (#custom)(value)? });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }
        }
    }
}
