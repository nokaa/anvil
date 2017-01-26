@0xc020c6481d06e29b;

interface Subscription {}

interface Editor(T) {
    # A source of messages of type T.

    subscribe @0 (plugin: Plugin(T)) -> (subscription: Subscription);
    # Registers `subscriber` to recieve published messages. Dropping the returned
    # `subscription` signals to the `Publisher` that the subscriber is no longer
    # interested in receiving messages.

    insert @1 (line: UInt64, column: UInt64, string: Text);
    # Inserts `string` at [`line`][`column`] of the file.

    writeFile @2 (path: Text);
}

interface Plugin(T) {
    pushMessage @0 (message: T) -> ();
    # Sends a message from a publisher to the subscriber. To help with flow control, the
    # subscriber should not return from this method until it is ready to process the  next message.
}