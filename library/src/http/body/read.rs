use super::{
    super::{
        super::{
            io::reader::*,
            std::{error::*, immutable::*},
        },
        pieces::*,
    },
    reader::*,
};

use {
    http::*,
    http_body::*,
    std::{io, result::Result, string::*},
    thiserror::*,
    tokio::io::*,
};

//
// ReadBodyIntoBytes
//

/// Read [Body] into [Bytes].
///
/// See also [BodyReader].
#[allow(async_fn_in_trait)]
pub trait ReadBodyIntoBytes
where
    Self: Sized,
{
    /// Read entire [Body] into [Bytes] and trailers.
    ///
    /// If `declared_size` is not [None] then that's the size we expect. Otherwise
    /// we'll try to read up to `max_size` and will expect at least `min_size`.
    ///
    /// If we read less than `min_size` *or* we did not read all the way to EOF will return a
    /// [ReadBodyError] with [FileTooLarge](io::ErrorKind::FileTooLarge) and [BodyPieces], the
    /// latter of which can be used by the caller to reconstruct the original body, e.g. with
    /// [BodyReader::new_with_first_bytes](super::reader::BodyReader::new_with_first_bytes).
    async fn read_into_bytes_or_pieces(
        self,
        declared_size: Option<usize>,
        min_size: usize,
        max_size: usize,
    ) -> Result<(Bytes, Vec<HeaderMap>), ErrorWithBodyPieces<ReadBodyError, Self>>;

    /// Read entire [Body] into [Bytes] and trailers.
    ///
    /// If we we did not read all the way to EOF will return a
    /// [FileTooLarge](io::ErrorKind::FileTooLarge) error.
    async fn read_into_bytes(self, max_size: usize) -> Result<(Bytes, Vec<HeaderMap>), ReadBodyError> {
        self.read_into_bytes_or_pieces(None, 0, max_size).await.map_err(|error| error.error)
    }

    /// Read entire [Body] into [String] and trailers.
    ///
    /// See [read_into_bytes](ReadBodyIntoBytes::read_into_bytes).
    async fn read_into_string_or_pieces(
        self,
        declared_size: Option<usize>,
        min_size: usize,
        max_size: usize,
    ) -> Result<(String, Vec<HeaderMap>), ErrorWithBodyPieces<ReadBodyError, Self>> {
        let (bytes, trailers) = self.read_into_bytes_or_pieces(declared_size, min_size, max_size).await?;
        let string =
            String::from_utf8(bytes.to_vec()).map_err(|error| ErrorWithBodyPieces::from(ReadBodyError::from(error)))?;
        Ok((string, trailers))
    }

    /// Read entire [Body] into [String] and trailers.
    ///
    /// If we we did not read all the way to EOF will return a
    /// [FileTooLarge](io::ErrorKind::FileTooLarge) error.
    async fn read_into_string(self, max_size: usize) -> Result<(String, Vec<HeaderMap>), ReadBodyError> {
        self.read_into_string_or_pieces(None, 0, max_size).await.map_err(|error| error.error)
    }
}

impl<BodyT> ReadBodyIntoBytes for BodyT
where
    BodyT: Body + Unpin,
    BodyT::Error: Into<CapturedError>,
{
    async fn read_into_bytes_or_pieces(
        self,
        declared_size: Option<usize>,
        min_size: usize,
        max_size: usize,
    ) -> Result<(Bytes, Vec<HeaderMap>), ErrorWithBodyPieces<ReadBodyError, Self>> {
        assert!(max_size >= min_size);

        let read_size = match declared_size {
            Some(declared_size) => {
                assert!(declared_size >= min_size);
                assert!(declared_size <= max_size);
                declared_size
            }

            None => max_size,
        };

        let reader = self.into_reader();

        let mut bytes = BytesMut::with_capacity(read_size);
        let (mut reader, _size) = read_at_most(reader, &mut bytes, read_size as u64)
            .await
            .map_err(|error| ErrorWithBodyPieces::from(ReadBodyError::from(error)))?;

        // We'll try to read just one more byte to see if we're complete
        match reader.read_u8().await {
            Ok(byte) => {
                println!("!!!!!!!!!!!!!!!!! {:?} {}", read_size, bytes.len());
                let (body, remainder, _trailers) = reader.into_inner();

                // Push back the byte we read and the remainder
                bytes.put_u8(byte);
                bytes.put(remainder);

                return Err(ErrorWithBodyPieces::new(
                    io::Error::new(io::ErrorKind::FileTooLarge, format!("body is bigger than {}", read_size)).into(),
                    Some(BodyPieces::new(body, bytes.into())),
                ));
            }

            Err(error) => {
                // Actually, we *do* expect EOF :)
                if error.kind() != io::ErrorKind::UnexpectedEof {
                    let (body, remainder, _trailers) = reader.into_inner();
                    bytes.put(remainder); // remainder *should* be empty
                    return Err(ErrorWithBodyPieces::new(error.into(), Some(BodyPieces::new(body, bytes.into()))));
                }
            }
        }

        let fulfilled_size = bytes.len();

        if let Some(declared_size) = declared_size
            && declared_size != fulfilled_size
        {
            // The declared size is wrong, but that's not in itself an error
            tracing::warn!("declared size is {} but actual body size is {}", declared_size, fulfilled_size);
        }

        if fulfilled_size < min_size {
            let (body, remainder, _trailers) = reader.into_inner();
            bytes.put(remainder); // remainder *should* be empty
            return Err(ErrorWithBodyPieces::new(
                io::Error::new(
                    io::ErrorKind::FileTooLarge,
                    format!("body is too big: {} > {}", fulfilled_size, min_size),
                )
                .into(),
                Some(BodyPieces::new(body, bytes.into())),
            ));
        }

        let (_body, remainder, trailers) = reader.into_inner();
        bytes.put(remainder); // remainder *should* be empty
        Ok((bytes.into(), trailers))
    }
}

//
// ReadBodyError
//

/// [ReadBodyIntoBytes] error.
#[derive(Debug, Error)]
pub enum ReadBodyError {
    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// UTF8.
    #[error("UTF8: {0}")]
    UTF8(#[from] FromUtf8Error),
}
