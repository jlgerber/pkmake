use super::*;
use std::env;
use serial_test::serial;

use crate::utils::setup_manifest_dir;

// test that we are getting what we expect when we call
// Build::defauot()
#[test]
pub fn default_produces_instance_with_expected_state() {
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
        package_root: None,
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
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
pub fn dry_run_given_bool_sets_state() {
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
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn dist_dir_given_some_str_updates_state() {
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
        package_root: None,
    };
    assert_eq!(result, expected);
    // now test it with a String
    let mut result = Build::default();
    result.dist_dir(Some("foo/bar".to_string()));
    assert_eq!(result, expected);
}

#[test]
fn dist_dir_given_some_string_updates_state() {
    let mut result = Build::default();
    result.dist_dir(Some("foo/bar".to_string()));
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
        package_root: None,
    };
    assert_eq!(result, expected);
    // now test it with a String
    let mut result = Build::default();
    result.dist_dir(Some("foo/bar".to_string()));
    assert_eq!(result, expected);
}

#[test]
fn flavors_given_some_vec_flavor_updates_state() {
    let mut result = Build::default();
    result
        .flavors(Some(vec![
            Flavor::Vanilla,
            Flavor::Named("foo".to_string()),
        ]))
        .unwrap();

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
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn flavors_given_none_sets_state_to_none() {
    let mut result = Build::default();
    result
        .flavors(Some(vec![
            Flavor::Vanilla,
            Flavor::Named("foo".to_string()),
        ]))
        .unwrap();
    result.flavors(None::<Vec<Flavor>>).unwrap();
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
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn platforms_given_some_vec_str_updates_state() {
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
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn platforms_given_some_vec_platform_updates_state() {
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
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn verbose_given_bool_updates_state() {
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
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn defines_given_vec_str_updates_state() {
    let mut result = Build::default();
    result.defines(Some(vec!["foo=bar", "ba=ba_blacksheep"]));
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
        defines: Some(vec!["foo=bar".to_string(), "ba=ba_blacksheep".to_string()]),
        work: false,
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn defines_given_vec_string_updates_state() {
    let mut result = Build::default();
    result.defines(Some(vec![
        "foo=bar".to_string(),
        "ba=ba_blacksheep".to_string(),
    ]));
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
        defines: Some(vec!["foo=bar".to_string(), "ba=ba_blacksheep".to_string()]),
        work: false,
        package_root: None,
    };
    assert_eq!(result, expected);
}

#[test]
fn build_given_mut_ref_to_self_produces_owned_instance() {
    let result = Build::default()
        .with_docs(false)
        .dry_run(true)
        .dist_dir(Some("foo/bar"))
        .flavors(Some(vec![Flavor::Vanilla]))
        .unwrap()
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
        package_root: None,
    };
    assert_eq!(result, expected);
}

//
// build_cmd
//

#[test]
#[serial]
fn build_cmd_given_default() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default().build_cmd();
    let expected = vec!["pk audit && pk build --with-docs".to_string()];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default().clean(true).build_cmd();
    let expected = vec!["pk audit && pk build --clean --with-docs".to_string()];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_distdir() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .build_cmd();
    let expected =
        vec!["pk audit && pk build --clean --dist-dir=./foo/bar --with-docs".to_string()];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_distdir() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .dist_dir(Some("./foo/bar"))
        .build_cmd();
    let expected =
        vec!["pk audit && pk build --dist-dir=./foo/bar --with-docs".to_string()];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavor() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_flavor() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --flavour=^,foo".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo --platform=cent6_64,cent7_64".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_platforms() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --platform=cent6_64,cent7_64".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms_showlevel() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .level(Some("DEV01"))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo --platform=cent6_64,cent7_64 --level=DEV01".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_showlevel() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .level(Some("DEV01"))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --level=DEV01".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms_worklevel() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .level(Some("DEV01.work"))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo --platform=cent6_64,cent7_64 --level=DEV01.work".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_worklevel() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .level(Some("DEV01.work"))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --level=DEV01.work".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms_showlevel_metadataonly() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .level(Some("DEV01"))
        .metadata_only(true)
        .build_cmd();
    // NOTICE that we do not add --with docs. there is no reason to build docs
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --flavour=^,foo --platform=cent6_64,cent7_64 --level=DEV01 --metadata-only".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_metadataonly() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .metadata_only(true)
        .build_cmd();
    // NOTICE that we do not add --with docs. there is no reason to build docs
    let expected = vec![
        "pk audit && pk build --metadata-only".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms_worklevel_overrides() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .level(Some("DEV01.work"))
        .overrides(Some(vec!["make=2.0.0","bs=2.1.0"]))
        .unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo --platform=cent6_64,cent7_64 --level=DEV01.work --override=make=2.0.0,bs=2.1.0".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_overrides() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
    .overrides(Some(vec!["make=2.0.0","bs=2.1.0"]))
        .unwrap()
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --override=make=2.0.0,bs=2.1.0".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}


// NB that --overrides behaves differently in pk build than most other multi flags in that it is a posix compliant multi flag. you use multiple invocations of the flag
// instead of comma separated list of values with one flag
#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms_worklevel_overrides_defines() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .level(Some("DEV01.work"))
        .overrides(Some(vec!["make=2.0.0","bs=2.1.0"]))
        .unwrap()
        .defines(Some(vec!["foo=bar", "la=deda"]))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo --platform=cent6_64,cent7_64 --level=DEV01.work --override=make=2.0.0,bs=2.1.0 -D=foo=bar -D=la=deda".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

// NB that --overrides behaves differently in pk build than most other multi flags in that it is a posix compliant multi flag. you use multiple invocations of the flag
// instead of comma separated list of values with one flag
#[test]
#[serial]
fn build_cmd_given_defines() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .defines(Some(vec!["foo=bar", "la=deda"]))
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs -D=foo=bar -D=la=deda".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}


#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms_worklevel_overrides_defines_verbose() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .level(Some("DEV01.work"))
        .overrides(Some(vec!["make=2.0.0","bs=2.1.0"]))
        .unwrap()
        .defines(Some(vec!["foo=bar", "la=deda"]))
        .verbose(true)
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo --platform=cent6_64,cent7_64 --level=DEV01.work --override=make=2.0.0,bs=2.1.0 -D=foo=bar -D=la=deda --verbose".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_verbose() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .verbose(true)
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --verbose".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_clean_distdir_flavors_platforms_worklevel_overrides_defines_verbose_work() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .clean(true)
        .dist_dir(Some("./foo/bar"))
        .flavors(Some(vec!["^", "foo"]))
        .unwrap()
        .platforms(Some(vec!["cent6","cent7"]))
        .unwrap()
        .level(Some("DEV01.work"))
        .overrides(Some(vec!["make=2.0.0","bs=2.1.0"]))
        .unwrap()
        .defines(Some(vec!["foo=bar", "la=deda"]))
        .verbose(true)
        .work(true)
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --clean --dist-dir=./foo/bar --with-docs --flavour=^,foo --platform=cent6_64,cent7_64 --level=DEV01.work --override=make=2.0.0,bs=2.1.0 -D=foo=bar -D=la=deda --verbose --work".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[serial]
fn build_cmd_given_work() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Build::default()
        .work(true)
        .build_cmd();
    let expected = vec![
        "pk audit && pk build --with-docs --work".to_string(),
    ];
    assert_eq!(result.unwrap(), expected);
}