use crate::errors::AppError;
use regex::Regex;
use std::fs;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

/// Given a task string, extracts file paths marked with '@' and returns
/// the cleaned task and a list of paths.
pub fn extract_files(task: &str) -> (String, Vec<PathBuf>) {
    let re = Regex::new(r"@(\S+)").unwrap();
    let mut inputs = Vec::new();
    let clean_task = re.replace_all(task, "").trim().to_string();

    for cap in re.captures_iter(task) {
        if let Some(matched) = cap.get(1) {
            inputs.push(PathBuf::from(matched.as_str()));
        }
    }

    (clean_task, inputs)
}

/// Ensures that the provided paths exist.
pub fn validate_files(paths: &[PathBuf]) -> Result<(), AppError> {
    for path in paths {
        if !path.exists() {
            return Err(AppError::FileError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Path does not exist: {}", path.display()),
            )));
        }
    }

    Ok(())
}

/// Reads and concatenates the contents of all provided file paths
/// (if a directory is given, it traverses it).
pub fn read_inputs(paths: &[PathBuf]) -> Result<String, AppError> {
    let mut combined = String::new();
    for path in paths {
        if path.is_file() {
            let content = fs::read_to_string(path)?;
            combined.push_str(&format!(
                "\n\n----- File: {} -----\n{}\n",
                path.display(),
                content
            ));
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                let file_path = entry.path();
                let content = fs::read_to_string(file_path)?;
                combined.push_str(&format!(
                    "\n\n----- File: {} -----\n{}\n",
                    file_path.display(),
                    content
                ));
            }
        }
    }

    Ok(combined)
}

/// Simple check to prevent directory traversal in file paths.
pub fn sanitize_path(path: &Path) -> Result<(), AppError> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(AppError::PathError(format!(
            "Directory traversal detected in path: {}",
            path.display()
        )));
    }

    Ok(())
}
