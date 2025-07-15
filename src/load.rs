use crate::errors::LoadError;
pub use libloading::{Library, Symbol};

pub trait Loadable
where
    Self: Sized,
{
    unsafe fn load(path: &str, symbol: &str) -> Result<Self, LoadError>;
}

pub struct Lib<T> {
    _lib: Library,
    pub f: Option<T>,
}

impl<T> Loadable for Lib<Symbol<'static, T>>
where
    T: 'static,
{
    unsafe fn load(path: &str, symbol: &str) -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(path) }?;

        let symbol_name = format!("{}\0", symbol);
        let f: Symbol<T> = unsafe {
            lib.get(symbol_name.as_bytes())
                .map_err(|_| LoadError::SymbolError {
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
