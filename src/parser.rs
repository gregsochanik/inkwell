use crate::game_state::Command;

/// Strip leading articles ("a", "an", "the") from a noun phrase.
fn strip_articles(s: &str) -> &str {
    let s = s.trim();
    for article in &["a ", "an ", "the "] {
        if let Some(rest) = s.strip_prefix(article) {
            return rest.trim();
        }
    }
    s
}

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

        // Looking — "look" alone shows the room, "look at X" examines an item
        "look" | "l" => {
            if let Some(target) = rest.strip_prefix("at ") {
                let target = strip_articles(target);
                if target.is_empty() {
                    Command::Look
                } else {
                    Command::Examine(target.to_string())
                }
            } else {
                Command::Look
            }
        }

        // Examine
        "examine" | "x" | "inspect" => {
            let target = strip_articles(&rest);
            if target.is_empty() {
                Command::Unknown("Examine what?".to_string())
            } else {
                Command::Examine(target.to_string())
            }
        }

        // Take / pick up
        "take" | "get" | "grab" | "pick" => {
            // handle "pick up X"
            let without_up = rest.strip_prefix("up ").unwrap_or(&rest);
            let target = strip_articles(without_up);
            if target.is_empty() {
                Command::Unknown("Take what?".to_string())
            } else {
                Command::Take(target.to_string())
            }
        }

        // Drop
        "drop" | "leave" => {
            let target = strip_articles(&rest);
            if target.is_empty() {
                Command::Unknown("Drop what?".to_string())
            } else {
                Command::Drop(target.to_string())
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
