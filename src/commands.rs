use crate::game_state::{Command, Direction, GameState};

/// Execute a parsed command against the game state, returning the text to display.
pub fn execute(cmd: &Command, state: &mut GameState) -> String {
    match cmd {
        Command::Go(dir_str) => cmd_go(dir_str, state),
        Command::Look => cmd_look(state),
        Command::Take(target) => cmd_take(target, state),
        Command::Drop(target) => cmd_drop(target, state),
        Command::Examine(target) => cmd_examine(target, state),
        Command::Inventory => cmd_inventory(state),
        Command::Help => cmd_help(),
        Command::Quit => {
            state.running = false;
            "Farewell, adventurer. Until next time!".to_string()
        }
        Command::Unknown(msg) => {
            if msg.is_empty() {
                "I beg your pardon?".to_string()
            } else {
                msg.clone()
            }
        }
    }
}

fn cmd_go(dir_str: &str, state: &mut GameState) -> String {
    let direction = match Direction::from_str(dir_str) {
        Some(d) => d,
        None => return format!("I don't know the direction '{}'.", dir_str),
    };

    let next_room_id = {
        let room = state.current_room();
        match room.exits.get(&direction) {
            Some(&id) => id,
            None => {
                return format!("You can't go {} from here.", direction.name());
            }
        }
    };

    state.current_room = next_room_id;
    cmd_look(state)
}

fn cmd_look(state: &GameState) -> String {
    let room = state.current_room();
    let mut text = format!("\n--- {} ---\n{}", room.name, room.description);

    // List items on the ground
    if !room.items.is_empty() {
        text.push_str("\n\nYou can see:");
        for &item_id in &room.items {
            if let Some(item) = state.items.get(item_id) {
                text.push_str(&format!("\n  {}", item.name));
            }
        }
    }

    // List exits in canonical N, S, E, W order
    let mut exit_dirs: Vec<&Direction> = room.exits.keys().collect();
    exit_dirs.sort_by_key(|d| match d {
        Direction::North => 0,
        Direction::South => 1,
        Direction::East => 2,
        Direction::West => 3,
    });
    let exits: Vec<&str> = exit_dirs.iter().map(|d| d.name()).collect();
    if !exits.is_empty() {
        text.push_str(&format!("\n\nExits: {}", exits.join(", ")));
    }

    text
}

fn cmd_take(target: &str, state: &mut GameState) -> String {
    // Find an item in the current room whose id or name contains the target string
    let found_item_id = {
        let room = state.current_room();
        room.items.iter().find(|&&item_id| {
            let matches_id = item_id.contains(target);
            let matches_name = state
                .items
                .get(item_id)
                .map(|i| i.name.to_lowercase().contains(target))
                .unwrap_or(false);
            matches_id || matches_name
        }).copied()
    };

    match found_item_id {
        Some(item_id) => {
            // Remove from room
            state.current_room_mut().items.retain(|&id| id != item_id);
            // Add to inventory
            state.inventory.push(item_id);
            let name = state.items.get(item_id).map(|i| i.name).unwrap_or(item_id);
            format!("You pick up {}.", name)
        }
        None => format!("You don't see '{}' here.", target),
    }
}

fn cmd_drop(target: &str, state: &mut GameState) -> String {
    let found_idx = state.inventory.iter().position(|&item_id| {
        let matches_id = item_id.contains(target);
        let matches_name = state
            .items
            .get(item_id)
            .map(|i| i.name.to_lowercase().contains(target))
            .unwrap_or(false);
        matches_id || matches_name
    });

    match found_idx {
        Some(idx) => {
            let item_id = state.inventory.remove(idx);
            state.current_room_mut().items.push(item_id);
            let name = state.items.get(item_id).map(|i| i.name).unwrap_or(item_id);
            format!("You drop {}.", name)
        }
        None => format!("You don't have '{}'.", target),
    }
}

fn cmd_examine(target: &str, state: &GameState) -> String {
    // "examine room" / "examine surroundings" acts as LOOK
    match target {
        "room" | "around" | "surroundings" | "area" | "here" => return cmd_look(state),
        _ => {}
    }

    // Check inventory first, then current room
    let item_id = state
        .inventory
        .iter()
        .chain(state.current_room().items.iter())
        .find(|&&id| {
            let matches_id = id.contains(target);
            let matches_name = state
                .items
                .get(id)
                .map(|i| i.name.to_lowercase().contains(target))
                .unwrap_or(false);
            matches_id || matches_name
        })
        .copied();

    match item_id {
        Some(id) => {
            if let Some(item) = state.items.get(id) {
                format!("{}\n{}", item.name, item.description)
            } else {
                "You see nothing special.".to_string()
            }
        }
        None => format!("You don't see '{}' here.", target),
    }
}

fn cmd_inventory(state: &GameState) -> String {
    if state.inventory.is_empty() {
        "You are carrying nothing.".to_string()
    } else {
        let mut text = "You are carrying:".to_string();
        for &item_id in &state.inventory {
            if let Some(item) = state.items.get(item_id) {
                text.push_str(&format!("\n  {}", item.name));
            }
        }
        text
    }
}

fn cmd_help() -> String {
    "\
Available commands:
  LOOK (L)              — Look around the current room
  GO <direction>        — Move in a direction (NORTH/N, SOUTH/S, EAST/E, WEST/W)
  NORTH/SOUTH/EAST/WEST — Shortcut for GO <direction>
  TAKE <item>           — Pick up an item
  DROP <item>           — Drop an item from your inventory
  EXAMINE <item> (X)    — Examine an item closely
  INVENTORY (I)         — List what you are carrying
  HELP (?)              — Show this help
  QUIT (Q)              — Leave the game"
        .to_string()
}
