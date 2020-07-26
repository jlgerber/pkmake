use super::*;

#[test]
fn from_vanilla() {
    let vans = vec!["^", "vanilla", "Vanilla", "VANILLA"];
    for van in vans {
        let result = Flavor::from_str(van);
        assert_eq!(result.unwrap(), Flavor::Vanilla);
    }
}

#[test]
fn from_named() {
    let nameds = vec!["foo", "FOO", "bar", "bla_f223_foo.bar"];
    for named in nameds {
        let result = Flavor::from_str(named);
        assert_eq!(result.unwrap(), Flavor::Named(named.to_string()));
    }
}
#[test]
fn from_from_trait() {
    let vals = vec!["foo", "bar"];
    for val in vals {
        let result = Flavor::from(val).unwrap();
        assert_eq!(result, Flavor::Named(val.to_string()));
    }
    assert_eq!(Flavor::from("^").unwrap(), Flavor::Vanilla);
    assert_eq!(Flavor::from("Vanilla").unwrap(), Flavor::Vanilla);
}
#[test]
fn from_given_invalid_str_is_err() {
    assert!(Flavor::from("foo bar#$#$").is_err());
}
#[test]
fn fails() {
    let fails = vec!["1foo", "foo bar", "bla_&^^", "FOO_!@#$%^&*()"];
    for fail in fails {
        let result = Flavor::from_str(fail);
        assert!(result.is_err());
    }
}
