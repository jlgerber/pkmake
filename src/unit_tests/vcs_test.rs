use super::*;

#[test]
fn is_both_true() {
    let vcs = Vcs::Both;
    assert!(vcs.is_both());
}

#[test]
fn is_both_false() {
    let vcs = Vcs::Git;
    assert!(!vcs.is_both());
    let vcs = Vcs::Svn;
    assert!(!vcs.is_both());
}

#[test]
fn can_convert_from_str() {
    let vcs=Vcs::try_from("git");
    assert!(vcs.is_ok());
}