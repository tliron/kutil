use super::{
    super::super::{
        std::{collections::*, immutable::*},
        transcoding::{transcode::*, *},
    },
    configuration::*,
    weight::*,
};

use std::io;

//
// CachedBody
//

/// Cached HTTP response body.
#[derive(Clone, Debug, Default)]
pub struct CachedBody {
    /// Representations.
    pub representations: FastHashMap<Encoding, Bytes>,
}

impl CachedBody {
    /// Constructor with an initial representation.
    ///
    /// If the `preferred_encoding` is different from the `encoding` then we will reencode.
    ///
    /// If an [Identity](Encoding::Identity) is created during this reencoding then it will also be
    /// stored if `keep_identity_encoding` is true.
    pub async fn new_with(
        bytes: Bytes,
        encoding: Encoding,
        preferred_encoding: Encoding,
        configuration: &EncodingConfiguration,
    ) -> io::Result<Self> {
        let mut representations = FastHashMap::default();

        if preferred_encoding == encoding {
            // It's already in the preferred encoding
            representations.insert(preferred_encoding, bytes);
        } else if encoding == Encoding::Identity {
            tracing::debug!("encoding to {}", preferred_encoding);

            let encoded_bytes = bytes.encode(&preferred_encoding).await?;

            representations.insert(preferred_encoding, encoded_bytes);
            if configuration.keep_identity_encoding {
                representations.insert(Encoding::Identity, bytes);
            }
        } else if preferred_encoding == Encoding::Identity {
            tracing::debug!("decoding from {}", encoding);

            let identity_bytes = bytes.decode(&encoding).await?;

            representations.insert(Encoding::Identity, identity_bytes);
        } else {
            tracing::debug!("reencoding from {} to {}", encoding, preferred_encoding);

            let identity_bytes = bytes.decode(&encoding).await?;
            let encoded_bytes = identity_bytes.encode(&preferred_encoding).await?;

            representations.insert(preferred_encoding, encoded_bytes);
            if configuration.keep_identity_encoding {
                representations.insert(Encoding::Identity, identity_bytes);
            }
        }

        Ok(Self { representations })
    }

    /// Returns the body [Bytes] in the specified encoding.
    ///
    /// If we don't have the specified encoding then we will reencode from another encoding,
    /// storing the result so that we won't have to encode it again.
    ///
    /// If an [Identity](Encoding::Identity) is created during this reencoding then it will also be
    /// stored if `keep_identity_encoding` is true.
    ///
    /// Returns a modified clone if reencoding caused a new encoding to be stored. Note that
    /// cloning should be cheap due to our use of [Bytes].
    pub async fn get(
        &self,
        encoding: &Encoding,
        configuration: &EncodingConfiguration,
    ) -> io::Result<(Bytes, Option<Self>)> {
        match (self.representations.get(encoding), encoding) {
            (Some(bytes), _) => Ok((bytes.clone(), None)),

            (None, Encoding::Identity) => {
                // Decode
                for from_encoding in ENCODINGS_BY_DECODING_COST {
                    if let Some(bytes) = self.representations.get(from_encoding) {
                        tracing::debug!("decoding from {}", from_encoding);

                        let identity_bytes = bytes.decode(from_encoding).await?;

                        let mut modified = self.clone();
                        modified.representations.insert(Encoding::Identity, identity_bytes.clone());

                        return Ok((identity_bytes, Some(modified)));
                    }
                }

                // This should never happen (but we don't want to panic here!)
                tracing::error!("no encodings");
                Ok((Default::default(), None))
            }

            (None, to_encoding) => {
                // Reencode
                if let Some(identity_bytes) = self.representations.get(&Encoding::Identity) {
                    tracing::debug!("encoding to {}", to_encoding);

                    let bytes = identity_bytes.encode(to_encoding).await?;

                    let mut modified = self.clone();
                    modified.representations.insert(to_encoding.clone(), bytes.clone());

                    Ok((bytes, Some(modified)))
                } else {
                    for from_encoding in ENCODINGS_BY_DECODING_COST {
                        if let Some(bytes) = self.representations.get(from_encoding) {
                            tracing::debug!("reencoding from {} to {}", from_encoding, to_encoding);

                            let identity_bytes = bytes.decode(from_encoding).await?;
                            let bytes = identity_bytes.encode(to_encoding).await?;

                            let mut modified = self.clone();
                            if configuration.keep_identity_encoding {
                                modified.representations.insert(Encoding::Identity, identity_bytes);
                            }
                            modified.representations.insert(to_encoding.clone(), bytes.clone());

                            return Ok((bytes, Some(modified)));
                        }
                    }

                    // This should never happen (but we don't want to panic here!)
                    tracing::error!("no encodings");
                    Ok((Default::default(), None))
                }
            }
        }
    }
}

impl CacheWeight for CachedBody {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<CachedBody>();
        const ENTRY_SIZE: usize = size_of::<Encoding>() + size_of::<Bytes>();

        let mut size = SELF_SIZE;

        for bytes in self.representations.values() {
            size += ENTRY_SIZE + bytes.len();
        }

        size
    }
}
