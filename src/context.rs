//! Context
//!
//! Models the valid context values which a user may supply to 
//! pk-make


// Internal crate imports
use crate::PkMakeError;

// External crate imports
use std::convert::TryFrom;
use std::str::FromStr;

/// Tme possible contexts that a user may request a recipe 
/// be executed in. 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Context {
    Facility,
    /// Shared indicates that the recipe is targeting a show. It is called Shared
    /// (as opposed to say Show), for historical reasons. The data used to all be stored
    /// in a SHARED directory at the SHOW level.
    // todo: change to Show 
    Shared,
    /// User indicates that the recipe is targeting a work directory
    /// on a show. It is called User for historical reasons (eg go <show> =user)
    // todo: change to Work
    User,
}

impl Context {
    /// get a &str representation of the Context
    pub fn as_str(&self) -> &str {
        match self {
            Self::Facility => "facility",
            Self::Shared => "shared",
            Self::User => "user",
        }
    }
    /// Convert a &str to a Context, fallibly.
    pub fn from<I>(input: I) -> Result<Self, PkMakeError>
    where
        I: AsRef<str>,
    {
        Self::try_from(input.as_ref())
    }
}

impl FromStr for Context {
    type Err = PkMakeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "facility" => Ok(Context::Facility),
            "shared" => Ok(Context::Shared),
            "user" => Ok(Context::User),
            _ => Err(PkMakeError::InvalidContext(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Context {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

//
// Import Tests
//
#[cfg(test)]
#[path = "./unit_tests/context_test.rs"]
mod context_test;
