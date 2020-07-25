use super::*;

// test that we are getting what we expect when we call
// Build::defauot()
#[test]
pub fn can_construct_default() {
    let result = Build::default();
    let expected = Build {
        clean: false,
        with_docs: true,
        dry_run: false,
        dist_dir: None,
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

// .with_docs test
#[test]
pub fn can_set_with_docs() {
    let mut result = Build::default();
    result.with_docs(false);
    let expected = Build {
        clean: false,
        with_docs: false, // set by with_docs above
        dry_run: false,
        dist_dir: None,
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

#[test]
pub fn can_set_dry_run() {
    let mut result = Build::default();
    result.dry_run(true);
    let expected = Build {
        clean: false,
        with_docs: true, // set by with_docs above
        dry_run: true,
        dist_dir: None,
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn can_set_dist_dir() {
    let mut result = Build::default();
    result.dist_dir(Some("foo/bar"));
    let expected = Build {
        clean: false,
        with_docs: true, // set by with_docs above
        dry_run: false,
        dist_dir: Some("foo/bar".to_string()),
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
    // now test it with a String
    let mut result = Build::default();
    result.dist_dir(Some("foo/bar".to_string()));
    assert_eq!(result, expected);
}

#[test]
fn can_set_flavors() {
    let mut result = Build::default();
    result.flavors(Some(vec![
        Flavor::Vanilla,
        Flavor::Named("foo".to_string()),
    ]));
    let mut flavs = HashSet::new();
    flavs.insert(Flavor::Vanilla);
    flavs.insert(Flavor::Named("foo".to_string()));
    let expected = Build {
        clean: false,
        with_docs: true, // set by with_docs above
        dry_run: false,
        dist_dir: None,
        flavors: Some(flavs),
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn setting_flavors_none_clears() {
    let mut result = Build::default();
    result.flavors(Some(vec![
        Flavor::Vanilla,
        Flavor::Named("foo".to_string()),
    ]));
    result.flavors(None);
    let expected = Build {
        clean: false,
        with_docs: true, // set by with_docs above
        dry_run: false,
        dist_dir: None,
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn can_set_platforms_from_strs() {
    let mut result = Build::default();
    result.platforms(Some(vec!["cent7", "cent6"])).unwrap();

    let mut pforms = HashSet::new();
    pforms.insert(Platform::Cent7_64);
    pforms.insert(Platform::Cent6_64);

    let expected = Build {
        clean: false,
        with_docs: true, // set by with_docs above
        dry_run: false,
        dist_dir: None,
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: Some(pforms),
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn can_set_platforms() {
    let mut result = Build::default();
    result
        .platforms(Some(vec![Platform::Cent6_64, Platform::Cent7_64]))
        .unwrap();

    let mut pforms = HashSet::new();
    pforms.insert(Platform::Cent7_64);
    pforms.insert(Platform::Cent6_64);

    let expected = Build {
        clean: false,
        with_docs: true, // set by with_docs above
        dry_run: false,
        dist_dir: None,
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: Some(pforms),
        verbose: false,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn can_set_verbose() {
    let mut result = Build::default();
    result.verbose(true);
    let expected = Build {
        clean: false,
        with_docs: true,
        dry_run: false,
        dist_dir: None,
        flavors: None,
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: true,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn can_build() {
    let result = Build::default()
        .with_docs(false)
        .dry_run(true)
        .dist_dir(Some("foo/bar"))
        .flavors(Some(vec![Flavor::Vanilla]))
        .verbose(true)
        .build();
    let mut flavs = HashSet::new();
    flavs.insert(Flavor::Vanilla);
    let expected = Build {
        clean: false,
        with_docs: false, // set by with_docs above
        dry_run: true,
        dist_dir: Some("foo/bar".to_string()),
        flavors: Some(flavs),
        level: None,
        metadata_only: false,
        overrides: None,
        platforms: None,
        verbose: true,
        defines: None,
        work: false,
    };
    assert_eq!(result, expected);
}
