use crate::color;
use crate::game_state::{Command, Direction, GameState, ItemId, RoomId};
use crate::triggers;

/// Maximum number of items the player can carry.
const MAX_INVENTORY: usize = 4;

/// Check if any item in a list matches the target string by id or name.
fn item_matches_any(items: &[ItemId], target: &str, state: &GameState) -> bool {
    items.iter().any(|&item_id| {
        let matches_id = item_id.contains(target);
        let matches_name = state
            .items
            .get(item_id)
            .map(|i| i.name.to_lowercase().contains(target))
            .unwrap_or(false);
        matches_id || matches_name
    })
}

/// Check if a target string matches any known item in the game.
fn item_exists_in_world(target: &str, state: &GameState) -> bool {
    state.items.values().any(|item| {
        item.id.contains(target) || item.name.to_lowercase().contains(target)
    })
}

// Room descriptions and puzzle logic are now handled by the trigger system.
// See triggers.rs for get_room_desc, get_room_short_desc, etc.

/// Execute a parsed command against the game state, returning the text to display.
pub fn execute(cmd: &Command, state: &mut GameState) -> String {
    match cmd {
        Command::Go(dir_str) => cmd_go(dir_str, state),
        Command::Look => cmd_look(state),
        Command::Take(target) => cmd_take(target, state),
        Command::Drop(target) => cmd_drop(target, state),
        Command::Examine(target) => cmd_examine(target, state),
        Command::TalkTo(target) => cmd_talk_to(target, state),
        Command::Wait => cmd_wait(state),
        Command::Use(target) => cmd_use(target, state),
        Command::Attack(target) => cmd_attack(target, state),
        Command::Riddle => cmd_riddle(state),
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

    // Check for trigger-based movement blocks
    if let Some(msg) = triggers::check_go(direction.name(), state) {
        return msg;
    }

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

    // First visit: show full description and mark visited.
    // Revisit: show short description.
    let first_visit = !state.visited_rooms.contains(next_room_id);
    state.visited_rooms.insert(next_room_id);

    if first_visit {
        cmd_look(state)
    } else {
        cmd_glance(state)
    }
}

/// Append NPC, item, and exit information to a room description.
fn append_room_details(text: &mut String, room_id: RoomId, state: &GameState) {
    let room = state.rooms.get(room_id).expect("room must exist");

    // List NPCs present
    let npc_ids = state.npcs_in_room(room_id);
    if !npc_ids.is_empty() {
        for npc_id in &npc_ids {
            if let Some(npc) = state.npcs.get(npc_id) {
                text.push_str(&format!("\n{} is here.", color::npc(npc.name)));
            }
        }
    }

    // List items on the ground
    if !room.items.is_empty() {
        text.push_str("\n\nYou can see:");
        for &item_id in &room.items {
            if let Some(item) = state.items.get(item_id) {
                text.push_str(&format!("\n  {}", color::item(item.name)));
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
        text.push_str(&format!(
            "\n\n{}",
            color::exits(&format!("Exits: {}", exits.join(", ")))
        ));
    }
}

/// Full room description (used by LOOK command and first visit).
fn cmd_look(state: &GameState) -> String {
    let room = state.current_room();
    let desc = triggers::get_room_desc(state);
    let room_id = state.current_room;
    let mut text = String::from("\n");

    // Show room art from YAML-referenced .ans file
    if let Some(art_path) = state.current_room().art {
        let full = format!("{}/{}", state.game_dir, art_path);
        if let Ok(room_art) = std::fs::read_to_string(full) {
            text.push_str(&room_art);
            text.push('\n');
        }
    }

    text.push_str(&color::room_title(&format!("--- {} ---", room.name)));
    text.push('\n');
    text.push_str(desc);
    append_room_details(&mut text, room_id, state);
    text
}

/// Short room description (used on revisit when navigating).
fn cmd_glance(state: &GameState) -> String {
    let room = state.current_room();
    let short = triggers::get_room_short_desc(state);
    let room_id = state.current_room;
    let mut text = format!(
        "\n{}\n{}",
        color::room_title(&format!("--- {} ---", room.name)),
        short
    );
    append_room_details(&mut text, room_id, state);
    text
}

fn cmd_take(target: &str, state: &mut GameState) -> String {
    // Check inventory capacity
    if state.inventory.len() >= MAX_INVENTORY {
        return "Your pockets are full! You'll need to drop something first.".to_string();
    }

    // Find an item in the current room whose id or name contains the target string
    let found_item_id = {
        let room = state.current_room();
        room.items
            .iter()
            .find(|&&item_id| {
                let matches_id = item_id.contains(target);
                let matches_name = state
                    .items
                    .get(item_id)
                    .map(|i| i.name.to_lowercase().contains(target))
                    .unwrap_or(false);
                matches_id || matches_name
            })
            .copied()
    };

    match found_item_id {
        Some(item_id) => {
            // Remove from room
            state.current_room_mut().items.retain(|&id| id != item_id);
            // Add to inventory
            state.inventory.push(item_id);
            let name = state.items.get(item_id).map(|i| i.name).unwrap_or(item_id);
            // Check for pickup triggers (e.g. bell on ring)
            let extra = triggers::check_pickup(item_id, state).unwrap_or_default();
            format!("{}You pick up {}.", extra, color::item(name))
        }
        None => {
            // Check if already carrying it
            let in_inventory = item_matches_any(&state.inventory, target, state);
            if in_inventory {
                "You're already carrying that!".to_string()
            } else if item_exists_in_world(target, state) {
                format!("You don't see '{}' here.", target)
            } else {
                format!("I don't know what '{}' is.", target)
            }
        }
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
            format!("You drop {}.", color::item(name))
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

    // Check NPCs in the current room
    let npc_ids = state.npcs_in_room(state.current_room);
    for npc_id in &npc_ids {
        if let Some(npc) = state.npcs.get(npc_id) {
            if npc.id.contains(target) || npc.name.to_lowercase().contains(target) {
                return format!("{}\n{}", color::npc(npc.name), npc.description);
            }
        }
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
                format!("{}\n{}", color::item(item.name), item.description)
            } else {
                "You see nothing special.".to_string()
            }
        }
        None => format!("You don't see '{}' here.", target),
    }
}

fn cmd_talk_to(target: &str, state: &mut GameState) -> String {
    // Find an NPC in the current room matching the target
    let npc_ids = state.npcs_in_room(state.current_room);
    let found_id = npc_ids
        .iter()
        .find(|&&npc_id| {
            if let Some(npc) = state.npcs.get(npc_id) {
                npc.id.contains(target) || npc.name.to_lowercase().contains(target)
            } else {
                false
            }
        })
        .copied();

    match found_id {
        Some(npc_id) => {
            // Check for talk triggers (e.g. Gollum → riddle game)
            if let Some(msg) = triggers::check_talk(target, state) {
                return msg;
            }

            let npc = state.npcs.get_mut(npc_id).expect("npc must exist");
            let line = npc.dialogue[npc.dialogue_index];
            npc.dialogue_index = (npc.dialogue_index + 1) % npc.dialogue.len();
            format!(
                "{} says: {}",
                color::npc(npc.name),
                color::dialogue(line)
            )
        }
        None => {
            // Check if the NPC exists but isn't here
            let exists = state.npcs.values().any(|npc| {
                npc.id.contains(target) || npc.name.to_lowercase().contains(target)
            });
            if exists {
                format!("You don't see '{}' here right now.", target)
            } else {
                format!("There is nobody called '{}' here.", target)
            }
        }
    }
}

// ============================================================
// Commands that delegate to the trigger system
// ============================================================

fn cmd_wait(state: &mut GameState) -> String {
    if let Some(msg) = triggers::check_wait(state) {
        return msg;
    }
    "Time passes...".to_string()
}

fn cmd_use(target: &str, state: &mut GameState) -> String {
    // Check triggers first — allows both inventory items and room scenery
    // interactions (triggers should use !has_item conditions when needed)
    if let Some(msg) = triggers::check_use(target, state) {
        return msg;
    }

    // No trigger matched — check if the player has something by that name
    let has_item = state.inventory.iter().any(|&id| {
        id.contains(target)
            || state.items.get(id)
                .map(|i| i.name.to_lowercase().contains(target))
                .unwrap_or(false)
    });

    if has_item {
        format!("You're not sure how to use the {}.", target)
    } else if item_exists_in_world(target, state) {
        "You don't have that.".to_string()
    } else {
        format!("You can't use '{}' here.", target)
    }
}

fn cmd_attack(target: &str, state: &mut GameState) -> String {
    // Check triggers first
    if let Some(msg) = triggers::check_attack(target, state) {
        return msg;
    }

    // Generic responses
    if state.inventory.iter().any(|&id| id == "sword") {
        format!("You wave your sword at {}. Nothing much happens.", target)
    } else {
        "You have nothing to fight with!".to_string()
    }
}

fn cmd_riddle(state: &mut GameState) -> String {
    // The RIDDLE command also triggers via talk_to logic
    if let Some(msg) = triggers::check_talk("gollum", state) {
        return msg;
    }

    // Check if any NPC that could play riddles is here
    let any_npc_here = !state.npcs_in_room(state.current_room).is_empty();
    if any_npc_here {
        "Nobody here seems interested in riddles.".to_string()
    } else {
        "There is nobody here to play riddles with.".to_string()
    }
}

fn cmd_inventory(state: &GameState) -> String {
    if state.inventory.is_empty() {
        "You are carrying nothing.".to_string()
    } else {
        let mut text = format!(
            "You are carrying ({}/{}):",
            state.inventory.len(),
            MAX_INVENTORY
        );
        for &item_id in &state.inventory {
            if let Some(item) = state.items.get(item_id) {
                text.push_str(&format!("\n  {}", color::item(item.name)));
            }
        }
        text
    }
}

fn cmd_help() -> String {
    format!(
        "\
Available commands:
  {look:<22} — Look around the current room
  {go:<22} — Move in a direction (NORTH/N, SOUTH/S, EAST/E, WEST/W)
  {dirs:<22} — Shortcut for GO <direction>
  {take:<22} — Pick up an item
  {drop:<22} — Drop an item from your inventory
  {examine:<22} — Examine an item or person closely
  {talk:<22} — Talk to someone nearby
  {use_cmd:<22} — Use an item you are carrying
  {wait:<22} — Wait and let time pass
  {riddle:<22} — Challenge someone to a game of riddles
  {attack:<22} — Attack something
  {save:<22} — Save your game to a file
  {load:<22} — Load a previously saved game
  {inv:<22} — List what you are carrying
  {help:<22} — Show this help
  {quit:<22} — Leave the game",
        look = "LOOK (L)",
        go = "GO <direction>",
        dirs = "NORTH/SOUTH/EAST/WEST",
        take = "TAKE <item>",
        drop = "DROP <item>",
        examine = "EXAMINE <item> (X)",
        talk = "TALK TO <name>",
        use_cmd = "USE <item>",
        wait = "WAIT (Z)",
        riddle = "RIDDLE",
        attack = "ATTACK <target>",
        save = "SAVE",
        load = "LOAD",
        inv = "INVENTORY (I)",
        help = "HELP (?)",
        quit = "QUIT (Q)",
    )
}
