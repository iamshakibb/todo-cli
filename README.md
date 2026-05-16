# todo-cli

A CLI-based todo list application built with Rust.

## What I Learned

### 1. Project Structure
- **src/main.rs**: Entry point - creates App and runs CLI
- **src/lib.rs**: Public API - exposes modules, handles file setup
- **src/app.rs**: Core business logic - manages todos in memory, handles persistence
- **src/todo.rs**: Data model - Todo struct with serialization
- **src/cli/**: CLI commands - add, ls, delete, complete, edit

### 2. Rust Fundamentals
- **Modules**: `mod` keyword to declare modules, `pub` to expose
- **Structs & Methods**: `impl` blocks for methods on structs
- **Enums**: Used for CLI commands and output formats
- **Error Handling**: `Result<>` type with `?` operator
- **Option Type**: `Option<T>` for nullable values

### 3. Crates Used
- **clap**: CLI argument parsing with derive macros
- **serde**: JSON serialization/deserialization
- **tabled**: Pretty table output for todos

### 4. Key Patterns
- **HashMap**: Store todos by ID for O(1) lookups
- **File I/O**: Read/write JSON file for persistence
- **Derive Macros**: `#[derive(Serialize, Deserialize, Parser)]`
- **Pattern Matching**: `match` for command routing

### 5. CLI Commands
```
todo-cli ls              # List all todos
todo-cli ls -c           # Show only completed
todo-cli ls -i           # Show only incomplete
todo-cli add "Task"      # Add a todo
todo-cli add "Task" -d "desc"  # With description
todo-cli delete --id 1   # Delete by ID
todo-cli complete --id 1 --complete complete  # Mark complete
todo-cli edit --id 1 --title "New" # Edit todo
```

## Commands to Run

```bash
cargo run -- add "Learn Rust"
cargo run -- ls
cargo run -- ls -i
cargo run -- complete --id 1 --complete complete
cargo run -- edit --id 1 --title "Updated"
cargo run -- delete --id 1
```