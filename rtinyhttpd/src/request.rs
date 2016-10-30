// Data structures for representing HTTP requests and strategies for serving them.
use std::path::PathBuf;
use http_error::HTTPError;

// TODO: Move method matching to another function
// TODO" Add version verification
// TODO: Do something with the other HTTP headers
// TODO: Do something with the request body for POST requests

/// Represents the HTTP method used by the client.
#[derive(Debug, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
    Other(String),
}

/// Represents the resource being requested by the client.
/// TODO: Expand this to include dynamic requests.
#[derive(Debug, PartialEq)]
pub enum HTTPResource {
    /// A static GET request for a file resource.
    Static(PathBuf),
    /// Something else.
    Other,
}

/// Represents a valid parsed request
#[derive(Debug, PartialEq)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub resource: HTTPResource,
}

pub fn parse_request(request_text: &str) -> Result<HTTPRequest, HTTPError> {
    // Local variables, to be populated later
    let method: HTTPMethod;

    // Split into multiple string slices; no copying of bytes is involved.
    let request_text_lines: Vec<&str> = request_text.split("\r\n").collect();
    // If there is not at least one line, the request cannot be valid.
    if request_text_lines.len() < 1 {
        return Err(HTTPError::new(400,
                                  "Bad Request",
                                  "Your web browser submitted an invalid request."));
    }
    // The first line should contain:
    // method <space> resource <space> version
    // So, split it into words on whitespace
    let mut first_line_words = request_text_lines[0].split_whitespace();

    // Place a slice representing each of the words in a variable
    // These are Option<&str>
    let potential_method = first_line_words.next();
    let potential_resource = first_line_words.next();
    let potential_version = first_line_words.next();

    // These slices store confirmed method, resource, and version string slices
    let method_str: &str;
    let resource_str: &str;
    let version_str: &str;

    // Store the method if it exists, error if it does not.
    match potential_method {
        Some(s) => {
            method_str = s;
        }
        None => {
            return Err(HTTPError::new(400,
                                      "Bad Request",
                                      "Your web browser submitted a request with no HTTP method."));
        }
    };
    // Store the resource path if it exists, error if it does not.
    match potential_resource {
        Some(s) => {
            resource_str = s;
        }
        None => {
            return Err(HTTPError::new(400,
                                      "Bad Request",
                                      "Your web browser submitted a request that did not \
                                       specify a resource."));
        }
    };
    // Store the version if it exists, error if it does not.
    match potential_version {
        Some(s) => {
            version_str = s;
        }
        None => {
            return Err(HTTPError::new(400,
                                      "Bad Request",
                                      "Your web browser submitted a request with no HTTP \
                                       version."));
        }
    };
    // Currently this implementation ignores the version. Here, check only for the
    // method. Note: this function won't error on an unsupported method, because it
    // is concerned only with parsing, not semantic understanding.
    // Also note that to_lowercase does produce a String, but there is still a lot less allocation \
    // going on that if this function worked with a string in the first place.
    match method_str.to_lowercase().as_ref() {
        "get" => method = HTTPMethod::GET,
        "post" => method = HTTPMethod::POST,
        // This case is inefficient but should happen quite rarely
        other_method_name => method = HTTPMethod::Other(String::from(other_method_name)),
    };

    // Using match here because different methods require different behaviour
    match method {
        // If the method is GET, simply build the HTTPRequest and be done.
        HTTPMethod::GET => {
            let r = HTTPRequest {
                // The method has been determined above, obviously, since this is an arm of
                // a match on it
                method: method,
                // Construct the path directly from the given resource_str
                resource: HTTPResource::Static(PathBuf::from(resource_str)),
            };
            // 　Return the successfully parsed request.
            return Ok(r);
        }
        HTTPMethod::POST => {
            let r = HTTPRequest {
                method: method,
                // Construct the path directly from the given resource_str
                resource: HTTPResource::Static(PathBuf::from(resource_str)),
            };
            // 　Return the successfully parsed request.
            return Ok(r);
        }
        HTTPMethod::Other(_) => {
            let r = HTTPRequest {
                method: method,
                // Construct the path directly from the given resource_str
                resource: HTTPResource::Static(PathBuf::from(resource_str)),
            };
            // 　Return the successfully parsed request.
            return Ok(r);
        }
    }
}
