//!
//! Platform
//!
//! representation of build platforms
//!
use crate::PkMakeError;
use anyhow::anyhow;
use anyhow::Error as AnyhowError;
use std::convert::TryFrom;
use std::str::FromStr;

/*
fc4_32
deb4_64
cent5_64
osx10_64
cent6_64
cent7_64
xp_32
xp_64
win7_64
win10_64

 */
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Platform {
    Win7_64,
    Win10_64,
    Osx10_64,
    Cent6_64,
    Cent7_64,
    Cent8_64,
    //Unknown(String),
}

impl Platform {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Win7_64 => "win7_64",
            Self::Win10_64 => "win10_64",
            Self::Osx10_64 => "osx10_64",
            Self::Cent6_64 => "cent6_64",
            Self::Cent7_64 => "cent7_64",
            Self::Cent8_64 => "cent8_64",
            //Self::Unknown(ref s) => s.as_str(),
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
impl FromStr for Platform {
    type Err = PkMakeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "win7_64" | "win7" => Ok(Self::Win7_64),
            "win10_64" | "win10" => Ok(Self::Win10_64),
            "osx10_64" | "osx10" => Ok(Self::Osx10_64),
            "cent6_64" | "cent6" => Ok(Self::Cent6_64),
            "cent7_64" | "cent7" => Ok(Self::Cent7_64),
            "cent8_64" | "cent8" => Ok(Self::Cent8_64),
            _ => Err(PkMakeError::InvalidPlatform(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Platform {
    type Error = PkMakeError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::from_str(input).map_err(|_e| PkMakeError::InvalidPlatform(input.to_string()))
    }
}

// impl From<&str> for Platform {
//     fn from(other: &str) -> Self {
//         match Platform::from_str(other) {
//             Ok(val) => val,
//             Err(_) => Self::Unknown(other.to_string()),
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_win7() {
        let wins = vec!["win7", "WIN7", "win7_64", "WIN7_64"];
        for win in wins {
            let result = Platform::from_str(win);
            assert_eq!(result.unwrap(), Platform::Win7_64);
        }
    }

    #[test]
    fn from_win10() {
        let wins = vec!["win10", "WIN10", "win10_64", "WIN10_64"];
        for win in wins {
            let result = Platform::from_str(win);
            assert_eq!(result.unwrap(), Platform::Win10_64);
        }
    }

    #[test]
    fn from_cent6() {
        let wins = vec!["cent6", "CENT6", "cent6_64", "CENT6_64"];
        for win in wins {
            let result = Platform::from_str(win);
            assert_eq!(result.unwrap(), Platform::Cent6_64);
        }
    }

    #[test]
    fn from_cent7() {
        let wins = vec!["cent7", "CENT7", "cent7_64", "CENT7_64"];
        for win in wins {
            let result = Platform::from_str(win);
            assert_eq!(result.unwrap(), Platform::Cent7_64);
        }
    }
    #[test]
    fn from_cent8() {
        let wins = vec!["cent8", "CENT8", "cent8_64", "CENT8_64"];
        for win in wins {
            let result = Platform::from_str(win);
            assert_eq!(result.unwrap(), Platform::Cent8_64);
        }
    }

    #[test]
    fn from_trait_impl() {
        let plats = vec!["cent8", "Cent8", "cent8_64"];
        for plat in plats {
            let result = Platform::from(plat).unwrap();
            assert_eq!(result, Platform::Cent8_64);
        }
        // test a bad input
        let result = Platform::from("foobarbla");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_strs() {
        let invalids = vec!["CentyWenty", "Cent 7", "125", "Cent7$%$@"];
        for invalid in invalids {
            let result = Platform::from_str(invalid);
            assert!(result.is_err());
        }
    }
}
