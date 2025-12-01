#[cfg(feature = "dynamic-loading")]
#[test]
fn test_plugin() {
    use super::*;
    use crate::load::{Lib, Loadable, Symbol};

    let toml = Toml::parse("test_asset/plugin.toml").unwrap();
    let hello_symbol = "hello_fn";

    toml.plugin.use_plugin(|plugin| {
        type UnsafeFn = unsafe extern "C" fn() -> *const std::ffi::c_char;

        if let Some(path) = &plugin.path {
            let hello_lib = unsafe {
                Lib::<Symbol<'static, UnsafeFn>>::load(path, hello_symbol)
                    .expect("Failed to load hello function")
            };

            if let Some(hello_fn) = &hello_lib.f {
                let result = unsafe { hello_fn() };
                println!("hello function loaded!");

                println!("{}", unsafe {
                    std::ffi::CStr::from_ptr(result).to_string_lossy()
                });
            }
        }
    });
}
