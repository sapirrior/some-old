# Project Inkless: High-Integrity Terminal Pager

Inkless is a minimalist, high-performance successor to GNU `less`, written in C11 with a focus on POSIX compatibility and zero external dependencies. It treats the terminal as a structured canvas, prioritizing readability through specialized layout algorithms.

## 1. Technical Specifications
- **Language**: C11 (`-std=c11`)
- **Portability**: POSIX-compliant (`_POSIX_C_SOURCE=200809L`)
- **Build System**: Makefile with strict flags (`-Wall -Wextra -pedantic`)
- **Dependencies**: None (Standard C Library & POSIX APIs only)

## 2. Architectural Design
Inkless follows a flat, modular architecture designed for maximum clarity and minimal boilerplate.
- **Single Header Rule**: The entire project's public API, structures, and constants are consolidated into exactly ONE unified header file (`source/inkless.h`).
- **Domain-Driven Implementation**: Implementation logic is grouped by domain into individual `.c` files directly within the `source/` directory. Subdirectories are strictly avoided.
- **AppState Centralization**: A global `AppState` struct anchors all domain-specific data, enabling decoupled interaction between modules.

### Source Organization:
- `source/main.c`: Entry point.
- `source/app.c`: Orchestration and event loop.
- `source/commands.c`: All user action handlers (navigation, search, system).
- `source/document.c`: File I/O and text buffering.
- `source/input.c`: Keystroke parsing.
- `source/layout.c`: Smart wrapping and coordinate mapping.
- `source/terminal.c`: POSIX raw mode and ANSI sequence management.
- `source/utils.c`: Generic helpers and the POSIX regex search engine.
- `source/view.c`: Screen rendering and prompt handling.
## 3. Core Features
- **Smart Word-Wrapping**: Splits text at spaces or hyphens to maintain legibility.
- **Dynamic Margins**: Automatically applies 8% side padding based on terminal width.
- **Responsive Resizing**: Full `SIGWINCH` support; layout and margins recompute instantly on terminal resize.
- **Pipe Support**: Seamlessly functions as a standard POSIX pager (e.g., `ls | inkl`), decoupling data input from interactive terminal control via `/dev/tty` redirection.
- **Advanced Navigation**:
    - Line-by-line (`j`, `k`)
    - Half-page (`d`, `u`)
    - Full-page (`f`, `Space`, `b`)
    - Document jumps (`g`, `G`, `<`, `>`)
    - **Go-to-Line**: Jump to a specific line via the colon prompt (e.g., `:100`).
    - **Line Numbers**: Toggle line number visibility via `:N`.
    - **Multi-File Navigation**: Transition between multiple files via `:n` (next) and `:p` (previous).
- **Interactive Help**: Access a structured, monochrome command reference by pressing `h`. Returns to document via `Esc`.
- **Regex Search Engine**:
    - Forward (`/`) and Backward (`?`) search using POSIX Extended Regular Expressions.
    - Repeating searches (`n`, `N`) with direction persistence and fixed iteration logic.
    - **Visual Highlighting**: Real-time ANSI-inverted highlighting of all matches.
    - **Wraparound Search**: Automatic document loop-back with "Search wrapped" status notification.
    - **Case Sensitivity**: Toggle case-insensitive matching via the colon prompt (`:i`).
    - **Clear Search**: Press `Esc` to instantly remove all active highlighting.
- **GNU Less Aesthetic**: Minimalist colon prompt with inverted `(END)` indicator at file completion.

## 4. Getting Started
### Build Instructions
To compile Inkless, ensure you have a C compiler (GCC/Clang) and Make installed.
```bash
make          # Compiles the binary to build/inkl
make clean    # Removes the build directory
```

### Usage
```bash
./build/inkl <filename>
```

## 5. Development Workflow
- **Version Control**: Use Git for all changes. The remote repository is hosted at `https://github.com/sapirrior/inkless`.
- **Commit Messages**: Strictly follow the [Conventional Commits](https://www.conventionalcommits.org/) specification (e.g., `feat:`, `fix:`, `docs:`, `refactor:`).
- **PR Process**: All major features should be developed on feature branches and merged via pull requests.

## 6. Coding Conventions
- **Naming**: `snake_case` for functions/variables, `PascalCase` for structs.
- **Hardened Integrity**:
    - **Graceful Failure**: Centralized `ink_die` error handling with automatic terminal restoration via `g_app` state tracking.
    - **Safe Allocation**: Mandatory use of `xmalloc`, `xrealloc`, and `xstrdup` wrappers to eliminate OOM boilerplate and ensure consistent exit behavior.
    - **Defensive I/O**: Strict CRLF handling during file load.
    - **Memory Safety**: OOB prevention in layout mapping and dynamic RenderBuf sizing to eliminate overflow risks.
    - **Robust Navigation**: Clamped scroll and view logic ensures stability during extreme terminal resizing.
    - **ANSI Hygiene**: Strict attribute resetting prevents UI glitches (e.g., screen inversion) during prompt interactions.
- **Tone**: Technical documentation and commit messages follow a factual, hyperbole-free professional standard.
