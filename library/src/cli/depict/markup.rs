use super::theme::*;

use {
    owo_colors::*,
    std::{fmt, io},
};

/// Depiction markup delimiter.
pub const MARKUP_DELIMITER: char = '|';

/// Depiction markup escape character.
pub const MARKUP_ESCAPE: char = '\\';

/// Depiction markup escaped delimiter.
pub const MARKUP_ESCAPED_DELIMITER: &str = "\\|";

/// If theme is [Some] will return a depiction, otherwise will remove the markup.
///
/// The markup notation is simple. Use `|style|text to style|` to depict the text with the style.
///
/// Supported styles are: `symbol`, `number`, `string`, `name`, `meta`, `error`, `heading`, and
/// `delimiter`.
///
/// Use `\|` to escape the `|` delimiter. [escape_depiction_markup] will do this for you.
pub fn depict_markup<ThingT>(thing: ThingT, theme: Option<&Theme>) -> String
where
    ThingT: fmt::Display,
{
    DepictionMarkupParser::new(thing.to_string().chars().collect()).depict(theme)
}

/// Escape depiction markup.
///
/// Replaces all `|` with `\|`.
///
/// See [depict_markup].
pub fn escape_depiction_markup<ThingT>(thing: ThingT) -> String
where
    ThingT: fmt::Display,
{
    thing.to_string().replace(MARKUP_DELIMITER, MARKUP_ESCAPED_DELIMITER)
}

/// Write depiction from markup.
///
/// See [depict_markup].
pub fn write_depiction_markup<WriteT, ThingT>(
    writer: &mut WriteT,
    thing: ThingT,
    theme: Option<&Theme>,
) -> io::Result<()>
where
    WriteT: io::Write,
    ThingT: fmt::Display,
{
    write!(writer, "{}", depict_markup(thing, theme))
}

/// Print depiction from markup to [anstream::stdout].
///
/// See [depict_markup].
pub fn print_depiction_markup<ThingT>(thing: ThingT, theme: Option<&Theme>)
where
    ThingT: fmt::Display,
{
    write_depiction_markup(&mut anstream::stdout(), thing, theme).expect("writing to stdout");
}

/// Print depiction from markup to [anstream::stderr].
///
/// See [depict_markup].
pub fn eprint_depiction_markup<ThingT>(thing: ThingT, theme: Option<&Theme>)
where
    ThingT: fmt::Display,
{
    write_depiction_markup(&mut anstream::stderr(), thing, theme).expect("writing to stderr");
}

impl Theme {
    /// Depict markup.
    ///
    /// See [depict_markup].
    pub fn depict_markup<ThingT>(&self, thing: ThingT) -> String
    where
        ThingT: fmt::Display,
    {
        depict_markup(thing, Some(self))
    }

    /// Write depiction from markup.
    ///
    /// See [depict_markup].
    pub fn write_depiction_markup<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write_depiction_markup(writer, thing, Some(self))
    }

    /// Print depiction from markup to [anstream::stdout].
    ///
    /// See [depict_markup].
    pub fn print_depiction_markup<ThingT>(&self, thing: ThingT)
    where
        ThingT: fmt::Display,
    {
        print_depiction_markup(thing, Some(self));
    }

    /// Print depiction from markup to [anstream::stderr].
    ///
    /// See [depict_markup].
    pub fn eprint_depiction_markup<ThingT>(&self, thing: ThingT)
    where
        ThingT: fmt::Display,
    {
        eprint_depiction_markup(thing, Some(self));
    }
}

//
// DepictionMarkupParser
//

/// Depiction markup parser.
struct DepictionMarkupParser {
    characters: Vec<char>,
    index: usize,
    start: usize,
    mode: Mode,
    result: String,
}

impl DepictionMarkupParser {
    /// Constructor.
    fn new(characters: Vec<char>) -> Self {
        Self { characters, index: 0, start: 0, mode: Mode::Unmarked, result: Default::default() }
    }

