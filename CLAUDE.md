# CLAUDE.md - Guidelines for JSON Field Report

## CLI Commands
- Build project: `cargo build`
- Run CLI version: `cargo run`
- Run in release mode: `cargo run --release`
- Check code without building: `cargo check`

## Web Interface Commands
- Start web server: `./serve.sh`
- Access web interface: http://localhost:8080

## Lint & Format Commands
- Format code: `cargo fmt`
- Check formatting: `cargo fmt -- --check`
- Run lints: `cargo clippy`
- Fix automatically fixable issues: `cargo clippy --fix`

## Code Style Guidelines
- Follow Rust conventions from https://rust-lang.github.io/api-guidelines/
- Use 4 spaces for indentation
- Use snake_case for variables, functions, and file names
- Use camelCase for JavaScript variables and functions
- Prefer Result<T, E> for error handling with meaningful error types
- Organize imports alphabetically
- Write descriptive comments for functions
- Keep functions small and focused on a single task