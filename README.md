# rtinyhttpd
A rewrite of J David Blackstone's tinyhttpd in Rust, in order to demonstrate
how to deal with legacy application rewrites.

## Structure
+ `legacy/` contains the code from the jdavidb's original program
    + `htdocs/` contains some example pages and CGI programs
+ `analysis/` contains my analysis of jdavidb's source
+ `rtinyhttpd/` contains the Rust/Cargo project for my rewrite
    + `src/` contains the actual source code
    + `tests/` contains integration tests for the application
