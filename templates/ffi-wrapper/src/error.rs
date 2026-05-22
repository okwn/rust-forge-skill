use thiserror::Error;

#[derive(Debug, Error)]
pub enum FfiError {
    #[error("allocation failed")]
    AllocationFailed,

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("null pointer received from C")]
    NullPointer,

    #[error("unknown error code: {0}")]
    Unknown(i32),

    #[error("C library error: {0:?}")]
    CLibraryError(super::bindings::LibError),
}

impl From<super::bindings::LibError> for FfiError {
    fn from(e: super::bindings::LibError) -> Self {
        FfiError::CLibraryError(e)
    }
}