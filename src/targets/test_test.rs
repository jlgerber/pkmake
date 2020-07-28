use super::*;
#[test]
fn can_construct_default() {
    let result = Test::default();
    let expected = Test {
        dist_dir: None,
        dry_run: false,
        verbose: false,
        defines: None,
        platforms: None,
        flavors: None,
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
        .platforms(Some(vec!["cent7_64", "cent6_64"])).unwrap()
        .flavors(Some(vec!["^", "for_maya"])).unwrap()
        .defines(Some(vec!["foo=bar"]))
        .package_root(Some("./foo/bar"))
        .build();

    let mut platform_hset = HashSet::new();
    platform_hset.insert(Platform::Cent7_64);
    platform_hset.insert(Platform::Cent6_64);

    let mut flavor_hset = HashSet::new();
    flavor_hset.insert(Flavor::Vanilla);
    flavor_hset.insert(Flavor::Named(String::from("for_maya")));
    
    let expected = Test {
        dist_dir: Some("foo/bar".to_string()),
        dry_run: true,
        verbose: true,
        platforms: Some(platform_hset),
        flavors: Some(flavor_hset),
        defines: Some(vec!["foo=bar".to_string()]),
        package_root: Some(std::path::PathBuf::from("./foo/bar")),
    };
    assert_eq!(result, expected);
}
