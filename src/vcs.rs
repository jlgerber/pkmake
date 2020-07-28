//! Vcs
// ! An enum that represents the possible vcs states for a project. A project may
//! either be in git, subversion, or both, in the case where the user is storing 
//! incremental changes locally in git, but pushing to subversion.
//! 
//! The code determines the vcs system(s) by introspecting disk at a location and
//! attempting to identify appropriate subdirectories (.git and/or .svn)
//! 
//! # Example
//! ```rust
//! # fn main() -> Result<(),Box<dyn std::error::Error>> {
//! use pk_make::Vcs;    
//! let vcs  = Vcs::from_path(".")?; 
//! # Ok(()) }
//! ```

// Internal crate imports
use crate::PkMakeError;

// External crate imports
use std::convert::TryFrom;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

/// Enumerate the potential vcs systems found at the root
/// of a project
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Vcs {
    Git,
    Svn,
    Both,
}

impl Vcs {
    /// Constructor function builds a Vcs variant from a location
    pub fn from_path<I>(loc: I) -> Result<Self, PkMakeError>
    where
        I: Into<PathBuf>,
    {
        let mut root = loc.into();
        // VCS Exists?
        root.push(".svn");
        let svn_dir_exists = root.exists();
        root.pop();
        root.push(".git");
        let git_dir_exists = root.exists();
        root.pop();
        // return appropriate variant
        if svn_dir_exists && git_dir_exists {
            Ok(Self::Both)
        } else if svn_dir_exists {
            Ok(Self::Svn)
        } else if git_dir_exists {
            Ok(Self::Git)
        } else {
            Err(PkMakeError::MissingVcs(format!("{:?}", root)))
        }
    }
    /// Return a string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Vcs::Git => "git",
            Vcs::Svn => "svn",
            Vcs::Both => "git+svn",
        }
    }

    pub fn is_both(&self) -> bool {
        match &self {
            Vcs::Both => true,
            _ => false,
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

impl FromStr for Vcs {
    type Err = PkMakeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "git" => Ok(Vcs::Git),
            "svn" => Ok(Vcs::Svn),
            "git+svn" | "svn+git" | "both" | "git&svn" | "svn&git" => Ok(Vcs::Both),
            _ => Err(PkMakeError::InvalidVcs(input.to_string())),
        }
    }
}

impl TryFrom<&str> for Vcs {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

impl fmt::Display for Vcs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
#[path = "./unit_tests/vcs_test.rs"]
mod vcs_test;
