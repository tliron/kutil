use super::super::super::{
    super::std::{foster::*, immutable::*},
    cache::*,
};

use std::{convert::*, str::*};

//
// MediaTypeSegment
//

/// [MediaType](super::media_type::MediaType) segment.
#[derive(Clone, Debug)]
pub struct MediaTypeSegment(pub FosterByteString);

delegate_newtype_of_foster_byte_string!(MediaTypeSegment);

impl CacheWeight for MediaTypeSegment {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaTypeSegment>();
        SELF_SIZE + self.0.len()
    }
}

impl FromStr for MediaTypeSegment {
    type Err = Infallible;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Ok(ByteString::from(representation).into())
    }
}
