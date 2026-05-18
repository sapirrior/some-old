# Contributing to Inkless

Thank you for your interest in contributing to Inkless. This project maintains high engineering standards through modularity, safety, and a focused user interface.

## Engineering Philosophy

- **High Integrity**: Leverage Rust's type system and ownership model to ensure memory safety and prevent undefined behavior.
- **Granular Modularity**: Maintain a strict separation of concerns using the "Connector" pattern. `mod.rs` files must not contain implementation logic.
- **Predictable Interface**: Adhere to the "One Action, One Command" philosophy to avoid command redundancy.

## Development Environment

- **Rust**: 2021 Edition or later.
- **Cargo**: Used for building, testing, and dependency management.

## Development Workflow

1. **Branching**: Create descriptive feature branches from `main`. We prefer a linear history; please rebase your changes against the latest `main` branch before submitting.
2. **Quality Checks**: Before submitting a Pull Request, ensure your changes pass the following checks:
   - `cargo fmt --all`: All code must conform to standard formatting.
   - `cargo clippy --all-targets -- -D warnings`: All code must be free of lints and warnings.
   - `cargo test`: All unit and integration tests must pass.

## Commit Guidelines

We strictly follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for all repository changes.

### Format
`<type>(<scope>): <description>`

### Types
- `feat`: A new feature.
- `fix`: A bug fix.
- `docs`: Documentation only changes.
- `style`: Changes that do not affect the meaning of the code (formatting, missing semi-colons, etc).
- `refactor`: A code change that neither fixes a bug nor adds a feature.
- `perf`: A code change that improves performance.
- `test`: Adding missing tests or correcting existing tests.
- `chore`: Changes to the build process or auxiliary tools and libraries.

### Example
`feat(search): implement case-insensitive matching`

Note: Breaking changes should be indicated by a `!` after the type/scope (e.g., `feat(ui)!: redesign status bar`).

## Submission Process

1. Fork the repository and create your feature branch.
2. Implement your changes and add tests where applicable.
3. Complete the **Quality Checks** listed above.
4. Submit a Pull Request with a concise description of the changes and the rationale behind them.

All contributions are licensed under the MIT License.
