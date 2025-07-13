#[macro_export]
macro_rules! impl_parse {
    // This macro takes two arguments:
    // 1. $struct:ident - The identifier for the struct (e.g., Toml, Ron, Json)
    // 2. $parser_fn:path - The path to the parsing function (e.g., toml::from_str, ron::from_str)
    ($struct:ty, $parser:expr) => {
        impl $struct {
            pub fn parse(path: &str) -> Result<Self, ParseError> {
                let content = std::fs::read_to_string(path).map_err(ParseError::ReadError)?;
                $parser(&content).map_err(|err| ParseError::FormatError(Box::new(err)))
            }
        }
    };
}
