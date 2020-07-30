//
// Unit tests for install.rs
//
use super::*;
use std::env;
use serial_test::serial;

use crate::utils::setup_manifest_dir;
use crate::utils::setup_manifest_dir2;

// This tests that we can construct an appropriate default
#[test]
fn can_construct_default() {
    let result = Install::default();
    let expected = Install {
        dry_run: false,
        with_docs: true,
        build_dir: None,
        context: None,
        show: None,
        sites: None,
        platforms: None,
        flavors: None,
        verbose: false,
        clean: false,
        dist_dir: None,
        level: None,
        overrides: None,
        defines: None,
        work: false,
        vcs: None,
        logfile: None,
        max_jobs: None,
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn build_given_context_and_show() {
    let result = Install::default()
        .clean(true)
        .dry_run(true)
        .with_docs(false)
        .build_dir(Some("foo/bar"))
        .context(Some("facility"))
        .unwrap()
        .show(Some("dev01"))
        .sites(Some(vec!["all"]))
        .unwrap()
        .platform(Some("cent7"))
        .unwrap()
        .flavor(Some("^"))
        .unwrap()
        .verbose(true)
        .dist_dir(Some("./foo/bar"))
        // cannot set level and show/context
        //.level(Some("DEV01.RD.0001"))
        .overrides(Some(vec!["foo=1.2.3"]))
        .unwrap()
        .defines(Some(vec!["bla=mange"]))
        .work(true)
        .logfile(Some("./log/logfile"))
        .max_jobs(Some(8))
        .package_root(Some("./some/root"))
        .build();

    let mut site_hs = HashSet::new();
    site_hs.insert(Site::All);

    let mut platforms_hs = HashSet::new();
    platforms_hs.insert(Platform::Cent7_64);

    let mut flavors_hs = HashSet::new();
    flavors_hs.insert(Flavor::Vanilla);

    let expected = Install {
        clean: true,
        dry_run: true,
        with_docs: false,
        build_dir: Some("foo/bar".to_string()),
        context: Some(Context::Facility),
        show: Some("dev01".to_string()),
        sites: Some(site_hs),
        platforms: Some(platforms_hs),
        flavors: Some(flavors_hs),
        verbose: true,
        dist_dir: Some("./foo/bar".to_string()),
        level: None,
        overrides: Some(vec![OverridePair::from("foo=1.2.3").unwrap()]),
        defines: Some(vec!["bla=mange".to_string()]),
        work: true,
        vcs: None,
        logfile: Some(PathBuf::from("./log/logfile")),
        max_jobs: Some(8),
        package_root: Some(PathBuf::from("./some/root")),
    };
    assert_eq!(result, expected);
}

#[test]
fn vcs_given_str() {
    let mut target = setup_manifest_dir(false);
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let mut result = Install::default();
    let result = result.vcs(Some("git"));
    assert!(result.is_ok());
}

#[test]
fn build_given_level() {
    let result = Install::default()
        .clean(true)
        .dry_run(true)
        .with_docs(false)
        .build_dir(Some("foo/bar"))
        .sites(Some(vec!["all"]))
        .unwrap()
        .platform(Some("cent7"))
        .unwrap()
        .flavor(Some("^"))
        .unwrap()
        .verbose(true)
        .dist_dir(Some("./foo/bar"))
        // cannot set level and show/context
        .level(Some("DEV01.RD.0001"))
        .unwrap()
        .overrides(Some(vec!["foo=1.2.3"]))
        .unwrap()
        .defines(Some(vec!["bla=mange"]))
        .work(true)
        .logfile(Some("./log/logfile"))
        .max_jobs(Some(8))
        .package_root(Some("./some/root"))
        .build();

    let mut site_hs = HashSet::new();
    site_hs.insert(Site::All);

    let mut platforms_hs = HashSet::new();
    platforms_hs.insert(Platform::Cent7_64);

    let mut flavors_hs = HashSet::new();
    flavors_hs.insert(Flavor::Vanilla);

    let expected = Install {
        clean: true,
        dry_run: true,
        with_docs: false,
        build_dir: Some("foo/bar".to_string()),
        context: None,
        show: None,
        sites: Some(site_hs),
        platforms: Some(platforms_hs),
        flavors: Some(flavors_hs),
        verbose: true,
        dist_dir: Some("./foo/bar".to_string()),
        level: Some("DEV01.RD.0001".into()),
        overrides: Some(vec![OverridePair::from("foo=1.2.3").unwrap()]),
        defines: Some(vec!["bla=mange".to_string()]),
        work: true,
        vcs: None,
        logfile: Some(PathBuf::from("./log/logfile")),
        max_jobs: Some(8),
        package_root: Some(PathBuf::from("./some/root")),
    };
    assert_eq!(result, expected);
}


#[test]
fn build_given_level_and_context_fails() {
    
    let result = || -> Result<Install,Box<dyn std::error::Error>> {  let r = Install::default()
        .clean(true)
        .dry_run(true)
        .with_docs(false)
        .build_dir(Some("foo/bar"))
        .sites(Some(vec!["all"]))
        ?
        .platform(Some("cent7"))
        ?
        .flavor(Some("^"))
        .unwrap()
        .verbose(true)
        .dist_dir(Some("./foo/bar"))
        .level(Some("DEV01.RD.0001"))?
        .context(Some("facility"))?        
        .overrides(Some(vec!["foo=1.2.3"]))
        .unwrap()
        .defines(Some(vec!["bla=mange"]))
        .work(true)
        .logfile(Some("./log/logfile"))
        .max_jobs(Some(8))
        .package_root(Some("./some/root"))
        .build();
        Ok(r)

    }();

   
    assert!(result.is_err());
}


#[test]
fn build_given_context_and_level_fails() {
    
    let result = || -> Result<Install,Box<dyn std::error::Error>> {  let r = Install::default()
        .clean(true)
        .dry_run(true)
        .with_docs(false)
        .build_dir(Some("foo/bar"))
        .sites(Some(vec!["all"]))
        ?
        .platform(Some("cent7"))
        ?
        .flavor(Some("^"))
        .unwrap()
        .verbose(true)
        .dist_dir(Some("./foo/bar"))
        .context(Some("facility"))
        ?
        // cannot set level and show/context
        .level(Some("DEV01.RD.0001"))?
        .overrides(Some(vec!["foo=1.2.3"]))
        .unwrap()
        .defines(Some(vec!["bla=mange"]))
        .work(true)
        .logfile(Some("./log/logfile"))
        .max_jobs(Some(8))
        .package_root(Some("./some/root"))
        .build();
        Ok(r)

    }();
    assert!(result.is_err());
}

//
// build_cmd tests
//

#[test]
#[serial]
fn build_cmd_given_default() {
    let root = setup_manifest_dir2("nonflavored_git");
    let mut target = root.clone();
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default().package_root(Some(root)).build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent7_64".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}", target.to_str().unwrap()),

    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean() {
    let root = setup_manifest_dir2("nonflavored_git");
    let mut target = root.clone();
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default().clean(true).package_root(Some(&root)).build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --with-docs --platform=cent7_64".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}", target.to_str().unwrap()),
    ];

    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_distdir() {
    let root = setup_manifest_dir2("nonflavored_git");
    let mut target = root.clone();
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .dist_dir(Some("./foo/bar"))
        //.package_root(Some(root))
        .build_cmd();
    let expected =
        vec!["pk audit && pk build --dist-dir=./foo/bar --with-docs --platform=cent7_64".to_string(),
        "pk install --level=DEV01.work --site=local --platform=cent7_64 ./foo/bar/makebridge-3.1.0".to_string(),
        ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_flavors() {
    let root = setup_manifest_dir(true);
    let mut target = root.clone();
    target.push("private"); target.push("dist"); target.push("vrayddbase-5.0.8");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        //.package_root(Some("."))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --flavour=^,foo --platform=cent7_64".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}", target.to_str().unwrap()),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}_foo", target.to_str().unwrap())
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_platforms() {
    let root = setup_manifest_dir2("nonflavored_git");
    let mut target = root.clone();
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        //.package_root(Some(root))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent6_64,cent7_64".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent6_64,cent7_64 {}", target.to_str().unwrap()),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_showlevel() {
    let mut target = setup_manifest_dir(false);
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .level(Some("DEV01")).unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent7_64".to_string(),
        format!("pk install --level=DEV01 --site=local --platform=cent7_64 {}", target.to_str().unwrap())
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_worklevel() {
    let mut target = setup_manifest_dir(false);
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .level(Some("DEV01.work")).unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent7_64".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}", target.to_str().unwrap())
    ];
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_overrides() {
    let mut target = setup_manifest_dir(false);
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
    .overrides(Some(vec!["make=2.0.0","bs=2.1.0"]))
        .unwrap()
        .build_cmd();

    let expected = vec![
        "pk audit && pk build --with-docs --override=make=2.0.0,bs=2.1.0 --platform=cent7_64".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}", target.to_str().unwrap())
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_defines() {
    let root = setup_manifest_dir2("nonflavored_git");
    let mut target = root.clone();
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .defines(Some(vec!["foo=bar", "la=deda"]))
        .package_root(Some(root))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent7_64 -D=foo=bar -D=la=deda".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}", target.to_str().unwrap())
    ];
    assert_eq!(result.unwrap(), expected);
}

// verify that setting the show and context overrides the default level which is set to SHOW.work
#[test]
#[serial]
fn build_cmd_given_defines_show_and_context() {
    let root = setup_manifest_dir2("nonflavored_git");
    let mut target = root.clone();
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");    
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .defines(Some(vec!["foo=bar", "la=deda"]))
        .show(Some("DEV01"))
        .package_root(Some(root))
        .context(Some("shared")).unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent7_64 -D=foo=bar -D=la=deda".to_string(),
        format!("pk install --level=DEV01 --site=local --platform=cent7_64 {}", target.to_str().unwrap())
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_verbose() {
    let mut target = setup_manifest_dir(false);
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");

    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Install::default()
        .verbose(true)
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent7_64 --verbose".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 --verbose {}", target.to_str().unwrap() )
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_work() {
    let mut target = setup_manifest_dir(false);
    target.push("private"); target.push("dist"); target.push("makebridge-3.1.0");

    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Install::default()
        .work(true)
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent7_64 --work".to_string(),
        format!("pk install --level=DEV01.work --site=local --platform=cent7_64 {}", target.to_str().unwrap() )
    ];
    assert_eq!(result.unwrap(), expected);
}
//
//

#[test]
#[serial]
fn build_cmd_given_facility_level_git() {
    let mut target = setup_manifest_dir(false);
    // this is a super ugly test. 
    target.pop();
    target.push("nonflavored_git");

    let mut source = target.clone();
    source.push(".gittest");
    let mut dest = target.clone();
        dest.push(".git");

    if source.exists() {
        
        std::fs::rename(&source, &dest).unwrap_or(());
    }
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .vcs(Some("git")).unwrap()
        .package_root(Some(&target))
        .level(Some("facility")).unwrap()
        .build_cmd();
    let expected = vec![
        "git-tag create --protect".to_string(),
    ];
    // move this back before testing - only uncomment if you can run tests in single threaded mode
    // if dest.exists() {
    //     std::fs::rename(dest, source).unwrap_or(());
    // }
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_facility_level_svn() {
    let mut target = setup_manifest_dir(false);
    // this is a super ugly test. 
    target.pop();
    target.push("nonflavored_svn");


    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .package_root(Some(&target))
        .level(Some("facility")).unwrap()
        .build_cmd();
    let expected = vec![
        "svn-tag create".to_string(),
    ];
    
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_facility_level_both_explicit_svn() {
    let  root = setup_manifest_dir2("nonflavored_both");
    // this is a super ugly test. 
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .package_root(Some(root))
        .level(Some("facility")).unwrap()
        .vcs(Some("svn")).unwrap()
        .build_cmd();
    let expected = vec![
        "svn-tag create".to_string(),
    ];
    
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_facility_level_both_set_git() {
    let mut target = setup_manifest_dir(false);
    // this is a super ugly test. 
    target.pop();
    target.push("nonflavored_both");

    let mut source = target.clone();
    source.push(".gittest");
    let mut dest = target.clone();
        dest.push(".git");

    if source.exists() {
        
        std::fs::rename(&source, &dest).unwrap_or(());
    }
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");

    let result = Install::default()
        .vcs(Some("git")).unwrap()
        .package_root(Some(&target))
        .level(Some("facility")).unwrap()
        .build_cmd();
    let expected = vec![
        "git-tag create --protect".to_string(),
    ];
    // // move this back before testing
    // if dest.exists() {
    //     std::fs::rename(dest, source).unwrap_or(());
    // }
    assert_eq!(result.unwrap(), expected);
}
