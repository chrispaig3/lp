# LibPlugin

![Image](https://github.com/user-attachments/assets/8c401358-3a23-4cea-8c52-368140caf06e)

> An embeddable plugin system for Rust applications.

- LibPlugin handles the majority of boilerplate required for implementing a functional plugin system; offering some flexibility in design & configuration.

***Still under early development; I would recommend against using it for anything serious at the moment.***

- Supported Formats:
    - ***Toml***
    - ***Ron***
    - ***Json***

### Basic Usage
```rust
use lp::{PluginManager, Toml};

fn main() {
    let toml = Toml::parse("plugin_manifest.toml")
        .expect("Failed to parse plugin manifest");

    toml.plugin.register(|plugin| {
        // custom logic to handle registering the plugin
    });

    toml.plugin.run(|plugin| {
        // custom logic to handle executing the plugin
    });

    toml.plugin.unregister(|plugin| {
        // custom logic to handle unregistering the plugin
    });
}
```

### Advanced Usage

```rust

// add to your project: cargo add lp -F dynamic-loading
use lp::{
    PluginManager, Toml,
    load::{Lib, Loadable, Symbol},
};

fn main() {
    let hello_symbol = "hello_fn";
    let toml = Toml::parse("plugin.toml").expect("File not found");
    toml.plugin.register(|plugin| {
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
```


### Example: Plugin Manifest

- TOML

```toml
[plugin]
name = "example"
version = "0.1.0"
authors = ["Author Name"]
description = "An example plugin"
license = "MIT"
path = "/path/to/plugin"
```
- RON
```ron
Ron(
    plugin: Plugin(
        name: "example",
        version: "0.1.0",
        authors: Some(["Author Name"]),
        description: Some("An example plugin"),
        license: Some("MIT"),
        path: Some("/path/to/plugin"),
    ),
)
```
- JSON
```json
{
    "plugin": {
        "name": "example",
        "version": "0.1.0",
        "authors": ["Author Name"],
        "description": "An example plugin",
        "license": "MIT",
        "path": "/path/to/plugin"
    }
}
```

#### Roadmap

- Improve Tests
    - Enhance Docs
    - Refactor the mock up to reflect a realistic use case.
