use response::*;
use request::*;
use resource::*;
use http_error::HTTPError;
use std::path::PathBuf;
#[test]
fn test_get_success() {
    // The Response that response_from_request should return
    let expected_response = Response::Static(StaticResponse::new(200, PathBuf::from("/")));
    // The Request to be handed to response_from_request
    let request = HTTPRequest {
        method: HTTPMethod::GET,
        resource: HTTPResource::Static(PathBuf::from("/")),
    };
    // Get the actual Response from response_from_request
    let actual_response = response_from_request(request, acquire_test_always_succeeding);
    assert_eq!(expected_response, actual_response);
}

#[test]
fn test_get_failure_not_found() {
    let expected_response = Response::Error(HTTPError::new(404,
                                                           "Resource Not Found",
                                                           "The server could not locate the \
                                                            requested resource."));
    let request = HTTPRequest {
        method: HTTPMethod::GET,
        resource: HTTPResource::Static(PathBuf::from("/")),
    };
    let actual_response = response_from_request(request, acquire_test_never_found);
    assert_eq!(expected_response, actual_response);
}

#[test]
fn test_get_failure_not_permitted() {
    let expected_response = Response::Error(HTTPError::new(501,
                                                           "Not Permitted To Read Resource",
                                                           "The server could not read data from \
                                                            the requested resource."));
    let request = HTTPRequest {
        method: HTTPMethod::GET,
        resource: HTTPResource::Static(PathBuf::from("/")),
    };
    let actual_response = response_from_request(request, acquire_test_never_permitted);
    assert_eq!(expected_response, actual_response);
}
