#### 12/21/16
I recently read about the editor kakoune, and it has reinvigorated my
interest in editors. My initial plans are to write an editor that uses
a server with plugins via rpc. In this situation, even the UI is
considered a plugin. An example of this would be the relationship between
ncmpcpp/mpc and mpd. I am of the belief that modal editing is a superior
paradigm, so I intend to do this. I plan to expirement greatly with the
command language, but my initial thoughts are to take inspiration from
kakoune for this. I am uncertain how the command language will be implemented,
but it seems likely to be done via the UI plugin rather than the server.

#### 3/23/16
When we write a file to disk, we are currently just writing directly to
the filename given. I think that I remember reading somewhere that you
should not do this, because if the write fails the old file will be gone.
If I remember correctly, you should write to a different file and then
rename it to the original file's name. Another thing is that Vim uses
a swap file for reasons that I not entirely sure of. I know that one
of it's purposes is to prevent editing a file in multiple instances
of Vim.

#### 3/21/16
Right now the redraw process for our UI is O(n^2). Because we typically
only change a single line, it should be possible to redraw only the
modified line, which is O(n). In the case of a line being added/removed,
we will likely be forced to redraw the entire file in O(n^2) time. This
is much better, because most modifications will not be adding/removing
line.

#### 3/16/16
I now have the beginnings of a user interface in place. A very basic
command mode has also been implemented. At this point I need be able
to display a file's contents in the user interface, and allow the user
to edit the contents. I am struggling with creating an implementation
for this. Currently, I am thinking that reading the file line-by-line
and storing each line in an slice might work. It should then be possible
to keep track of things like line-length, which fixes some movement
problems. This also makes editing easier, as we will know what line we
are editing, and we will also know which character is being manipulated.

#### 3/15/16
I initially thought that a client-server setup would be best, but I
am now having second thoughts. For now, I believe that it will be
best to simply implement everything in one process, and later migrate
to a client-server system if this seems better.
