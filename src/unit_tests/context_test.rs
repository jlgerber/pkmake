use super::*;
// tests from_str where input is facility
#[test]
fn can_build_from_facility_str() {
    let facs = vec!["facility", "Facility", "FACILITY"];
    for fac in facs {
        let result = Context::from_str(fac);
        assert_eq!(result.unwrap(), Context::Facility);
    }
}
// tests from_str where input is shared
#[test]
fn can_build_from_shared_str() {
    let shareds = vec!["shared", "SHARED", "Shared"];
    for shared in shareds {
        let result = Context::from_str(shared);
        assert_eq!(result.unwrap(), Context::Shared);
    }
}
// tests from_str where input is user
#[test]
fn can_build_from_user() {
    let usrs = vec!["user", "USER", "User"];
    for usr in usrs {
        let result = Context::from_str(usr);
        assert_eq!(result.unwrap(), Context::User);
    }
}
// tests from_str where input should return an Err
#[test]
fn other_inputs_will_error() {
    let errs = vec!["fio", "BAR", "USERS", "FACLILITY", "one two", "$)@!"];
    for err in errs {
        let result = Context::from_str(err);
        assert!(result.is_err());
    }}
// tests From<&str> where input is valid (user, facility, shared)
#[test]
fn from_valid_str() {
    let vals = vec![
        ("user", Context::User),
        ("facility", Context::Facility),
        ("shared", Context::Shared),
        ("USER", Context::User),
        ("FACILITY", Context::Facility),
        ("SHARED", Context::Shared),
    ];
    for val in vals {
        let result = Context::from(val.0);
        assert_eq!(result.unwrap(), val.1);
    }
}
// tests From where input is not a valid str and confirms that it returns an error
#[test]
fn from_invalid_str_returns_error() {
    let result = Context::from("foobar");
    assert!(result.is_err());
}
