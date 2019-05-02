use std::env;

use std::path::PathBuf;

fn main() {
    for path in env::args().skip(1).map(|s| PathBuf::from(s)) {
        println!("{:?}", path);
    }
}
