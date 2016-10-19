# First pass 

`ISspace` is a macro which converts its argument to an int before passing 
it to the library function isspace, allowing the programmer to directly 
check whether char values are whitespace without explicit casting every 
time.

### main()

+ Takes void, meaning the program has no command line arguments (and thus
probably allows little configuration). This should be changed.
+ Ignores SIGPIPE, which should be appropriately handled.

### startup()

+ Takes a reference to a `u_short`, which it modifies to provide dynamic
generation of port numbers. This is a useful feature, but it's not exposed
via the command line and I'd prefer to use a more functional approach,
returning a tuple of (socket, port number).
