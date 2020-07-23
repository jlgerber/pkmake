use anyhow::Error as AnyError;
use anyhow::anyhow;
use shellfn::shell;
use std::path::Path;
use crate::Flavor;
use serde::Deserialize;

#[derive(Debug,PartialEq,Eq)]
pub struct ManifestInfo {
    pub name: String,
    pub version: String,
    pub flavors: Vec<Flavor>,
}

impl ManifestInfo {
    pub fn from_path(manifest: &Path) -> Result<ManifestInfo, AnyError>
    {
        Err(anyhow!("not implemented"))    
    }
    // retrieve the name and version as a tuple 
    fn get_name_and_version(manifest: &Path) -> Result<(String,String),AnyError> {
       match _get_name_and_version(manifest.to_str().unwrap()) {
            Ok(mut result) => {
                if result.len() == 2 {
                    let mut name = result.pop().unwrap();
                    name.pop(); // remove \n
                    let mut version = result.pop().unwrap();
                    version.pop();
                    Ok((name,version))
                } else {
                    Err(anyhow!("wrong number of items returned"))
                }
            },
            Err(e) => Err(anyhow!("{}",e))
       } 
    }
}

#[shell]
fn _get_name_and_version(manifest: &str) -> Result<Vec<String>,AnyError> {
    r#"
    echo `pk namifest --field=name,version -f $MANIFEST`
    "#
}

/// minimal flavour information from manifest
#[derive(Debug,PartialEq,Eq,Hash,Deserialize)]
pub struct Flavour {
    name: String
}
impl Flavour {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}
#[derive(Debug,PartialEq,Eq,Hash,Deserialize)]
pub struct Manifest {
    name: String,
    version: String,
    flavours: Option<Vec<Flavour>>
}

impl Manifest {
    /// Generate a Manifest instance from a Path
    pub fn from_path(mani: &Path) -> Result<Manifest, AnyError> {
        let contents= std::fs::read_to_string(mani)?;
        let manifest: Manifest = serde_yaml::from_str(&contents)?;
        Ok(manifest)
        
    }
    /// Genreate a ManifestInfo from a Manifest
    pub fn to_info(self) -> ManifestInfo {
        let flavors = self.flavours
                        .unwrap_or(vec![Flavour{name: "^".to_string()}])
                        .iter()
                        .map(|ref v| Flavor::from(v.as_str()))
                        .collect::<Vec<_>>();
        ManifestInfo {
            name: self.name.clone(),
            version: self.version.clone(),
            flavors: flavors
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn can_read_manifest_with_flavors() {
        let mut manifest= PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest.push("egs"); 
        manifest.push("manifests");
        manifest.push("flavored");
        manifest.push("manifest.yaml");
        let result = Manifest::from_path(&manifest);
        //assert_eq!(format!("{:?}",result).as_str(), "");
        assert!(result.is_ok());
    }

    #[test]
    fn can_make_manifest_info_frm_mani_with_flavors() {
        let mut manifest= PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest.push("egs"); 
        manifest.push("manifests");
        manifest.push("flavored");
        manifest.push("manifest.yaml");
        let result = Manifest::from_path(&manifest).unwrap().to_info();
        let expected = ManifestInfo{
            name: "vrayddbase".into(),
            version: "5.0.8".into(),
            flavors: vec![
                Flavor::Named("vray4.0.29259_for_maya2018".into()), 
                Flavor::Named("vray4.0.29567_for_maya2018".into()), 
                Flavor::Named("vray4.0.29635_for_maya2018".into()), 
                Flavor::Named("vray4.0.29727_for_maya2018".into()), 
                Flavor::Named("vray4.0.29747_for_maya2018".into()), 
                Flavor::Named("vray4.0.29789_for_maya2018".into()), 
                Flavor::Named("vray4.0.29799_for_maya2018".into()), 
                Flavor::Named("vray4.0.29849_for_maya2018".into()), 
                Flavor::Named("vray4.0.29904_for_maya2018".into()), 
                Flavor::Named("vray4.0.29935_for_maya2018".into()), 
                Flavor::Named("vray4.0.30046_for_maya2018".into()), 
                Flavor::Named("vray4.0.30046_for_maya2020".into())]
        };
        assert_eq!(result, expected);    
    }
    
    #[test]
    fn can_read_manifest_without_flavors() {
        let mut manifest= PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest.push("egs"); 
        manifest.push("manifests");
        manifest.push("nonflavored");
        manifest.push("manifest.yaml");
        let result = Manifest::from_path(&manifest);
        //assert_eq!(format!("{:?}",result).as_str(), "");
        assert!(result.is_ok());
    }

    #[test]
    fn can_make_manifest_info_frm_mani_without_flavors() {
        let mut manifest= PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest.push("egs"); 
        manifest.push("manifests");
        manifest.push("nonflavored");
        manifest.push("manifest.yaml");
        let result = Manifest::from_path(&manifest).unwrap().to_info();
        let expected = ManifestInfo{
            name: "makebridge".into(),
            version: "3.1.0".into(),
            flavors: vec![
                Flavor::Vanilla,
            ]
        };
        assert_eq!(result, expected);    
    }
 
}
