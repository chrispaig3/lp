use serde::Deserialize;

mod errors;
pub mod load;
mod parse;
mod tests;
use errors::ParseError;

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
    .map_err(|err| PErr::Format(err.into())));
impl_parse!(Ron, |file| ron::from_str(file)
    .map_err(|err| PErr::Format(err.into())));
impl_parse!(Json, |file| serde_json::from_str(file)
    .map_err(|err| PErr::Format(err.into())));
