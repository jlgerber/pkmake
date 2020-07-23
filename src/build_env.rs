use std::path::PathBuf;
use crate::flavor::Flavor;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use crate::vcs::Vcs;
use crate::platform::Platform;
use std::str::FromStr;

pub struct ManifestInfo {
    pub name: String,
    pub version: String,
    pub flavors: Vec<Flavor>,
}
pub struct BuildEnv {
    pub package_root: PathBuf,
    pub dd_os: Platform,
    pub private_dir: PathBuf,
    pub build_dir: PathBuf,
    pub dist_dir: PathBuf,
    pub vcs: Vcs,
    pub manifest: PathBuf,
}

const MANIFESTS: [&'static str;2] = ["manifest.yaml","pk.yaml"];

impl BuildEnv {
    /// New up a BuildEnv given a path to the root directory of a project's 
    /// source code, wherein the manifest should live, along with the vcs 
    /// directory.
    pub fn new<I>(package_root: I) -> Result<Self, AnyError> 
    where
        I: Into<PathBuf>
    {    
        let package_root = package_root.into();

        if !package_root.exists() {
            return Err(anyhow!("Path: '{:?}' does not exist", package_root));
    
        }
        
        let root = package_root.clone();

        let dd_os = Platform::from_str(&std::env::var("DD_OS")?)?;
        
       // calc private dir
        let mut private_dir = root.clone();
        private_dir.push("private");

        // build dir
        let mut build_dir = root.clone();
        build_dir.push("build");
        
        // dist dir 
        let mut dist_dir = root.clone();
        dist_dir.push("dist");

        let vcs = Vcs::from_str(&root.to_str().unwrap())?;

        let manifest = Self::get_manifest(package_root.clone())?;

        Ok(BuildEnv{
            package_root,
            dd_os,
            private_dir,
            build_dir,
            dist_dir,
            vcs,
            manifest
        })
    }
    // retrieve the manifest if possible
    fn get_manifest(mut manifest: PathBuf) -> Result<PathBuf, AnyError> {
        let mut found = false;
        for m in &MANIFESTS {
            manifest.push(m);
            if manifest.exists() {
                found = true;
                break;
            }
            manifest.pop();
        }
        if !found {
            // one last place to chek
            manifest.push("manifest");
            manifest.push("manifest");
            if !manifest.exists() {
                manifest.pop();
                manifest.pop();
                return Err(anyhow!("Manifest not found at root '{:?}'",manifest));
            }
        }
        Ok(manifest) 
    }
}

