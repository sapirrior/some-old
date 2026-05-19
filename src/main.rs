mod app;
mod commands;
mod document;
mod input;
mod layout;
mod terminal;
mod utils;
mod view;

mod errors;

#[cfg(test)]
mod tests_io;

use app::App;
use std::env;
use std::io::{self, IsTerminal};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 1 && args[0] == "--v" {
        println!("inkless version {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.is_empty() && io::stdin().is_terminal() {
        eprintln!("Usage: inkl <filename> [filename...]");
        eprintln!("   or: <command> | inkl");
        std::process::exit(1);
    }

    let mut app = App::new(args);
    app.run();
}