    /// Depict.
    ///
    /// See [depict_markup].
    fn depict(mut self, theme: Option<&Theme>) -> String {
        let length = self.characters.len();
        while self.index < length {
            let c = self.characters[self.index];

            match c {
                MARKUP_DELIMITER => {
                    match theme {
                        Some(theme) => match self.mode {
                            Mode::Unmarked => self.mode = Mode::Style,
                            Mode::Style => self.mode = (&self.characters[self.start..self.index]).into(),
                            Mode::Symbol => self.append_styled(&theme.symbol_style),
                            Mode::Number => self.append_styled(&theme.number_style),
                            Mode::String => self.append_styled(&theme.string_style),
                            Mode::Name => self.append_styled(&theme.name_style),
                            Mode::Meta => self.append_styled(&theme.meta_style),
                            Mode::Error => self.append_styled(&theme.error_style),
                            Mode::Heading => self.append_styled(&theme.heading_style),
                            Mode::Delimiter => self.append_styled(&theme.delimiter_style),
                            _ => self.append(),
                        },

                        None => match self.mode {
                            Mode::Unmarked => self.mode = Mode::Style,
                            Mode::Style => self.mode = Mode::Plain,
                            _ => self.append(),
                        },
                    }

                    self.start = self.index + 1;
                    self.index += 1;
                }

                MARKUP_ESCAPE => {
                    if self.index < length - 1 {
                        let next = self.characters[self.index + 1];
                        if next == MARKUP_DELIMITER {
                            // Escaped delimiter
                            self.result.push(MARKUP_DELIMITER);
                            self.index += 2;
                        } else {
                            // Normal character
                            self.result.push(MARKUP_ESCAPE);
                            self.index += 1;
                        }
                    } else {
                        // Normal character
                        self.result.push(MARKUP_ESCAPE);
                        self.index += 1;
                    }
                }

                _ => {
                    if matches!(self.mode, Mode::Unmarked) {
                        self.result.push(c);
                    }

                    self.index += 1;
                }
            }
        }

        self.result
    }

    fn append(&mut self) {
        let start = self.start;
        let end = self.index;

        if end > start {
            self.result.reserve(end - start + 1);
            for c in &self.characters[start..end] {
                self.result.push(*c);
            }
        }

        self.mode = Mode::Unmarked;
    }

    fn append_styled(&mut self, style: &Style) {
        let start = self.start;
        let end = self.index;

        if end > start {
            let styled: String = self.characters[start..end].iter().collect();
            self.result.push_str(&style.style(styled).to_string());
        }

        self.mode = Mode::Unmarked;
    }
}

//
// Mode
//

#[derive(Clone, Copy, Debug, Default)]
enum Mode {
    // Outside of markup
    #[default]
    Unmarked,

    // Inside "style" segment: |style|text to style|
    Style,

    // Inside "text to style" segment: |style|text to style|
    Symbol,
    Number,
    String,
    Name,
    Meta,
    Error,
    Heading,
    Delimiter,
    Plain, // no styling
}

impl From<&[char]> for Mode {
    fn from(value: &[char]) -> Self {
        match value {
            ['s', 'y', 'm', 'b', 'o', 'l'] => Mode::Symbol,
            ['n', 'u', 'm', 'b', 'e', 'r'] => Mode::Number,
            ['s', 't', 'r', 'i', 'n', 'g'] => Mode::String,
            ['n', 'a', 'm', 'e'] => Mode::Name,
            ['m', 'e', 't', 'a'] => Mode::Meta,
            ['e', 'r', 'r', 'o', 'r'] => Mode::Error,
            ['h', 'e', 'a', 'd', 'i', 'n', 'g'] => Mode::Heading,
            ['d', 'e', 'l', 'i', 'm', 'i', 't', 'e', 'r'] => Mode::Delimiter,
            _ => Mode::Plain,
        }
    }
}
