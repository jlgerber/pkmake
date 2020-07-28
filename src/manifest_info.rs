//! ManifestInfo
//!
//! A struct whihc provides a minimal amount of information from the package manifest, as required
//! by the rest of the pk-make implementation.

// Internal crate imports
use crate::Flavor;

// External ctate imports
use serde::Deserialize;
use anyhow::Error as AnyError;
use std::path::Path;


/// Minimal manifest information in a form that is convenient for us to consume. This
/// struct is generated after parsing the manifest using serde...
#[derive(Debug, PartialEq, Eq)]
pub struct ManifestInfo {
    pub name: String,
    pub version: String,
    pub flavors: Vec<Flavor>,
}

impl ManifestInfo {
    /// Generate a ManifestInfo from a &Path
    pub fn from_path(manifest: &Path) -> Result<ManifestInfo, AnyError> {
        Ok(Manifest::from_path(&manifest)?.to_info()?)
    }
    /// Retrieve the name of the package from the manifest.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// Retrieve the package version from the manifest.
    pub fn version(&self) -> &str {
        self.version.as_str()
    }
    //
    // THIS IS AN ALTERNATE VERSION WHICH RELIES ON PK MANIFEST INSTEAD OF 
    // SERDE. 
    /* 
    // retrieve the name and version as a tuple
    fn get_name_and_version(manifest: &Path) -> Result<(String, String), AnyError> {
        match _get_name_and_version(manifest.to_str().unwrap()) {
            Ok(mut result) => {
                if result.len() == 2 {
                    let mut name = result.pop().unwrap();
                    name.pop(); // remove \n
                    let mut version = result.pop().unwrap();
                    version.pop();
                    Ok((name, version))
                } else {
                    Err(anyhow!("wrong number of items returned"))
                }
            }
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
    */
}

//
//  Serde Structures
//

/// Minimal flavour information from manifest. This is populated by serde
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Flavour {
    #[serde(alias = "Name")]
    name: String,
}

impl Flavour {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

/// Struct to deserialize the part of the manfiest in which
/// we are interested. The struct implements a method which produces
/// a ManifestInfo struct and consumes itself in the process.
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Manifest {
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Version")]
    version: String,
    #[serde(alias = "Flavours")]
    flavours: Option<Vec<Flavour>>,
}

impl Manifest {
    /// Generate a Manifest instance from a Path
    pub fn from_path(mani: &Path) -> Result<Manifest, AnyError> {
        let contents = std::fs::read_to_string(mani)?;
        let manifest: Manifest = serde_yaml::from_str(&contents)?;
        Ok(manifest)
    }

    /// Generate a ManifestInfo from a Manifest, consuming self in the process
    pub fn to_info(self) -> Result<ManifestInfo, crate::PkMakeError> {
        let flavors: Result<Vec<_>, _> = self
            .flavours
            .unwrap_or_else(|| {
                vec![Flavour {
                    name: "^".to_string(),
                }]
            })
            .iter()
            .map(|v| Flavor::from(v.as_str()))
            .collect();
        let flavors = flavors?;
        Ok(ManifestInfo {
            name: self.name.clone(),
            version: self.version,
            flavors,
        })
    }
}

//
// Import Tests
//
#[cfg(test)]
#[path = "./unit_tests/manifest_info_test.rs"]
mod manifest_info_test;
