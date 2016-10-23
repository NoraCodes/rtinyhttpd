# Design of the New Server

## Basic Control Flow
The basic flow of the server is pretty simple: a request comes in, is checked for validity, and is served. 

Serving involves either sending a static file or executing a CGI script.

## Data structures

Serving an error reuqires knowing what went wrong (a status code) and a message to inform the user of that, in the form of a title and a body.

Serving a static request requires knowing what resource was requested.

Serving a CGI request requires knowing what resource was requested and what input was given.

To model these, an enum could be used.
