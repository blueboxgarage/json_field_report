# CLAUDE.md - Guidelines for JSON Field Report

## Build & Run Commands
- Build project: `cargo build`
- Run project: `cargo run`
- Run in release mode: `cargo run --release`
- Check code without building: `cargo check`

## Test Commands
- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`
- Run tests with output: `cargo test -- --nocapture`

## Lint & Format Commands
- Format code: `cargo fmt`
- Check formatting: `cargo fmt -- --check`
- Run lints: `cargo clippy`
- Fix automatically fixable issues: `cargo clippy --fix`

## Code Style Guidelines
- Follow Rust conventions from https://rust-lang.github.io/api-guidelines/
- Use 4 spaces for indentation
- Use snake_case for variables, functions, and file names
- Use CamelCase for types and structs
- Prefer Result<T, E> for error handling with meaningful error types
- Organize imports alphabetically
- Write descriptive comments for public functions
- Keep functions small and focused on a single task