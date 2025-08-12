use super::{super::attributes::*, generator::*};

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate field handler.
    pub fn generate_handle_field(&self, field: &Field, last: bool) -> TokenStream {
        let field_name = &field.name;
        let quoted_field_name = field.name.to_string().to_token_stream();

        let tag = match &field.attribute.tag {
            Some(tag) => quote! {
                #tag(self, #quoted_field_name, writer, context)?;
            },

            None => Default::default(),
        };

        let write_value = field.attribute.value_as.generate_write_value(&field.attribute.value_style);

        let (indent, item_context) = match self.struct_attribute.branch {
            Branch::Thin => (
                quote! { context.indent_into_branch(writer, #last)?; },
                quote! { &context.child().with_inline(true).increase_indentation_branch(#last) },
            ),

            Branch::Thick => (
                quote! { context.indent_into_thick_branch(writer, #last)?; },
                quote! { &context.child().with_inline(true).increase_indentation_thick_branch(#last) },
            ),

            Branch::Double => (
                quote! { context.indent_into_double_branch(writer, #last)?; },
                quote! { &context.child().with_inline(true).increase_indentation_double_branch(#last) },
            ),
        };

        let mut write = match &field.attribute.iter {
            Iter::None => quote! {
                let item_context = #item_context;
                let child_context = &item_context.child().with_inline(true);
                #write_value
                #tag
            },

            Iter::Item => quote! {
                let child_context = #item_context;
                let mut empty = true;

                for item in value {
                    empty = false;

                    child_context.indent_into(writer, ::kutil::cli::depict::utils::DEPICT_INTO_LIST_ITEM)?;
                    let value = item;
                    #write_value

                    #tag
                }

                if empty {
                    context.separate(writer)?;
                    context.theme.write_delimiter(writer, "[]")?;
                }
            },

            Iter::KeyValue => {
                let write_key = field.attribute.key_as.generate_write_value(&field.attribute.key_style);
                quote! {
                    let item_context = #item_context;
                    let mut empty = true;

                    {
                        use ::kutil::cli::depict::DepictionFormatUtilities;
                        match item_context.get_format() {
                            ::kutil::cli::depict::DepictionFormat::Optimized => {
                                let key_context = item_context.child().with_separator(true).with_format(::kutil::cli::depict::DepictionFormat::Compact);
                                let value_context = item_context.child().with_inline(true).with_separator(true).increase_indentation();

                                for (k, v) in value {
                                    empty = false;

                                    item_context.indent_into(writer, ::kutil::cli::depict::utils::DEPICT_INTO_MAP_ENTRY)?;
                                    let value = k;
                                    let child_context = &key_context;
                                    #write_key

                                    context.theme.write_delimiter(writer, ::kutil::cli::depict::utils::DEPICT_INTO_MAP_ENTRY_SEPARATOR)?;
                                    let value = v;
                                    let child_context = &value_context;
                                    #write_value

                                    #tag
                                }
                            }

                            _ => {
                                let child_context = item_context;

                                for (k, v) in value {
                                    empty = false;

                                    item_context.indent_into(writer, ::kutil::cli::depict::utils::DEPICT_INTO_MAP_KEY)?;
                                    let value = k;
                                    #write_key

                                    item_context.indent_into(writer, ::kutil::cli::depict::utils::DEPICT_INTO_MAP_VALUE)?;
                                    let value = v;
                                    #write_value

                                    #tag
                                }
                            }
                        }
                    }

                    if empty {
                        context.separate(writer)?;
                        context.theme.write_delimiter(writer, "{}")?;
                    }
                }
            }
        };

        write = match &field.attribute.option {
            true => quote! {
                match &self.#field_name {
                    ::std::option::Option::Some(value) => {
                        context.separate(writer)?;
                        context.theme.write_symbol(writer, "Some")?;
                        #write
                    },

                    ::std::option::Option::None => {
                        context.separate(writer)?;
                        context.theme.write_symbol(writer, "None")?;
                    },
                }
            },

            false => quote! {
                let value = &self.#field_name;
                #write
            },
        };

        quote! {
            #indent
            context.theme.write_meta(writer, #quoted_field_name)?;
            context.theme.write_delimiter(writer, ':')?;
            #write
        }
    }
}
