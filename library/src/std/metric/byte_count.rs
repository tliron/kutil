use super::{super::string::*, units::*};

use {
    num_traits::cast,
    std::{fmt, num::*, str::*},
};

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
    ///
    /// Does not check for multiplication overflow!
    pub const fn from_kibibytes(kibibytes: u64) -> Self {
        Self(kibibytes * KIBI)
    }

    /// Constructor.
    ///
    /// Does not check for multiplication overflow!
    pub const fn from_mebibytes(mebibytes: u64) -> Self {
        Self(mebibytes * MEBI)
    }

    /// Constructor.
    ///
    /// Does not check for multiplication overflow!
    pub const fn from_gibibytes(gibibytes: u64) -> Self {
        Self(gibibytes * GIBI)
    }

    /// Constructor.
    ///
    /// Does not check for multiplication overflow!
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
        cast::<_, usize>(self.0).expect("byte count as usize")
        // match cast::<_, usize>(self.0) {
        //     Some(count) => count,
        //     None => 0,
        // }
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
                    Ok(integer) => {
                        let Some(count) = integer.checked_mul(unit) else {
                            return Err(format!("cannot multiply: {} * {}", integer, unit).into());
                        };

                        Ok(count.into())
                    }

                    Err(_) => {
                        let float: f64 = number.parse().map_err(|error: ParseFloatError| error.to_string())?;

                        if float >= 0.0 {
                            let Some(unit) = cast::<_, f64>(unit) else {
                                return Err(format!("cannot cast to float: {}", unit).into());
                            };

                            let count = (float * unit).round();
                            let Some(count) = cast::<_, u64>(count) else {
                                return Err(format!("cannot cast to unsigned integer: {}", count).into());
                            };

                            Ok(count.into())
                        } else {
                            Err(format!("cannot be negative: {}", float).into())
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
