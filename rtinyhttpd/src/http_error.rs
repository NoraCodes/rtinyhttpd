/// Represents an error that occured in the HTTP pipeline.
pub struct HTTPError {
    status_code: u16,
    message_title: String,
    message_body: String,
}
