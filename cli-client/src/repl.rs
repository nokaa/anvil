use commands;
use error::*;

use std::io::{self, Write};
use std::path::Path;
use std::u64;

pub fn run<P>(path: P) -> Result<()>
    where P: AsRef<Path>
{
    let path = path.as_ref();
    io::stdout().write(b"\n>>> ")?;
    io::stdout().flush()?;

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let cmd = commands::Command::new(path).unwrap();
        if buffer.starts_with("openFile") {
            let mut input = buffer.split_whitespace();
            input.next();
            let file_name = input.next().unwrap();
            cmd.open_file(file_name)?;
        } else if buffer.starts_with("writeFile") {
            let mut input = buffer.split_whitespace();
            input.next();
            let file_name = input.next().unwrap();
            cmd.write_file(file_name)?;
        } else if buffer.starts_with("get") {
            let mut input = buffer.split_whitespace();
            input.next();
            let start = input.next().unwrap();
            let end = input.next().unwrap();
            let start = u64::from_str_radix(start, 10).unwrap();
            let end = u64::from_str_radix(end, 10).unwrap();

            let lines = cmd.get(start, end)?;
            io::stdout().write(lines.as_bytes())?;
            io::stdout().write(b"\n")?;
        } else if buffer.starts_with("insert") {
            let mut input = buffer.split_whitespace();
            input.next();
            let line = input.next().unwrap();
            let column = input.next().unwrap();
            let line = u64::from_str_radix(line, 10).unwrap();
            let column = u64::from_str_radix(column, 10).unwrap();
            let text = input.next().unwrap();

            cmd.insert(line, column, text)?;
        } else if buffer.starts_with("delete") {
            let mut input = buffer.split_whitespace();
            input.next();
            let line = input.next().unwrap();
            let column = input.next().unwrap();
            let length = input.next().unwrap();
            let line = u64::from_str_radix(line, 10).unwrap();
            let column = u64::from_str_radix(column, 10).unwrap();
            let length = u64::from_str_radix(length, 10).unwrap();

            cmd.delete(line, column, length)?;
        } else if buffer.starts_with("replace") {
            let mut input = buffer.split_whitespace();
            input.next();
            let line = input.next().unwrap();
            let column = input.next().unwrap();
            let length = input.next().unwrap();
            let line = u64::from_str_radix(line, 10).unwrap();
            let column = u64::from_str_radix(column, 10).unwrap();
            let length = u64::from_str_radix(length, 10).unwrap();
            let character = input.next().unwrap().chars().nth(0).unwrap();

            cmd.replace(line, column, length, character)?;
        } else if buffer.starts_with("quit") {
            cmd.quit()?;
            break;
        } else {
            io::stdout().write(b"Invalid input\n")?;
        }

        io::stdout().write(b">>> ")?;
        io::stdout().flush()?;
    }

    Ok(())
}
