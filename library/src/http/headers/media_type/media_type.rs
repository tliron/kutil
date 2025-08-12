use super::{
    super::super::{
        super::std::{borrow::*, foster::*, string::*},
        cache::*,
    },
    segment::*,
};

use {
    http::header::*,
    std::{cmp::*, convert::*, fmt, hash::*, str::*},
};

//
// MediaType
//

/// Media type value in HTTP headers.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MediaType {
    /// Main segment.
    pub main: MediaTypeSegment,

    /// Subtype segment.
    pub subtype: MediaTypeSegment,
}

impl MediaType {
    /// Constructor.
    pub const fn new(main: MediaTypeSegment, subtype: MediaTypeSegment) -> Self {
        Self { main, subtype }
    }

    /// Constructor.
    pub fn new_owned(main: String, subtype: String) -> Self {
        Self::new(main.into(), subtype.into())
    }

    /// Constructor.
    pub const fn new_fostered(main: &'static str, subtype: &'static str) -> Self {
        Self::new(MediaTypeSegment::new_fostered(main), MediaTypeSegment::new_fostered(subtype))
    }
}

impl IntoOwned for MediaType {
    fn into_owned(self) -> Self {
        match self.main {
            MediaTypeSegment(Foster::Owned(_)) => match self.subtype {
                MediaTypeSegment(Foster::Owned(_)) => {
                    // Both main and subtype are already owned
                    self
                }

                MediaTypeSegment(Foster::Fostered(subtype)) => {
                    // Main is owned, subtype isn't
                    Self::new(self.main, subtype.into())
                }
            },

            MediaTypeSegment(Foster::Fostered(main)) => match self.subtype {
                MediaTypeSegment(Foster::Owned(_)) => {
                    // Subtype is owned, main isn't
                    Self::new(main.into(), self.subtype)
                }

                MediaTypeSegment(Foster::Fostered(subtype)) => {
                    // Both are not owned
                    Self::new_owned(main.into(), subtype.into())
                }
            },
        }
    }
}

impl CacheWeight for MediaType {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaType>();
        SELF_SIZE + self.main.cache_weight() + self.subtype.cache_weight()
    }
}

impl Into<HeaderValue> for MediaType {
    fn into(self) -> HeaderValue {
        HeaderValue::from_str(&self.to_string()).expect("media type in HTTP header")
    }
}

impl FromStr for MediaType {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        let (main, subtype) = representation.split_once("/").ok_or_else(|| "missing '/'")?;
        let Ok(main) = main.parse();
        let Ok(subtype) = subtype.parse();
        Ok(Self::new(main, subtype))
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.main, self.subtype)
    }
}
