use crate::game_state::{Command, Direction, GameState, ItemId, RoomId};

/// Maximum number of items a hobbit can carry.
const MAX_INVENTORY: usize = 4;

// --- Troll Clearing alternate descriptions (when trolls are alive) ---

const TROLL_ALIVE_DESC: &str = "\
A trampled clearing littered with bones and broken carts. Three enormous \
trolls sit around a sputtering fire, arguing about how to cook their \
latest catch. They turn their ugly heads toward you and snarl. The path \
north toward the waterfalls is blocked by their bulk.";

const TROLL_ALIVE_SHORT: &str = "\
You are in the Troll Clearing. Three enormous trolls block the path.";

// --- Riddle game data ---

const RIDDLE_QUESTIONS: [&str; 3] = [
    "\"What has roots as nobody sees,\n  Is taller than trees,\n  Up, up, up it goes,\n  And yet never grows?\"",
    "\"Thirty white horses on a red hill,\n  First they champ,\n  Then they stamp,\n  Then they stand still.\"",
    "\"A box without hinges, key, or lid,\n  Yet golden treasure inside is hid.\"",
];

const RIDDLE_ANSWERS: [&str; 3] = ["mountain", "teeth", "egg"];

// --- Win text ---

const WIN_TEXT: &str = "\
You consult the weathered map and find the hidden mark on the western \
face of the mountain. Moon-letters shimmer in the fading light, \
revealing a keyhole cunningly disguised in the rock.

You insert the ornate key and turn it. With a grinding of ancient \
stone, a door swings open where there was only bare rock before.

A passage leads deep into the heart of the mountain. After what \
feels like hours of walking, you emerge into an enormous hall \
filled with gold, jewels, and treasure beyond imagining.

You have found the dragon's hoard of Erebor!

================================================================
                    CONGRATULATIONS!

    You have completed HOBITTY: An 80s Text Adventure!

    Dobo Daggins, burglar extraordinaire, has reclaimed
    the treasure of the Lonely Mountain.

    Thank you for playing!
================================================================";

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

/// Get the appropriate full description for a room, considering puzzle state.
fn get_room_desc(state: &GameState) -> &'static str {
    let room = state.current_room();
    match room.id {
        "troll_clearing" if !state.flags.contains("trolls_defeated") => TROLL_ALIVE_DESC,
        _ => {
            if room.items.is_empty() {
                room.description_when_empty.unwrap_or(room.description)
            } else {
                room.description
            }
        }
    }
}

/// Get the appropriate short description for a room, considering puzzle state.
fn get_room_short_desc(state: &GameState) -> &'static str {
    let room = state.current_room();
    match room.id {
        "troll_clearing" if !state.flags.contains("trolls_defeated") => TROLL_ALIVE_SHORT,
        _ => room.short_description,
    }
}

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

    // --- Puzzle guards ---

    // Troll clearing: trolls block the north passage
    if state.current_room == "troll_clearing"
        && direction == Direction::North
        && !state.flags.contains("trolls_defeated")
    {
        return "The trolls snarl and block your path north! They're \
                enormous and very angry. You'll need to find another \
                way past them."
            .to_string();
    }

    // Goblin cave: hint about eastern passage if riddle not won
    if state.current_room == "goblin_cave" && direction == Direction::East {
        if !state.flags.contains("riddle_won") {
            return "The cave seems to continue east, but the passages \
                    twist and double back on themselves. You can't find \
                    a way through. Gollum watches you from the shadows, \
                    muttering about games and riddles..."
                .to_string();
        }
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
                text.push_str(&format!("\n{} is here.", npc.name));
            }
        }
    }

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
}

/// Full room description (used by LOOK command and first visit).
fn cmd_look(state: &GameState) -> String {
    let room = state.current_room();
    let desc = get_room_desc(state);
    let room_id = state.current_room;
    let mut text = format!("\n--- {} ---\n{}", room.name, desc);
    append_room_details(&mut text, room_id, state);
    text
}

