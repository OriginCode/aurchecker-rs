use alpm::Version;
use std::path::Path;

pub fn check_existance(path: &str) -> bool {
    let p = Path::new(path);
    p.exists()
}
