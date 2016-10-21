# First pass 

`ISspace` is a macro which converts its argument to an int before passing 
it to the library function isspace, allowing the programmer to directly 
check whether char values are whitespace without explicit casting every 
time.

### main()

+ Takes void, meaning the program has no command line arguments (and thus
probably allows little configuration). This should be changed.
+ Ignores SIGPIPE, which should be appropriately handled.

Pseudocode:

```
Open and configure a server socket

Until the process is terminated:
     Wait for a client to request a connection
     Try to open a connection with that client
     Open a new thread to deal with that connection's request
```

### startup()

+ Takes a reference to a `u_short`, which it modifies to provide dynamic
generation of port numbers. This is a useful feature, but it's not exposed
via the command line and I'd prefer to use a more functional approach,
returning a tuple of (socket, port number).

### handle_request()

```
accept_request takes a socket connecting to the client
     read a line from the client
     log (to stdout) the received request
     
     copy what is assumed to be the method into another buffer
     if the method isn't GET or POST:
          handle an unimplemented method somehow
     if the method is POST:
          make a note that this request will require executing a CGI script

     copy what is assumed to be the URL into another buffer
     if the method is GET and the url has a ? in it:
          make a note that this request will require executing a CGI script
     construct the path to the requested resource by prepending "htdocs" to the url
          (note - I'd like to make this customizable in the rewrite)

     if the URL is /:
          add 'index.html' to the path

     if the resource being requested doesn't exist:
          handle a not found error somehow
     if the resource is a directory:
          append "/index.html" to the file path
          (note- the existence of THIS file isn't checked!)
     if the file is executable:
          make a note that this request will require executing a CGI script
     
     if this request requires executing a CGI script:
          handle executing a CGI script
     otherwise:
          handle serving a static file
```
