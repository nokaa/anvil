@0xc020c6481d06e29b;

interface Editor {
    # An editor.

    openFile @0 (path: Text);
    # Opens `path` for editing. If `path` does not exist, it is created.

    writeFile @1 (path: Text);
    # Writes the contents of this editor to the file specified by path.

    get @2 (startLine: UInt64, endLine: UInt64) -> (lines: Text);
    # Gets a range of lines.

    insert @3 (line: UInt64, column: UInt64, string: Text);
    # Inserts `string` at [`line`][`column`] of the file.

    delete @4 (line: UInt64, column: UInt64, length: UInt64);
    # Delete the string at [`line`][`column`] with `length`.

    quit @5 ();
    # Quits this editor.
}
