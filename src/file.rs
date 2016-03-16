use std::fs::File;
use std::io::{self, Read, Write};

#[allow(dead_code)]
/// Helper function for reading from a file. Reads from `filename` and returns a `Vec<u8>`.
pub fn read_file(filename: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = try!(File::open(filename));
    let mut buf: Vec<u8> = vec![];
    try!(f.read_to_end(&mut buf));

    Ok(buf)
}

/// Returns `true` if `filename` exists, `false` otherwise.
pub fn file_exists(filename: &str) -> bool {
    match File::open(filename) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[allow(dead_code)]
/// Helper function for writing to a file. Writes `contents` to `filename`.
pub fn write_file(filename: &str, data: &[u8]) -> Result<(), io::Error> {
    let mut file = try!(File::create(filename));
    try!(file.write_all(data));
    Ok(())
}

#[allow(dead_code)]
/// Takes a `&[u8]` and returns a `Vec<u8>` containing all values until the first 0.
pub fn get_nonzero_bytes(data: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    for ch in data {
        if *ch == 0u8 {
            break;
        } else {
            buf.push(*ch);
        }
    }

    buf
}
