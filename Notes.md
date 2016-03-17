3/15/16
I initially thought that a client-server setup would be best, but I am now having second thoughts. For now, I believe that it will be best to simply implement everything in one process, and later migrate to a client-server system if this seems better.

3/16/16
I now have the beginnings of a user interface in place. A very basic command mode has also been implemented. At this point I need be able to display a file's contents in the user interface, and allow the user to edit the contents. I am struggling with creating an implementation for this. Currently, I am thinking that reading the file line-by-line and storing each line in an slice might work. It should then be possible to keep track of things like line-length, which fixes some movement problems. This also makes editing easier, as we will know what line we are editing, and we will also know which character is being manipulated.
