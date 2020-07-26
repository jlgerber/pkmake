use super::*;

#[test]
fn can_build_local() {
    let locals = vec!["local", "Local", "LOCAL"];
    for local in locals {
        let result = Site::from_str(local);
        assert_eq!(result.unwrap(), Site::Local);
    }
}

#[test]
fn can_build_all() {
    let alls = vec!["all", "All", "ALL"];
    for all_ in alls {
        let result = Site::from_str(all_);
        assert_eq!(result.unwrap(), Site::All);
    }
}

#[test]
fn can_build_named() {
    let named = vec![
        ("hyderabad", NamedSite::Hyderabad),
        ("playa", NamedSite::Playa),
        ("portland", NamedSite::Portland),
        ("montreal", NamedSite::Montreal),
        ("vancouver", NamedSite::Vancouver),
    ];
    for nm in named {
        let result = Site::from_str(nm.0);
        assert_eq!(result.unwrap(), Site::Named(nm.1));
    }
}

#[test]
fn errors_when_invalid_chars_used() {
    let invalids = vec!["foo bar", "foo1", "foo,bar", "bar$#%"];
    for invalid in invalids {
        let result = Site::from_str(invalid);
        assert!(result.is_err());
    }}
