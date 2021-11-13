use serde::Deserialize;

#[non_exhaustive]
#[derive(Deserialize)]
/// Configuration for the server instance
pub struct Config {
    /// The database the server connects to
    pub db_uri: String,
    #[serde(default)]
    /// If the tracing extension should be enabled
    pub tracing: bool,
}
