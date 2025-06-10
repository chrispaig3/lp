use std::fs;
use toml;
use serde::Deserialize;

// Plugin Manifest
#[derive(Debug, Deserialize)]
pub struct Toml {
    pub plugin: Plugin,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Plugin {
    pub authors: Option<Vec<String>>,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub path: Option<String>,
}

// Plugin Management
impl Plugin {
    /// Registers the plugin, adding it to the appropriate directory
    pub fn register(&self, f: impl Fn(Plugin)) {
        f(self.clone());
    }

    /// Unregisters the plugin, removing it from the directory
    pub fn unregister(&self, f: impl Fn(Plugin)) {
        f(self.clone());
    }

    /// Executes the plugin
    pub fn run(&self, f: impl Fn(Plugin)) {
        f(self.clone());
    }
}

impl Toml {
    pub fn parse(path: &str) -> Result<Self, toml::de::Error> {
        let toml = fs::read_to_string(path).unwrap();
        match toml::from_str(&toml) {
            Ok(toml) => Ok(toml),
            Err(e) => Err(e),
        }
    }
}

mod tests {
    #[test]
    fn test_plugin() {
        use super::*;

        let toml = Toml::parse("test_asset/plugin.toml").unwrap();
        
        toml.plugin.register(|plugin| {
            assert_eq!(plugin.name, "test_asset".to_string());
            assert_eq!(plugin.version, "0.1.0".to_string());
            assert_eq!(plugin.path, Some("/path/to/test_asset".to_string()));
        });
    }
}
