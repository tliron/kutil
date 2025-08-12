use super::super::context::*;

use std::io::*;

/// Depict field separator.
pub const DEPICT_FIELD_SEPARATOR: char = ':';

/// Depict field.
pub fn depict_field<WriteT, WriteNestedT>(
    meta: &str,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
    write_nested: WriteNestedT,
) -> Result<()>
where
    WriteT: Write,
    WriteNestedT: Fn(&mut WriteT, &DepictionContext) -> Result<()>,
{
    context.indent_into_branch(writer, last)?;
    context.theme.write_meta(writer, meta)?;
    context.theme.write_delimiter(writer, DEPICT_FIELD_SEPARATOR)?;
    write_nested(writer, &context.child().with_inline(true).with_separator(true).increase_indentation_branch(last))
}
