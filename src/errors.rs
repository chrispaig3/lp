#[cfg(feature = "dynamic-loading")]
use libloading;

#[cfg(feature = "default")]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to read plugin manifest")]
    Read(#[from] std::io::Error),
    #[error("Failed to parse file; format may be unsupported")]
    Format(#[from] Box<dyn std::error::Error>),
}

#[cfg(feature = "dynamic-loading")]
#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Failed to load dynamic library")]
    Library(#[from] libloading::Error),
    #[error("Failed to find symbol in library")]
    Symbol { symbol_name: String },
}
