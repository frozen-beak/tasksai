use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author = "TasksAI",
    version = "1.0",
    about = "A CLI tool to generate technical plans, analyze code for bugs, improve performance, add documentation, and generate tests.",
    long_about = "TasksAI is a versatile command-line tool that leverages a Gemini API to help developers streamline their workflow."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a detailed technical plan for a task.
    #[command(
        about = "Generate a detailed technical plan.",
        long_about = "Generate a detailed, actionable technical plan that includes objectives, implementation steps, and a file manifest. \
You can reference files in the task description using '@<file_path>'. The generated plan will be written to the specified output file."
    )]
    Plan {
        /// The task description (supports file references using '@<file_path>').
        task: String,

        /// Enable interactive confirmation before processing.
        #[arg(short, long)]
        interactive: bool,

        /// The output file where the generated plan will be written.
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Analyze the provided files for potential bugs and vulnerabilities.
    #[command(
        about = "Analyze code for bugs and vulnerabilities.",
        long_about = "Perform static analysis on one or more files or directories to identify potential bugs and security vulnerabilities. \
The output is a formatted list of detected issues."
    )]
    Check {
        /// One or more file or directory paths to analyze.
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },

    /// Analyze the provided files for performance improvements.
    #[command(
        about = "Analyze code for performance issues.",
        long_about = "Perform static analysis on one or more files or directories to identify potential performance bottlenecks. \
The tool provides suggestions to enhance the performance of your code."
    )]
    Perf {
        /// One or more file or directory paths to analyze.
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },

    /// Add documentation in code.
    #[command(
        about = "Generate and insert documentation into a code file.",
        long_about = "Automatically generate documentation comments for the specified code file using a Gemini API. \
The generated documentation is directly added into the code file."
    )]
    Docs {
        /// The code file for which to generate documentation.
        #[arg(required = true)]
        file: PathBuf,
    },

    /// Write unit tests for an input file.
    #[command(
        about = "Generate unit tests for a code file.",
        long_about = "Automatically generate unit tests for the specified code file using a Gemini API. \
The generated tests will be written to the provided output file."
    )]
    Test {
        /// The code file for which to generate tests.
        #[arg(required = true)]
        file: PathBuf,

        /// The output file where the generated tests will be written.
        #[arg(short, long, required = true)]
        output: PathBuf,
    },
}

impl Cli {
    pub fn parse() -> Self {
        <Cli as Parser>::parse()
    }
}
