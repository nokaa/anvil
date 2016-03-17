/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

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

pub fn read_file_lines(filename: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let f = try!(File::open(filename));
    let mut lines: Vec<Vec<u8>> = vec![];
    let mut line: Vec<u8> = vec![];

    for byte in f.bytes() {
        match byte {
            Err(e) => return Err(e),
            Ok(b) => match b {
                b'\n' => {
                    lines.push(line);
                    line = vec![];
                }
                // We do nothing on the carriage return for now.
                // TODO(nokaa): We should decide what to do here.
                b'\r' => { }
                _ => line.push(b),
            }
        }
    }

    Ok(lines)
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
