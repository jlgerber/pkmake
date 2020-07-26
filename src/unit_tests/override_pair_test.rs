use super::*;

#[test]
fn can_get_name_and_version() {
    let op = OverridePair::from_str("foo=1.3.2").unwrap();
    assert_eq!(op.name(), "foo");
    assert_eq!(op.version(), "1.3.2");
}

#[test]
fn none_set_correctly() {
    let op = OverridePair::try_from("foo1.3.2");
    assert!(op.is_err());
}
