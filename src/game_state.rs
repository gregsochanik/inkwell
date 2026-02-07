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
    pub pending_riddle: Option<usize>,
    pub turn: u32,
    pub running: bool,
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
