use super::super::{context::*, depict::*, format::*};

use std::io::*;

/// Depict into map key.
pub const DEPICT_INTO_MAP_KEY: &str = "?";

/// Depict into map value.
pub const DEPICT_INTO_MAP_VALUE: &str = ":";

/// Depict into map entry.
pub const DEPICT_INTO_MAP_ENTRY: &str = "⚬"; // U+26AC

/// Depict map entry separator.
pub const DEPICT_INTO_MAP_ENTRY_SEPARATOR: &str = " ⇨"; // U+21E8

/// Depict map start.
pub const DEPICT_MAP_START: char = '{';

/// Depict map end.
pub const DEPICT_MAP_END: char = '}';

/// Depict map entry separator.
pub const DEPICT_MAP_ENTRY_SEPARATOR: char = ',';

/// Depict map key-value separator.
pub const DEPICT_MAP_KEY_VALUE_SEPARATOR: char = ':';

/// Write an [Iterator] of [Depict] as a map.
pub fn depict_map<'own, KeyT, ValueT, IteratorT, WriteT>(
    iterator: IteratorT,
    override_format: Option<DepictionFormat>,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> Result<()>
where
    KeyT: 'own + Depict,
    ValueT: 'own + Depict,
    IteratorT: Iterator<Item = (&'own KeyT, &'own ValueT)>,
    WriteT: Write,
{
    let mut iterator = iterator.peekable();

    if iterator.peek().is_none() {
        context.separate(writer)?;
        return context.theme.write_delimiter(writer, format!("{}{}", DEPICT_MAP_START, DEPICT_MAP_END));
    }

    let format = override_format.unwrap_or_else(|| context.get_format());

    match format {
        DepictionFormat::Compact => {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, DEPICT_MAP_START)?;

            while let Some((key, value)) = iterator.next() {
                key.depict(writer, context)?;
                context.theme.write_delimiter(writer, DEPICT_MAP_KEY_VALUE_SEPARATOR)?;
                value.depict(writer, context)?;
                if iterator.peek().is_some() {
                    context.theme.write_delimiter(writer, DEPICT_MAP_ENTRY_SEPARATOR)?;
                }
            }

            context.theme.write_delimiter(writer, DEPICT_MAP_END)
        }

        DepictionFormat::Optimized => {
            let key_context = context.child().with_separator(true).with_format(DepictionFormat::Compact);
            let value_context = context.child().with_inline(true).with_separator(true).increase_indentation();

            let mut first = true;
            while let Some((key, value)) = iterator.next() {
                context.separate_or_indent_into(writer, DEPICT_INTO_MAP_ENTRY, first)?;
                key.depict(writer, &key_context)?;

                context.theme.write_delimiter(writer, DEPICT_INTO_MAP_ENTRY_SEPARATOR)?;
                value.depict(writer, &value_context)?;

                first = false;
            }

            Ok(())
        }

        DepictionFormat::Verbose => {
            let child_context = context.child().with_separator(true).increase_indentation();

            let mut first = true;
            while let Some((key, value)) = iterator.next() {
                context.separate_or_indent_into(writer, DEPICT_INTO_MAP_KEY, first)?;
                key.depict(writer, &child_context)?;

                context.indent_into(writer, DEPICT_INTO_MAP_VALUE)?;
                value.depict(writer, &child_context)?;

                first = false;
            }

            Ok(())
        }
    }
}
