use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid task: {0}")]
    InvalidTask(String),

    #[error("Merge conflict: {0}")]
    MergeConflict(String),

    #[error("Invalid date time: {0}")]
    InvalidDateTime(String),
}

pub type Result<T> = std::result::Result<T, TodoError>;
