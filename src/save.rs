//! Save and load game state to/from a file.

use std::fs;

use crate::game_state::GameState;
use crate::loader;
use crate::triggers;

const SAVE_FILENAME: &str = "save.dat";
const SAVE_VERSION: &str = "ADVENTURE_SAVE_V1";

/// Build the full save file path inside the game directory.
fn save_path(game_dir: &str) -> String {
    format!("{}/{}", game_dir, SAVE_FILENAME)
}

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

    match &state.pending_riddle {
        Some(pr) => lines.push(format!("pending_riddle={},{}", pr.riddle_id, pr.question_index)),
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

    let path = save_path(&state.game_dir);
    fs::write(&path, lines.join("\n"))
        .map_err(|e| format!("Failed to save: {}", e))?;

    Ok(format!("Game saved to '{}'.", path))
}

pub fn load_game(game_dir: &str) -> Result<GameState, String> {
    let path = save_path(game_dir);
    let content = fs::read_to_string(&path)
        .map_err(|_| "No save file found. Use SAVE to save your game first.".to_string())?;

    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() || lines[0] != SAVE_VERSION {
        return Err("Save file is corrupted or from a different version.".to_string());
    }

    let game_file = format!("{}/game.yaml", game_dir);
    let mut state = loader::load_game_yaml(&game_file, game_dir)
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
                            if !flag.is_empty() {
                                // Leak the flag string so it lives as &'static str
                                let leaked: &'static str = Box::leak(flag.to_string().into_boxed_str());
                                state.flags.insert(leaked);
                            }
                        }
                    }
                }
                "pending_riddle" => {
                    if value == "none" {
                        state.pending_riddle = None;
                    } else if let Some((rid, idx_str)) = value.split_once(',') {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            let riddle_id: &'static str = Box::leak(rid.to_string().into_boxed_str());
                            state.pending_riddle = Some(crate::game_state::PendingRiddle {
                                riddle_id,
                                question_index: idx,
                            });
                        }
                    }
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

    // Replay structural trigger effects (e.g. open exits) based on current flags
    triggers::replay_structural_effects(&mut state);

    Ok(state)
}
