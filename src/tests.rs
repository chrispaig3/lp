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
