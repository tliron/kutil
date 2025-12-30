use std::{collections::*, fmt};

/// Tag separator.
pub const TAG_SEPARATOR: char = ':';

//
// FormatSpecificationTags
//

/// Format specification tags.
pub struct FormatSpecificationTags<'spec> {
    /// Tags.
    pub tags: BTreeSet<&'spec str>,
}

impl<'spec> FormatSpecificationTags<'spec> {
    /// Remove a tag.
    ///
    /// Return true if removed.
    pub fn remove(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    /// Remove a tag with a prefix and return what is after the prefix.
    pub fn remove_prefix(&mut self, prefix: &str) -> Option<&str> {
        let mut iterator = self.tags.iter();
        while let Some(tag) = iterator.next() {
            if tag.starts_with(prefix) {
                return Some(&tag[prefix.len()..]);
            }
        }
        None
    }

    /// Pack.
    pub fn pack(&self) -> Option<String> {
        if !self.tags.is_empty() { Some(self.to_string()) } else { None }
    }
}

impl<'spec> fmt::Display for FormatSpecificationTags<'spec> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.tags.is_empty() {
            let mut iterator = self.tags.iter().peekable();
            while let Some(tag) = iterator.next() {
                if iterator.peek().is_some() {
                    fmt::Display::fmt(tag, formatter)?;
                    write!(formatter, "{}", TAG_SEPARATOR)?;
                } else {
                    fmt::Display::fmt(tag, formatter)?;
                }
            }
        }

        Ok(())
    }
}

impl<'spec> From<&'spec str> for FormatSpecificationTags<'spec> {
    fn from(specification: &'spec str) -> Self {
        Self { tags: specification.split(TAG_SEPARATOR).collect() }
    }
}
