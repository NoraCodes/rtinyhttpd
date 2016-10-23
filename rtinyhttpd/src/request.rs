// Data structures for representing HTTP requests and strategies for serving them.
use std::path::PathBuf;

/// Represents the HTTP method used by the client.
pub enum HTTPMethod {
    GET,
    POST,
    Other(String),
}

/// Represents the resource being requested by the client.
/// TODO: Expand this to include dynamic requests.
pub enum HTTPResource {
    /// A static GET request for a file resource.
    Static(PathBuf),
    /// Something else.
    Other,
}

/// Represents a valid parsed request
pub struct HTTPRequest {
    method: HTTPMethod,
    resource: HTTPResource,
}
