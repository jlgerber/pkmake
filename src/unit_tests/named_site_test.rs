use super::*;

#[test]
fn instance_from_str() {
    let sites = vec![
        ("hyderabad", NamedSite::Hyderabad),
        ("HYDERABAD", NamedSite::Hyderabad),
        ("playa", NamedSite::Playa),
        ("Playa", NamedSite::Playa),
        ("Playa Vista", NamedSite::Playa),
        ("playavista", NamedSite::Playa),
        ("portland", NamedSite::Portland),
        ("Portland", NamedSite::Portland),
        ("Montreal", NamedSite::Montreal),
        ("vancouver", NamedSite::Vancouver),
        ("Vancouver", NamedSite::Vancouver),
    ];
    for site in sites {
        let result = NamedSite::from_str(site.0);
        assert_eq!(result.unwrap(), site.1);
    }
}
#[test]
fn err_from_bad_str() {
    let result = NamedSite::from_str("fluboxland");
    assert!(result.is_err());
}

#[test]
fn instance_from() {
    let sites = vec![
        ("hyderabad", NamedSite::Hyderabad),
        ("HYDERABAD", NamedSite::Hyderabad),
        ("playa", NamedSite::Playa),
        ("Playa", NamedSite::Playa),
        ("Playa Vista", NamedSite::Playa),
        ("playavista", NamedSite::Playa),
        ("portland", NamedSite::Portland),
        ("Portland", NamedSite::Portland),
        ("Montreal", NamedSite::Montreal),
        ("vancouver", NamedSite::Vancouver),
        ("Vancouver", NamedSite::Vancouver),
    ];
    for site in sites {
        let result = NamedSite::from(site.0);
        assert_eq!(result.unwrap(), site.1);
    }
}

#[test]
fn instance_from_bad_input() {
    let result = NamedSite::from("fluboxland");
    assert!(result.is_err());
}
