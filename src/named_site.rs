//! NamedSite
//! Enumerates our locations and provides a simple api for conversion from / to &str
use crate::PkMakeError;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
/// List of Valid Sites, along with a placeholder for unknown sites. This is used
/// by non-fallible constructor methods.
pub enum NamedSite {
    Hyderabad,
    Playa,
    Portland,
    Montreal,
    Vancouver,
}

impl NamedSite {
    
    pub fn as_str(&self) -> &str {
        match self {
            Self::Hyderabad => "hyderabad",
            Self::Playa => "playa",
            Self::Portland => "portland",
            Self::Montreal => "montreal",
            Self::Vancouver => "vancouver",
        }
    }

    /// Overrides the auto generated trait impl of from to provide a
    /// fallible version
    pub fn from<I>(input: I) -> Result<Self, PkMakeError>
    where
        I: AsRef<str>,
    {
        Self::try_from(input.as_ref())
    }
}
impl fmt::Display for NamedSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hyderabad => write!(f, "Hyderabad"),
            Self::Playa => write!(f, "Playa Vista"),
            Self::Portland => write!(f, "Portland"),
            Self::Montreal => write!(f, "Montreal"),
            Self::Vancouver => write!(f, "Vancouver"),
        }
    }
}

impl FromStr for NamedSite {
    type Err = PkMakeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "hyderabad" => Ok(Self::Hyderabad),
            "playa" | "playa vista" | "playavista" => Ok(Self::Playa),
            "portland" => Ok(Self::Portland),
            "montreal" => Ok(Self::Montreal),
            "vancouver" => Ok(Self::Vancouver),
            _ => Err(PkMakeError::InvalidSite(input.to_string())),
        }
    }
}

impl TryFrom<&str> for NamedSite {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(test)]
#[path = "./unit_tests/named_site_test.rs"]
mod named_site_test;
