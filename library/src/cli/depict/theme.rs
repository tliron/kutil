use {
    owo_colors::*,
    std::{fmt, io, sync::*},
};

/// Default [Theme].
pub static DEFAULT_THEME: LazyLock<Theme> = LazyLock::new(|| Theme::default());

/// Plain [Theme].
pub static PLAIN_THEME: LazyLock<Theme> = LazyLock::new(|| Theme::plain());

//
// Theme
//

/// Collection of [Style]s suitable for terminals.
///
/// See [Depict](super::depict::Depict).
#[derive(Clone, Debug)]
pub struct Theme {
    /// For symbols: true, false, null, None, etc.
    pub symbol_style: Style,

    /// For numbers.
    pub number_style: Style,

    /// For strings and characters.
    pub string_style: Style,

    /// For names of types, instances, etc.
    pub name_style: Style,

    /// For metadata.
    pub meta_style: Style,

    /// For errors.
    pub error_style: Style,

    /// For headings.
    pub heading_style: Style,

    /// For delimiters.
    pub delimiter_style: Style,
}

impl Theme {
    /// Plain theme.
    pub fn plain() -> Self {
        Self {
            symbol_style: Default::default(),
            number_style: Default::default(),
            string_style: Default::default(),
            name_style: Default::default(),
            meta_style: Default::default(),
            error_style: Default::default(),
            heading_style: Default::default(),
            delimiter_style: Default::default(),
        }
    }

    /// Apply symbol style.
    pub fn symbol<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.symbol_style.style(thing)
    }

    /// Apply number style.
    pub fn number<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.number_style.style(thing)
    }

    /// Apply string style.
    pub fn string<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.string_style.style(thing)
    }

    /// Apply name style.
    pub fn name<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.name_style.style(thing)
    }

    /// Apply meta style.
    pub fn meta<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.meta_style.style(thing)
    }

    /// Apply error style.
    pub fn error<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.error_style.style(thing)
    }

    /// Apply heading style.
    pub fn heading<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.heading_style.style(thing)
    }

    /// Apply delimiter style.
    pub fn delimiter<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.delimiter_style.style(thing)
    }

    /// Write [fmt::Display] in symbol style.
    pub fn write_symbol<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.symbol(thing))
    }

    /// Write [fmt::Display] in number style.
    pub fn write_number<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.number(thing))
    }

    /// Write [fmt::Display] in string style.
    pub fn write_string<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.string(thing))
    }

    /// Write [fmt::Display] in name style.
    pub fn write_name<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.name(thing))
    }

    /// Write [fmt::Display] in meta style.
    pub fn write_meta<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.meta(thing))
    }

    /// Write [fmt::Display] in error style.
    pub fn write_error<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.error(thing))
    }

    /// Write [fmt::Display] in heading style.
    pub fn write_heading<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.heading(thing))
    }

    /// Write [fmt::Display] in delimiter style.
    pub fn write_delimiter<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.delimiter(thing))
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            symbol_style: Style::default().yellow(),
            number_style: Style::default().magenta(),
            string_style: Style::default().cyan(),
            name_style: Style::default().green(),
            meta_style: Style::default().blue().italic(),
            error_style: Style::default().red().bold(),
            heading_style: Style::default().green().bold().underline(),
            delimiter_style: Style::default().dimmed(),
        }
    }
}
