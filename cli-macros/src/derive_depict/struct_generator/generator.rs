use super::super::attributes::*;

use {deluxe::*, proc_macro2::*, quote::*, syn::spanned::*};

//
// Generator
//

/// Generator for `#[derive(Depict)]`.
#[derive(Default)]
pub struct StructGenerator {
    /// Name of the struct for which we are generating.
    pub struct_name: TokenStream,

    /// The generics of the struct for which we are generating.
    pub struct_generics: syn::Generics,

    /// Struct-level attribute.
    pub struct_attribute: StructAttribute,

    /// The depict fields.
    pub depict_fields: Vec<Field>,
}

impl StructGenerator {
    /// Generate.
    pub fn generate(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
        let generator = Self::new(input)?;
        Ok(generator.generate_impl_depict())
    }

    /// Constructor.
    pub fn new(input: &mut syn::DeriveInput) -> syn::Result<Self> {
        let mut generator = Self::default();

        generator.struct_name = input.ident.to_token_stream();
        generator.struct_generics = input.generics.clone();
        generator.struct_attribute = extract_attributes(input)?;

        match &mut input.data {
            syn::Data::Struct(data) => {
                for field in data.fields.iter_mut() {
                    let mut field_attribute: FieldAttribute = extract_attributes(field)?;

                    if !matches!(field_attribute.iter, Iter::KeyValue)
                        && !matches!(field_attribute.key_as, As::Display)
                        && !matches!(field_attribute.key_style, Style::None)
                    {
                        return Err(syn::Error::new(
                            field.span(),
                            "`depict` attribute: cannot use `key_as` and `key_style` without `iter(kv)`",
                        ));
                    }

                    if matches!(field_attribute.value_as, As::Depict)
                        && !matches!(field_attribute.value_style, Style::None)
                    {
                        return Err(syn::Error::new(
                            field.span(),
                            "`depict` attribute: cannot use `as(depict)` with `style`",
                        ));
                    }

                    if matches!(field_attribute.key_as, As::Depict) && !matches!(field_attribute.key_style, Style::None)
                    {
                        return Err(syn::Error::new(
                            field.span(),
                            "`depict` attribute: cannot use `key_as(depict)` with `key_style`",
                        ));
                    }

                    if field_attribute.skip {
                        continue;
                    }

                    if field_attribute.tag.is_none() && generator.struct_attribute.tag.is_some() {
                        field_attribute.tag = generator.struct_attribute.tag.clone();
                    }

                    let field_name = field
                        .ident
                        .as_ref()
                        .ok_or_else(|| syn::Error::new(field.span(), "`depict` attribute: unnamed field"))?;

                    generator
                        .depict_fields
                        .push(Field { name: field_name.to_token_stream(), attribute: field_attribute });
                }
            }

            _ => return Err(syn::Error::new(input.ident.span(), "`Depict`: not a struct")),
        }

        Ok(generator)
    }
}

//
// Field
//

/// Generator field.
pub struct Field {
    /// Field name.
    pub name: TokenStream,

    /// Field attribute.
    pub attribute: FieldAttribute,
}
