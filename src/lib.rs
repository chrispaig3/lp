use std::fs;
use toml;
use serde::Deserialize;


// Plugin Manifest
#[derive(Debug, Deserialize)]
pub struct Toml {
    pub plugin: Plugin,
}

#[derive(Debug, Deserialize)]
pub struct Plugin {
    pub authors: Option<Vec<String>>,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub path: String,
}

// Plugin Management
impl Plugin {
    /// Registers the plugin, adding it to the appropriate directory
    /// Because application architecture may vary, you may want to implement your own logic for registering plugins.
    pub fn register(&self, _: impl Fn(Plugin)) {}

    /// Unregisters the plugin, removing it from the directory
    /// Because application architecture may vary, you may want to implement your own logic for unregistering plugins.
    pub fn unregister(&self, _: impl Fn(Plugin)) {}

    /// Executes the plugin
    /// At the end of the day, it is up to you to implement the logic for what the plugin does when it runs.
    pub fn run(&self, _: impl Fn(Plugin)) {}
}

impl Toml {
    pub fn parse(path: &str) -> Result<Self, toml::de::Error> {
        let toml =  fs::read_to_string(path).unwrap();
        match toml::from_str(&toml) {
            Ok(toml) => Ok(toml),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_parsing() {
        let toml = Toml::parse("test_asset/plugin.toml").unwrap();
        /* 
        toml.plugin.register(|plugin| {
            println!("Plugin registered: {:?}", plugin);
        });
        */

        assert_eq!(toml.plugin.name, "test_asset".to_string());
        assert_eq!(toml.plugin.version, "0.1.0".to_string());
    }
}
