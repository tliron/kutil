use super::{
    super::{
        super::std::{immutable::*, string::*},
        cache::*,
    },
    preferences::*,
};

use std::{convert::*, fmt, str::*};

//
// ETag
//

/// ETag value in HTTP headers.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ETag {
    /// Tag.
    pub tag: ByteString,

    /// Weak.
    pub weak: bool,
}

impl ETag {
    /// Constructor.
    pub fn new(tag: ByteString, weak: bool) -> Self {
        Self { tag, weak }
    }

    /// Parse list.
    pub fn parse_list(representation: &str) -> Option<Vec<Self>> {
        let tags: Vec<_> = representation.split(",").map(|tag| tag.parse()).flatten().collect();
        if !tags.is_empty() { Some(tags) } else { None }
    }
}

impl CacheWeight for ETag {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<ETag>();
        SELF_SIZE + self.tag.len()
    }
}

impl FromStr for ETag {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        let mut tag = representation.trim();

        if tag.ends_with("\"") {
            tag = &tag[..tag.len() - 1];
        } else {
            return Err("missing end '\"'".into());
        }

        let weak = if tag.starts_with("W/\"") {
            tag = &tag[3..];
            true
        } else if tag.starts_with("\"") {
            tag = &tag[1..];
            false
        } else {
            return Err("missing start '\"'".into());
        };

        if tag.contains("\"") {
            return Err("contains '\"'".into());
        }

        Ok(Self::new(tag.into(), weak))
    }
}

impl fmt::Display for ETag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.weak { write!(formatter, "W/\"{}\"", self.tag) } else { write!(formatter, "\"{}\"", self.tag) }
    }
}

//
// ETagList
//

/// List of [ETag].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ETagList(pub Vec<ETag>);

impl FromStr for ETagList {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        let tags: Vec<_> = representation.split(",").map(|tag| tag.parse()).flatten().collect();
        if !tags.is_empty() { Ok(Self(tags)) } else { Err("no tags".into()) }
    }
}

//
// ETagMatcher
//

/// [ETag] matcher.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ETagMatcher(pub Selector<ETagList>);

impl ETagMatcher {
    /// Whether any one of our tags matches the reference.
    ///
    /// [Any](Selector::Any) will always match. Weak tags will *never* match.
    pub fn matches(&self, reference: Option<&ETag>) -> bool {
        return match &self.0 {
            Selector::Any => true,

            Selector::Specific(selector) => {
                if let Some(reference) = reference
                    && !reference.weak
                    && selector.0.contains(&reference)
                {
                    return true;
                }

                false
            }
        };
    }
}

impl FromStr for ETagMatcher {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Ok(Self(representation.parse()?))
    }
}
