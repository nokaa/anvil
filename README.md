# Forge
Forge is my experiment with text editors. I wanted to see what makes a text editor a text editor.

[![Build Status](https://travis-ci.org/nokaa/forge.svg?branch=master)](https://travis-ci.org/nokaa/forge)

At this time Forge launches a TCP server at `0.0.0.0:3000` and listens for input. Upon receiving input, it writes the input to a file `test` in the current directory.

It should be stated that at this point in time, Forge operates over TCP. This means that if you run forge on a computer, it is possible to interact with it from another computer. This allows for some interesting future experiments. For example, we might allow for multiple clients to connect at once. This would allow for livecoding sessions where one user writes, and others view the work. This could be useful for things like job applications, where applicants often need to use an unfamiliar online editor. Using Forge with this setup would allow the applicant to use their preferred environment, while also allowing the interviewer to easily view work.

An example client may be found at `examples/client.rs`. Dependency versions are listed at the top of this file.
