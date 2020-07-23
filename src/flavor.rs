use anyhow::anyhow;
use anyhow::Error as AnyhowError;
use std::str::FromStr;

/// A flavor may either be vanilla or named
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Flavor {
    Vanilla,
    Named(String),
    Unknown(String),
}
impl Flavor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Vanilla => "^",
            Self::Named(ref s) => s.as_str(),
            Self::Unknown(ref s) => s.as_str(),
        }
    }
}

fn is_named_flavor(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '.'
}

impl FromStr for Flavor {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "^" | "vanilla" => Ok(Self::Vanilla),
            _ if s.chars().next().unwrap_or('1').is_alphabetic()
                && s.chars().all(is_named_flavor) =>
            {
                Ok(Self::Named(s.to_string()))
            }
            _ => Err(anyhow!("Invalid Flavor: '{}'", s)),
        }
    }
}

impl From<&str> for Flavor {
    fn from(other: &str) -> Self {
        match Flavor::from_str(other) {
            Ok(val) => val,
            Err(_) => Flavor::Unknown(other.to_string()),
        }
    }
}

impl std::default::Default for Flavor {
    fn default() -> Self {
        Flavor::Vanilla
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vanilla() {
        let vans = vec!["^", "vanilla", "Vanilla", "VANILLA"];
        for van in vans {
            let result = Flavor::from_str(van);
            assert_eq!(result.unwrap(), Flavor::Vanilla);
        }
    }

    #[test]
    fn from_named() {
        let nameds = vec!["foo", "FOO", "bar", "bla_f223_foo.bar"];
        for named in nameds {
            let result = Flavor::from_str(named);
            assert_eq!(result.unwrap(), Flavor::Named(named.to_string()));
        }
    }
    #[test]
    fn from_from_trait() {
        let vals = vec!["foo", "bar"];
        for val in vals {
            let result = Flavor::from(val);
            assert_eq!(result, Flavor::Named(val.to_string()));
        }
        assert_eq!(Flavor::from("^"), Flavor::Vanilla);
        assert_eq!(Flavor::from("Vanilla"), Flavor::Vanilla);
        assert_eq!(
            Flavor::from("foo bar#$$"),
            Flavor::Unknown("foo bar#$$".to_string())
        );
    }

    #[test]
    fn fails() {
        let fails = vec!["1foo", "foo bar", "bla_&^^", "FOO_!@#$%^&*()"];
        for fail in fails {
            let result = Flavor::from_str(fail);
            assert!(result.is_err());
        }
    }
}
