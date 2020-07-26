use crate::named_site::NamedSite;
use crate::PkMakeError;
use std::convert::TryFrom;
use std::str::FromStr;

/// enum representing valid state of Sites input
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Site {
    Local,
    All,
    Named(NamedSite),
}
// // helper function to test whether a character is alphabetic
// // or a comma. Used for named site(s)
// fn site_tst(c: char) -> bool {
//     c.is_alphabetic() //|| c == ','
// }

impl Site {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Local => "local",
            Self::All => "all",
            Self::Named(named_site) => named_site.to_str(),
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
impl FromStr for Site {
    type Err = PkMakeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Site::Local),
            "all" => Ok(Site::All),
            _ => {
                // if !s.chars().all(site_tst) {
                //     Err(PkMakeError::InvalidSite(s.to_string()))
                // } else
                // {
                match NamedSite::from_str(s) {
                    Ok(site) => Ok(Site::Named(site)),
                    Err(_) => Err(PkMakeError::InvalidSite(s.to_string())),
                }
                //}
            }
        }
    }
}
impl TryFrom<&str> for Site {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}

#[cfg(test)]
#[path = "./unit_tests/site_test.rs"]
mod site_test;
