use super::*;
use serial_test::serial;
use crate::utils::setup_manifest_dir;
use std::env;
use indexmap::IndexSet as HashSet;

#[test]
fn can_build_default() {
    let result = Docs::default();
    let expected = Docs {
        dry_run: false,
        dist_dir: None,
        verbose: false,
        defines: None,
        flavors: None,
        platforms: None,
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn can_update_build() -> Result<(), Box<dyn std::error::Error>> {
    let result = Docs::default()
        .dist_dir(Some("foo/bar"))
        .dry_run(true)
        .verbose(true)
        .defines(Some(vec!["foo=bar"]))
        .package_root(Some("./foo/bar"))
        .flavors(Some(vec!["^"]))?
        .platforms(Some(vec!["cent6", "cent7"]))?
        .build();

        let mut flavor_hs = HashSet::new();
        flavor_hs.insert(Flavor::Vanilla);

        let mut platform_hs = HashSet::new();
        platform_hs.insert(Platform::Cent6_64);
        platform_hs.insert(Platform::Cent7_64);
         
    let expected = Docs {
        dist_dir: Some("foo/bar".to_string()),
        dry_run: true,
        verbose: true,
        defines: Some(vec!["foo=bar".to_string()]),
        flavors: Some(flavor_hs),
        platforms: Some(platform_hs),
        package_root: Some(std::path::PathBuf::from("./foo/bar")),
    };
    assert_eq!(result, expected);
    Ok(())
}

//
// build_cmp
//

#[test]
#[serial]
fn build_cmp_given_verbose()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Docs::default()
        .verbose(true)
        .build_cmd()?;
    //let docs_ref: Vec<&str> = docs.iter().map(AsRef::as_ref).collect();
    // NB verbose does not get passed through to the recipe as it is not part
    // of the run-recipe interface
    assert_eq!(docs,
    vec![format!("pk run-recipe docs")]
    );
    Ok(())
}

#[test]
#[serial]
fn build_cmp_given_defines()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Docs::default()
        .defines(Some(vec!["foo=bar","bla=blabla"]))
        .build_cmd()?;
    //let docs_ref: Vec<&str> = docs.iter().map(AsRef::as_ref).collect();
    // NB verbose does not get passed through to the recipe as it is not part
    // of the run-recipe interface
    assert_eq!(docs,
    vec![format!("pk run-recipe docs --define=foo=bar --define=bla=blabla")]
    );
    Ok(())
}

#[test]
#[serial]
fn build_cmp_given_distdir()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Docs::default()
        .dist_dir(Some("./some/dir"))
        .build_cmd()?;
    //let docs_ref: Vec<&str> = docs.iter().map(AsRef::as_ref).collect();
    // NB verbose does not get passed through to the recipe as it is not part
    // of the run-recipe interface
    assert_eq!(docs,
    vec![format!("pk run-recipe docs --dist-dir=./some/dir")]
    );
    Ok(())
}

#[test]
#[serial]
fn build_cmp_given_packageroot()  -> Result<(),Box<dyn std::error::Error>> {
    let root = setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Docs::default()
        .package_root(Some(root))
        .build_cmd()?;
    //let docs_ref: Vec<&str> = docs.iter().map(AsRef::as_ref).collect();
    // NB package_root does not get passed through to the recipe. It is however 
    // necessary to discover the manifest and is a valid argument none the less
    assert_eq!(docs,
    vec![format!("pk run-recipe docs")]
    );
    Ok(())
}


#[test]
#[serial]
fn build_cmp_given_flavors()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    
    let docs = Docs::default()
        .flavors(Some(vec!["^", "red"]))?
        .build_cmd()?;
    
    assert_eq!(docs,
    vec![format!("pk run-recipe docs --flavour=^,red")]
    );
    Ok(())
}


#[test]
#[serial]
fn build_cmp_given_platforms()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    
    // note that i am using a platform shorthand supported by 
    // Platform::TryFrom<&str> (that is dropping the _64)
    let docs = Docs::default()
        .platforms(Some(vec!["cent6", "cent7"]))?
        .build_cmd()?;
    
    assert_eq!(docs,
    vec![format!("pk run-recipe docs --platform=cent6_64,cent7_64")]
    );
    Ok(())
}