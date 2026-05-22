use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("item not found: {0}")]
    NotFound(Uuid),

    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("name cannot be empty")]
    EmptyName,
}

impl Error {
    pub fn not_found(id: Uuid) -> Self {
        Error::NotFound(id)
    }
}