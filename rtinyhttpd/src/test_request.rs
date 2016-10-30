use request::*;
use std::path::PathBuf;

#[test]
fn test_parse_get_root() {
    let expected_request_struct = HTTPRequest {
        method: HTTPMethod::GET,
        resource: HTTPResource::Static(PathBuf::from("/")),
    };
    assert_eq!(parse_request("GET / HTTP/1.1").unwrap(),
               expected_request_struct);
}

#[test]
fn test_parse_get_resource() {
    let expected_request_struct = HTTPRequest {
        method: HTTPMethod::GET,
        resource: HTTPResource::Static(PathBuf::from("/some_page.html")),
    };
    assert_eq!(parse_request("GET /some_page.html HTTP/1.1").unwrap(),
               expected_request_struct);
}

#[test]
fn test_parse_post_resource() {
    let expected_request_struct = HTTPRequest {
        method: HTTPMethod::POST,
        resource: HTTPResource::Static(PathBuf::from("/some_cgi_thingy.sh")),
    };
    assert_eq!(parse_request("POST /some_cgi_thingy.sh HTTP/1.1").unwrap(),
               expected_request_struct);
}

#[test]
fn test_parse_trace_resource() {
    let expected_request_struct = HTTPRequest {
        method: HTTPMethod::Other(String::from("trace")),
        resource: HTTPResource::Static(PathBuf::from("/some_page.html")),
    };
    assert_eq!(parse_request("TRACE /some_page.html HTTP/1.1").unwrap(),
               expected_request_struct);
}
