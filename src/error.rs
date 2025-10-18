pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Project error: {0}")]
    Project(String),
    #[error("AI error: {0}")]
    AgentError(String),
    #[error("Template error: {0}")]
    Template(String),
    #[error("TOML serialization error: {0}")]
    Toml(#[from] toml::ser::Error),
}
