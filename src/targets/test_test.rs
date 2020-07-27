use super::*;
#[test]
fn can_construct_default() {
    let result = Test::default();
    let expected = Test {
        dist_dir: None,
        dry_run: false,
        verbose: false,
        defines: None,
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
        .defines(Some(vec!["foo=bar"]))
        .package_root(Some("./foo/bar"))
        .build();
    let expected = Test {
        dist_dir: Some("foo/bar".to_string()),
        dry_run: true,
        verbose: true,
        defines: Some(vec!["foo=bar".to_string()]),
        package_root: Some(std::path::PathBuf::from("./foo/bar")),
    };
    assert_eq!(result, expected);
}
