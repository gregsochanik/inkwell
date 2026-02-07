use std::collections::HashMap;

use crate::game_state::{Direction, GameState, Item, Room};

/// Build the initial game state with all rooms, items, and connections.
pub fn build_world() -> GameState {
    let mut rooms = HashMap::new();

    // --- Rooms ---

    let mut bag_end_exits = HashMap::new();
    bag_end_exits.insert(Direction::East, "the_hill");
    rooms.insert(
        "bag_end",
        Room {
            id: "bag_end",
            name: "Bag End",
            description:
                "You are standing in a comfortable hobbit-hole. A round green door \
                 leads outside. The walls are lined with shelves full of books, \
                 pantry jars, and maps. A warm fire crackles in the hearth. \
                 There is a strong smell of seed-cake.",
            exits: bag_end_exits,
            items: vec!["map"],
        },
    );

    let mut hill_exits = HashMap::new();
    hill_exits.insert(Direction::West, "bag_end");
    hill_exits.insert(Direction::South, "green_dragon");
    hill_exits.insert(Direction::East, "trollshaw");
    rooms.insert(
        "the_hill",
        Room {
            id: "the_hill",
            name: "The Hill",
            description:
                "You stand on a grassy hill overlooking the Shire. The sky is \
                 wide and blue. Bag End is behind you to the west. A winding \
                 path leads south toward a cheerful inn, and east toward dark \
                 woods.",
            exits: hill_exits,
            items: vec![],
        },
    );

    let mut inn_exits = HashMap::new();
    inn_exits.insert(Direction::North, "the_hill");
    rooms.insert(
        "green_dragon",
        Room {
            id: "green_dragon",
            name: "The Green Dragon Inn",
            description:
                "A warm and noisy inn. Hobbits sit at wooden tables drinking \
                 ale and smoking pipe-weed. A stout barkeep polishes mugs \
                 behind the counter. Songs drift from a corner table. The door \
                 leads back north to the Hill.",
            exits: inn_exits,
            items: vec!["elven_bread"],
        },
    );

    let mut trollshaw_exits = HashMap::new();
    trollshaw_exits.insert(Direction::West, "the_hill");
    trollshaw_exits.insert(Direction::East, "troll_clearing");
    rooms.insert(
        "trollshaw",
        Room {
            id: "trollshaw",
            name: "Trollshaw Forest",
            description:
                "Gnarled trees press close on all sides. The path is dim and \
                 overgrown. Strange sounds echo between the trunks. You feel \
                 distinctly unwelcome here. The way continues east, or you \
                 can retreat west.",
            exits: trollshaw_exits,
            items: vec!["sword"],
        },
    );

    let mut troll_exits = HashMap::new();
    troll_exits.insert(Direction::West, "trollshaw");
    troll_exits.insert(Direction::North, "rivendell");
    rooms.insert(
        "troll_clearing",
        Room {
            id: "troll_clearing",
            name: "Troll Clearing",
            description:
                "A trampled clearing littered with bones and broken carts. \
                 Three large stone shapes loom in the centre — the remains \
                 of trolls turned to stone at dawn. A faint path leads north \
                 toward distant waterfalls.",
            exits: troll_exits,
            items: vec!["key"],
        },
    );

    let mut rivendell_exits = HashMap::new();
    rivendell_exits.insert(Direction::South, "troll_clearing");
    rivendell_exits.insert(Direction::East, "misty_pass");
    rooms.insert(
        "rivendell",
        Room {
            id: "rivendell",
            name: "Rivendell",
            description:
                "The Last Homely House gleams in the afternoon sun. Elven \
                 music drifts on the breeze. Waterfalls cascade into crystal \
                 pools below. You feel rested and safe — for now. A steep \
                 path climbs east into the mountains.",
            exits: rivendell_exits,
            items: vec![],
        },
    );

    let mut misty_exits = HashMap::new();
    misty_exits.insert(Direction::West, "rivendell");
    misty_exits.insert(Direction::East, "goblin_cave");
    rooms.insert(
        "misty_pass",
        Room {
            id: "misty_pass",
            name: "Misty Mountains Pass",
            description:
                "A narrow ledge clings to the mountainside. Cold wind howls \
                 and snow stings your face. Thunder rumbles in the distance. \
                 The path is treacherous. A dark crack in the rock leads east \
                 into the mountain itself.",
            exits: misty_exits,
            items: vec![],
        },
    );

    let mut goblin_exits = HashMap::new();
    goblin_exits.insert(Direction::West, "misty_pass");
    rooms.insert(
        "goblin_cave",
        Room {
            id: "goblin_cave",
            name: "Goblin Cave",
            description:
                "A damp, dark cave deep inside the mountain. Water drips from \
                 the ceiling and the air smells foul. Strange eyes glint in \
                 the darkness. On the floor, something glimmers faintly.",
            exits: goblin_exits,
            items: vec!["ring"],
        },
    );

    // --- Items ---

    let mut items = HashMap::new();

    items.insert(
        "map",
        Item {
            id: "map",
            name: "a weathered map",
            description:
                "An old parchment map showing the Lonely Mountain, with a \
                 secret door marked in red ink. Moon-letters shimmer faintly \
                 along the border.",
        },
    );

    items.insert(
        "sword",
        Item {
            id: "sword",
            name: "a glowing elvish sword",
            description:
                "A short blade of elvish make. It glows faintly blue. Runes \
                 along the blade read 'Sting'. It is just the right size for \
                 a hobbit.",
        },
    );

    items.insert(
        "elven_bread",
        Item {
            id: "elven_bread",
            name: "a loaf of elven bread",
            description:
                "A dense, golden loaf wrapped in a large leaf. One small bite \
                 fills you with warmth and energy. It smells faintly of honey.",
        },
    );

    items.insert(
        "key",
        Item {
            id: "key",
            name: "an ornate key",
            description:
                "A heavy iron key with dwarvish runes engraved on its bow. \
                 It looks very old and very important.",
        },
    );

    items.insert(
        "ring",
        Item {
            id: "ring",
            name: "a plain gold ring",
            description:
                "A simple gold ring, surprisingly warm to the touch. When you \
                 hold it, the world seems to go quiet for a moment.",
        },
    );

    GameState {
        rooms,
        items,
        current_room: "bag_end",
        inventory: Vec::new(),
        running: true,
    }
}
