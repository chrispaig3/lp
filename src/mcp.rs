// opt-in support for model context protocol
#[cfg(feature = "mcp")]
pub use rmcp::{
    model::CallToolRequestParam,
    service::ServiceExt,
    transport::{ConfigureCommandExt, TokioChildProcess},
};

#[cfg(feature = "mcp")]
trait Transmittable {
    fn send(&self) -> T;
}

#[cfg(feature = "mcp")]
struct Mcp<T> {
    packet: T,
}

#[cfg(feature = "mcp")]
impl<T> Transmittable for Mcp<T> {
    // server
    fn send(&self) -> T {}
}
