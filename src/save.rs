//! Save and load game state to/from a file.

use std::fs;

use crate::game_state::{Direction, GameState};
use crate::loader;

const SAVE_FILE: &str = "hobitty.sav";
const SAVE_VERSION: &str = "HOBITTY_SAVE_V1";

pub fn save_game(state: &GameState) -> Result<String, String> {
    let mut lines = Vec::new();

    lines.push(SAVE_VERSION.to_string());
    lines.push(format!("current_room={}", state.current_room));
    lines.push(format!("turn={}", state.turn));
    lines.push(format!("inventory={}", state.inventory.join(",")));

    let mut visited: Vec<&str> = state.visited_rooms.iter().copied().collect();
    visited.sort();
    lines.push(format!("visited={}", visited.join(",")));

    let mut flags: Vec<&str> = state.flags.iter().copied().collect();
    flags.sort();
    lines.push(format!("flags={}", flags.join(",")));

    match state.pending_riddle {
        Some(idx) => lines.push(format!("pending_riddle={}", idx)),
        None => lines.push("pending_riddle=none".to_string()),
    }

    // Room items (sorted by room id for deterministic output)
    let mut room_ids: Vec<&str> = state.rooms.keys().copied().collect();
    room_ids.sort();
    for room_id in room_ids {
        if let Some(room) = state.rooms.get(room_id) {
            lines.push(format!("room_items:{}={}", room_id, room.items.join(",")));
        }
    }

    // NPC state (sorted by npc id)
    let mut npc_ids: Vec<&str> = state.npcs.keys().copied().collect();
    npc_ids.sort();
    for npc_id in npc_ids {
        if let Some(npc) = state.npcs.get(npc_id) {
            lines.push(format!(
                "npc:{}={},{}",
                npc_id, npc.current_room, npc.dialogue_index
            ));
        }
    }

    fs::write(SAVE_FILE, lines.join("\n"))
        .map_err(|e| format!("Failed to save: {}", e))?;

    Ok(format!("Game saved to '{}'.", SAVE_FILE))
}

pub fn load_game() -> Result<GameState, String> {
    let content = fs::read_to_string(SAVE_FILE)
        .map_err(|_| "No save file found. Use SAVE to save your game first.".to_string())?;

    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() || lines[0] != SAVE_VERSION {
        return Err("Save file is corrupted or from a different version.".to_string());
    }

    let mut state = loader::load_game_yaml("game.yaml")
        .map_err(|e| format!("Cannot load game definition: {}", e))?;

    for line in &lines[1..] {
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "current_room" => {
                    if let Some(&room_id) = state.rooms.keys().find(|&&k| k == value) {
                        state.current_room = room_id;
                    }
                }
                "turn" => {
                    state.turn = value.parse().unwrap_or(0);
                }
                "inventory" => {
                    state.inventory.clear();
                    if !value.is_empty() {
                        for item_str in value.split(',') {
                            if let Some(&item_id) =
                                state.items.keys().find(|&&k| k == item_str)
                            {
                                state.inventory.push(item_id);
                            }
                        }
                    }
                }
                "visited" => {
                    state.visited_rooms.clear();
                    if !value.is_empty() {
                        for room_str in value.split(',') {
                            if let Some(&room_id) =
                                state.rooms.keys().find(|&&k| k == room_str)
                            {
                                state.visited_rooms.insert(room_id);
                            }
                        }
                    }
                }
                "flags" => {
                    state.flags.clear();
                    if !value.is_empty() {
                        for flag in value.split(',') {
                            match flag {
                                "trolls_defeated" => {
                                    state.flags.insert("trolls_defeated");
                                }
                                "riddle_won" => {
                                    state.flags.insert("riddle_won");
                                }
                                _ => {}
                            }
                        }
                    }
                }
                "pending_riddle" => {
                    state.pending_riddle = if value == "none" {
                        None
                    } else {
                        value.parse().ok()
                    };
                }
                _ if key.starts_with("room_items:") => {
                    let room_id_str = &key["room_items:".len()..];
                    if let Some(room) =
                        state.rooms.values_mut().find(|r| r.id == room_id_str)
                    {
                        room.items.clear();
                        if !value.is_empty() {
                            for item_str in value.split(',') {
                                if let Some(&item_id) =
                                    state.items.keys().find(|&&k| k == item_str)
                                {
                                    room.items.push(item_id);
                                }
                            }
                        }
                    }
                }
                _ if key.starts_with("npc:") => {
                    let npc_id_str = &key["npc:".len()..];
                    let parts: Vec<&str> = value.split(',').collect();
                    if parts.len() == 2 {
                        if let Some(npc) =
                            state.npcs.values_mut().find(|n| n.id == npc_id_str)
                        {
                            let room_str = parts[0];
                            if room_str == "nowhere" {
                                npc.current_room = "nowhere";
                            } else if let Some(&room_id) =
                                state.rooms.keys().find(|&&k| k == room_str)
                            {
                                npc.current_room = room_id;
                            }
                            npc.dialogue_index = parts[1].parse().unwrap_or(0);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Reconstruct dynamic exits based on flags
    if state.flags.contains("riddle_won") {
        if let Some(room) = state.rooms.get_mut("goblin_cave") {
            room.exits.insert(Direction::East, "beorns_hall");
        }
    }

    Ok(state)
}
