use alpm::vercmp;
use std::cmp::Ordering;

pub fn strvercmp(a: &str, b: &str) -> bool {
    vercmp(a.as_bytes().to_vec(), b.as_bytes().to_vec()) == Ordering::Greater
}