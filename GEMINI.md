# Project Inkless: High-Integrity Terminal Pager (Rust Edition)

Inkless is a minimalist, high-performance successor to GNU `less`, rewritten in Rust (2021 Edition) with a focus on safety, modularity, and zero manual memory management. It treats the terminal as a structured canvas, prioritizing readability through specialized layout algorithms.

## 1. Technical Specifications
- **Language**: Rust (2021 Edition)
- **Portability**: Multi-platform via `crossterm`
- **Build System**: Cargo
- **Primary Dependencies**:
    - `crossterm`: Cross-platform terminal manipulation.
    - `regex`: Performance-oriented regular expressions.

## 2. Architectural Design
Inkless follows a highly granular, modern Rust module architecture designed for maximum clarity and strict separation of concerns.

### Strict Module Connector Pattern
Every module follows the **Connector (Facade)** pattern. `mod.rs` files contain **zero implementation logic** and serve strictly as gateways for submodules and public re-exports.

### Directory Structure:
- `src/main.rs`: Entry point and CLI parsing.
- `src/app/`: Application orchestration.
    - `types.rs`: Central `App` state.
    - `lifecycle.rs`: Init and file switching.
    - `event_loop.rs`: Main run loop.
- `src/document/`: Text buffering and I/O.
    - `io.rs`: File and stream loading.
    - `state.rs`: Buffer management.
- `src/layout/`: Smart wrapping and coordinate mapping.
    - `compute.rs`: Layout calculation logic.
- `src/terminal/`: Terminal state and RAII cleanup.
    - `guard.rs`: `TerminalGuard` for automatic raw mode restoration.
- `src/input/`: Keystroke parsing using `crossterm`.
- `src/view/`: Screen rendering and prompt handling.
- `src/commands/`: Domain-specific command handlers.
    - `nav.rs`, `search.rs`, `sys.rs`.
- `src/utils/`: Shared utilities (e.g., regex search engine).

## 3. Core Features
- **Smart Word-Wrapping**: Splits text at spaces or hyphens to maintain legibility.
- **Dynamic Margins**: Automatically applies 8% side padding based on terminal width.
- **Responsive Resizing**: Dynamic `SIGWINCH` handling; layout and margins recompute instantly on terminal resize.
- **Pipe Support**: Seamlessly functions as a standard pager (e.g., `ls | inkl`).
- **Advanced Navigation**:
    - Line-by-line (`j`, `k`)
    - Half-page (`d`, `u`)
    - Full-page (`f`, `Space`, `b`)
    - Document jumps (`g`, `G`, `<`, `>`)
    - **Go-to-Line**: Jump to a specific line via the colon prompt (e.g., `:100`).
    - **Line Numbers**: Toggle line number visibility via `:N`.
- **Interactive Help**: Press `h` for a command reference.
- **Regex Search Engine**:
    - Forward (`/`) and Backward (`?`) search.
    - Repeating searches (`n`, `N`) with direction persistence.
    - **Visual Highlighting**: Inverted ANSI highlighting of all matches.

## 4. Getting Started
### Build Instructions
Ensure you have the Rust toolchain (Cargo) installed.
```bash
cargo build --release
```

### Usage
```bash
./target/release/inkless <filename>
# or
cargo run -- <filename>
```

## 5. Development Workflow
- **Commit Messages**: Strictly follow [Conventional Commits](https://www.conventionalcommits.org/).
- **Standard Checks**: Always run `cargo clippy` and `cargo fmt` before submitting PRs.

## 6. Coding Conventions
- **RAII Patterns**: Use `Drop` implementations for all resource management (Terminal, Buffers).
- **Hardened Integrity**:
    - **Fail-Fast**: Use panics for fatal terminal errors; `TerminalGuard` ensures safe restoration.
    - **Type Safety**: Leverage Rust's ownership and type system to eliminate OOB and UAF risks.
    - **ANSI Hygiene**: Rely on `crossterm` for attribute management to prevent UI glitches.
