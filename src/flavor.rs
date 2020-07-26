use crate::PkMakeError;
use std::convert::TryFrom;
use std::str::FromStr;

/// A flavor may either be vanilla or named
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Flavor {
    Vanilla,
    Named(String),
    Unknown(String),
}
impl Flavor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Vanilla => "^",
            Self::Named(ref s) => s.as_str(),
            Self::Unknown(ref s) => s.as_str(),
        }
    }
    /// Convert a &str to a Flavor, fallibly.
    pub fn from<I>(input: I) -> Result<Self, PkMakeError>
    where
        I: AsRef<str>,
    {
        Self::try_from(input.as_ref())
    }
}

fn is_named_flavor(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '.'
}

impl FromStr for Flavor {
    type Err = PkMakeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "^" | "vanilla" => Ok(Self::Vanilla),
            _ if s.chars().next().unwrap_or('1').is_alphabetic()
                && s.chars().all(is_named_flavor) =>
            {
                Ok(Self::Named(s.to_string()))
            }
            _ => Err(PkMakeError::InvalidFlavor(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Flavor {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

impl std::default::Default for Flavor {
    fn default() -> Self {
        Flavor::Vanilla
    }
}
#[cfg(test)]
#[path = "./unit_tests/flavor_test.rs"]
mod flavor_test;
