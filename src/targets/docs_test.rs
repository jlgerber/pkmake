use super::*;

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
fn can_update_build() {
    let result = Docs::default()
        .dist_dir(Some("foo/bar"))
        .dry_run(true)
        .verbose(true)
        .defines(Some(vec!["foo=bar"]))
        .package_root(Some("./foo/bar"))
        .build();
    let expected = Docs {
        dist_dir: Some("foo/bar".to_string()),
        dry_run: true,
        verbose: true,
        defines: Some(vec!["foo=bar".to_string()]),
        flavors: None,
        platforms: None,
        package_root: Some(std::path::PathBuf::from("./foo/bar")),
    };
    assert_eq!(result, expected);
}
