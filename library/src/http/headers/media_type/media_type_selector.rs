use super::{
    super::{
        super::{
            super::std::{borrow::*, foster::*, string::*},
            cache::*,
        },
        preferences::*,
    },
    media_type::*,
    segment::*,
};

use std::{cmp::*, convert::*, fmt, hash::*, str::*};

//
// MediaTypeSelector
//

/// [MediaType](super::media_type::MediaType) selector.
///
/// Either of the segments can be [Any](Selector::Any). However, if `main` is [Any](Selector::Any),
/// `subtype` must also be [Any](Selector::Any). Use [Self::is_valid] to check for this.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MediaTypeSelector {
    /// Main segment.
    pub main: Selector<MediaTypeSegment>,

    /// Subtype segment.
    pub subtype: Selector<MediaTypeSegment>,
}

impl MediaTypeSelector {
    /// Any.
    pub const ANY: Self = Self::new(Selector::Any, Selector::Any);

    /// Constructor.
    pub const fn new(main: Selector<MediaTypeSegment>, subtype: Selector<MediaTypeSegment>) -> Self {
        Self { main, subtype }
    }

    /// Constructor.
    pub fn new_owned(main: String, subtype: String) -> Self {
        Self::new(Selector::Specific(main.into()), Selector::Specific(subtype.into()))
    }

    /// Constructor.
    pub const fn new_fostered(main: &'static str, subtype: &'static str) -> Self {
        Self::new(
            Selector::Specific(MediaTypeSegment::new_fostered(main)),
            Selector::Specific(MediaTypeSegment::new_fostered(subtype)),
        )
    }

    /// Whether we are valid.
    ///
    /// If `main` is [Any](Selector::Any), `subtype` must also be [Any](Selector::Any).
    pub fn is_valid(&self) -> bool {
        self.main.is_specific() || !self.subtype.is_specific()
    }
}

impl IsSpecific for MediaTypeSelector {
    fn is_specific(&self) -> bool {
        self.main.is_specific() && self.subtype.is_specific()
    }
}

impl IntoOwned for MediaTypeSelector {
    fn into_owned(self) -> Self {
        match self.main {
            Selector::Any | Selector::Specific(MediaTypeSegment(Foster::Owned(_))) => match self.subtype {
                Selector::Any | Selector::Specific(MediaTypeSegment(Foster::Owned(_))) => {
                    // Both main and subtype are already owned
                    self
                }

                Selector::Specific(MediaTypeSegment(Foster::Fostered(subtype))) => {
                    // Main is owned, subtype isn't
                    Self::new(self.main, Selector::Specific(subtype.into()))
                }
            },

            Selector::Specific(MediaTypeSegment(Foster::Fostered(main))) => match self.subtype {
                Selector::Any | Selector::Specific(MediaTypeSegment(Foster::Owned(_))) => {
                    // Subtype is owned, main isn't
                    Self::new(Selector::Specific(main.into()), self.subtype)
                }

                Selector::Specific(MediaTypeSegment(Foster::Fostered(subtype))) => {
                    // Both are not owned
                    Self::new_owned(main.into(), subtype.into())
                }
            },
        }
    }
}

impl CacheWeight for MediaTypeSelector {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaTypeSelector>();
        SELF_SIZE + self.main.cache_weight() + self.subtype.cache_weight()
    }
}

impl From<MediaType> for MediaTypeSelector {
    fn from(media_type: MediaType) -> Self {
        Self::new(media_type.main.into(), media_type.subtype.into())
    }
}

impl PartialEq<MediaType> for MediaTypeSelector {
    fn eq(&self, other: &MediaType) -> bool {
        (self.main == other.main) && (self.subtype == other.subtype)
    }
}

impl FromStr for MediaTypeSelector {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match representation.split_once("/") {
            Some((main, subtype)) => {
                // Note: allowed because parse can be Infallible!
                let Ok(main) = main.parse();
                let Ok(subtype) = subtype.parse();
                Ok(Self::new(main, subtype))
            }

            None => Err("missing '/'".into()),
        }
    }
}

impl fmt::Display for MediaTypeSelector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.main, self.subtype)
    }
}
