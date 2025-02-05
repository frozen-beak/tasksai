use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing API key in environment")]
    MissingApiKey,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("File processing error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Directory traversal error: {0}")]
    WalkDirError(#[from] walkdir::Error),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing required sections: {0:?}")]
    MissingSections(Vec<&'static str>),
}
