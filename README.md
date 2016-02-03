# Forge
Forge is my experiment with text editors. I wanted to see what makes a text editor a text editor.

[![Build Status](https://travis-ci.org/nokaa/forge.svg?branch=master)](https://travis-ci.org/nokaa/forge)

At this time Forge launches a TCP server at `0.0.0.0:3000` and listens for input. Upon receiving input, it writes the input to a file `test` in the current directory.

An example client may be found at `examples/client.rs`. Dependency versions are listed at the top of this file.
