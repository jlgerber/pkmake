use anyhow::anyhow;
use anyhow::Error as AnyError;
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
    Unknown,
}

impl Vcs {
    /// Constructor function builds a Vcs variant from a location
    pub fn from_path<I>(loc: I) -> Self
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
            Self::Both
        } else if svn_dir_exists {
            Self::Svn
        } else if git_dir_exists {
            Self::Git
        } else {
            Self::Unknown
        }
    }
    /// Return a string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Vcs::Git => "git",
            Vcs::Svn => "svn",
            Vcs::Both => "git+svn",
            Vcs::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        match &self {
            Vcs::Unknown => true,
            _ => false,
        }
    }
    pub fn is_both(&self) -> bool {
        match &self {
            Vcs::Both => true,
            _ => false,
        }
    }
}

impl FromStr for Vcs {
    type Err = AnyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "git" => Ok(Vcs::Git),
            "svn" => Ok(Vcs::Svn),
            "git+svn" | "svn+git" | "both" | "git&svn" | "svn&git" => Ok(Vcs::Both),
            _ => Err(anyhow!("'{}' unrecognized vcs", input)),
        }
    }
}

impl fmt::Display for Vcs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
