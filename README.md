# Forge [![Build Status](https://travis-ci.org/nokaa/forge.svg?branch=master)](https://travis-ci.org/nokaa/forge)
Forge is my experiment with text editors. I wanted to see what makes a
text editor a text editor.

At this time, Forge is shaping up to be a basic Vi clone. It currently
has a very basic command mode and an insert mode. We are able to display
file contents in a user interface, and we also have a basic prompt which
shows the filename.

We can now open and view files. We are able to modify files in insert
mode and command mode, and write these changes to disk.

Please see `Commands.md` for a list of commands and their functions.

### Coming Soon (TM)
- Completing the replace command implementation
- More stable file writing implementation; e.g. writing to a temp file
- Split command mode into command mode and normal mode

### Build
To build forge, you must have the Rust compiler and Cargo installed on
your system. Forge is tested on the current stable, beta, and nightly
compilers. Forge is currently only tested on Linux. Forge does not build
on OSX due to an issue with rustty, and build status on Windows is unknown.

```
git clone http://github.com/nokaa/forge
cd forge
cargo build
cargo run src/main.rs
```
