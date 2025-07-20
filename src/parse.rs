#[macro_export]
macro_rules! impl_parse {
    ($struct:ty, $parser:expr) => {
        impl $struct {
            pub fn parse(path: &str) -> Result<Self, ParseError> {
                let file = std::fs::read_to_string(path).map_err(ParseError::Read)?;
                $parser(&file).map_err(|err| ParseError::Format(Box::new(err)))
            }
        }
    };
}
