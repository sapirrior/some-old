# Inkless (Rust Edition)

**Inkless** is a minimalist, high-integrity terminal pager rewritten in Rust. It serves as a modern successor to GNU `less`, prioritizing readability, modularity, and memory safety.

![Version](https://img.shields.io/badge/version-0.2.0-blue)
![Rust](https://img.shields.io/badge/rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Granular Modularity**: Architected with a strict "Connector" pattern for high maintainability.
- **Smart Word-Wrapping**: Maintains legibility by splitting text at logical boundaries.
- **Dynamic Margins**: Automatic side padding for a cleaner reading experience.
- **Regex Search**: Powerful forward/backward searching with real-time visual highlighting.
- **Responsive Design**: Full terminal resize support with instant layout recomputation.
- **Memory Safety**: Zero manual memory management; protected by Rust's ownership model.
- **Zero-Cost Abstractions**: High performance with minimal overhead.

## Architecture

Inkless uses a modern directory-based module system:
- **`src/app/`**: Application state and main event loop.
- **`src/terminal/`**: RAII-based raw mode management.
- **`src/document/`**: Safe file I/O and buffering.
- **`src/layout/`**: Coordinate mapping and wrapping algorithms.
- **`src/commands/`**: Modular command dispatchers (Nav, Search, Sys).

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Cargo)

### Build
```bash
git clone https://github.com/sapirrior/inkless.git
cd inkless
cargo build --release
```

## Usage

```bash
# Paging a file
./target/release/inkless my_file.txt

# Paging from a pipe
ls -R | ./target/release/inkless
```

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `Down` | Scroll down one line |
| `k` / `Up` | Scroll up one line |
| `f` / `Space` | Page down |
| `b` | Page up |
| `g` / `<` | Jump to top |
| `G` / `>` | Jump to end |
| `/` | Search forward |
| `?` | Search backward |
| `n` / `N` | Repeat search / Reverse repeat |
| `:N` | Toggle line numbers |
| `:q` / `q` | Quit |
| `h` | Help |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
