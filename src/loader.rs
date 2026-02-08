//! Load a game definition from a YAML file and produce a GameState.
//!
//! YAML strings are leaked (`Box::leak`) to produce `&'static str` values
//! that the rest of the engine expects.  This is fine because game data is
//! loaded once and lives for the entire process.

use std::collections::{HashMap, HashSet};
use std::fs;

use serde::Deserialize;

use crate::game_state::{
    Condition, ConditionalDesc, Direction, Effect, GameState, Item, Meta,
    Npc, RiddleDef, RiddleQuestion, Room, Trigger,
};
// Effect is used in both build_state and validate

// ── YAML schema types ──────────────────────────────────────────────

#[derive(Deserialize)]
pub struct GameDef {
    pub game: GameMeta,
    pub rooms: HashMap<String, RoomDef>,
    pub items: HashMap<String, ItemDef>,
    #[serde(default)]
    pub npcs: HashMap<String, NpcDef>,
    #[serde(default)]
    pub triggers: Vec<TriggerDef>,
    #[serde(default)]
    pub riddles: HashMap<String, RiddleGameDef>,
}

#[derive(Deserialize)]
pub struct GameMeta {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub tagline: String,
    pub start_room: String,
    #[serde(default)]
    pub banner: String,
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
    #[serde(default)]
    pub conditional_descriptions: Vec<ConditionalDescDef>,
    /// Path to an external .ans art file.
    #[serde(default)]
    pub art: Option<String>,
}

#[derive(Deserialize)]
pub struct ConditionalDescDef {
    #[serde(default)]
    pub conditions: Vec<ConditionDef>,
    pub description: String,
    #[serde(default)]
    pub short_description: Option<String>,
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

// ── Trigger / Riddle YAML types ────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionDef {
    InRoom(String),
    HasItem(String),
    FlagSet(String),
    FlagUnset(String),
}

#[derive(Deserialize)]
pub struct AddItemDef {
    pub room: String,
    pub item: String,
}

#[derive(Deserialize)]
pub struct OpenExitDef {
    pub room: String,
    pub direction: String,
    pub destination: String,
}

