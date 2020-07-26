use thiserror::Error;

#[derive(Error, Debug)]
pub enum PkMakeError {
    #[error("Conversion Failed for '{0}'")]
    ConvertFrom(String),
    #[error("Invalid Site:'{0}'")]
    InvalidSite(String),
    #[error("Invalid Flavor:'{0}'")]
    InvalidFlavor(String),
    #[error("Invalid Context:'{0}'")]
    InvalidContext(String),
    #[error("Invalid Platform:'{0}'")]
    InvalidPlatform(String),
    #[error("Invalid Vcs:'{0}'")]
    InvalidVcs(String),
    #[error("Missing Vcs from path:'{0}'")]
    MissingVcs(String),
}
