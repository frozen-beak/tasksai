//! Defines custom error types for the application.
//!
//! This module centralizes error handling by defining specific error enums
//! for different parts of the application, such as API key handling, response parsing,
//! file operations, path manipulations, directory traversal, and HTTP requests.

use thiserror::Error;

/// Represents the main application error type, encompassing various error scenarios.
#[derive(Debug, Error)]
pub enum AppError {
    /// Indicates that the API key is missing from the environment variables.
    #[error("Missing API key in environment")]
    MissingApiKey,

    /// Represents an invalid response format from an external API or service.
    /// Contains a descriptive message about the invalid format.
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// Represents an error during file processing operations.
    /// Wraps the standard `std::io::Error` to provide more context.
    #[error("File processing error: {0}")]
    FileError(#[from] std::io::Error),

    /// Represents an error related to file paths or path manipulation.
    /// Contains a descriptive message about the path error.
    #[error("Path error: {0}")]
    PathError(String),

    /// Represents an error during directory traversal using the `walkdir` crate.
    /// Wraps the `walkdir::Error` to provide more context.
    #[error("Directory traversal error: {0}")]
    WalkDirError(#[from] walkdir::Error),

    /// Represents an error during HTTP requests using the `reqwest` crate.
    /// Wraps the `reqwest::Error` to provide more context.
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

/// Represents validation errors for configuration or data.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Indicates that required sections are missing during validation.
    /// Contains a vector of missing section names (as string slices).
    #[error("Missing required sections: {0:?}")]
    MissingSections(Vec<&'static str>),
}