#[derive(Deserialize)]
pub struct GameOverDef {
    pub message: String,
    pub hint: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectDef {
    Print(String),
    SetFlag(String),
    Bell(()),
    AddItem(AddItemDef),
    OpenExit(OpenExitDef),
    RemoveNpc(String),
    GameOver(GameOverDef),
    Win(String),
    StartRiddle(String),
}

#[derive(Deserialize)]
pub struct TriggerDef {
    pub id: String,
    pub event: String,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub conditions: Vec<ConditionDef>,
    pub effects: Vec<EffectDef>,
}

#[derive(Deserialize)]
pub struct RiddleQuestionDef {
    pub question: String,
    pub answer: String,
}

#[derive(Deserialize)]
pub struct RiddleGameDef {
    pub npc: String,
    pub intro: String,
    pub questions: Vec<RiddleQuestionDef>,
    pub correct_response: String,
    pub win_text: String,
    pub win_effects: Vec<EffectDef>,
    pub lose_text: String,
    pub lose_hint: String,
}

// ── Helpers ────────────────────────────────────────────────────────

/// Leak a String to get a &'static str.  The memory is never freed,
/// which is intentional — game data lives for the whole process.
fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn convert_condition(c: ConditionDef) -> Condition {
    match c {
        ConditionDef::InRoom(s) => Condition::InRoom(leak(s)),
        ConditionDef::HasItem(s) => Condition::HasItem(leak(s)),
        ConditionDef::FlagSet(s) => Condition::FlagSet(leak(s)),
        ConditionDef::FlagUnset(s) => Condition::FlagUnset(leak(s)),
    }
}

fn convert_effect(e: EffectDef) -> Effect {
    match e {
        EffectDef::Print(s) => Effect::Print(leak(s)),
        EffectDef::SetFlag(s) => Effect::SetFlag(leak(s)),
        EffectDef::Bell(_) => Effect::Bell,
        EffectDef::AddItem(a) => Effect::AddItem { room: leak(a.room), item: leak(a.item) },
        EffectDef::OpenExit(o) => Effect::OpenExit {
            room: leak(o.room), direction: leak(o.direction), destination: leak(o.destination),
        },
        EffectDef::RemoveNpc(s) => Effect::RemoveNpc(leak(s)),
        EffectDef::GameOver(g) => Effect::GameOver { message: leak(g.message), hint: leak(g.hint) },
        EffectDef::Win(s) => Effect::Win(leak(s)),
        EffectDef::StartRiddle(s) => Effect::StartRiddle(leak(s)),
    }
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
/// `game_dir` is the directory containing the game definition (for resolving
/// relative paths like art files and save files).
/// Runs schema validation after building; errors are fatal, warnings print to stderr.
pub fn load_game_yaml(path: &str, game_dir: &str) -> Result<GameState, String> {
    let yaml = fs::read_to_string(path)
        .map_err(|e| format!("Cannot read '{}': {}", path, e))?;
    let def = parse_yaml(&yaml)?;
    let mut state = build_state(def)?;
    state.game_dir = game_dir.to_string();
    validate(&state)?;
    Ok(state)
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

        let conditional_descriptions: Vec<ConditionalDesc> = room_def.conditional_descriptions
            .into_iter()
            .map(|cd| ConditionalDesc {
                conditions: cd.conditions.into_iter().map(convert_condition).collect(),
                description: leak(cd.description),
                short_description: cd.short_description.map(|s| leak(s)),
            })
            .collect();

        rooms.insert(id, Room {
            id,
            name: leak(room_def.name),
            description: leak(room_def.description),
            short_description: leak(room_def.short_description),
            description_when_empty: room_def.description_when_empty.map(|s| leak(s)),
            exits,
            items: room_items,
            conditional_descriptions,
            art: room_def.art.map(|s| leak(s)),
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

    // -- Triggers --
    let triggers: Vec<Trigger> = def.triggers
        .into_iter()
        .map(|t| Trigger {
            id: leak(t.id),
            event: leak(t.event),
            target: t.target.map(|s| leak(s)),
            direction: t.direction.map(|s| leak(s)),
            conditions: t.conditions.into_iter().map(convert_condition).collect(),
            effects: t.effects.into_iter().map(convert_effect).collect(),
        })
        .collect();

    // -- Riddles --
    let mut riddles: HashMap<&'static str, RiddleDef> = HashMap::new();
    for (id, rdef) in def.riddles {
        let id = leak(id);
        riddles.insert(id, RiddleDef {
            npc: leak(rdef.npc),
            intro: leak(rdef.intro),
            questions: rdef.questions.into_iter().map(|q| RiddleQuestion {
                question: leak(q.question),
                answer: leak(q.answer),
            }).collect(),
            correct_response: leak(rdef.correct_response),
            win_text: leak(rdef.win_text),
            win_effects: rdef.win_effects.into_iter().map(convert_effect).collect(),
            lose_text: leak(rdef.lose_text),
            lose_hint: leak(rdef.lose_hint),
        });
    }

    // -- Meta --
    let meta = Meta {
        title: leak(def.game.title),
        subtitle: leak(def.game.subtitle),
        tagline: leak(def.game.tagline),
        banner: leak(def.game.banner),
        intro: leak(def.game.intro),
    };

    // -- Assemble GameState --
    let start_room = leak(def.game.start_room);

    if !rooms.contains_key(start_room) {
        return Err(format!("Start room '{}' not found in rooms", start_room));
    }

    let mut visited_rooms = HashSet::new();
    visited_rooms.insert(start_room);

    Ok(GameState {
        game_dir: ".".to_string(),
        meta,
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
        triggers,
        riddles,
    })
}

// ── Validation ─────────────────────────────────────────────────────

/// Validate a built GameState for broken references and missing data.
/// Returns Err with all errors joined if any are found.
/// Warnings are printed to stderr but don't prevent loading.
fn validate(state: &GameState) -> Result<(), String> {
    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // -- Room exit destinations must exist --
    for (room_id, room) in &state.rooms {
        for (dir, &dest) in &room.exits {
            if !state.rooms.contains_key(dest) {
                errors.push(format!(
                    "Room '{}': exit {} points to non-existent room '{}'",
                    room_id, dir.name(), dest
                ));
            }
        }

        // -- Room items must be defined --
        for &item_id in &room.items {
            if !state.items.contains_key(item_id) {
                errors.push(format!(
                    "Room '{}': references non-existent item '{}'",
                    room_id, item_id
                ));
            }
        }

        // -- Art file should exist on disk --
        if let Some(art_path) = room.art {
            let full_art = format!("{}/{}", state.game_dir, art_path);
            if !std::path::Path::new(&full_art).exists() {
                warnings.push(format!(
                    "Room '{}': art file '{}' not found",
                    room_id, art_path
                ));
            }
        }
    }

    // -- NPC start rooms must exist --
    for (npc_id, npc) in &state.npcs {
        if npc.current_room != "nowhere" && !state.rooms.contains_key(npc.current_room) {
            errors.push(format!(
                "NPC '{}': start_room '{}' does not exist",
                npc_id, npc.current_room
            ));
        }

        for &room_id in &npc.allowed_rooms {
            if !state.rooms.contains_key(room_id) {
                warnings.push(format!(
                    "NPC '{}': allowed_room '{}' does not exist",
                    npc_id, room_id
                ));
            }
        }

        if npc.dialogue.is_empty() {
            warnings.push(format!(
                "NPC '{}': has no dialogue lines",
                npc_id
            ));
        }
    }

    // -- Trigger validation --
    let valid_events = ["go", "wait", "use", "attack", "talk_to", "pickup"];
    for trigger in &state.triggers {
        if !valid_events.contains(&trigger.event) {
            warnings.push(format!(
                "Trigger '{}': unknown event type '{}'",
                trigger.id, trigger.event
            ));
        }

        // Validate effects that reference rooms/items/NPCs
        for effect in &trigger.effects {
            validate_effect(effect, trigger.id, state, &mut errors);
        }
    }

    // -- Riddle validation --
    for (riddle_id, riddle) in &state.riddles {
        if riddle.questions.is_empty() {
            errors.push(format!(
                "Riddle '{}': has no questions",
                riddle_id
            ));
        }

        // NPC must exist
        if !state.npcs.contains_key(riddle.npc) {
            warnings.push(format!(
                "Riddle '{}': references NPC '{}' which does not exist",
                riddle_id, riddle.npc
            ));
        }

        // Validate win effects
        for effect in &riddle.win_effects {
            validate_effect(effect, &format!("riddle '{}'", riddle_id), state, &mut errors);
        }
    }

    // -- Check that StartRiddle effects reference existing riddles --
    for trigger in &state.triggers {
        for effect in &trigger.effects {
            if let Effect::StartRiddle(riddle_id) = effect {
                if !state.riddles.contains_key(riddle_id) {
                    errors.push(format!(
                        "Trigger '{}': start_riddle references non-existent riddle '{}'",
                        trigger.id, riddle_id
                    ));
                }
            }
        }
    }

    // -- Items defined but never placed --
    for &item_id in state.items.keys() {
        let placed = state.rooms.values().any(|r| r.items.contains(&item_id));
        let in_trigger = state.triggers.iter().any(|t| {
            t.effects.iter().any(|e| matches!(e, Effect::AddItem { item, .. } if *item == item_id))
        });
        let in_riddle = state.riddles.values().any(|r| {
            r.win_effects.iter().any(|e| matches!(e, Effect::AddItem { item, .. } if *item == item_id))
        });
        if !placed && !in_trigger && !in_riddle {
            warnings.push(format!(
                "Item '{}': defined but never placed in any room, trigger, or riddle",
                item_id
            ));
        }
    }

    // -- Print warnings --
    for w in &warnings {
        eprintln!("Warning: {}", w);
    }

    // -- Return errors --
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Validation failed with {} error(s):\n  - {}",
            errors.len(),
            errors.join("\n  - ")
        ))
    }
}

/// Validate a single effect for broken references.
fn validate_effect(
    effect: &Effect,
    context: &str,
    state: &GameState,
    errors: &mut Vec<String>,
) {
    match effect {
        Effect::AddItem { room, item } => {
            if !state.rooms.contains_key(room) {
                errors.push(format!(
                    "{}: add_item references non-existent room '{}'",
                    context, room
                ));
            }
            if !state.items.contains_key(item) {
                errors.push(format!(
                    "{}: add_item references non-existent item '{}'",
                    context, item
                ));
            }
        }
        Effect::OpenExit { room, destination, .. } => {
            if !state.rooms.contains_key(room) {
                errors.push(format!(
                    "{}: open_exit references non-existent room '{}'",
                    context, room
                ));
            }
            if !state.rooms.contains_key(destination) {
                errors.push(format!(
                    "{}: open_exit destination '{}' does not exist",
                    context, destination
                ));
            }
        }
        Effect::RemoveNpc(npc_id) => {
            if !state.npcs.contains_key(npc_id) {
                errors.push(format!(
                    "{}: remove_npc references non-existent NPC '{}'",
                    context, npc_id
                ));
            }
        }
        _ => {}
    }
}
