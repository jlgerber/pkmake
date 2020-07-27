//
// Unit tests for install.rs
//
use super::*;

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
fn can_build() {
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
        .logfile(Some("./log/logfile"))
        .max_jobs(Some(8))
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
        work: false,
        vcs: None,
        logfile: Some(PathBuf::from("./log/logfile")),
        max_jobs: Some(8),
        package_root: None,
    };
    assert_eq!(result, expected);
}
