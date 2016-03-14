use std::fs::File;
use std::io::{self, Read, Write};

/// Helper function for reading from a file. Reads from `filename` and returns a `Vec<u8>`.
pub fn read_file(filename: String) -> Vec<u8> {
    let mut reader = File::open(filename).ok().expect("Unable to open file");
    let mut buf: Vec<u8> = vec![];
    reader.read_to_end(&mut buf).unwrap();

    buf
}

/// Helper function for writing to a file. Writes `contents` to `filename`.
pub fn write_file(filename: String, contents: Vec<u8>) -> Result<(), io::Error> {
    let mut file = try!(File::create(filename));
    try!(file.write_all(&contents[..]));
    Ok(())
}

/// Helper function for writing to a file. Writes `contents` to `filename`.
pub fn write_file_slice(filename: String, contents: &[u8]) -> Result<(), io::Error> {
    let mut file = try!(File::create(filename));
    try!(file.write_all(contents));
    Ok(())
}

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
