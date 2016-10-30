/// Defines functions for common errors, to save time and allow rapid refactoring.
use http_error::HTTPError;

// 4xx: Bad Request.

pub fn bad_request_generic() -> HTTPError {
    HTTPError::new(400,
                   "Bad Request",
                   "Your web browser submitted an invalid request.")
}

pub fn bad_request_no_method() -> HTTPError {
    HTTPError::new(400,
                   "Bad Request",
                   "Your web browser submitted a request with no HTTP method.")
}

pub fn bad_request_no_resource() -> HTTPError {
    HTTPError::new(400,
                   "Bad Request",
                   "Your web browser submitted a request that did not specify a resource.")
}

pub fn bad_request_no_version() -> HTTPError {
    HTTPError::new(400,
                   "Bad Request",
                   "Your web browser submitted a request with no HTTP version.")
}

pub fn bad_request_not_found() -> HTTPError {
    HTTPError::new(404,
                   "Resource Not Found",
                   "The server could not locate the requested resource.")
}

// 5xx: There is some problem with the server.
pub fn server_err_not_implemented() -> HTTPError {
    HTTPError::new(501,
                   "Not Implemented",
                   "This functionality is not implemented.")
}
