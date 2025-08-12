use super::{context::*, theme::*};

use std::io::*;

const TO_STRING_BUFFER_CAPACITY: usize = 1024;

//
// Depict
//

/// Depict the object in a manner suitable for terminals.
///
/// May include colors and styles.
pub trait Depict {
    /// Write a depiction suitable for terminals.
    ///
    /// Required behavior for implementations:
    ///
    /// 1. Depictions *must not* end in a newline.
    /// 2. If *not* starting with a newline and *not* empty, *must* call [DepictionContext::separate] first.
    /// 3. All lines *after* the first (but *not* the first) *must* start with the [DepictionContext] indentation.
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> Result<()>
    where
        WriteT: Write;

    /// Write the depiction with a final newline.
    fn write_depiction<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> Result<()>
    where
        WriteT: Write,
    {
        self.depict(writer, context)?;
        writeln!(writer)
    }

    /// Write the depiction with a final newline.
    ///
    /// Uses default [Theme].
    fn write_default_depiction<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        self.write_depiction(writer, &DepictionContext::new(&Theme::default()))
    }

    /// Write the depiction with a final newline.
    ///
    /// Uses plain [Theme].
    fn write_plain_depiction<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        self.write_depiction(writer, &DepictionContext::new(&Theme::plain()))
    }

    /// Print the depiction to [anstream::stdout] with a final newline.
    ///
    /// Panics on write [Error].
    fn print_depiction(&self, context: &DepictionContext) {
        self.write_depiction(&mut anstream::stdout(), context).expect("writing to stdout");
    }

    /// Print the depiction to [anstream::stdout] with a final newline.
    ///
    /// Uses default [Theme].
    ///
    /// Panics on write [Error].
    fn print_default_depiction(&self) {
        self.print_depiction(&DepictionContext::new(&Theme::default()));
    }

    /// Print the depiction to [anstream::stdout] with a final newline.
    ///
    /// Uses plain [Theme].
    ///
    /// Panics on write [Error].
    fn print_plain_depiction(&self) {
        self.print_depiction(&DepictionContext::new(&Theme::plain()));
    }

    /// Print the depiction to [anstream::stderr] with a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_depiction(&self, context: &DepictionContext) {
        self.write_depiction(&mut anstream::stderr(), context).expect("writing to stderr");
    }

    /// Print the depiction to [anstream::stderr] with a final newline.
    ///
    /// Uses default [Theme].
    ///
    /// Panics on write [Error].
    fn eprint_default_depiction(&self) {
        self.eprint_depiction(&DepictionContext::new(&Theme::default()));
    }

    /// Print the depiction to [anstream::stderr] with a final newline.
    ///
    /// Uses plain [Theme].
    ///
    /// Panics on write [Error].
    fn eprint_plain_depiction(&self) {
        self.eprint_depiction(&DepictionContext::new(&Theme::plain()));
    }

    /// Capture [depict](Depict::depict) into a [String].
    fn to_depiction(&self, context: &DepictionContext) -> Result<String> {
        let mut writer = Vec::with_capacity(TO_STRING_BUFFER_CAPACITY);
        self.depict(&mut writer, context)?;
        String::from_utf8(writer.into()).map_err(Error::other)
    }

    /// Capture [depict](Depict::depict) into a [String].
    ///
    /// Uses default [Theme].
    fn to_default_depiction(&self) -> Result<String> {
        self.to_depiction(&DepictionContext::new(&Theme::default()))
    }

    /// Capture [depict](Depict::depict) into a [String].
    ///
    /// Uses plain [Theme].
    fn to_plain_depiction(&self) -> Result<String> {
        self.to_depiction(&DepictionContext::new(&Theme::plain()))
    }
}
