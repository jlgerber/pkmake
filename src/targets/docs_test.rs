use super::*;

#[test]
fn can_build_default() {
    let result = Docs::default();
    let expected = Docs {
        dry_run: false,
        build_dir: None,
        verbose: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn can_update_build() {
    let result = Docs::default()
        .build_dir(Some("foo/bar"))
        .dry_run(true)
        .verbose(true)
        .build();
    let expected = Docs {
        build_dir: Some("foo/bar".to_string()),
        dry_run: true,
        verbose: true,
    };
    assert_eq!(result, expected);
}