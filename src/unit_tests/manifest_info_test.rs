use super::*;
use std::path::PathBuf;

#[test]
fn can_read_manifest_with_flavors() {
    let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
    let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.push("egs");
    manifest.push("manifests");
    manifest.push("flavored");
    manifest.push("manifest.yaml");
    let result = Manifest::from_path(&manifest).unwrap().to_info();
    let expected = ManifestInfo {
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
            Flavor::Named("vray4.0.30046_for_maya2020".into()),
        ],
    };
    assert_eq!(result.unwrap(), expected);
}
#[test]
fn can_read_manifest_without_flavors() {
    let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
    let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.push("egs");
    manifest.push("manifests");
    manifest.push("nonflavored");
    manifest.push("manifest.yaml");
    let result = Manifest::from_path(&manifest).unwrap().to_info();
    let expected = ManifestInfo {
        name: "makebridge".into(),
        version: "3.1.0".into(),
        flavors: vec![Flavor::Vanilla],
    };
    assert_eq!(result.unwrap(), expected);
}
