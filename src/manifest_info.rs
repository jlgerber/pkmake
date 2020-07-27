use crate::Flavor;
use anyhow::Error as AnyError;
use serde::Deserialize;
//use shellfn::shell;
use serde_aux::prelude::*;
use std::path::Path;
/// minimal manifest information in a form that is convenient for us to consume. This
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
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn version(&self) -> &str {
        self.version.as_str()
    }
    /* THIS IS AN ALTERNATE VERSION WHICH RELIES ON PK MANIFEST
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
/*
#[shell]
fn _get_name_and_version(manifest: &str) -> Result<Vec<String>, AnyError> {
    r#"
    echo `pk namifest --field=name,version -f $MANIFEST`
    "#
}
*/
/// minimal flavour information from manifest
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Flavour {
    name: String,
}
impl Flavour {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Manifest {
    name: String,
    version: String,
    flavours: Option<Vec<Flavour>>,
}

impl Manifest {
    /// Generate a Manifest instance from a Path
    pub fn from_path(mani: &Path) -> Result<Manifest, AnyError> {
        let contents = std::fs::read_to_string(mani)?;
        let manifest: Manifest = serde_yaml::from_str(&contents)?;
        Ok(manifest)
    }
    /// Genreate a ManifestInfo from a Manifest
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

#[cfg(test)]
#[path = "./unit_tests/manifest_info_test.rs"]
mod manifest_info_test;
