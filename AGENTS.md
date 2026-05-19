# Project Inkless: High-Integrity Terminal Pager

Inkless is a minimalist terminal pager written in Rust (2024 Edition). It prioritizes safety, modularity, and readability through specialized layout algorithms and a strict command philosophy.

## 1. Technical Specifications
- **Language**: Rust (2024 Edition)
- **Portability**: Multi-platform via `crossterm`
- **Build System**: Cargo
- **Versioning**: Synchronized with `Cargo.toml` via `env!("CARGO_PKG_VERSION")`.
- **Primary Dependencies**:
    - `crossterm`: Cross-platform terminal manipulation (with `use-dev-tty` feature).
    - `regex`: Performance-oriented regular expressions.

## 2. Architectural Design
Inkless follows a highly granular module architecture with a strict separation of concerns.

### Strict Module Connector Pattern
Every module follows the **Connector (Facade)** pattern. `mod.rs` files contain **zero implementation logic** and serve strictly as gateways for submodules and public re-exports.

### Directory Structure:
- `src/main.rs`: Entry point, CLI parsing, and `--v` flag handling.
- `src/app/`: Application orchestration (`types.rs`, `lifecycle.rs`, `event_loop.rs`).
- `src/document/`: Text buffering and I/O (`io.rs`, `state.rs`, `types.rs`).
- `src/layout/`: Smart wrapping and coordinate mapping (`compute.rs`, `state.rs`, `types.rs`).
- `src/terminal/`: Terminal state and RAII cleanup (`guard.rs`, `utils.rs`).
- `src/input/`: Keystroke parsing using `crossterm` (`keys.rs`, `reader.rs`).
- `src/view/`: Screen rendering and prompt handling (`render.rs`, `prompt.rs`).
- `src/errors/`: Modularized error types following the connector pattern (`types.rs`).
- `src/commands/`: Domain-specific command handlers.
    - `dispatch.rs`: Central command router.
    - `cmd_nav.rs`: Navigation logic.
    - `cmd_search.rs`: Search logic.
    - `cmd_sys.rs`: System and colon commands.
    - `cmd_utils.rs`: Command shared utilities.
- `src/utils/`: Shared utilities (e.g., regex search engine).

## 3. Core Features
- **Smart Word-Wrapping**: Logical splitting at spaces or hyphens.
- **Dynamic Margins**: Automatic 8% side padding.
- **Responsive Resizing**: Signal-driven, real-time re-layout on `SIGWINCH`. Uses an optimized event-draining mechanism to ensure lag-free performance during rapid window dragging.
- **Pipe Support**: Seamlessly functions as a standard pager (e.g., `ls | inkl`). Redirects TTY control for interactive events when stdin is a pipe.
- **Live Follow Mode**: Non-blocking background I/O for real-time file monitoring (triggered by `F`).
- **One Action, One Command**: A consistent, non-redundant command set (e.g., `q` to quit, no redundant `:q`).
- **Regex Search Engine**: Forward/backward searching with inverted ANSI highlighting.
- **Dynamic Versioning**: View current version via `--v` flag or help screen.

## 4. Getting Started
### Build Instructions
```bash
cargo build --release
```

### Usage
```bash
./target/release/inkl <filename>
./target/release/inkl --v
```

## 5. Development Workflow
- **Commit Messages**: Strictly follow [Conventional Commits](https://www.conventionalcommits.org/).
- **Standard Checks**: `cargo clippy` and `cargo fmt` are mandatory.

## 6. Coding Conventions
- **RAII Patterns**: Mandatory use of `Drop` for terminal state management.
- **Structured Error Handling**: Prefer `Result` propagation over panics. Captured errors are displayed in the UI status bar.
- **Async I/O**: Use background threads for line reading to maintain UI responsiveness.
- **Instruction-Free Connector Files**: `mod.rs` must not contain implementation.
