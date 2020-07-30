use super::*;
use serial_test::serial;
use crate::utils::setup_manifest_dir;
use std::env;
use indexmap::IndexSet as HashSet;


#[test]
fn can_build_default() {
    let result = Run::default();
    let expected = Run {
        dry_run: false,
        verbose: false,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: Vec::new(),
    };
    assert_eq!(result, expected);
}

#[test]
fn can_update_build() {
    let result = Run::default()
        .dry_run(true)
        .verbose(true)
        .vars(vec!["foo".into(), "bar".into()])
        .build();
    let expected = Run {
        dry_run: true,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo".into(), "bar".into()],
    };
    assert_eq!(result, expected);
}
#[test]
fn propagates_dry_run_and_verbose_to_vars_if_set() {
    let mut result = Run::default()
        .dry_run(true)
        .verbose(true)
        .vars(vec!["foo".into(), "bar".into()])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: true,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec![
            "foo".into(),
            "bar".into(),
            "--verbose".into(),
            "--dry-run".into(),
        ],
    };
    assert_eq!(result, expected);
}
#[test]
fn propagates_dry_run_and_verbose_to_struct_if_set_in_vars() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec![
            "foo".into(),
            "bar".into(),
            "--verbose".into(),
            "--dry-run".into(),
        ])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: true,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec![
            "foo".into(),
            "bar".into(),
            "--verbose".into(),
            "--dry-run".into(),
        ],
    };
    assert_eq!(result, expected);
}
#[test]
fn propagates_dry_run_and_verbose_to_struct_if_set_in_vars_as_short_flags() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec!["foo".into(), "-n".into(), "-v".into(), "bar".into()])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: true,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo".into(), "-n".into(), "-v".into(), "bar".into()],
    };
    assert_eq!(result, expected);
}
#[test]
fn propagates_verbose_to_struct_if_set_in_vars_as_short_flags() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec!["foo".into(), "-v".into(), "bar".into()])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: false,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo".into(), "-v".into(), "bar".into()],
    };
    assert_eq!(result, expected);
}

#[test]
fn propagates_dry_run_to_struct_if_set_in_vars_as_short_flags() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec!["foo".into(), "-n".into(), "bar".into()])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: true,
        verbose: false,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo".into(), "-n".into(), "bar".into()],
    };
    assert_eq!(result, expected);
}

#[test]
fn doit_errors_if_first_vars_is_a_flag() {
    let mut result = Run::default().vars(vec!["-v".into(), "bla".into()]).build();
    let result = result.doit();
    assert!(result.is_err());
}

#[test]
fn doit_errors_if_vars_is_empty() {
    let mut result = Run::default().build();
    let result = result.doit();
    assert!(result.is_err());
}

//
// build_cmd
//
