mod art;
mod color;
mod commands;
mod game_state;
mod loader;
mod npc;
mod parser;
mod save;
mod triggers;
mod world;

use std::io::{self, Write};

fn main() {
    print_banner();

    let mut state = loader::load_game_yaml("game.yaml")
        .unwrap_or_else(|e| {
            eprintln!("Failed to load game: {}", e);
            std::process::exit(1);
        });

    // Show the starting room
    println!("{}", commands::execute(&game_state::Command::Look, &mut state));
    println!("\nWhat will you do?");

    while state.running {
        print!("\n{}> {}", color::BOLD, color::RESET);
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

        let trimmed = input.trim().to_lowercase();

        // Handle save/load before anything else (works even during riddles)
        if trimmed == "save" {
            match save::save_game(&state) {
                Ok(msg) => println!("{}", msg),
                Err(msg) => println!("{}", msg),
            }
            continue;
        }
        if trimmed == "load" || trimmed == "restore" {
            match save::load_game() {
                Ok(loaded) => {
                    state = loaded;
                    println!("Game loaded successfully!");
                    println!(
                        "{}",
                        commands::execute(&game_state::Command::Look, &mut state)
                    );
                }
                Err(msg) => println!("{}", msg),
            }
            continue;
        }

        // If a riddle is pending, handle input as an answer
        let output = if state.pending_riddle.is_some() {
            triggers::handle_riddle_answer(&input, &mut state)
        } else {
            let cmd = parser::parse(&input);
            commands::execute(&cmd, &mut state)
        };
        println!("{}", output);

        // Run NPC tick (idle behaviour, movement) — skip during riddle game
        if state.running && state.pending_riddle.is_none() {
            state.turn += 1;
            let npc_output = npc::tick(&mut state);
            if !npc_output.is_empty() {
                print!("{}", npc_output);
            }
        }
    }
}

fn print_banner() {
    print!("\n{}{}", color::BOLD, color::CYAN);
    println!(r#"================================================================"#);
    println!(r#"     _   _  ___  ____ ___ _____ _______   __"#);
    println!(r#"    | | | |/ _ \| __ )_ _|_   _|_   _\ \ / /"#);
    println!(r#"    | |_| | | | |  _ \| |  | |   | |  \ V /"#);
    println!(r#"    |  _  | |_| | |_) | |  | |   | |   | |"#);
    println!(r#"    |_| |_|\___/|____/___| |_|   |_|   |_|"#);
    print!("{}", color::RESET);
    println!();
    println!(
        "{}        An 80s Text Adventure in Rust{}",
        color::YELLOW, color::RESET
    );
    println!(
        "{}     Loosely based on The Hobbit (1982){}",
        color::DIM, color::RESET
    );
    println!(
        "{}{}================================================================{}",
        color::BOLD, color::CYAN, color::RESET
    );
    println!();
    println!("In a hole in the ground there lived a hobbit. Not a nasty,");
    println!("dirty, wet hole — it was a hobbit-hole, and that means comfort.");
    println!();
    println!(
        "You are {}Dobo Daggins{}, a respectable hobbit of {}Bag End{}. One",
        color::BOLD, color::RESET, color::BOLD, color::RESET
    );
    println!("morning a wizard and thirteen dwarves arrive at your door and");
    println!("before you know it, you've been swept up in an adventure.");
    println!();
    println!(
        "Type {}HELP{} for a list of commands.",
        color::BOLD, color::RESET
    );
}
