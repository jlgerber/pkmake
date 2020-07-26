use thiserror::Error;

#[derive(Error, Debug)]
pub enum PkMakeError {
    #[error("Conversion Failed for '{0}'")]
    ConvertFrom(String),
    #[error("Invalid Site:'{0}'")]
    InvalidSite(String),
    #[error("Invalid Flavor:'{0}'")]
    InvalidFlavor(String),
}
