/// Represents an error that occured in the HTTP pipeline.
#[derive(Debug, PartialEq)]
pub struct HTTPError {
    status_code: u16,
    message_title: String,
    message_body: String,
}

impl HTTPError {
    pub fn new(status_code: u16, message_title: &str, message_body: &str) -> HTTPError {
        HTTPError {
            status_code: status_code,
            message_title: String::from(message_title),
            message_body: String::from(message_body),
        }
    }
}
