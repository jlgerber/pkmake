//!
//! Platform models the valid platforms that a user may request be built
//!

// Internal crate imports
use crate::PkMakeError;


// External crate imports
use std::convert::TryFrom;
use std::str::FromStr;


 /// The set of platforms that are active.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Platform {
    /// As of this writing, we sstil hvae some legacy windows 
    Win7_64,
    /// This is our current windows platform
    Win10_64,
    /// We have osx defined in our platforms config. However, we 
    /// currently do not have osx build machines. 
    Osx10_64,
    /// Our former mainstay, and still actively built
    Cent6_64,
    /// Our current primary platform
    Cent7_64,
    /// The next primary linux platform; not currently used, but should be comming up.
    Cent8_64,
}

impl Platform {
    /// Return a string literal representing the enum.
    // we could use an external dependency to generate this via macro...
    pub fn as_str(&self) -> &str {
        match self {
            Self::Win7_64 => "win7_64",
            Self::Win10_64 => "win10_64",
            Self::Osx10_64 => "osx10_64",
            Self::Cent6_64 => "cent6_64",
            Self::Cent7_64 => "cent7_64",
            Self::Cent8_64 => "cent8_64",
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
/// Convert a str to a Platform. It is case insensitive (at the small cost of a string allocation)
/// and accepts names which drop the _64, assuming that we are 64 bit.
impl FromStr for Platform {
    type Err = PkMakeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "win7_64" | "win7" => Ok(Self::Win7_64),
            "win10_64" | "win10" => Ok(Self::Win10_64),
            "osx10_64" | "osx10" => Ok(Self::Osx10_64),
            "cent6_64" | "cent6" => Ok(Self::Cent6_64),
            "cent7_64" | "cent7" => Ok(Self::Cent7_64),
            "cent8_64" | "cent8" => Ok(Self::Cent8_64),
            _ => Err(PkMakeError::InvalidPlatform(s.to_string())),
        }
    }
}

/// Fallible conversion from a &str to a Platform. TryInto<Platform> will be auto implemented
/// for &str as well, thanks to Rust's helpful developers.
impl TryFrom<&str> for Platform {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input).map_err(|_e| PkMakeError::InvalidPlatform(input.to_string()))
    }
}
#[cfg(test)]
#[path = "./unit_tests/platform_test.rs"]
mod platform_test;
