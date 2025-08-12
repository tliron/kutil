use super::super::{context::*, depict::*, format::*};

use std::io::*;

/// Depict into list item.
pub const DEPICT_INTO_LIST_ITEM: &str = "⦁︎"; // '\u{2981}'

/// Depict list start.
pub const DEPICT_LIST_START: char = '[';

/// Depict list end.
pub const DEPICT_LIST_END: char = ']';

/// Depict list separator.
pub const DEPICT_LIST_SEPARATOR: char = ',';

/// Write an [Iterator] of [Depict] as a list.
pub fn depict_list<'own, ItemT, IteratorT, WriteT>(
    iterator: IteratorT,
    override_format: Option<DepictionFormat>,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> Result<()>
where
    ItemT: 'own + Depict,
    IteratorT: Iterator<Item = &'own ItemT>,
    WriteT: Write,
{
    let mut iterator = iterator.peekable();

    if iterator.peek().is_none() {
        context.separate(writer)?;
        return context.theme.write_delimiter(writer, format!("{}{}", DEPICT_LIST_START, DEPICT_LIST_END));
    }

    let format = override_format.unwrap_or_else(|| context.get_format());

    match format {
        DepictionFormat::Compact => {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, DEPICT_LIST_START)?;

            let child_context = context.child().with_separator(false);

            while let Some(item) = iterator.next() {
                item.depict(writer, &child_context)?;
                if iterator.peek().is_some() {
                    context.theme.write_delimiter(writer, DEPICT_LIST_SEPARATOR)?;
                }
            }

            context.theme.write_delimiter(writer, DEPICT_LIST_END)
        }

        DepictionFormat::Optimized | DepictionFormat::Verbose => {
            let child_context = context.child().with_separator(true).increase_indentation();

            let mut first = true;
            for item in iterator {
                context.separate_or_indent_into(writer, DEPICT_INTO_LIST_ITEM, first)?;
                item.depict(writer, &child_context)?;

                first = false;
            }

            Ok(())
        }
    }
}
