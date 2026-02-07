use std::collections::{HashMap, HashSet};

use crate::game_state::{Direction, GameState, Item, Npc, Room};

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
            short_description: "You are back in the cosy comfort of Bag End.",
            description_when_empty: None,
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
            short_description: "You stand on the Hill, overlooking the Shire.",
            description_when_empty: None,
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
            short_description: "You are in the warm and noisy Green Dragon Inn.",
            description_when_empty: None,
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
            short_description: "You are in the dim depths of Trollshaw Forest.",
            description_when_empty: None,
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
            short_description: "You are in the Troll Clearing. Stone trolls loom nearby.",
            description_when_empty: None,
            exits: troll_exits,
            items: vec![],
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
            short_description: "You are in Rivendell. Elven music drifts on the breeze.",
            description_when_empty: None,
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
            short_description: "You are on the treacherous Misty Mountains Pass.",
            description_when_empty: None,
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
            short_description: "You are in the dark Goblin Cave. Water drips around you.",
            description_when_empty: Some(
                "A damp, dark cave deep inside the mountain. Water drips from \
                 the ceiling and the air smells foul. Strange eyes glint in \
                 the darkness. The floor is bare rock, slick with moisture.",
            ),
            exits: goblin_exits,
            items: vec!["ring"],
        },
    );

    // --- New rooms (east of mountains, unlocked by puzzles) ---

    let mut beorn_exits = HashMap::new();
    beorn_exits.insert(Direction::West, "goblin_cave");
    beorn_exits.insert(Direction::East, "mirkwood");
    rooms.insert(
        "beorns_hall",
        Room {
            id: "beorns_hall",
            name: "Beorn's Hall",
            description:
                "You emerge from the eastern side of the Misty Mountains into \
                 a wide valley. A great wooden hall stands amid fields of \
                 clover, where enormous bees buzz lazily. The hall belongs to \
                 Beorn, a skin-changer — half man, half bear. The air smells \
                 of honey and fresh hay. The mountains rise behind you to \
                 the west, and open lands stretch east.",
            short_description: "You are in Beorn's wide valley. His great hall stands nearby.",
            description_when_empty: None,
            exits: beorn_exits,
            items: vec![],
        },
    );

    let mut mirkwood_exits = HashMap::new();
    mirkwood_exits.insert(Direction::West, "beorns_hall");
    mirkwood_exits.insert(Direction::East, "lake_town");
    rooms.insert(
        "mirkwood",
        Room {
            id: "mirkwood",
            name: "Mirkwood",
            description:
                "You enter the dark eaves of Mirkwood. The trees grow so thick \
                 that barely any light reaches the forest floor. Strange sounds \
                 echo in the gloom — the scuttle of many legs, the snap of twigs. \
                 Thick cobwebs hang between the branches above. A thin path \
                 weaves east through the trees. Every instinct tells you not to \
                 leave it.",
            short_description:
                "You are in the dark depths of Mirkwood. Cobwebs hang overhead.",
            description_when_empty: None,
            exits: mirkwood_exits,
            items: vec![],
        },
    );

    let mut lake_exits = HashMap::new();
    lake_exits.insert(Direction::West, "mirkwood");
    lake_exits.insert(Direction::East, "lonely_mountain");
    rooms.insert(
        "lake_town",
        Room {
            id: "lake_town",
            name: "Lake-town",
            description:
                "A bustling town built on wooden stilts over the Long Lake. \
                 Fishermen mend their nets on the docks and children peer \
                 at you from doorways. Smoke rises from chimneys and the \
                 smell of fish stew fills the air. To the east, the Lonely \
                 Mountain rises like a dark tower against the sky, a thin \
                 wisp of smoke curling from its peak.",
            short_description: "You are in Lake-town. The Lonely Mountain looms to the east.",
            description_when_empty: None,
            exits: lake_exits,
            items: vec![],
        },
    );

    let mut mountain_exits = HashMap::new();
    mountain_exits.insert(Direction::West, "lake_town");
    rooms.insert(
        "lonely_mountain",
        Room {
            id: "lonely_mountain",
            name: "The Lonely Mountain",
            description:
                "You stand at the foot of Erebor — the Lonely Mountain. \
                 The great peak towers above you, its slopes bare and grey. \
                 Somewhere on the western face, hidden from prying eyes, \
                 lies a secret door. Dragon-smoke drifts from a vent high \
                 above. The air is hot and smells of sulphur.",
            short_description:
                "You stand before the Lonely Mountain. Dragon-smoke drifts above.",
            description_when_empty: None,
            exits: mountain_exits,
            items: vec![],
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

    // --- NPCs ---

    let mut npcs = HashMap::new();

    npcs.insert(
        "gandalf",
        Npc {
            id: "gandalf",
            name: "Gandalf the Grey",
            description:
                "A tall old man in a grey cloak and pointed hat. His eyes \
                 twinkle with a knowing light beneath bushy eyebrows. He \
                 leans on a gnarled wooden staff.",
            current_room: "bag_end",
            allowed_rooms: vec![
                "bag_end", "the_hill", "trollshaw", "troll_clearing",
                "rivendell", "misty_pass",
            ],
            dialogue: vec![
                "\"Good morning! Or is it? What do you mean by good morning?\"",
                "\"I am looking for someone to share in an adventure.\"",
                "\"You'll have a tale or two to tell when you come back.\"",
                "\"The world is not in your books and maps. It's out there.\"",
                "\"All good stories deserve embellishment.\"",
            ],
            dialogue_index: 0,
            idle_texts: vec![
                "Gandalf puffs on his pipe, blowing a smoke ring shaped like a ship.",
                "Gandalf peers at you over his spectacles.",
                "Gandalf taps his staff on the ground thoughtfully.",
                "Gandalf mutters something about being late — or perhaps early.",
                "Gandalf gazes into the distance, as if seeing things far away.",
            ],
        },
    );

    npcs.insert(
        "thorin",
        Npc {
            id: "thorin",
            name: "Thorin Oakenshield",
            description:
                "A proud dwarf with a long beard and a fur-trimmed cloak. \
                 He carries himself with the bearing of a king, though his \
                 eyes hold a deep sadness.",
            current_room: "the_hill",
            allowed_rooms: vec![
                "bag_end", "the_hill", "green_dragon", "trollshaw",
                "troll_clearing", "rivendell",
            ],
            dialogue: vec![
                "\"I am Thorin, son of Thrain, son of Thror, King under the Mountain.\"",
                "\"We seek to reclaim our homeland from the dragon Smaug.\"",
                "\"This quest is not for the faint-hearted, halfling.\"",
                "\"The Arkenstone... I must have it.\"",
                "\"If more of us valued food and cheer above hoarded gold, \
                 it would be a merrier world.\"",
            ],
            dialogue_index: 0,
            idle_texts: vec![
                "Thorin sits down and starts singing about gold.",
                "Thorin strokes his beard and stares into the fire.",
                "Thorin polishes his sword with a distant look in his eyes.",
                "Thorin mutters darkly about dragons.",
                "Thorin hums a deep dwarven melody.",
            ],
        },
    );

    npcs.insert(
        "elrond",
        Npc {
            id: "elrond",
            name: "Elrond Half-elven",
            description:
                "An ageless elf-lord with dark hair and wise grey eyes. \
                 He radiates calm authority. A circlet of silver rests \
                 upon his brow.",
            current_room: "rivendell",
            allowed_rooms: vec!["rivendell"],
            dialogue: vec![
                "\"Welcome to Rivendell, little one.\"",
                "\"This map has moon-letters. Hold it up to the moonlight.\"",
                "\"The road ahead is perilous. Rest here while you can.\"",
                "\"Your blade is of elvish make — Orcrist's kin. It will glow \
                 blue when goblins are near.\"",
                "\"Even the smallest person can change the course of the future.\"",
            ],
            dialogue_index: 0,
            idle_texts: vec![
                "Elrond studies an ancient tome with quiet concentration.",
                "Elrond gazes at the waterfalls, lost in memory.",
                "Elrond speaks softly to a passing elf in Sindarin.",
            ],
        },
    );

    npcs.insert(
        "gollum",
        Npc {
            id: "gollum",
            name: "Gollum",
            description:
                "A wretched, thin creature with large pale eyes that glow \
                 in the dark. He crouches on a rock, muttering to himself \
                 and wringing his bony hands.",
            current_room: "goblin_cave",
            allowed_rooms: vec!["goblin_cave"],
            dialogue: vec![
                "\"What has it got in its pocketses, precious?\"",
                "\"We likes riddles, don't we, precious? Yes we does!\"",
                "\"It's ours, precious. It came to us on our birthday.\"",
                "\"Thief! Baggins! We hates it forever!\"",
                "\"Gollum! Gollum!\"",
            ],
            dialogue_index: 0,
            idle_texts: vec![
                "Gollum splashes in the dark water, catching a blind fish.",
                "Gollum mutters \"my precious\" over and over to himself.",
                "Gollum watches you with huge, unblinking eyes.",
                "Gollum hisses and retreats into the shadows.",
            ],
        },
    );

    npcs.insert(
        "beorn",
        Npc {
            id: "beorn",
            name: "Beorn",
            description:
                "A massive man with wild, dark hair and a thick beard. He \
                 moves with the heavy grace of a bear. His eyes are sharp \
                 and watchful, but not unkind.",
            current_room: "beorns_hall",
            allowed_rooms: vec!["beorns_hall"],
            dialogue: vec![
                "\"I don't much like dwarves. But I like goblins even less.\"",
                "\"You may stay the night. My animals will see to your needs.\"",
                "\"The forest of Mirkwood lies to the east. Do not leave the path!\"",
                "\"I am Beorn. Some call me a skin-changer. It is not polite to ask why.\"",
                "\"Take some honey-cakes for the road. You will need your strength.\"",
            ],
            dialogue_index: 0,
            idle_texts: vec![
                "Beorn pours a great bowl of cream for a cat the size of a dog.",
                "Beorn sharpens a massive axe, humming a deep tune.",
                "Beorn gazes out at the mountains with a faraway look.",
                "A huge black dog trots up to Beorn and rests its head on his knee.",
            ],
        },
    );

    let mut visited_rooms = HashSet::new();
    visited_rooms.insert("bag_end");

    GameState {
        rooms,
        items,
        npcs,
        current_room: "bag_end",
        inventory: Vec::new(),
        visited_rooms,
        flags: HashSet::new(),
        pending_riddle: None,
        turn: 0,
        running: true,
    }
}
