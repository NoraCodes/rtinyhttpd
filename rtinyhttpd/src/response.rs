// Data structures for representing responses
use std::path::{PathBuf, Path};
use request::*;
use resource::AcquisitionError;
use http_error::HTTPError;
/// Represents a response to a valid request for a static resource.
#[derive(Debug, PartialEq)]
pub struct StaticResponse {
    status_code: u16,
    resource_path: PathBuf,
}

impl StaticResponse {
    pub fn new(status_code: u16, resource_path: PathBuf) -> StaticResponse {
        StaticResponse {
            status_code: status_code,
            resource_path: resource_path,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Error(HTTPError),
    Static(StaticResponse),
}

pub fn response_from_request<F>(request: HTTPRequest, acquisition_lambda: F) -> Response
    where F: Fn(PathBuf) -> Result<PathBuf, AcquisitionError>
{
    match request.method {
        HTTPMethod::GET => {
            // GET can be asking for static or dynamic content
            match request.resource {
                HTTPResource::Static(p) => {
                    // A static GET request. Just look for the file and move on.
                    match acquisition_lambda(p) {
                        Ok(p) => {
                            Response::Static(StaticResponse {
                                status_code: 200,
                                resource_path: p,
                            })
                        }
                        Err(e) => Response::Error(bad_request_not_found),
                    }
                }
                _ => Response::Error(server_err_not_implemented()),
            }
        }
        _ => {
            unimplemented!();
        }
    }
}
