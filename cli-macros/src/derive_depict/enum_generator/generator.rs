use super::super::attributes::*;

use {deluxe::*, proc_macro2::*, quote::*, syn::spanned::*};

//
// Generator
//

/// Generator for `#[derive(Depict)]`.
#[derive(Default)]
pub struct EnumGenerator {
    /// Name of the enum for which we are generating.
    pub enum_name: TokenStream,

    /// The generics of the enum for which we are generating.
    pub enum_generics: syn::Generics,

    /// Enum-level attribute.
    pub enum_attribute: EnumAttribute,

    /// The variants.
    pub variants: Vec<Variant>,
}

impl EnumGenerator {
    /// Generate.
    pub fn generate(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
        let generator = Self::new(input)?;
        Ok(generator.generate_impl_depict())
    }

    /// Constructor.
    pub fn new(input: &mut syn::DeriveInput) -> syn::Result<Self> {
        let mut generator = Self::default();

        generator.enum_name = input.ident.to_token_stream();
        generator.enum_generics = input.generics.clone();
        generator.enum_attribute = extract_attributes(input)?;

        match &mut input.data {
            syn::Data::Enum(data) => {
                for variant in data.variants.iter_mut() {
                    match &variant.fields {
                        syn::Fields::Unnamed(fields) => {
                            if fields.unnamed.len() != 1 {
                                return Err(syn::Error::new(
                                    variant.ident.span(),
                                    "`Depict`: variants with more than 1 unnamed field are not supported",
                                ));
                            }
                        }

                        syn::Fields::Named(_) => {
                            return Err(syn::Error::new(
                                variant.ident.span(),
                                "`Depict`: variants with named fields are not supported",
                            ));
                        }

                        syn::Fields::Unit => {}
                    }

                    let variant_attribute: VariantAttribute = extract_attributes(variant)?;

                    if !matches!(variant_attribute.iter, Iter::KeyValue)
                        && !matches!(variant_attribute.key_as, As::Display)
                        && !matches!(variant_attribute.key_style, Style::None)
                    {
                        return Err(syn::Error::new(
                            variant.span(),
                            "`depict` attribute: cannot use key_as and key_style without iter(kv)",
                        ));
                    }

                    if matches!(variant_attribute.value_as, As::Depict)
                        && !matches!(variant_attribute.value_style, Style::None)
                    {
                        return Err(syn::Error::new(
                            variant.span(),
                            "`depict` attribute: cannot use as(depict) with style",
                        ));
                    }

                    if matches!(variant_attribute.key_as, As::Depict)
                        && !matches!(variant_attribute.key_style, Style::None)
                    {
                        return Err(syn::Error::new(
                            variant.span(),
                            "`depict` attribute: cannot use key_as(depict) with key_style",
                        ));
                    }

                    let variant_name = &variant.ident;

                    generator.variants.push(Variant {
                        name: variant_name.to_token_stream(),
                        attribute: variant_attribute,
                        has_fields: !variant.fields.is_empty(),
                    });
                }
            }

            _ => return Err(syn::Error::new(input.ident.span(), "`Depict`: not an enum")),
        }

        Ok(generator)
    }
}

//
// Variant
//

/// Generator variant.
pub struct Variant {
    /// Variant name.
    pub name: TokenStream,

    /// Variant attribute.
    pub attribute: VariantAttribute,

    /// Whether the variant has fields.
    pub has_fields: bool,
}
