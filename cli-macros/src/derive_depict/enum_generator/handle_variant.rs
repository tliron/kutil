use super::{super::attributes::*, generator::*};

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate variant handler.
    pub fn generate_handle_variant(&self, variant: &Variant) -> TokenStream {
        let variant_name = &variant.name;

        let write_variant = if self.enum_attribute.variant {
            let quoted_enum_name = self.enum_name.to_string().to_token_stream();
            let quoted_variant_name = variant.name.to_string().to_token_stream();
            quote! {
                if match context.configuration.get("variant") {
                    Some(variant) => variant == "true",
                    None => true,
                } {
                    context.separate(writer)?;
                    ::std::write!(writer, "{}{}{}",
                        context.theme.symbol(#quoted_enum_name),
                        context.theme.delimiter("::"),
                        context.theme.symbol(#quoted_variant_name),
                    )?;
                }
            }
        } else {
            Default::default()
        };

        let write_value = variant.attribute.value_as.generate_write_value(&variant.attribute.value_style);

        let mut write = match &variant.attribute.iter {
            Iter::None => quote! {
                let child_context = context;
                #write_value
            },

            Iter::Item => quote! {
                let child_context = context;
                let mut empty = true;

                for item in value {
                    empty = false;

                    child_context.indent_into(writer, ::kutil::cli::depict::utils::DEPICT_INTO_LIST_ITEM)?;
                    let value = item;
                    #write_value
                }

                if empty {
                    context.separate(writer)?;
                    context.theme.write_delimiter(writer, "[]")?;
                }
            },

            Iter::KeyValue => {
                let write_key = variant.attribute.key_as.generate_write_value(&variant.attribute.key_style);
                quote! {
                    let item_context = context;
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
                                    let child_context = key_context;
                                    #write_key

                                    context.theme.write_delimiter(writer, ::kutil::cli::depict::utils::DEPICT_MAP_ENTRY_SEPARATOR)?;
                                    let value = v;
                                    let child_context = value_context;
                                    #write_value
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

        write = match &variant.attribute.option {
            true => quote! {
                match value {
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
                #write
            },
        };

        if variant.has_fields {
            quote! {
                Self::#variant_name(value) => {
                    #write_variant
                    #write
                },
            }
        } else {
            quote! {
                Self::#variant_name => {
                    #write_variant
                },
            }
        }
    }
}
