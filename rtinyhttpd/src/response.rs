// Data structures for representing responses
use std::path::PathBuf;
use request;
use http_error::HTTPError;
/// Represents a response to a valid request for a static resource.
pub struct StaticResponse {
    status_code: u16,
    resource_path: PathBuf,
}

pub enum Response {
    Error(HTTPError),
    Static(StaticResponse),
}
