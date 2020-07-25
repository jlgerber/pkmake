use crate::PkMakeError;
//use anyhow::anyhow;
//use anyhow::Error as AnyError;
use std::convert::TryFrom;
//use lazy_static::lazy_static;
//use regex::Regex;
use crate::utils::*;
use std::str::FromStr;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
        let pieces = input.split("=").collect::<Vec<_>>();
        if pieces.len() != 2 {
            // return Err(anyhow!(
            //     "Invalid override specification: <package>=<version> '{}'",
            //     input
            // ));
            return Err(PkMakeError::ConvertFrom(input.to_string()));
        }
        Ok(Self {
            value: input.to_string(),
            name_end: pieces[0].chars().count() as u16,
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
// impl From<&str> for OverridePair {
//     fn from(input: &str) -> Self {
//         match Self::from_str(input) {
//             Ok(p) => p,
//             Err(_) => Self {
//                 value: "NONE=NONE".into(),
//                 name_end: 4,
//                 length: 9,
//             },
//         }
//     }
// }

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
        let op = OverridePair::try_from("foo1.3.2");
        assert!(op.is_err());
    }
}
