use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a detailed technical plan for a task.
    Plan {
        /// The task description (can include file references using '@<file_path>')
        task: String,

        /// Enable interactive confirmation before processing.
        #[arg(short, long)]
        interactive: bool,

        /// The required output file where the generated plan will be written.
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Analyze the provided files for potential bugs and vulnerabilities.
    Check {
        /// One or more file or directory paths to analyze.
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },

    /// Analyze the provided files for performance improvements.
    Perf {
        /// One or more file or directory paths to analyze.
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },

    /// Add documentation in code
    Docs {
        /// A code file to generate the code for
        #[arg(required = true)]
        file: PathBuf,
    },

    /// Write unit tests for a input file
    Test {
        /// A code file to write test for
        #[arg(required = true)]
        file: PathBuf,

        /// The required output file where the generated plan will be written.
        #[arg(short, long, required = true)]
        output: PathBuf,
    },
}

impl Cli {
    pub fn parse() -> Self {
        <Cli as Parser>::parse()
    }
}
