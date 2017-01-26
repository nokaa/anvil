@0xc020c6481d06e29b;

interface Editor(T) {
    # An editor.

    insert @0 (line: UInt64, column: UInt64, string: Text);
    # Inserts `string` at [`line`][`column`] of the file.

    writeFile @1 (path: Text);
    # Writes the contents of this editor to the file specified by path.
}
