use super::*;

#[test]
fn from_win7() {
    let wins = vec!["win7", "WIN7", "win7_64", "WIN7_64"];
    for win in wins {
        let result = Platform::from_str(win);
        assert_eq!(result.unwrap(), Platform::Win7_64);
    }
}

#[test]
fn from_win10() {
    let wins = vec!["win10", "WIN10", "win10_64", "WIN10_64"];
    for win in wins {
        let result = Platform::from_str(win);
        assert_eq!(result.unwrap(), Platform::Win10_64);
    }
}

#[test]
fn from_cent6() {
    let wins = vec!["cent6", "CENT6", "cent6_64", "CENT6_64"];
    for win in wins {
        let result = Platform::from_str(win);
        assert_eq!(result.unwrap(), Platform::Cent6_64);
    }
}

#[test]
fn from_cent7() {
    let wins = vec!["cent7", "CENT7", "cent7_64", "CENT7_64"];
    for win in wins {
        let result = Platform::from_str(win);
        assert_eq!(result.unwrap(), Platform::Cent7_64);
    }
}
#[test]
fn from_cent8() {
    let wins = vec!["cent8", "CENT8", "cent8_64", "CENT8_64"];
    for win in wins {
        let result = Platform::from_str(win);
        assert_eq!(result.unwrap(), Platform::Cent8_64);
    }
}

#[test]
fn from_trait_impl() {
    let plats = vec!["cent8", "Cent8", "cent8_64"];
    for plat in plats {
        let result = Platform::from(plat).unwrap();
        assert_eq!(result, Platform::Cent8_64);
    }
    // test a bad input
    let result = Platform::from("foobarbla");
    assert!(result.is_err());
}

#[test]
fn invalid_strs() {
    let invalids = vec!["CentyWenty", "Cent 7", "125", "Cent7$%$@"];
    for invalid in invalids {
        let result = Platform::from_str(invalid);
        assert!(result.is_err());
    }}
