use std::path::PathBuf;

pub enum AcquisitionError {
    /// Used when a resource simply doesn't exist.
    NotFoundError,
    /// Used when a resource exists, but the server can't read it.
    NotPermittedToReadError,
}

/// A test acquisition strategy that doesn't actually check anything and always succeeds
pub fn acquire_test_always_succeeding(path: PathBuf) -> Result<PathBuf, AcquisitionError> {
    Ok(path)
}

/// A test acquisition strategy that doesn't actually check anything and never finds a resource
pub fn acquire_test_never_found(path: PathBuf) -> Result<PathBuf, AcquisitionError> {
    Err(AcquisitionError::NotFoundError)
}

/// A test acquisition strategy that doesn't actually check anything and never has permission to
/// access a resource
pub fn acquire_test_never_permitted(path: PathBuf) -> Result<PathBuf, AcquisitionError> {
    Err(AcquisitionError::NotPermittedToReadError)
}
