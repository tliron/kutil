use super::{super::std::immutable::*, encoding::*, transcode::*};

use {
    async_compression::tokio::{bufread, write},
    std::io,
    tokio::io::*,
};

impl Transcode for Bytes {
    async fn encode(&self, encoding: &Encoding) -> io::Result<Self> {
        match encoding {
            Encoding::Identity => Ok(self.clone()),

            Encoding::Brotli => {
                let mut encoder = write::BrotliEncoder::new(Vec::default());
                encoder.write_all(self).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }

            Encoding::Deflate => {
                let mut encoder = write::DeflateEncoder::new(Vec::default());
                encoder.write_all(self).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }

            Encoding::GZip => {
                let mut encoder = write::GzipEncoder::new(Vec::default());
                encoder.write_all(self).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }

            Encoding::Zstandard => {
                let mut encoder = write::ZstdEncoder::new(Vec::default());
                encoder.write_all(self).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }
        }
    }

    async fn decode(&self, encoding: &Encoding) -> io::Result<Self> {
        match encoding {
            Encoding::Identity => Ok(self.clone()),

            Encoding::Brotli => {
                let mut decoder = bufread::BrotliDecoder::new(BufReader::new(self.as_ref()));
                let mut buffer = Vec::default();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }

            Encoding::Deflate => {
                let mut decoder = bufread::DeflateDecoder::new(BufReader::new(self.as_ref()));
                let mut buffer = Vec::default();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }

            Encoding::GZip => {
                let mut decoder = bufread::GzipDecoder::new(BufReader::new(self.as_ref()));
                let mut buffer = Vec::default();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }

            Encoding::Zstandard => {
                let mut decoder = bufread::ZstdDecoder::new(BufReader::new(self.as_ref()));
                let mut buffer = Vec::default();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }
        }
    }
}
