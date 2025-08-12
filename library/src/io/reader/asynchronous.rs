use super::super::super::std::immutable::*;

use {std::io, tokio::io::*};

/// Read at most a maximum number of bytes and return the reader and number of bytes read.
///
/// We'll stop reading if:
///
/// * We've read the maximum number of bytes
/// * The buffer runs out of capacity
/// * The reader runs out of bytes
///
/// For the first two conditions the reader may still have bytes available to be read.
///
/// Note that running out of bytes is the third success condition, not an error. Thus this function
/// will never return an [UnexpectedEof](io::ErrorKind::UnexpectedEof). In other words, an EOF is
/// expected in this case.
pub async fn read_at_most<ReadT, BufMutT>(
    reader: ReadT,
    buffer: &mut BufMutT,
    max_count: u64,
) -> io::Result<(ReadT, usize)>
where
    ReadT: AsyncRead + Unpin,
    BufMutT: BufMut,
{
    let mut limited_reader = reader.take(max_count);
    let mut total = 0;

    loop {
        match limited_reader.read_buf(buffer).await {
            Ok(count) => {
                if count == 0 {
                    break;
                } else {
                    total += count;
                }
            }

            Err(error) => {
                if error.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(error);
                }
            }
        }
    }

    Ok((limited_reader.into_inner(), total))
}
