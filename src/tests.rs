#[test]
fn test_plugin() {
    use super::*;
    use std::path::Path;

    let toml = Toml::parse("test_asset/plugin.toml").expect("File not found");
    let ron = Ron::parse("test_asset/plugin.ron").expect("File not found");
    let json = Json::parse("test_asset/plugin.json").expect("File not found");

    assert_eq!(ron.plugin.name, "test_asset");
    assert_eq!(json.plugin.name, "test_asset");

    toml.plugin.register(|plugin| {
        if let Some(path) = &plugin.path
            && let Some(description) = &plugin.description
            && let Some(authors) = plugin.authors
            && let Some(license) = &plugin.license
        {
            if !Path::new(path).exists() {
                println!("Plugin directory does not exist, creating @: {:?}", path);
                // set up plugin dir
                // move plugin files to dir
                assert_eq!(path, &"/path/to/test_asset".to_string());
                assert_eq!(authors, vec!["chrispaig3"]);
                assert_eq!(description, &"A test asset for the plugin manager.");
                assert_eq!(license, &"MIT");
            } else {
                println!("Plugin directory already exists: {:?}", path);
            }
        }
    });
}

#[test]
#[cfg(feature = "dynamic-loading")]
fn test_load() {
    use crate::load::{Lib, Loadable, Symbol};
    let path = "/Users/chris/Dev/lp/test_fn/target/debug/libtest_fn.dylib";
    let symbol = "add_fn";

    println!("Loading add function...");
    unsafe {
        type UnsafeFn = unsafe extern "C" fn(i32, i32) -> i32;
        let add_lib = Lib::<Symbol<'static, UnsafeFn>>::load(path, symbol)
            .expect("Failed to load add function");

        println!("add function loaded successfully!");
        if let Some(add_fn) = &add_lib.f {
            let result = add_fn(5, 3);
            println!("Result from add(5, 3): {}", result);
        }
    }

    let hello_symbol = "hello_fn";
    unsafe {
        type UnsafeFn = unsafe extern "C" fn() -> *const std::ffi::c_char;
        let hello_lib = Lib::<Symbol<'static, UnsafeFn>>::load(path, hello_symbol)
            .expect("Failed to load hello function");

        println!("hello function loaded successfully!");
        if let Some(hello_fn) = &hello_lib.f {
            let result = hello_fn();
            println!(
                "Result from hello(): {}",
                std::ffi::CStr::from_ptr(result).to_string_lossy()
            );
        }
    }
}
