use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use sha2::Digest;
use sha2::Sha256;

use hex;

const BUFFER_SIZE: usize = 8192;

fn shasum(path: &PathBuf) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut hasher = Sha256::new();

    while let Ok(read_size) = file.read(&mut buffer) {
        if read_size == 0 {
            break;
        }

        hasher.input(&mut buffer[..read_size]);
    }

    Ok(hex::encode(hasher.result()))
}

fn main() {
    for path in env::args().skip(1).map(|s| PathBuf::from(s)) {
        println!("{} {}", shasum(&path).unwrap(), path.display());
    }
}
