use std::str::FromStr;
use crate::named_site::NamedSite;

/// enum representing valid state of Sites input
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Site {
    Local,
    All,
    Named(NamedSite)
}
// helper function to test whether a character is alphabetic
// or a comma. Used for named site(s)
fn site_tst(c: char) -> bool {
    c.is_alphabetic() //|| c == ','
}

impl FromStr for Site {

    type Err=String; //todo replace with custom 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Site::Local),
            "all" => Ok(Site::All),
            _ => {
                if !s.chars().all(site_tst) {
                    Err("must be alphabetic".into())
                } else {
                    match NamedSite::from_str(s) {
                        Ok(site) => Ok(Site::Named(site)),
                        Err(_) => Err(format!("Invalid Site Name {}", s))
                    }
                }
            }
        }
    }
}

impl From<&str> for Site {
    fn from(input: &str) -> Self {
        match Site::from_str(input) {
            Err(_) => Site::Local,
            Ok(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_local() {
        let locals = vec!["local", "Local", "LOCAL"];
        for local in locals {
            let result = Site::from_str(local);
            assert_eq!(result, Ok(Site::Local));
        }
    }

    #[test]
    fn can_build_all() {
        let alls = vec!["all", "All", "ALL"];
        for all_ in alls {
            let result = Site::from_str(all_);
            assert_eq!(result, Ok(Site::All));
        }
    }

    #[test]
    fn can_build_named() {
        let named = vec![
            ("hyderabad", NamedSite::Hyderabad), 
            ("playa",NamedSite::Playa), 
            ("portland", NamedSite::Portland),
            ("montreal", NamedSite::Montreal),
            ("vancouver", NamedSite::Vancouver)
        ];
        for nm in named {
            let result = Site::from_str(nm.0);
            assert_eq!(result, Ok(Site::Named(nm.1)));
        }
    }

    #[test]
    fn errors_when_invalid_chars_used() {
        let invalids = vec!["foo bar", "foo1", "foo,bar","bar$#%"];
        for invalid in invalids {
            let result = Site::from_str(invalid);
            assert!(result.is_err());
        }
    }
}
