use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate `impl Depict`.
    pub fn generate_impl_depict(&self) -> TokenStream {
        let mut segments = Vec::default();

        for variant in &self.variants {
            segments.push(self.generate_handle_variant(variant));
        }

        let enum_name = &self.enum_name;
        let (impl_generics, struct_generics, where_clause) = self.enum_generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                ::kutil::cli::depict::Depict
                for #enum_name #struct_generics
                #where_clause
            {
                fn
                    depict
                    <WriteT>
                    (
                        &self,
                        writer: &mut WriteT,
                        context: &::kutil::cli::depict::DepictionContext,
                    )
                    -> ::std::io::Result<()>
                    where WriteT: ::std::io::Write
                {
                    match self {
                        #(#segments)*
                    }

                    Ok(())
                }
            }
        }
    }
}
