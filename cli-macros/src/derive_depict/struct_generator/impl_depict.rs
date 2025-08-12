use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate `impl Depict`.
    pub fn generate_impl_depict(&self) -> TokenStream {
        let mut segments = Vec::default();

        let mut iterator = self.depict_fields.iter().peekable();
        while let Some(depict_field) = iterator.next() {
            segments.push(self.generate_handle_field(depict_field, iterator.peek().is_none()));
        }

        let tag = match &self.struct_attribute.tag {
            Some(tag) => quote! {
                #tag(self, "", writer, context)?;
            },
            None => Default::default(),
        };

        let struct_name = &self.struct_name;
        let quoted_struct_name = struct_name.to_string().to_token_stream();
        let (impl_generics, struct_generics, where_clause) = self.struct_generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                ::kutil::cli::depict::Depict
                for #struct_name #struct_generics
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
                    let mut context = context.child();

                    if match context.configuration.remove("heading") {
                        Some(heading) => heading == "true",
                        None => true,
                    } {
                        context.separate(writer)?;
                        context.theme.write_heading(writer, #quoted_struct_name)?;
                    }

                    let context = &mut context.with_separator(true);

                    #tag

                    #(#segments)*

                    Ok(())
                }
            }
        }
    }
}
