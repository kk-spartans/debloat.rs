# AGENTS.md

## Commands
- Build: `cargo build` / `cargo build --release`

## WARNING
NEVER run the build artifact - this app makes destructive changes to the OS

## Post-Task Actions
After completing every task:
1. Run `cargo clippy -- -W clippy::pedantic`
2. Fix any errors or warnings from clippy
3. Only after fixing all warnings and errors, run `cargo fmt`

## Code Style
- **Imports**: Group std imports, external crates; sort alphabetically
- **Naming**: snake_case for functions/variables, PascalCase for types
- **Errors**: Return `Result<T, String>` with `format!` for error messages
- **Error handling**: Use `if let Err(e)` / `if let Ok(...)`, `let _ =` to ignore errors deliberately
- **Logging**: Use `println!` with `[START]`, `[OK]`, `[ERROR]`, `[WARN]` prefixes; `eprintln!` for errors. Keep logs consistent with other parts of the codebase, even across different modules.
- **Unsafe**: Use unsafe blocks for Windows API calls; keep them minimal and scoped
- **Early returns**: Use early returns/early exits for cleaner control flow
- **No comments**: Don't add comments unless explicitly requested
