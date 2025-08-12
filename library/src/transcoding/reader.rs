use super::encoding::*;

use {
    ::tokio::io::*,
    async_compression::{tokio::bufread::*, *},
    pin_project::*,
    std::{io, pin::*, task::*},
};

//
// TranscodingReader
//

/// [AsyncRead] wrapper that can encode, decode, or pass through.
#[pin_project(project = Projected)]
pub enum TranscodingReader<ReadT>
where
    ReadT: AsyncRead,
{
    /// Passthrough.
    Passthrough(#[pin] ReadT),

    /// Encode Brotli.
    EncodeBrotli(#[pin] BrotliEncoder<BufReader<ReadT>>),

    /// Decode Brotli.
    DecodeBrotli(#[pin] BrotliDecoder<BufReader<ReadT>>),

    /// Encode Deflate.
    EncodeDeflate(#[pin] DeflateEncoder<BufReader<ReadT>>),

    /// Decode Deflate.
    DecodeDeflate(#[pin] DeflateDecoder<BufReader<ReadT>>),

    /// Encode GZip.
    EncodeGZip(#[pin] GzipEncoder<BufReader<ReadT>>),

    /// Decode GZip.
    DecodeGZip(#[pin] GzipDecoder<BufReader<ReadT>>),

    /// Encode Zstandard.
    EncodeZstandard(#[pin] ZstdEncoder<BufReader<ReadT>>),

    /// Decode Zstandard.
    DecodeZstandard(#[pin] ZstdDecoder<BufReader<ReadT>>),
}

impl<ReadT> AsyncRead for TranscodingReader<ReadT>
where
    ReadT: AsyncRead,
{
    fn poll_read(self: Pin<&mut Self>, context: &mut Context<'_>, buffer: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        match self.project() {
            Projected::Passthrough(reader) => reader.poll_read(context, buffer),
            Projected::EncodeBrotli(reader) => reader.poll_read(context, buffer),
            Projected::DecodeBrotli(reader) => reader.poll_read(context, buffer),
            Projected::EncodeGZip(reader) => reader.poll_read(context, buffer),
            Projected::DecodeGZip(reader) => reader.poll_read(context, buffer),
            Projected::EncodeDeflate(reader) => reader.poll_read(context, buffer),
            Projected::DecodeDeflate(reader) => reader.poll_read(context, buffer),
            Projected::EncodeZstandard(reader) => reader.poll_read(context, buffer),
            Projected::DecodeZstandard(reader) => reader.poll_read(context, buffer),
        }
    }
}

//
// IntoTranscodingReader
//

/// Into [TranscodingReader].
pub trait IntoTranscodingReader<ReadT>
where
    ReadT: AsyncRead,
{
    /// As passthrough [TranscodingReader].
    fn into_passthrough_reader(self) -> TranscodingReader<ReadT>;

    /// As encoding [TranscodingReader].
    fn into_encoding_reader(self, encoding: &Encoding, level: Level) -> TranscodingReader<ReadT>;

    /// As decoding [TranscodingReader].
    fn into_decoding_reader(self, encoding: &Encoding) -> TranscodingReader<ReadT>;
}

impl<ReadT> TranscodingReader<ReadT>
where
    ReadT: AsyncRead,
{
    /// Inner reader.
    pub fn inner(&self) -> &ReadT {
        match self {
            Self::Passthrough(reader) => reader,
            Self::EncodeBrotli(reader) => reader.get_ref().get_ref(),
            Self::DecodeBrotli(reader) => reader.get_ref().get_ref(),
            Self::EncodeDeflate(reader) => reader.get_ref().get_ref(),
            Self::DecodeDeflate(reader) => reader.get_ref().get_ref(),
            Self::EncodeGZip(reader) => reader.get_ref().get_ref(),
            Self::DecodeGZip(reader) => reader.get_ref().get_ref(),
            Self::EncodeZstandard(reader) => reader.get_ref().get_ref(),
            Self::DecodeZstandard(reader) => reader.get_ref().get_ref(),
        }
    }
}

impl<ReadT> IntoTranscodingReader<ReadT> for ReadT
where
    ReadT: AsyncRead,
{
    fn into_passthrough_reader(self) -> TranscodingReader<ReadT> {
        TranscodingReader::Passthrough(self)
    }

    fn into_encoding_reader(self, encoding: &Encoding, level: Level) -> TranscodingReader<ReadT> {
        if *encoding == Encoding::Identity {
            tracing::debug!("not encoding");
        } else {
            tracing::debug!("encoding to {}", encoding);
        }

        match encoding {
            Encoding::Identity => self.into_passthrough_reader(),

            Encoding::Brotli => {
                TranscodingReader::EncodeBrotli(BrotliEncoder::with_quality(BufReader::new(self), level))
            }

            Encoding::Deflate => {
                TranscodingReader::EncodeDeflate(DeflateEncoder::with_quality(BufReader::new(self), level))
            }

            Encoding::GZip => TranscodingReader::EncodeGZip(GzipEncoder::with_quality(BufReader::new(self), level)),

            Encoding::Zstandard => {
                TranscodingReader::EncodeZstandard(ZstdEncoder::with_quality(BufReader::new(self), level))
            }
        }
    }

    fn into_decoding_reader(self, encoding: &Encoding) -> TranscodingReader<ReadT> {
        if *encoding == Encoding::Identity {
            tracing::debug!("not decoding");
        } else {
            tracing::debug!("decoding from {}", encoding);
        }

        match encoding {
            Encoding::Identity => self.into_passthrough_reader(),
            Encoding::Brotli => TranscodingReader::DecodeBrotli(BrotliDecoder::new(BufReader::new(self))),
            Encoding::Deflate => TranscodingReader::DecodeDeflate(DeflateDecoder::new(BufReader::new(self))),
            Encoding::GZip => TranscodingReader::DecodeGZip(GzipDecoder::new(BufReader::new(self))),
            Encoding::Zstandard => TranscodingReader::DecodeZstandard(ZstdDecoder::new(BufReader::new(self))),
        }
    }
}
