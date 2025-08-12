use super::{super::string::*, units::*};

use std::{fmt, num::*, str::*};

//
// ByteCount
//

/// Parsed byte count.
#[derive(Clone, Copy, Debug, Default)]
pub struct ByteCount(pub u64);

impl ByteCount {
    /// Constructor.
    pub const fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Constructor.
    pub const fn from_kibibytes(kibibytes: u64) -> Self {
        Self(kibibytes * KIBI)
    }

    /// Constructor.
    pub const fn from_mebibytes(mebibytes: u64) -> Self {
        Self(mebibytes * MEBI)
    }

    /// Constructor.
    pub const fn from_gibibytes(gibibytes: u64) -> Self {
        Self(gibibytes * GIBI)
    }

    /// Constructor.
    pub const fn from_tebibytes(tebibytes: u64) -> Self {
        Self(tebibytes * TEBI)
    }

    fn split(representation: &str) -> Option<(&str, &str)> {
        for (index, c) in representation.char_indices().rev() {
            if c == ' ' {
                return Some((&representation[..index], &representation[index + 1..]));
            }

            if c.is_ascii_digit() {
                let index = index + 1;
                return Some((&representation[..index], &representation[index..]));
            }
        }

        None
    }
}

impl From<u64> for ByteCount {
    fn from(bytes: u64) -> Self {
        Self(bytes)
    }
}

impl Into<u64> for ByteCount {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<usize> for ByteCount {
    fn into(self) -> usize {
        // TODO: casting errors
        self.0 as usize
    }
}

impl FromStr for ByteCount {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match Self::split(representation) {
            Some((number, unit)) => {
                let unit = parse_metric_unit(unit)?;

                let integer: Result<u64, _> = number.parse();
                match integer {
                    Ok(integer) => Ok(Self(integer * unit)),

                    Err(_) => {
                        let float: f64 = number.parse().map_err(|error: ParseFloatError| error.to_string())?;

                        if float >= 0.0 {
                            // TODO: casting errors
                            let unit = unit as f64;
                            Ok(Self((float * unit).round() as u64))
                        } else {
                            Err("cannot be negative".into())
                        }
                    }
                }
            }

            None => Ok(Self(representation.parse().map_err(|error: ParseIntError| error.to_string())?)),
        }
    }
}

impl fmt::Display for ByteCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} bytes", self.0)
    }
}
