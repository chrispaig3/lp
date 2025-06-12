use std::fs;
use toml;
use serde::Deserialize;

pub trait PluginManager {
    fn register(&self, f: impl Fn(Plugin));
    fn unregister(&self, f: impl Fn(Plugin));
    fn run(&self, f: impl Fn(Plugin));
}

// Toml Manifest
#[derive(Debug, Deserialize)]
pub struct Toml {
    pub plugin: Plugin,
}

/* 
// Ron Manifest
#[derive(Debug, Deserialize)]
pub struct Ron {
    pub plugin: Plugin,
}

#[derive(Debug, Deserialize)]
pub struct Json {
    pub plugin: Plugin,
}

// Yaml Manifest
#[derive(Debug, Deserialize)]
pub struct Yaml {
    pub plugin: Plugin,
}
*/

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
impl PluginManager for Plugin {
    /// Registers the plugin, adding it to the appropriate directory
    fn register(&self, f: impl Fn(Plugin)) {
        f(self.clone());
    }

    /// Unregisters the plugin, removing it from the directory
    fn unregister(&self, f: impl Fn(Plugin)) {
        f(self.clone());
    }

    /// Executes the plugin
    fn run(&self, f: impl Fn(Plugin)) {
        f(self.clone());
    }
}

impl Toml {
    pub fn parse(path: &str) -> Result<Self, toml::de::Error> {
        let toml = fs::read_to_string(path).unwrap();
        toml::from_str(&toml)
    }
}

mod tests {
    #[test]
    fn test_plugin() {
        use super::*;
        use std::path::Path;

        let toml = Toml::parse("test_asset/plugin.toml").unwrap();
        
        toml.plugin.register(|plugin| {
            if let Some(path) = &plugin.path {
               if !Path::new(path).exists() {
                    println!("Plugin directory does not exist, creating @: {:?}", path);
                    // set up plugin dir
                    // move plugin files to dir
                    assert_eq!(plugin.path, Some("/path/to/test_asset".to_string()));
                } else {
                    println!("Plugin directory already exists: {:?}", path);
                }
            }
        });
    }
}