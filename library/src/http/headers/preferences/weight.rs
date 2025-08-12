use std::fmt;

//
// Weight
//

/// Weight ("q") value in an HTTP header. Used to specify quality, priority, etc.
///
/// See [IETF RFC 7231 section 5.3.1](https://datatracker.ietf.org/doc/html/rfc7231#section-5.3.1).
///
/// Stored as an integer value from 0 to 1000. We use an integer rather than a float in order
/// to avoid comparison issues.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Weight(u16);

impl Weight {
    /// Max [Weight] (1000).
    pub const MAX: Weight = Weight(1000);

    /// Constructor.
    pub const fn new(weight: u16) -> Self {
        Self(weight)
    }

    /// Parse.
    pub fn parse(representation: &str) -> Option<Self> {
        const MAX: u16 = 1000;

        // Based on:
        // https://github.com/tower-rs/tower-http/blob/main/tower-http/src/content_encoding.rs

        let mut chars = representation.trim().chars();

        // Parse "q=" (case-insensitively).
        match chars.next() {
            Some('q' | 'Q') => (),
            _ => return None,
        };

        match chars.next() {
            Some('=') => (),
            _ => return None,
        };

        // Parse leading digit. Since valid q-values are between 0.000 and 1.000, only "0" and "1"
        // are allowed.
        let mut value = match chars.next() {
            Some('0') => 0,
            Some('1') => MAX,
            _ => return None,
        };

        // Parse optional decimal point.
        match chars.next() {
            Some('.') => {}
            None => return Some(Self(value)),
            _ => return None,
        };

        // Parse optional fractional digits. The value of each digit is multiplied by `factor`.
        // Since the q-value is represented as an integer between 0 and 1000, `factor` is `100` for
        // the first digit, `10` for the next, and `1` for the digit after that.
        let mut factor = 100;
        loop {
            match chars.next() {
                Some(n @ '0'..='9') => {
                    // If `factor` is less than `1`, three digits have already been parsed. A
                    // q-value having more than 3 fractional digits is invalid.
                    if factor < 1 {
                        return None;
                    }

                    // Add the digit's value multiplied by `factor` to `value`.
                    value += factor * (n as u16 - '0' as u16);
                }

                None => {
                    // No more characters to parse. Check that the value representing the q-value is
                    // in the valid range.
                    return if value <= MAX { Some(Self(value)) } else { None };
                }

                _ => return None,
            };

            factor /= 10;
        }
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            1000 => fmt::Display::fmt("q=1", formatter),

            0 => fmt::Display::fmt("q=0", formatter),

            mut weight => {
                if weight % 100 == 0 {
                    // e.g. .800 -> .8
                    weight /= 100;
                } else if weight % 10 == 0 {
                    // e.g. .830 -> .83
                    weight /= 10;
                }

                write!(formatter, "q=0.{}", weight)
            }
        }
    }
}
