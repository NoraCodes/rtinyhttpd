// Data structures for representing responses
use std::path::PathBuf;
use request;

/// Represents a response indicating an error.
pub struct ErrorResponse {
    status_code: u16,
    message_title: String,
    message_body: String,
}

/// Represents a response to a valid request for a static resource.
pub struct StaticResponse {
    status_code: u16,
    resource_path: PathBuf,
}

pub enum Response {
    Error(ErrorResponse),
    Static(StaticResponse),
}
