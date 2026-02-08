use std::collections::{HashMap, HashSet};

/// Cardinal directions for navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Direction> {
        match s {
            "north" | "n" => Some(Direction::North),
            "south" | "s" => Some(Direction::South),
            "east" | "e" => Some(Direction::East),
            "west" | "w" => Some(Direction::West),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
        }
    }
}

pub type RoomId = &'static str;
pub type ItemId = &'static str;
pub type NpcId = &'static str;

// ── Trigger system types ───────────────────────────────────────────

/// A condition that must be true for a trigger to fire.
#[derive(Debug, Clone)]
pub enum Condition {
    InRoom(&'static str),
    HasItem(&'static str),
    FlagSet(&'static str),
    FlagUnset(&'static str),
}

/// An effect produced when a trigger fires.
#[derive(Debug, Clone)]
pub enum Effect {
    Print(&'static str),
    SetFlag(&'static str),
    Bell,
    AddItem { room: &'static str, item: &'static str },
    OpenExit { room: &'static str, direction: &'static str, destination: &'static str },
    RemoveNpc(&'static str),
    GameOver { message: &'static str, hint: &'static str },
    Win(&'static str),
    StartRiddle(&'static str),
}

/// A trigger: an event + conditions → effects.
#[derive(Debug, Clone)]
pub struct Trigger {
    #[allow(dead_code)] // useful for debugging
    pub id: &'static str,
    pub event: &'static str,               // "go", "wait", "use", "attack", "talk_to", "pickup"
    pub target: Option<&'static str>,       // item/npc target
    pub direction: Option<&'static str>,    // for "go" events
    pub conditions: Vec<Condition>,
    pub effects: Vec<Effect>,
}

/// A conditional room description override.
#[derive(Debug, Clone)]
pub struct ConditionalDesc {
    pub conditions: Vec<Condition>,
    pub description: &'static str,
    pub short_description: Option<&'static str>,
}

/// A single riddle question/answer pair.
#[derive(Debug, Clone)]
pub struct RiddleQuestion {
    pub question: &'static str,
    pub answer: &'static str,
}

/// A riddle game definition (e.g. Gollum's riddles).
#[derive(Debug, Clone)]
pub struct RiddleDef {
    pub npc: &'static str,
    pub intro: &'static str,
    pub questions: Vec<RiddleQuestion>,
    pub correct_response: &'static str,
    pub win_text: &'static str,
    pub win_effects: Vec<Effect>,
    pub lose_text: &'static str,
    pub lose_hint: &'static str,
}

/// Tracks an in-progress riddle game.
#[derive(Debug, Clone)]
pub struct PendingRiddle {
    pub riddle_id: &'static str,
    pub question_index: usize,
}

// ── Core game types ────────────────────────────────────────────────

/// A room / location in the game world.
#[derive(Debug, Clone)]
pub struct Room {
    pub id: RoomId,
    pub name: &'static str,
    pub description: &'static str,
    /// Short description shown on revisit (when navigating back).
    pub short_description: &'static str,
    /// Alternate description shown when all original items have been taken.
    pub description_when_empty: Option<&'static str>,
    pub exits: HashMap<Direction, RoomId>,
    pub items: Vec<ItemId>,
    /// Description overrides based on game state.
    pub conditional_descriptions: Vec<ConditionalDesc>,
}

/// An item that can be picked up, dropped, and examined.
#[derive(Debug, Clone)]
pub struct Item {
    pub id: ItemId,
    pub name: &'static str,
    pub description: &'static str,
}

/// A non-player character who inhabits the world.
#[derive(Debug, Clone)]
pub struct Npc {
    pub id: NpcId,
    pub name: &'static str,
    pub description: &'static str,
    pub current_room: RoomId,
    /// Rooms this NPC is allowed to wander into.
    pub allowed_rooms: Vec<RoomId>,
    /// Dialogue lines cycled through when you TALK TO them.
    pub dialogue: Vec<&'static str>,
    /// Index tracking which dialogue line comes next.
    pub dialogue_index: usize,
    /// Idle flavour text shown randomly when the NPC is in your room.
    pub idle_texts: Vec<&'static str>,
}

/// Parsed player command.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Go(String),
    Look,
    Take(String),
    Drop(String),
    Examine(String),
    TalkTo(String),
    Wait,
    Use(String),
    Attack(String),
    Riddle,
    Inventory,
    Help,
    Quit,
    Unknown(String),
}

/// The full mutable game state.
pub struct GameState {
    pub rooms: HashMap<RoomId, Room>,
    pub items: HashMap<ItemId, Item>,
    pub npcs: HashMap<NpcId, Npc>,
    pub current_room: RoomId,
    pub inventory: Vec<ItemId>,
    pub visited_rooms: HashSet<RoomId>,
    pub flags: HashSet<&'static str>,
    pub pending_riddle: Option<PendingRiddle>,
    pub turn: u32,
    pub running: bool,
    /// Triggers loaded from the game definition.
    pub triggers: Vec<Trigger>,
    /// Riddle game definitions.
    pub riddles: HashMap<&'static str, RiddleDef>,
}

impl GameState {
    pub fn current_room(&self) -> &Room {
        self.rooms.get(self.current_room).expect("current room must exist")
    }

    pub fn current_room_mut(&mut self) -> &mut Room {
        self.rooms.get_mut(self.current_room).expect("current room must exist")
    }

    /// Return NPC ids of all NPCs currently in the given room.
    pub fn npcs_in_room(&self, room_id: RoomId) -> Vec<NpcId> {
        self.npcs
            .values()
            .filter(|npc| npc.current_room == room_id)
            .map(|npc| npc.id)
            .collect()
    }
}
