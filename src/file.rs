/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use std::fs::File;
use std::io::{self, Read, Write};

use crypto::digest::Digest;
use crypto::sha2::Sha512;

/// Reads all data from `filename`, putting each line into a
/// `Vec<u8>`. A vector of all the lines, `Vec<Vec<u8>>` is
/// returned if successful.
pub fn read_file_lines(filename: &str, line_length: usize) -> Result<Vec<Vec<u8>>, io::Error> {
    let f = try!(File::open(filename));
    let mut lines: Vec<Vec<u8>> = vec![];
    let mut line: Vec<u8> = vec![];

    for byte in f.bytes() {
        match byte {
            Err(e) => return Err(e),
            Ok(b) => match b {
                b'\n' => {
                    line.push(b);
                    lines.push(line);
                    line = vec![];
                }
                // We do nothing on the carriage return for now.
                // TODO(nokaa): We should decide what to do here.
                b'\r' => { }
                _ => {
                    if line.len() == line_length {
                        lines.push(line);
                        line = vec![b];
                    } else {
                        line.push(b);
                    }
                }
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

/// Writes all data in `data` to `filename`.
pub fn write_file_lines(filename: &str, data: &Vec<Vec<u8>>) -> Result<(), io::Error> {
    let mut file = try!(File::create(filename));
    for line in data {
        try!(file.write(&line[..]));
    }

    Ok(())
}

pub fn sha512_file(filename: &str) -> Result<String, io::Error> {
    // Read file contents for hashing
    let mut f = try!(File::open(filename));
    let mut buf: Vec<u8> = vec![];
    try!(f.read_to_end(&mut buf));

    let mut hasher = Sha512::new();
    hasher.input(&buf);

    let hex = hasher.result_str();
    Ok(hex)
}
