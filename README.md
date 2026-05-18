# Inkless

Inkless is a minimalist, high-integrity terminal pager written in Rust. It focuses on modularity, readability, and memory safety, providing a modern alternative to traditional pagers like GNU `less`.

## Key Features

- **Smart Word-Wrapping**: Logical line breaks at spaces or hyphens to maintain text legibility.
- **Dynamic Margins**: Automatic side padding for improved readability on wide displays.
- **Regex Search**: Pattern-based forward and backward searching with real-time visual highlighting.
- **Responsive Resizing**: Immediate layout recomputation upon terminal window resizing.
- **Strict Command Philosophy**: A consistent, non-redundant interface following a "One Action, One Command" rule.

## Installation

Ensure you have the Rust toolchain (2021 edition or later) and Cargo installed.

```bash
cargo build --release
```

The binary will be available at `./target/release/inkless`.

## Usage

```bash
# View a specific file
inkless filename.txt

# View output from a pipe
ls -R | inkless

# Check version
inkless --v
```

## Command Reference

| Key | Action |
| :--- | :--- |
| `j` / `Down` | Scroll down one line |
| `k` / `Up` | Scroll up one line |
| `d` | Scroll down half a page |
| `u` | Scroll up half a page |
| `f` / `PageDown` | Scroll down one page |
| `b` / `PageUp` | Scroll up one page |
| `g` / `Home` | Jump to the beginning of the document |
| `G` / `End` | Jump to the end of the document |
| `/` | Search forward for a pattern |
| `?` | Search backward for a pattern |
| `n` | Repeat the last search in the same direction |
| `N` | Repeat the last search in the reverse direction |
| `:n` | Open the next file in the argument list |
| `:p` | Open the previous file in the argument list |
| `:N` | Toggle line number visibility |
| `:<num>` | Jump to specific line number |
| `h` | Open help screen |
| `q` | Exit Inkless |
| `Esc` | Clear search highlights or return from help |

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
