//!
//! Platform
//!
//! representation of build platforms
//!
use crate::PkMakeError;
//use anyhow::anyhow;
//use anyhow::Error as AnyhowError;
use std::convert::TryFrom;
use std::str::FromStr;

/*
fc4_32
deb4_64
cent5_64
osx10_64
cent6_64
cent7_64
xp_32
xp_64
win7_64
win10_64

 */
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Platform {
    Win7_64,
    Win10_64,
    Osx10_64,
    Cent6_64,
    Cent7_64,
    Cent8_64,
}

impl Platform {
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

impl TryFrom<&str> for Platform {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input).map_err(|_e| PkMakeError::InvalidPlatform(input.to_string()))
    }
}
#[cfg(test)]
#[path = "./unit_tests/platform_test.rs"]
mod platform_test;
