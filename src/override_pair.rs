use anyhow::anyhow;
use anyhow::Error as AnyError;
//use lazy_static::lazy_static;
//use regex::Regex;
use crate::utils::*;
use std::str::FromStr;
#[derive(Debug, PartialEq, Eq)]
pub struct OverridePair {
    value: String,
    name_end: u16,
    length: u16,
}
impl OverridePair {
    pub fn name(&self) -> &str {
        self.value.as_str().substring(0, self.name_end as usize)
    }

    pub fn version(&self) -> &str {
        self.value
            .as_str()
            .substring((self.name_end + 1) as usize, (self.length - 1) as usize)
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}
impl FromStr for OverridePair {
    type Err = AnyError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // lazy_static! {
        //     static ref RE: Regex = Regex::new("[:alpha][a-zA-Z0-9_.]+=[:alpha:]+\.[:alpha:]").unwrap();
        // }
        let pieces = input.split("=").collect::<Vec<_>>();
        if pieces.len() != 2 {
            return Err(anyhow!(
                "Invalid override specification: <package>=<version> '{}'",
                input
            ));
        }
        Ok(Self {
            value: input.to_string(),
            name_end: pieces[0].chars().count() as u16,
            length: input.chars().count() as u16,
        })
    }
}

impl From<&str> for OverridePair {
    fn from(input: &str) -> Self {
        match Self::from_str(input) {
            Ok(p) => p,
            Err(_) => Self {
                value: "NONE=NONE".into(),
                name_end: 4,
                length: 9,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_name_and_version() {
        let op = OverridePair::from_str("foo=1.3.2").unwrap();
        assert_eq!(op.name(), "foo");
        assert_eq!(op.version(), "1.3.2");
    }

    #[test]
    fn none_set_correctly() {
        let op = OverridePair::from("foo1.3.2");
        assert_eq!(op.name(), "NONE");
        assert_eq!(op.version(), "NONE");
    }
}
