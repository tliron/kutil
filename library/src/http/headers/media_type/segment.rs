use super::super::super::super::std::{foster::*, immutable::*};

use std::{convert::*, str::*};

//
// MediaTypeSegment
//

/// [MediaType](super::media_type::MediaType) segment.
#[derive(Clone, Debug)]
pub struct MediaTypeSegment(pub FosterByteString);

delegate_newtype_of_foster_byte_string!(MediaTypeSegment);

impl FromStr for MediaTypeSegment {
    type Err = Infallible;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Ok(ByteString::from(representation).into())
    }
}