/// Short room description (used on revisit when navigating).
fn cmd_glance(state: &GameState) -> String {
    let room = state.current_room();
    let short = get_room_short_desc(state);
    let room_id = state.current_room;
    let mut text = format!("\n--- {} ---\n{}", room.name, short);
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
            format!("You pick up {}.", name)
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

    // Check NPCs in the current room
    let npc_ids = state.npcs_in_room(state.current_room);
    for npc_id in &npc_ids {
        if let Some(npc) = state.npcs.get(npc_id) {
            if npc.id.contains(target) || npc.name.to_lowercase().contains(target) {
                return format!("{}\n{}", npc.name, npc.description);
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
                format!("{}\n{}", item.name, item.description)
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
            let npc = state.npcs.get_mut(npc_id).expect("npc must exist");
            let line = npc.dialogue[npc.dialogue_index];
            npc.dialogue_index = (npc.dialogue_index + 1) % npc.dialogue.len();
            format!("{} says: {}", npc.name, line)
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
// Iteration 3: Puzzle & Win Condition commands
// ============================================================

fn cmd_wait(state: &mut GameState) -> String {
    // Troll clearing puzzle
    if state.current_room == "troll_clearing" && !state.flags.contains("trolls_defeated") {
        state.flags.insert("trolls_defeated");

        // Add key to the room
        if let Some(room) = state.rooms.get_mut("troll_clearing") {
            room.items.push("key");
        }

        return "You hold very still and wait...\n\n\
                The trolls continue arguing:\n\
                \"Let's roast 'em slowly!\" says Tom.\n\
                \"No, sit on 'em and squash 'em into jelly!\" says Bert.\n\
                \"You're both ninnies!\" says William.\n\n\
                From somewhere in the trees, a voice remarkably like \
                William's calls out — \"Dawn take you all, and be stone \
                to you!\"\n\n\
                The trolls look up just as the first rays of sunlight \
                break over the tree-tops. One by one, they stiffen, \
                their skin turns grey, and they become... stone.\n\n\
                Silence fills the clearing. Among the trolls' belongings, \
                you notice an ornate key glinting in the morning light."
            .to_string();
    }

    "Time passes...".to_string()
}

fn cmd_use(target: &str, state: &mut GameState) -> String {
    // --- Key ---
    if target.contains("key") {
        if !state.inventory.iter().any(|&id| id == "key") {
            return "You don't have a key.".to_string();
        }
        if state.current_room == "lonely_mountain" {
            if state.inventory.iter().any(|&id| id == "map") {
                // WIN!
                state.running = false;
                return WIN_TEXT.to_string();
            } else {
                return "You try the key on the mountainside, but without \
                        a map you have no idea where the secret door is."
                    .to_string();
            }
        }
        return "There is nothing to use the key on here.".to_string();
    }

    // --- Ring ---
    if target.contains("ring") {
        if !state.inventory.iter().any(|&id| id == "ring") {
            return "You don't have a ring.".to_string();
        }
        return "You slip the ring onto your finger. The world dims \
                and fades to shadow. Strange shapes swirl at the edges \
                of your vision. You quickly take it off again, heart \
                pounding."
            .to_string();
    }

    // --- Sword ---
    if target.contains("sword") || target.contains("sting") {
        if !state.inventory.iter().any(|&id| id == "sword") {
            return "You don't have a sword.".to_string();
        }
        return "You draw the elvish blade. It hums faintly.".to_string();
    }

    // --- Bread ---
    if target.contains("bread") {
        if !state.inventory.iter().any(|&id| id == "elven_bread") {
            return "You don't have any bread.".to_string();
        }
        return "You take a bite of the elven bread. It fills you \
                with warmth and renewed energy. Delicious!"
            .to_string();
    }

    // --- Map ---
    if target.contains("map") {
        if !state.inventory.iter().any(|&id| id == "map") {
            return "You don't have a map.".to_string();
        }
        if state.current_room == "lonely_mountain" {
            return "You study the map carefully. Moon-letters shimmer \
                    along the border, pointing to a spot on the western \
                    face of the mountain. If only you had a key to open \
                    the secret door..."
                .to_string();
        }
        return "You study the weathered map. It shows the Lonely \
                Mountain and a secret door marked in red ink."
            .to_string();
    }

    format!("You're not sure how to use the {}.", target)
}

fn cmd_attack(target: &str, state: &mut GameState) -> String {
    // Troll clearing: attacking trolls is fatal
    if state.current_room == "troll_clearing" && !state.flags.contains("trolls_defeated") {
        if target.contains("troll") {
            state.running = false;
            return "You charge at the trolls, but they are enormous — \
                    each one three times your size. Tom grabs you before \
                    you can even swing and stuffs you into a sack.\n\n\
                    \"We'll have this one for supper!\" says Bert.\n\n\
                    === GAME OVER ===\n\n\
                    You have been caught by trolls. Perhaps patience \
                    would have served you better than bravery."
                .to_string();
        }
    }

    // Goblin cave: attacking Gollum is fatal
    if state.current_room == "goblin_cave" {
        if target.contains("gollum") || target.contains("creature") {
            state.running = false;
            return "You lunge at Gollum, but he is far quicker than \
                    he looks. He vanishes into the shadows and you hear \
                    a terrible hiss behind you. In the pitch darkness \
                    of the cave, you never see what hits you.\n\n\
                    === GAME OVER ===\n\n\
                    Violence was not the answer here. Perhaps riddles \
                    would have been wiser."
                .to_string();
        }
    }

    // Generic responses
    if state.inventory.iter().any(|&id| id == "sword") {
        format!(
            "You wave your sword at the {}. Nothing much happens.",
            target
        )
    } else {
        "You have nothing to fight with! And really, is violence \
         the hobbit way?"
            .to_string()
    }
}

fn cmd_riddle(state: &mut GameState) -> String {
    // Check if Gollum is in the room
    let gollum_here = state
        .npcs_in_room(state.current_room)
        .iter()
        .any(|&id| id == "gollum");

    if !gollum_here {
        return "There is nobody here to play riddles with.".to_string();
    }

    if state.flags.contains("riddle_won") {
        return "You've already won the riddle game. Gollum eyes you \
                resentfully from the shadows."
            .to_string();
    }

    // Start the riddle game
    state.pending_riddle = Some(0);
    format!(
        "Gollum's eyes light up. \"Riddles! We plays a game of riddles, \
         precious! If it wins, we shows it the way out. If it loses... \
         we eats it whole!\"\n\nGollum asks:\n{}",
        RIDDLE_QUESTIONS[0]
    )
}

/// Handle a riddle answer (called from main loop when a riddle is pending).
pub fn handle_riddle_answer(input: &str, state: &mut GameState) -> String {
    let riddle_idx = match state.pending_riddle {
        Some(idx) => idx,
        None => return String::new(),
    };

    let answer = input.trim().to_lowercase();

    // Allow quitting during riddle game
    if answer == "quit" || answer == "q" {
        state.running = false;
        state.pending_riddle = None;
        return "Farewell, adventurer. Until next time!".to_string();
    }

    let correct = answer.contains(RIDDLE_ANSWERS[riddle_idx]);

    if correct {
        if riddle_idx + 1 < RIDDLE_QUESTIONS.len() {
            // Next riddle
            state.pending_riddle = Some(riddle_idx + 1);
            format!(
                "\"Yess, yess...\" Gollum hisses grudgingly. \"It knows \
                 this one, precious. But can it answer THIS?\"\n\n\
                 Gollum asks:\n{}",
                RIDDLE_QUESTIONS[riddle_idx + 1]
            )
        } else {
            // Won all riddles!
            state.pending_riddle = None;
            state.flags.insert("riddle_won");

            // Move Gollum away
            if let Some(gollum) = state.npcs.get_mut("gollum") {
                gollum.current_room = "nowhere";
            }

            // Open the eastern passage from goblin cave
            if let Some(room) = state.rooms.get_mut("goblin_cave") {
                room.exits.insert(Direction::East, "beorns_hall");
            }

            "\"Curse it! CURSE IT!\" Gollum shrieks, tearing at his \
             thin hair. \"It wins, precious. Tricky, nasty hobbitses!\"\n\n\
             Gollum, bound by his wretched promise, slinks away into \
             the darkness. As he goes, you notice a faint draught of \
             fresh air from the east — a hidden passage!\n\n\
             A new exit has opened to the east."
                .to_string()
        }
    } else {
        // Wrong answer — death
        state.running = false;
        state.pending_riddle = None;
        "\"WRONG!\" Gollum shrieks with glee. \"Wrong, wrong, WRONG!\"\n\n\
         Gollum lunges from the shadows with terrible speed. In the \
         pitch darkness of the cave, you never stood a chance.\n\n\
         === GAME OVER ===\n\n\
         You have been eaten by Gollum. Perhaps next time, brush up \
         on your riddles."
            .to_string()
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
  EXAMINE <item> (X)    — Examine an item or person closely
  TALK TO <name>        — Talk to someone nearby
  USE <item>            — Use an item you are carrying
  WAIT (Z)              — Wait and let time pass
  RIDDLE                — Challenge someone to a game of riddles
  ATTACK <target>       — Attack something (not very hobbit-like)
  INVENTORY (I)         — List what you are carrying
  HELP (?)              — Show this help
  QUIT (Q)              — Leave the game"
        .to_string()
}
