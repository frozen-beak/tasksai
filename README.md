# TasksAI

TasksAI is a CLI tool designed to help streamline your workflow

![demo](./media/demo.gif)

## Running Locally

1. **Install Rust**  
   Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

2. **Clone this Repository**  
   ```bash
   git clone https://github.com/frozen-beak/tasksai.git
   cd tasksai
   ```

3. **Configure API Key**  
   Get your `API_KEY` from [AiStudio](https://aistudio.google.com/) and add it to 
   a `.env` file in the project root:
   
   ```env
   API_KEY=your_api_key_here
   ```

4. **Build and Run**  
   To see the help documentation:

   ```bash
   cargo run -- -h
   ```

## Available Commands

Below is a list of available commands with usage instructions and descriptions:

### `plan`

Generates a detailed technical plan for a specified task.

**Usage:**  
```bash
cargo run -- plan "<task_description> [@<file_path> ...]" -o <output_file> [--interactive]
```

**Info:**  

- The **task description** can include file references (e.g., `@src/main.rs`).
- The **output file** is required; the generated plan is written there.
- Use the `--interactive` flag to enable interactive confirmation before processing.

**Example:**  

```bash
cargo run -- plan "Implement feature X @src/lib.rs" -o plan.md --interactive
```

### `check`

Analyzes one or more files or directories for potential bugs and security vulnerabilities.  

**Usage:**  
```bash
cargo run -- check <file_or_directory> [<file_or_directory> ...]
```

**Example:**  
```bash
cargo run -- check src/
```

### `perf`

Analyzes one or more files or directories for performance improvements.

**Usage:**  
```bash
cargo run -- perf <file_or_directory> [<file_or_directory> ...]
```

**Example:**  
```bash
cargo run -- perf src/
```

### `docs`

Generates and inserts documentation directly into a specified code file.

**Usage:**  
```bash
cargo run -- docs <file>
```

**Example:**  
```bash
cargo run -- docs src/main.rs
```

### `test`

Generates unit tests for a specified code file and writes the tests to the provided output file.

**Usage:**  
```bash
cargo run -- test <file> -o <output_file>
```

**Example:**  
```bash
cargo run -- test src/main.rs -o tests/main_tests.rs
```

## Additional Notes

- **API Key:**  
  Ensure your `.env` file contains a valid `API_KEY` before running any command.

- **Interactive Mode:**  
  The `plan` command offers an interactive mode for extra confirmation, which is useful for avoiding unintended operations.

- **File References in Task:**  
  When generating plans, you can reference other files in the task description by prefixing the file path with `@`. The tool will read and include their contents in the prompt.

Happy coding!
