use std::time::*;

/// Current [Unix time](https://en.wikipedia.org/wiki/Unix_time).
pub fn unix_time() -> Result<u64, SystemTimeError> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}
