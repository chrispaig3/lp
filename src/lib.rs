use serde::Deserialize;
use std::fs;
use toml;

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

// Ron Manifest
#[derive(Debug, Deserialize)]
pub struct Ron {
    pub plugin: Plugin,
}

// Json Manifest
#[derive(Debug, Deserialize)]
pub struct Json {
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

impl Ron {
    pub fn parse(path: &str) -> Result<Self, ron::error::SpannedError> {
        let ron = fs::read_to_string(path).unwrap();
        ron::from_str(&ron)
    }
}

impl Json {
    pub fn parse(path: &str) -> Result<Self, serde_json::Error> {
        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json)
    }
}

mod tests {
    #[test]
    fn test_plugin() {
        use super::*;
        use std::path::Path;

        let toml = Toml::parse("test_asset/plugin.toml").unwrap();
        let ron = Ron::parse("test_asset/plugin.ron").unwrap();
        let json = Json::parse("test_asset/plugin.json").unwrap();

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
}
