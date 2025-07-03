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
### Plugin Manifest Example

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
    - Refactor the mock up to reflect a realistic use case.
    
