#[macro_export]
macro_rules! impl_parse {
    // This macro takes two arguments:
    // 1. $struct_name:ident - The identifier for the struct (e.g., Toml, Ron, Json)
    // 2. $parser_fn:path - The path to the parsing function (e.g., toml::from_str, ron::from_str)
    ($struct_name:ident, $parser_fn:path) => {
        impl $struct_name {
            pub fn parse(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
                let content = fs::read_to_string(path)?;
                $parser_fn(&content).map_err(|e| e.into())
            }
        }
    };
}
