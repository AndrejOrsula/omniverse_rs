use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmniverseSysError {
    #[error("Initialization error: {0}")]
    InitializationError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Value error: {0}")]
    ValueError(String),
}
