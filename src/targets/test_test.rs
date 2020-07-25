use super::*;
#[test]
fn can_construct_default() {
    let result = Test::default();
    let expected = Test {
        build_dir: None,
        dry_run: false,
        verbose: false,
    };

    assert_eq!(result, expected);
}

#[test]
fn can_modify_and_build() {
    let result = Test::default()
        .build_dir(Some("foo/bar"))
        .dry_run(true)
        .verbose(true)
        .build();
    let expected = Test {
        build_dir: Some("foo/bar".to_string()),
        dry_run: true,
        verbose: true,
    };
    assert_eq!(result, expected);
}
