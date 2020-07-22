use std::str::FromStr;
use std::fmt;
use anyhow::anyhow;
use anyhow::Error as AnyhowError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// List of Valid Sites, along with a placeholder for unknown sites. This is used 
/// by non-fallible constructor methods. 
pub enum NamedSite {
    Hyderabad,
    Playa,
    Portland,
    Montreal,
    Vancouver,
    Unknown(String)
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
            Self::Unknown(ref val) => write!(f, "{} (Unknown)", val)
        }
    }  
}

impl FromStr for NamedSite {
    type Err = AnyhowError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "hyderabad" => Ok(Self::Hyderabad),
            "playa" | "playa vista" | "playavista" => Ok(Self::Playa),
            "portland" => Ok(Self::Portland),
            "montreal" => Ok(Self::Montreal),
            "vancouver" => Ok(Self::Vancouver),
            _ => Err(anyhow!("{} is not a valid NamedSite", input))
        }
    }
}

impl From<&str> for NamedSite {
    fn from(input: &str) -> NamedSite {
        match NamedSite::from_str(input) {
            Ok(site) => site,
            Err(_) => Self::Unknown(input.to_string()),
        }
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
           ("Montreal",NamedSite::Montreal), 
           ("vancouver",NamedSite::Vancouver),
            ("Vancouver", NamedSite::Vancouver)
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
            ("Montreal",NamedSite::Montreal), 
            ("vancouver",NamedSite::Vancouver),
            ("Vancouver", NamedSite::Vancouver)
        ];
        for site in sites {
            let result = NamedSite::from(site.0);
            assert_eq!(result, site.1);
        }
    }

    #[test]
    fn instance_from_bad_input() {
        let result = NamedSite::from("fluboxland");
        assert_eq!(result, NamedSite::Unknown("fluboxland".to_string()));
    }
}


