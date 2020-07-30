use super::*;
use serial_test::serial;
use crate::utils::setup_manifest_dir;
use std::env;


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
        .vars(vec!["foo", "bar"])
        .build();
    let expected = Run {
        dry_run: true,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo", "bar"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    };
    assert_eq!(result, expected);
}
#[test]
fn propagates_dry_run_and_verbose_to_vars_if_set() {
    let mut result = Run::default()
        .dry_run(true)
        .verbose(true)
        .vars(vec!["foo", "bar"])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: true,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec![
            "foo",
            "bar",
            "--verbose",
            "--dry-run",
        ].into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    };
    assert_eq!(result, expected);
}
#[test]
fn propagates_dry_run_and_verbose_to_struct_if_set_in_vars() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec![
            "foo",
            "bar",
            "--verbose",
            "--dry-run",
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
            "foo",
            "bar",
            "--verbose",
            "--dry-run",
        ].into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    };
    assert_eq!(result, expected);
}
#[test]
fn propagates_dry_run_and_verbose_to_struct_if_set_in_vars_as_short_flags() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec!["foo", "-n", "-v", "bar"])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: true,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo", "-n", "-v", "bar"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    };
    assert_eq!(result, expected);
}


#[test]
fn propagates_verbose_to_struct_if_set_in_vars_as_short_flags() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec!["foo", "-v", "bar"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>())
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: false,
        verbose: true,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo", "-v", "bar"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    };
    assert_eq!(result, expected);
}

#[test]
fn propagates_dry_run_to_struct_if_set_in_vars_as_short_flags() {
    let mut result = Run::default()
        .dry_run(false)
        .verbose(false)
        .vars(vec!["foo", "-n", "bar"])
        .build();
    result.fix_args().unwrap();
    let expected = Run {
        dry_run: true,
        verbose: false,
        package_root: None,
        flavors: None,
        platforms: None,
        vars: vec!["foo", "-n", "bar"].into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    };
    assert_eq!(result, expected);
}

#[test]
fn doit_errors_if_first_vars_is_a_flag() {
    let mut result = Run::default().vars(vec!["-v", "bla"]).build();
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


#[test]
#[serial]
fn build_cmd_given_default() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Run::default().build_cmd();
    
    assert!(result.is_err());
}


#[test]
#[serial]
fn build_cmd_given_build_target() -> Result<(), Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Run::default()
        .vars(vec!["foobar"])
        .build_cmd()?;

    let result = result.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    assert_eq!(result, 
        vec!["pk run-recipe foobar"]
    );

    Ok(())
}

#[test]
#[serial]
fn build_cmd_given_build_target_and_additional_vars() -> Result<(), Box<dyn std::error::Error>> {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Run::default()
        .vars(vec!["foobar", "--bla", "foo"])
        .build_cmd()?;
        
    let result = result.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    assert_eq!(result, 
        vec!["pk run-recipe foobar --bla foo"]
    );

    Ok(())
}


#[test]
#[serial]
fn build_cmd_given_vars_starting_with_flag() {
    setup_manifest_dir(false);
    env::set_var("DD_SHOW", "DEV01");
    env::set_var("DD_OS", "cent7_64");
    let result = Run::default().vars(vec!["--foo", "--bar"]).build_cmd();
    
    assert!(result.is_err());
}
