use super::generator::*;

use {proc_macro2::*, quote::*};

impl Generator {
    /// Generate `impl FromStr`.
    pub fn generate_impl_from_str(&self) -> TokenStream {
        let mut segments = Vec::<TokenStream>::default();

        for variant in &self.display_variants {
            let mut iterator = variant.strings.iter().peekable();
            while let Some(variant_string) = iterator.next() {
                if self.enum_attribute.lowercase {
                    let variant_string =
                        syn::LitStr::new(&variant_string.value().to_lowercase(), variant_string.span());
                    segments.push(quote! { #variant_string });
                } else {
                    segments.push(quote! { #variant_string });
                }

                if iterator.peek().is_some() {
                    segments.push(quote! { | });
                }
            }

            let variant_name = &variant.name;
            segments.push(quote! {
                => Ok(Self::#variant_name),
            })
        }

        let enum_name = &self.enum_name;
        let (impl_generics, enum_generics, where_clause) = self.enum_generics.split_for_impl();

        let representation_modifier = if self.enum_attribute.lowercase {
            quote! {
                .to_lowercase().as_str()
            }
        } else {
            Default::default()
        };

        let error = match &self.enum_attribute.error {
            Some(error) => error.to_token_stream(),
            None => quote! { ::kutil::std::string::ParseError },
        };

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                ::std::str::FromStr for #enum_name #enum_generics
                #where_clause
            {
                type Err = #error;

                fn from_str(representation: &str) -> Result<Self, Self::Err> {
                    match representation #representation_modifier {
                        #(#segments)*
                        _ => Err(representation.into()),
                    }
                }
            }
        }
    }
}
