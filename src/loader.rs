//! Load a game definition from a YAML file and produce a GameState.
//!
//! YAML strings are leaked (`Box::leak`) to produce `&'static str` values
//! that the rest of the engine expects.  This is fine because game data is
//! loaded once and lives for the entire process.

use std::collections::{HashMap, HashSet};
use std::fs;

use serde::Deserialize;

use crate::game_state::{Direction, GameState, Item, Npc, Room};

// ── YAML schema types ──────────────────────────────────────────────

#[derive(Deserialize)]
pub struct GameDef {
    pub game: GameMeta,
    pub rooms: HashMap<String, RoomDef>,
    pub items: HashMap<String, ItemDef>,
    #[serde(default)]
    pub npcs: HashMap<String, NpcDef>,
}

#[derive(Deserialize)]
pub struct GameMeta {
    pub title: String,
    pub subtitle: String,
    #[serde(default)]
    pub tagline: String,
    pub start_room: String,
    #[serde(default)]
    pub intro: String,
}

#[derive(Deserialize)]
pub struct RoomDef {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub short_description: String,
    #[serde(default)]
    pub description_when_empty: Option<String>,
    #[serde(default)]
    pub exits: HashMap<String, String>,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Deserialize)]
pub struct ItemDef {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct NpcDef {
    pub name: String,
    pub description: String,
    pub start_room: String,
    #[serde(default)]
    pub allowed_rooms: Vec<String>,
    #[serde(default)]
    pub dialogue: Vec<String>,
    #[serde(default)]
    pub idle_texts: Vec<String>,
}

// ── Helpers ────────────────────────────────────────────────────────

/// Leak a String to get a &'static str.  The memory is never freed,
/// which is intentional — game data lives for the whole process.
fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn parse_direction(s: &str) -> Option<Direction> {
    match s {
        "north" | "n" => Some(Direction::North),
        "south" | "s" => Some(Direction::South),
        "east" | "e" => Some(Direction::East),
        "west" | "w" => Some(Direction::West),
        _ => None,
    }
}

// ── Public API ─────────────────────────────────────────────────────

/// Parse a YAML string into a GameDef.
pub fn parse_yaml(yaml: &str) -> Result<GameDef, String> {
    serde_yaml::from_str(yaml).map_err(|e| format!("YAML parse error: {}", e))
}

/// Load a YAML file and return a fully initialised GameState.
pub fn load_game_yaml(path: &str) -> Result<GameState, String> {
    let yaml = fs::read_to_string(path)
        .map_err(|e| format!("Cannot read '{}': {}", path, e))?;
    let def = parse_yaml(&yaml)?;
    build_state(def)
}

/// Convert a parsed GameDef into a playable GameState.
pub fn build_state(def: GameDef) -> Result<GameState, String> {
    // -- Items --
    let mut items: HashMap<&'static str, Item> = HashMap::new();
    for (id, item_def) in def.items {
        let id = leak(id);
        items.insert(id, Item {
            id,
            name: leak(item_def.name),
            description: leak(item_def.description),
        });
    }

    // -- Rooms --
    let mut rooms: HashMap<&'static str, Room> = HashMap::new();
    for (id, room_def) in def.rooms {
        let id = leak(id);

        let mut exits = HashMap::new();
        for (dir_str, dest) in room_def.exits {
            let dir = parse_direction(&dir_str)
                .ok_or_else(|| format!("Room '{}': unknown direction '{}'", id, dir_str))?;
            exits.insert(dir, leak(dest));
        }

        let room_items: Vec<&'static str> = room_def.items
            .into_iter()
            .map(|s| leak(s))
            .collect();

        rooms.insert(id, Room {
            id,
            name: leak(room_def.name),
            description: leak(room_def.description),
            short_description: leak(room_def.short_description),
            description_when_empty: room_def.description_when_empty.map(|s| leak(s)),
            exits,
            items: room_items,
        });
    }

    // -- NPCs --
    let mut npcs: HashMap<&'static str, Npc> = HashMap::new();
    for (id, npc_def) in def.npcs {
        let id = leak(id);
        npcs.insert(id, Npc {
            id,
            name: leak(npc_def.name),
            description: leak(npc_def.description),
            current_room: leak(npc_def.start_room),
            allowed_rooms: npc_def.allowed_rooms.into_iter().map(|s| leak(s)).collect(),
            dialogue: npc_def.dialogue.into_iter().map(|s| leak(s)).collect(),
            dialogue_index: 0,
            idle_texts: npc_def.idle_texts.into_iter().map(|s| leak(s)).collect(),
        });
    }

    // -- Assemble GameState --
    let start_room = leak(def.game.start_room);

    if !rooms.contains_key(start_room) {
        return Err(format!("Start room '{}' not found in rooms", start_room));
    }

    let mut visited_rooms = HashSet::new();
    visited_rooms.insert(start_room);

    Ok(GameState {
        rooms,
        items,
        npcs,
        current_room: start_room,
        inventory: Vec::new(),
        visited_rooms,
        flags: HashSet::new(),
        pending_riddle: None,
        turn: 0,
        running: true,
    })
}
