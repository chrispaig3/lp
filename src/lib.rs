use serde::Deserialize;

mod parse;

#[derive(Debug)]
pub enum ParseError {
    ReadError(std::io::Error),
    FormatError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ReadError(err) => write!(f, "Failed to read file to string: {}", err),
            ParseError::FormatError(err) => write!(f, "Failed to parse format: {}", err),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::ReadError(err) => Some(err),
            ParseError::FormatError(err) => Some(&**err),
        }
    }
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

pub trait PluginManager {
    fn register(&self, f: impl Fn(Plugin));
    fn unregister(&self, f: impl Fn(Plugin));
    fn run(&self, f: impl Fn(Plugin));
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

// implements parse fn for each format
type PErr = ParseError;
impl_parse!(Toml, |file| toml::from_str(file)
    .map_err(|err| PErr::FormatError(err.into())));
impl_parse!(Ron, |file| ron::from_str(file)
    .map_err(|err| PErr::FormatError(err.into())));
impl_parse!(Json, |file| serde_json::from_str(file)
    .map_err(|err| PErr::FormatError(err.into())));

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
