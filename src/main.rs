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
    let mut state = loader::load_game_yaml("game.yaml")
        .unwrap_or_else(|e| {
            eprintln!("Failed to load game: {}", e);
            std::process::exit(1);
        });

    print_banner(&state);

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

fn print_banner(state: &game_state::GameState) {
    let m = &state.meta;
    let sep = "================================================================";

    // Title banner
    println!(
        "\n{}{}{}{}",
        color::BOLD, color::CYAN, sep, color::RESET
    );
    if !m.banner.is_empty() {
        print!(
            "{}{}{}{}",
            color::BOLD,
            color::CYAN,
            m.banner.trim_end(),
            color::RESET
        );
        println!();
    }
    // Subtitle / tagline
    if !m.subtitle.is_empty() {
        println!(
            "{}        {}{}",
            color::YELLOW, m.subtitle, color::RESET
        );
    }
    if !m.tagline.is_empty() {
        println!(
            "{}     {}{}",
            color::DIM, m.tagline, color::RESET
        );
    }
    println!(
        "{}{}{}{}",
        color::BOLD, color::CYAN, sep, color::RESET
    );

    // Intro text (supports {bold:text} templates)
    if !m.intro.is_empty() {
        println!();
        println!("{}", triggers::process_template(m.intro));
    }
}
