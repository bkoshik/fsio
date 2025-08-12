use std::path::PathBuf;
use nix::errno::Errno;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Errno: {0}")]
    Errno(#[from] Errno),

    #[error("unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("failed to parse file {path:?}: {source}")]
    ParseError {
        path: PathBuf,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("validation failed: {0}")]
    ValidationError(String),

    #[error("file too large: {size} bytes (limit {limit} bytes)")]
    FileTooLarge {
        size: u64,
        limit: u64
    },

    #[error("internal library logic error: {0}")]
    Internal(String),
}