use crate::game_state::Command;

/// Parse a line of player input into a Command.
pub fn parse(input: &str) -> Command {
    let input = input.trim().to_lowercase();
    let mut words = input.split_whitespace();

    let verb = match words.next() {
        Some(w) => w,
        None => return Command::Unknown(String::new()),
    };

    let rest: String = words.collect::<Vec<&str>>().join(" ");

    match verb {
        // Movement — "go north", "go n", or just "north" / "n"
        "go" | "walk" | "move" => {
            if rest.is_empty() {
                Command::Unknown("Go where?".to_string())
            } else {
                Command::Go(rest)
            }
        }
        "north" | "n" | "south" | "s" | "east" | "e" | "west" | "w" => {
            Command::Go(verb.to_string())
        }

        // Looking
        "look" | "l" => Command::Look,

        // Examine
        "examine" | "x" | "inspect" => {
            if rest.is_empty() {
                Command::Unknown("Examine what?".to_string())
            } else {
                Command::Examine(rest)
            }
        }

        // Take / pick up
        "take" | "get" | "grab" | "pick" => {
            // handle "pick up X"
            let target = rest.strip_prefix("up ").unwrap_or(&rest);
            if target.is_empty() {
                Command::Unknown("Take what?".to_string())
            } else {
                Command::Take(target.to_string())
            }
        }

        // Drop
        "drop" | "leave" => {
            if rest.is_empty() {
                Command::Unknown("Drop what?".to_string())
            } else {
                Command::Drop(rest)
            }
        }

        // Inventory
        "inventory" | "i" => Command::Inventory,

        // Help
        "help" | "?" => Command::Help,

        // Quit
        "quit" | "q" | "exit" => Command::Quit,

        _ => Command::Unknown(format!("I don't understand '{}'.", input)),
    }
}
