# AGENTS.md - Coding Guidelines for Windows Debloater

## Build Commands
- Build release: `cargo build --release`
- Build debug: `cargo build`

## Test Commands
- Run all tests: `cargo test`
- Run specific test: `cargo test <test_name>` (no tests currently exist)

## Lint and Format Commands
- Lint: `cargo clippy -- -W clippy::pedantic`
- Format: `cargo fmt`

## Code Style Guidelines

### Imports
- Group imports: std, external crates, local modules
- Use explicit imports, avoid glob imports
- Keep imports organized and minimal

### Naming Conventions
- Functions/variables: snake_case
- Modules/structs/enums: PascalCase
- Constants: SCREAMING_SNAKE_CASE
- File names: snake_case.rs

### Error Handling
- Use `Result<T, String>` for fallible operations
- Provide descriptive error messages with context
- Propagate errors up the call stack with `?` operator
- Use `eprintln!` for error output, `println!` for info

### Code Structure
- Keep functions short and focused (<50 lines preferred)
- Use meaningful variable names
- Prefer immutable bindings (`let`) over mutable (`let mut`)
- Use early returns to reduce nesting

### Comments
- Use comments sparingly, only when code intent isn't obvious
- Prefer self-documenting code over comments
- Use `//` for line comments

### Formatting
- Use default rustfmt configuration
- 4-space indentation
- Line length: ~100 characters max

### Safety
- Prefer safe Rust; use `unsafe` only when necessary
- Handle Windows API errors properly
- Validate inputs and paths before use
- DO NOT run the build artifacts at ANY cost; if testing is needed, ask the user - they will run it in a VM and provide the results.