use alpm::vercmp;
use std::cmp::Ordering;
use std::path::Path;

pub fn check_existance(path: &str) -> bool {
    let p = Path::new(path);
    p.exists()
}

pub fn strvercmp(a: &str, b: &str) -> bool {
    vercmp(a.as_bytes().to_vec(), b.as_bytes().to_vec()) == Ordering::Greater
}

#[test]
fn test_strvercmp() {
    assert_eq!(strvercmp("0.0.1", "0.0.2"), false);
    assert_eq!(strvercmp("0.0.1", "0.0.1"), false);
    assert_eq!(strvercmp("0.0.2", "0.0.1"), true);
}
