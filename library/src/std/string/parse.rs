use super::super::error::*;

//
// ParseStr
//

/// Parse string.
pub trait ParseStr<ParsedT> {
    /// Parse string.
    fn parse(representation: &str) -> Result<ParsedT, ParseError>;
}

//
// ParseError
//

message_error!(ParseError, "parse");
