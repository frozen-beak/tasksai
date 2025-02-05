mod cli;
mod errors;
mod file_utils;
mod gemini_client;
mod prompts;

use std::time::Duration;

use cli::{Cli, Commands};
use dialoguer::{theme::ColorfulTheme, Confirm};
use dotenv::dotenv;
use errors::{AppError, ValidationError};
use file_utils::{extract_files, read_inputs, sanitize_path, validate_files};
use gemini_client::GeminiClient;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<(), AppError> {
    dotenv().ok();
    let cli = Cli::parse();

    // Retrieve API key from the environment.
    let api_key = std::env::var("API_KEY").map_err(|_| AppError::MissingApiKey)?;
    let client = GeminiClient::new(api_key);

    match cli.command {
        Commands::Plan {
            task,
            interactive,
            output,
        } => {
            // Confirm the operation if interactive mode is enabled.
            if interactive
                && !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Proceed with plan generation?\nTask: {}", task))
                    .interact()?
            {
                println!("Operation cancelled.");
                return Ok(());
            }

            // Extract file references from the task and validate each path.
            let (clean_task, inputs) = extract_files(&task);
            for path in &inputs {
                sanitize_path(path)?;
            }
            validate_files(&inputs)?;

            // Read the contents from the provided files (if any).
            let file_contents = read_inputs(&inputs)?;

            // Construct the prompt combining the clean task and file contents (if any).
            let user_prompt = if file_contents.is_empty() {
                clean_task.clone()
            } else {
                format!("{}\n\n{}", clean_task, file_contents)
            };

            // Create and start the loading spinner.
            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Generating plan...");
            spinner.enable_steady_tick(Duration::from_millis(100));
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );

            // Generate the plan using the Gemini API.
            let generated_plan = client.generate_plan(user_prompt)?;

            // Stop the spinner.
            spinner.finish_with_message("Plan generated!");

            // Validate that the plan contains the required sections.
            validate_generated_plan(&generated_plan).unwrap();

            // Write the generated plan to the required output file.
            std::fs::write(&output, &generated_plan)?;
            println!("Plan generated and written to: {}", output.display());
        }

        Commands::Check { files } => {
            // Validate each file/directory path and read its content.
            for path in &files {
                sanitize_path(path)?;
            }
            validate_files(&files)?;
            let code_contents = read_inputs(&files)?;

            // Create and start the loading spinner.
            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Performing static analysis...");
            spinner.enable_steady_tick(Duration::from_millis(100));
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );

            // Invoke the static analysis endpoint.
            let bug_report = client.check_code(code_contents)?;

            // Stop the spinner.
            spinner.finish_with_message("Static analysis complete!");

            let bugs: Vec<String> = serde_json::from_str(&bug_report).map_err(|e| {
                AppError::InvalidResponse(format!("Error parsing bug report JSON: {}", e))
            })?;

            if bugs.is_empty() {
                println!("Bug Analysis Report: No bugs found!");
            } else {
                println!("Bug Analysis Report:\n");
                for (i, bug) in bugs.iter().enumerate() {
                    println!("{}. {}", i + 1, bug);
                }
            }

            println!("");
        }

        Commands::Perf { files } => {
            // Validate each file/directory path and read its content.
            for path in &files {
                sanitize_path(path)?;
            }
            validate_files(&files)?;
            let code_contents = read_inputs(&files)?;

            // Create and start the loading spinner.
            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Checking for performance improvements...");
            spinner.enable_steady_tick(Duration::from_millis(100));
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );

            // Invoke the static analysis endpoint.
            let bug_report = client.improve_performance(code_contents)?;

            // Stop the spinner.
            spinner.finish_with_message("Checked for performance improvements");

            let bugs: Vec<String> = serde_json::from_str(&bug_report).map_err(|e| {
                AppError::InvalidResponse(format!("Error parsing the response: {}", e))
            })?;

            if bugs.is_empty() {
                println!("No improvements found!");
            } else {
                println!("Performance Improvements:\n");
                for (i, bug) in bugs.iter().enumerate() {
                    println!("{}. {}", i + 1, bug);
                }
            }

            println!("");
        }

        Commands::Docs { file } => {
            sanitize_path(&file)?;
            validate_files(&[file.clone()])?;
            let code_contents = read_inputs(&[file.clone()])?;

            // Create and start the loading spinner.
            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Generating docs...");
            spinner.enable_steady_tick(Duration::from_millis(100));
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );

            // Invoke the docs generation endpoint.
            let updated_code = client.generate_docs(code_contents)?;

            // Stop the spinner.
            spinner.finish_with_message("Generated docs");

            // Unescape the returned string to convert "\n" sequences into actual newlines.
            let unescaped_code: String =
                serde_json::from_str(&updated_code).unwrap_or_else(|_| updated_code.clone());

            // Update the input file with the new code.
            std::fs::write(&file, &unescaped_code)?;

            println!("Added docs in -> {}", file.display());
        }
    }

    Ok(())
}

/// Validates that the generated plan contains the required sections.
fn validate_generated_plan(plan: &str) -> Result<(), ValidationError> {
    // Update the required sections based on the plan structure.
    let required_sections = [
        "## Objectives",
        "## Implementation Steps",
        "## File Manifest",
    ];
    let missing: Vec<_> = required_sections
        .iter()
        .filter(|&&section| !plan.contains(section))
        .copied()
        .collect();

    if !missing.is_empty() {
        return Err(ValidationError::MissingSections(missing));
    }
    Ok(())
}
