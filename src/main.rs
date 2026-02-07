mod commands;
mod game_state;
mod parser;
mod world;

use std::io::{self, Write};

fn main() {
    print_banner();

    let mut state = world::build_world();

    // Show the starting room
    println!("{}", commands::execute(&game_state::Command::Look, &mut state));
    println!("\nWhat will you do?");

    while state.running {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }

        let cmd = parser::parse(&input);
        let output = commands::execute(&cmd, &mut state);
        println!("{}", output);
    }
}

fn print_banner() {
    println!(
        r#"
================================================================
     _   _  ___  ____ ___ _____ _______   __
    | | | |/ _ \| __ )_ _|_   _|_   _\ \ / /
    | |_| | | | |  _ \| |  | |   | |  \ V /
    |  _  | |_| | |_) | |  | |   | |   | |
    |_| |_|\___/|____/___| |_|   |_|   |_|

        An 80s Text Adventure in Rust
     Loosely based on The Hobbit (1982)
================================================================

In a hole in the ground there lived a hobbit. Not a nasty,
dirty, wet hole — it was a hobbit-hole, and that means comfort.

You are Dobo Daggins, a respectable hobbit of Bag End. One
morning a wizard and thirteen dwarves arrive at your door and
before you know it, you've been swept up in an adventure.

Type HELP for a list of commands.
"#
    );
}
