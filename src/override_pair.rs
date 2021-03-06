//! OverridePair
//! provides an efficient means of storing an override with a minimal 
//! number of allocations. The expected input is a &str
//! of the form <name>=<version> and provides a means of 
// ! retrieving the name and version as &str without additional allocation 

// Internal crate imports
use crate::utils::*;
use crate::PkMakeError;

// External crate imports
use std::convert::TryFrom;
use std::str::FromStr;


/// Represents a key,value pair joined by an equal token.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct OverridePair {
    value: String,
    name_end: u16,
    length: u16,
}

impl OverridePair {
    /// Retrieve the name of the OverridePair
    pub fn name(&self) -> &str {
        self.value.as_str().substring(0, self.name_end as usize)
    }

    /// Retrieve the version of the OverridePair
    pub fn version(&self) -> &str {
        self.value
            .as_str()
            .substring((self.name_end + 1) as usize, (self.length - 1) as usize)
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
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
impl FromStr for OverridePair {
    type Err = PkMakeError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
      
        if input.matches('=').count() != 1 || input.ends_with('=') {
            return Err(PkMakeError::ConvertFrom(input.to_string()));
        }
       
        Ok(Self {
            value: input.to_string(),
            name_end: input.chars().take_while(|c| c != &'=').count() as u16,
            length: input.chars().count() as u16,
        })
    }
}

impl TryFrom<&str> for OverridePair {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}
#[cfg(test)]
#[path = "./unit_tests/override_pair_test.rs"]
mod override_pair_test;
