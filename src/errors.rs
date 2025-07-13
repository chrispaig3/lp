#[derive(Debug)]
pub enum ParseError {
    ReadError(std::io::Error),
    FormatError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ReadError(err) => write!(f, "Failed to read file to string: {}", err),
            ParseError::FormatError(err) => write!(f, "Failed to parse format: {}", err),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::ReadError(err) => Some(err),
            ParseError::FormatError(err) => Some(&**err),
        }
    }
}