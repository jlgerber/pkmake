use super::*;
use serial_test::serial;
use crate::utils::setup_manifest_dir;
use std::env;
use indexmap::IndexSet as HashSet;

#[test]
fn can_construct_default() {
    let result = Test::default();
    let expected = Test {
        dist_dir: None,
        dry_run: false,
        verbose: false,
        defines: None,
        platforms: None,
        flavors: None,
        package_root: None,
    };

    assert_eq!(result, expected);
}

#[test]
fn can_modify_and_build() {
    let result = Test::default()
        .dist_dir(Some("foo/bar"))
        .dry_run(true)
        .verbose(true)
        .platforms(Some(vec!["cent7_64", "cent6_64"])).unwrap()
        .flavors(Some(vec!["^", "for_maya"])).unwrap()
        .defines(Some(vec!["foo=bar"]))
        .package_root(Some("./foo/bar"))
        .build();

    let mut platform_hset = HashSet::new();
    platform_hset.insert(Platform::Cent7_64);
    platform_hset.insert(Platform::Cent6_64);

    let mut flavor_hset = HashSet::new();
    flavor_hset.insert(Flavor::Vanilla);
    flavor_hset.insert(Flavor::Named(String::from("for_maya")));

    let expected = Test {
        dist_dir: Some("foo/bar".to_string()),
        dry_run: true,
        verbose: true,
        platforms: Some(platform_hset),
        flavors: Some(flavor_hset),
        defines: Some(vec!["foo=bar".to_string()]),
        package_root: Some(std::path::PathBuf::from("./foo/bar")),
    };
    assert_eq!(result, expected);
}

//
// build_cmp 
//

#[test]
#[serial]
fn build_cmp()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .build_cmd()?;
   
    assert_eq!(docs,
    vec![format!("pk run-recipe test")]
    );
    Ok(())
}


#[test]
#[serial]
fn build_cmp_given_distdir()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .dist_dir(Some("./foo/bar"))
        .build_cmd()?;
   
    assert_eq!(docs,
    vec![format!("pk run-recipe test --dist-dir=./foo/bar")]
    );
    Ok(())
}

#[test]
#[serial]
fn build_cmp_given_dry_run()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .dry_run(true)
        .build_cmd()?;
   
    assert_eq!(docs,
    vec![format!("pk run-recipe test --dry-run")]
    );
    Ok(())
}

#[test]
#[serial]
fn build_cmp_given_verbose()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .verbose(true)
        .build_cmd()?;
   
    // verbose does not get emitted by build_cmd
    assert_eq!(docs,
    vec![format!("pk run-recipe test")]
    );
    Ok(())
}


#[test]
#[serial]
fn build_cmp_given_defines()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .defines(Some(vec!["foo=bar","bla=blabla"]))
        .build_cmd()?;
   
    assert_eq!(docs,
    vec![format!("pk run-recipe test --define=foo=bar --define=bla=blabla")]
    );
    Ok(())
}


#[test]
#[serial]
fn build_cmp_given_platforms()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .platforms(Some(vec!["cent6","cent7"]))?
        .build_cmd()?;
   
    assert_eq!(docs,
    vec![format!("pk run-recipe test --platform=cent6_64,cent7_64")]
    );
    Ok(())
}

#[test]
#[serial]
fn build_cmp_given_flavors()  -> Result<(),Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .flavors(Some(vec!["^","red"]))?
        .build_cmd()?;
   
    assert_eq!(docs,
    vec![format!("pk run-recipe test --flavour=^,red")]
    );
    Ok(())
}

#[test]
#[serial]
fn build_cmp_given_packageroot()  -> Result<(),Box<dyn std::error::Error>> {
    let root = setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let docs = Test::default()
        .package_root(Some(root))
        .build_cmd()?;
    
    // NB package_root does not get passed through to the recipe. It is however 
    // necessary to discover the manifest and is a valid argument none the less
    assert_eq!(docs,
    vec![format!("pk run-recipe test")]
    );
    Ok(())
}
