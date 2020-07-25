use crate::PkMakeError;
//use anyhow::anyhow;
//use anyhow::Error as AnyhowError;
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
    Unknown(String),
}

impl NamedSite {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Hyderabad => "hyderabad",
            Self::Playa => "playa",
            Self::Portland => "portland",
            Self::Montreal => "montreal",
            Self::Vancouver => "vancouver",
            Self::Unknown(ref val) => val,
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
        //write!(f, "({}, {})", self.x, self.y)
        match self {
            Self::Hyderabad => write!(f, "Hyderabad"),
            Self::Playa => write!(f, "Playa Vista"),
            Self::Portland => write!(f, "Portland"),
            Self::Montreal => write!(f, "Montreal"),
            Self::Vancouver => write!(f, "Vancouver"),
            Self::Unknown(ref val) => write!(f, "{} (Unknown)", val),
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

// impl From<&str> for NamedSite {
//     fn from(input: &str) -> NamedSite {
//         match NamedSite::from_str(input) {
//             Ok(site) => site,
//             Err(_) => Self::Unknown(input.to_string()),
//         }
//     }
// }

impl TryFrom<&str> for NamedSite {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_from_str() {
        let sites = vec![
            ("hyderabad", NamedSite::Hyderabad),
            ("HYDERABAD", NamedSite::Hyderabad),
            ("playa", NamedSite::Playa),
            ("Playa", NamedSite::Playa),
            ("Playa Vista", NamedSite::Playa),
            ("playavista", NamedSite::Playa),
            ("portland", NamedSite::Portland),
            ("Portland", NamedSite::Portland),
            ("Montreal", NamedSite::Montreal),
            ("vancouver", NamedSite::Vancouver),
            ("Vancouver", NamedSite::Vancouver),
        ];
        for site in sites {
            let result = NamedSite::from_str(site.0);
            assert_eq!(result.unwrap(), site.1);
        }
    }
    #[test]
    fn err_from_bad_str() {
        let result = NamedSite::from_str("fluboxland");
        assert!(result.is_err());
    }

    #[test]
    fn instance_from() {
        let sites = vec![
            ("hyderabad", NamedSite::Hyderabad),
            ("HYDERABAD", NamedSite::Hyderabad),
            ("playa", NamedSite::Playa),
            ("Playa", NamedSite::Playa),
            ("Playa Vista", NamedSite::Playa),
            ("playavista", NamedSite::Playa),
            ("portland", NamedSite::Portland),
            ("Portland", NamedSite::Portland),
            ("Montreal", NamedSite::Montreal),
            ("vancouver", NamedSite::Vancouver),
            ("Vancouver", NamedSite::Vancouver),
        ];
        for site in sites {
            let result = NamedSite::from(site.0);
            assert_eq!(result.unwrap(), site.1);
        }
    }

    #[test]
    fn instance_from_bad_input() {
        let result = NamedSite::from("fluboxland");
        assert!(result.is_err());
    }
}
