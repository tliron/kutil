use super::generator::*;

use {proc_macro2::*, quote::*};

impl Generator {
    /// Generate `impl Display`.
    pub fn generate_impl_display(&self) -> TokenStream {
        let mut segments = Vec::<TokenStream>::default();

        for variant in &self.display_variants {
            let variant_name = &variant.name;

            segments.push(quote! {
                Self::#variant_name =>
            });

            let variant_string = variant.strings.first().expect("at least one string");
            if self.enum_attribute.lowercase {
                let variant_string = syn::LitStr::new(&variant_string.value().to_lowercase(), variant_string.span());
                segments.push(quote! {
                    #variant_string,
                });
            } else {
                segments.push(quote! {
                    #variant_string,
                });
            }
        }

        let enum_name = &self.enum_name;
        let (impl_generics, enum_generics, where_clause) = self.enum_generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl
            #impl_generics
                 #enum_name #enum_generics
            {
                /// As string.
                pub fn as_str(self) -> &'static str {
                    match self {
                        #(#segments)*
                    }
                }
            }

            #[automatically_derived]
            impl
            #impl_generics
                ::std::convert::Into<&'static str> for #enum_name #enum_generics
            {
                fn into(self) -> &'static str {
                    self.as_str()
                }
            }

            #[automatically_derived]
            impl
                #impl_generics
                ::std::fmt::Display for #enum_name #enum_generics
                #where_clause
            {
                fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::std::fmt::Display::fmt(self.as_str(), formatter)
                }
            }
        }
    }
}
