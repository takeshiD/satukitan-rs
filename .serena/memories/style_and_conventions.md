# Style and Conventions
- Follow idiomatic Rust style; use `cargo fmt` for formatting and `cargo clippy` for linting.
- Parser should leverage the `nom` crate per project requirement.
- Implement REPL and file execution entry points; adopt modular organization (lexer/parser/evaluator modules) as implementation matures.
- Add concise comments only for complex logic per project instructions.