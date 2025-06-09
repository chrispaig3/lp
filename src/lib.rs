use std::fs;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Toml {
    pub plugin: Plugin,
}

#[derive(Debug, Deserialize)]
pub struct Plugin {
    pub authors: Option<Vec<String>>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub path: String,
}

impl Plugin {
    pub fn register(&self, _: impl Fn(Plugin)) {}

    pub fn unregister(&self, _: impl Fn(Plugin)) {}

    pub fn run(&self, _: impl Fn(Plugin)) {}
}

impl Toml {
    pub fn parse_toml(path: &str) -> Result<Self, toml::de::Error> {
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
        let toml = Toml::parse_toml("test_asset/plugin.toml").unwrap();
        /* 
        toml.plugin.register(|plugin| {
            println!("Plugin registered: {:?}", plugin);
        });
        */

        assert_eq!(toml.plugin.name, Some("test_asset".to_string()));
        assert_eq!(toml.plugin.version, Some("0.1.0".to_string()));
    }
}
