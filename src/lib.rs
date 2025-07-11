use serde::Deserialize;
use std::fs;
use toml;

mod parse;

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

// implements parse fn for each format
impl_parse!(Toml, toml::from_str);
impl_parse!(Ron, ron::from_str);
impl_parse!(Json, serde_json::from_str);

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

mod tests {
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
}
