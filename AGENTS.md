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

### Directory Structure and Component Responsibilities:
- `src/main.rs`: Entry point, parses command-line arguments (including handling of the `--v` version flag), verifies interactive TTY redirection, and constructs/runs the main `App` instance.
- `src/app/`: Application orchestration.
  - `core.rs`: Defines the central `App` struct holding active state (document, layout, scroll position, files list, regex search pattern/direction, error/loading/follow modes, and terminal size).
  - `event_loop.rs`: Orchestrates the main execution loop, coordinates rendering and user-input handling, and runs non-blocking I/O line-draining tasks.
- `src/document/`: Text buffering and source loading.
  - `core.rs`: Implements the `Document` struct that buffers the raw lines.
  - `io.rs`: Spawns a background thread to read from a file descriptor or stdin stream, communicating line additions back asynchronously via standard channels.
- `src/layout/`: Coordinate mapping and formatting.
  - `core.rs`: Defines the `Layout` struct tracking wrapping mappings between raw document indices and visual layout lines.
  - `compute.rs`: Performs layout wrapping calculations, formatting lines using an 8% width margin, and wrapping logically at spaces or hyphens.
- `src/terminal/`: Terminal state management.
  - `guard.rs`: Implements `TerminalGuard` using the RAII pattern via the `Drop` trait to automatically restore raw mode and alternate screen states upon exits.
  - `utils.rs`: Queries current terminal size using `crossterm::terminal::size`.
- `src/input/`: Keystroke detection and events.
  - `keys.rs`: Declares the `Key` enum representing supported interactive keystrokes and resize signals.
  - `reader.rs`: Reads and parses keyboard/resize inputs. Drains consecutive resize events during window dragging to prevent thrashing.
- `src/view/`: Screen graphics and visual widgets.
  - `render.rs`: Renders document lines (incorporating inverted ANSI colors for search pattern hits), help overlay, and the status bar.
  - `prompt.rs`: Draws the bottom user-input prompt for search queries and colon commands.
- `src/errors/`: Standardized error propagation.
  - `types.rs`: Declares custom error variants (`InklessError`) wrapping standard I/O or terminal errors.
- `src/commands/`: Action handlers and key maps.
  - `dispatch.rs`: Maps incoming key events to their respective command handler modules.
  - `cmd_nav.rs`: Handles scrolling movement logic (up, down, half-page, full-page, home, end) and follow mode toggles.
  - `cmd_search.rs`: Manages input prompts for forward/backward search and navigates to matching patterns.
  - `cmd_sys.rs`: Evaluates colon actions (`:n` for next file, `:p` for previous file, `:N` for toggling line numbers, and line index jumps).
  - `cmd_utils.rs`: Shares routines for drawing prompts and clamping scroll bounds.
- `src/utils/`: Auxiliary engines.
  - `search.rs`: Implements backward and forward regex searching, incorporating wrapping search boundaries.

## 3. Core Features
- **Smart Word-Wrapping**: Logical splitting at spaces or hyphens to keep text highly legible.
- **Dynamic Margins**: Automatic side padding calculated at 8% of the terminal width.
- **Responsive Resizing**: Signal-driven, real-time re-layout on `SIGWINCH` with optimized event draining to prevent layout lag.
- **Pipe Support**: Automatically detects when input is redirected (e.g. `cat file.txt | inkl`) and handles non-blocking streaming.
- **Live Follow Mode**: Follows growing logs or slow pipe streams concurrently without blocking the user interface (activated by pressing `F`).
- **One Action, One Command**: Strict, simple command set with zero duplicate command combinations.
- **Regex Search Engine**: Performs wrapping forward (`/`) and backward (`?`) regex searches with highlighted matches.
- **Dynamic Versioning**: View current version via `--v` flag or help screen.

## 4. Getting Started
### Build Instructions
```bash
cargo build --release
```

### Usage
```bash
# View local files
./target/release/inkl <filename>

# Read from standard input pipe
cat <filename> | ./target/release/inkl

# Version info
./target/release/inkl --v
```

## 5. Development Workflow
- **Commit Messages**: Strictly follow [Conventional Commits](https://www.conventionalcommits.org/).
- **Standard Checks**: `cargo clippy` and `cargo fmt` are mandatory.

## 6. Coding Conventions
- **RAII Patterns**: Mandatory use of `Drop` for terminal state management (`TerminalGuard`).
- **Structured Error Handling**: Prefer `Result` propagation over panics. Captured errors are displayed in the UI status bar.
- **Async I/O**: Use background threads for line reading to maintain UI responsiveness.
- **Instruction-Free Connector Files**: `mod.rs` connector files must not contain implementation.
