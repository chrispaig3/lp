#[cfg(feature = "dynamic-loading")]
use crate::errors::LoadError;
#[cfg(feature = "dynamic-loading")]
pub use libloading::{Library, Symbol};

#[cfg(feature = "dynamic-loading")]
pub trait Loadable
where
    Self: Sized,
{
    unsafe fn load(path: &str, symbol: &str) -> Result<Self, LoadError>;
}

#[cfg(feature = "dynamic-loading")]
pub struct Lib<T> {
    _lib: Library,
    pub f: Option<T>,
}

#[cfg(feature = "dynamic-loading")]
impl<T> Loadable for Lib<Symbol<'static, T>>
where
    T: 'static,
{
    unsafe fn load(path: &str, symbol: &str) -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(path) }?;

        let symbol_name = format!("{}\0", symbol);
        let f: Symbol<T> = unsafe {
            lib.get(symbol_name.as_bytes())
                .map_err(|_| LoadError::Symbol {
                    symbol_name: symbol.into(),
                })
        }?;

        let f: Symbol<'static, T> = unsafe { std::mem::transmute(f) };

        Ok(Self {
            _lib: lib,
            f: Some(f),
        })
    }
}
