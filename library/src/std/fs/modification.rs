use std::{fs::*, hash::*, io, path::*};

/// Calculates an identifier for a file based on its modification date.
///
/// The actual value is intended to be opaque, however it should be unique for every file
/// modification.
pub fn file_modification_identifier<PathT>(path: PathT) -> io::Result<u64>
where
    PathT: AsRef<Path>,
{
    let modified = metadata(path)?.modified()?;
    let mut hasher = DefaultHasher::default();
    modified.hash(&mut hasher);
    Ok(hasher.finish())
}
