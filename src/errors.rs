#[cfg(feature = "dynamic-loading")]
use libloading;

#[derive(Debug)]
pub enum ParseError {
    ReadError(std::io::Error),
    FormatError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ReadError(err) => write!(f, "{}", err),
            ParseError::FormatError(err) => write!(f, "{}", err),
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

#[cfg(feature = "dynamic-loading")]
#[derive(Debug)]
pub enum LoadError {
    LibraryError(libloading::Error),
    SymbolError { symbol_name: String },
}

#[cfg(feature = "dynamic-loading")]
impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::LibraryError(err) => write!(f, "Failed to load library: {}", err),
            LoadError::SymbolError { symbol_name } => {
                write!(f, "Failed to find symbol in library: {}", symbol_name)
            }
        }
    }
}

#[cfg(feature = "dynamic-loading")]
impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::LibraryError(err) => Some(err),
            LoadError::SymbolError { .. } => None,
        }
    }
}

#[cfg(feature = "dynamic-loading")]
impl From<libloading::Error> for LoadError {
    fn from(err: libloading::Error) -> Self {
        LoadError::LibraryError(err)
    }
}
