# Hobitty - An 80s Text Adventure in Rust

## Overview

A text adventure game inspired by the classic 1982 Melbourne House *The Hobbit*.
The player takes on the role of Dobo Daggins, a comfortable homebody who gets
swept up in an adventure with a band of dwarves and a wizard to reclaim treasure
from a dragon.

The game uses a text parser, room-based navigation, inventory system, and NPC
interactions — all hallmarks of the 80s text adventure genre.

---

## Iteration 1 — Minimum Playable Game (MVP)

**Goal:** Compilable, runnable game loop you can test-drive immediately.

### Features

- **Game loop** — read input, parse, execute, print output, repeat
- **Text parser** — simple verb-noun parser (e.g. `GO NORTH`, `TAKE SWORD`, `LOOK`, `INVENTORY`, `QUIT`)
- **Rooms / Locations** — a small map of ~8 connected rooms representing:
  - Bag End (start)
  - The Hill
  - The Green Dragon Inn
  - Trollshaw Forest
  - Troll Clearing
  - Rivendell
  - Misty Mountains Pass
  - Goblin Cave
- **Navigation** — cardinal directions (NORTH, SOUTH, EAST, WEST) to move between rooms
- **Room descriptions** — atmospheric, 80s-style prose for each room
- **Items** — a handful of items that can be picked up, dropped, and examined:
  - Map, Sword (Sting), Elven Bread, Key, Ring
- **Inventory** — carry items, list them with `INVENTORY`
- **LOOK** command — re-display the current room description and visible items
- **HELP** command — list available commands
- **QUIT** command — exit the game

### Architecture (Iteration 1)

```
src/
  main.rs          — entry point, game loop, welcome banner
  parser.rs        — tokenize and parse player input into commands
  world.rs         — room definitions, map connectivity, item placement
  game_state.rs    — player state (current room, inventory), game tick logic
  commands.rs      — command execution (go, take, drop, look, inventory, etc.)
```

### Data Model

- **Room**: id, name, description, exits (HashMap<Direction, RoomId>), items (Vec<ItemId>)
- **Item**: id, name, description
- **Direction**: North, South, East, West
- **Command**: Go(Direction), Take(String), Drop(String), Look, Inventory, Help, Quit, Unknown(String)
- **GameState**: current_room, player_inventory, rooms, items, running flag

---

## Iteration 2 — NPCs & Dialogue ✅ DONE

- ✅ Gandalf, Thorin, Elrond, and Gollum as NPCs present in rooms
- ✅ NPCs have idle behaviour text (e.g. "Thorin sits down and starts singing about gold.")
- ✅ `TALK TO <npc>` command with cycling dialogue lines
- ✅ NPCs can move between adjacent allowed rooms on their own each turn (simple AI)
- ✅ EXAMINE works on NPCs
- ✅ NPC arrival/departure messages when player is in the room

## Iteration 3 — Puzzles & Win Condition ✅ DONE

- ✅ Troll encounter puzzle — trolls alive in clearing, block north passage.
  WAIT triggers Gandalf's voice tricking them until dawn turns them to stone.
  ATTACK trolls = death. Key revealed after puzzle solved.
- ✅ Riddle game with Gollum — RIDDLE command starts 3-riddle game
  (mountain, teeth, egg). Win = eastern passage opens. Wrong answer = death.
- ✅ Key usage — USE KEY at the Lonely Mountain (with map) = win condition
- ✅ Win condition — reach Lonely Mountain with map + key, USE KEY to win
- ✅ Death / game-over states — attacking trolls, attacking Gollum,
  wrong riddle answer all result in GAME OVER
- ✅ 3 new rooms: Beorn's Hall, Lake-town, The Lonely Mountain
- ✅ USE command with flavour responses for all items (ring, sword, bread, map)
- ✅ WAIT, ATTACK, RIDDLE commands added

## Iteration 4 — Polish & Atmosphere ✅ DONE

- ~~Randomised NPC flavour text~~ (done in iteration 2)
- ✅ Colour output (ANSI codes) — room titles (bold cyan), NPC names (bold green),
  item names (yellow), exits (bold), death text (bright red), win text (bright yellow),
  riddle text (italic magenta), NPC dialogue (green), coloured title banner
- ✅ Save / load game state — SAVE and LOAD commands, custom line-based format
  (`hobitty.sav`), handles full state reconstruction including room items, NPC
  positions, puzzle flags, inventory, and visited rooms
- ✅ Sound effects via terminal bell — bell on game over, win, troll puzzle solved,
  and picking up the ring
- ✅ New room: Mirkwood (between Beorn's Hall and Lake-town) with atmospheric
  cobweb/spider description
- ✅ New NPC: Beorn in Beorn's Hall with dialogue and idle behaviour
- ✅ New module: `src/color.rs` for ANSI colour constants and helper functions
- ✅ New module: `src/save.rs` for save/load logic
- ✅ Coloured title screen with bold ASCII art banner
- ✅ Updated HELP to include SAVE/LOAD commands

---

## Iteration 5 — Data-Driven Engine (YAML)

**Goal:** Separate the game engine from the game content. All game content
(rooms, items, NPCs, puzzles, dialogue, win/death conditions) moves into a
YAML file. The Rust engine becomes a generic text adventure runtime that can
play any game defined in this format.

### 5a — YAML Schema & Loader (foundation) ✅ DONE

- ✅ Added `serde` + `serde_yaml` crates
- ✅ Designed YAML schema: game metadata, rooms, items, NPCs
- ✅ Created `game.yaml` with all existing Hobitty content
- ✅ New `src/loader.rs` module: parse YAML → `GameState` (uses `Box::leak`
  to convert `String` → `&'static str`, keeping all engine code unchanged)
- ✅ Replaced `world::build_world()` with YAML loader in `main.rs` and `save.rs`
- ✅ Game compiles, runs, and plays identically from YAML

### 5b — Puzzles & Triggers in YAML ✅ DONE

- ✅ Designed trigger/event system: triggers have event type, optional target/
  direction, conditions (in_room, has_item, flag_set, flag_unset), and effects
  (print, set_flag, bell, add_item, open_exit, remove_npc, game_over, win,
  start_riddle). Uses YAML `!tag` syntax for enum variants.
- ✅ New `src/triggers.rs` module: condition evaluation, effect execution,
  text template processing (`{npc:text}`, `{item:text}`, `{dialogue:text}`,
  `{exits:text}`, `{bell}`), riddle game engine, room description overrides
- ✅ Added trigger types to `game_state.rs`: Condition, Effect, Trigger,
  ConditionalDesc, RiddleDef, RiddleQuestion, PendingRiddle
- ✅ All YAML deserialization types in `loader.rs` with conversion to runtime types
- ✅ Moved troll puzzle (wait → trolls turn to stone) to YAML trigger
- ✅ Moved troll/Gollum death conditions to YAML attack triggers
- ✅ Moved Gollum riddle game to YAML riddles section
- ✅ Moved win condition (USE KEY + map at Lonely Mountain) to YAML trigger
- ✅ Moved all USE item responses to YAML triggers
- ✅ Moved blocked exits (trolls block north, riddle blocks east) to YAML
- ✅ Moved conditional room descriptions (troll clearing) to YAML
- ✅ Moved ring pickup bell to YAML pickup trigger
- ✅ Removed all hardcoded puzzle logic from `commands.rs`
- ✅ Updated `save.rs`: generic flag loading, trigger-based exit replay
- ✅ Full playthrough verified: all puzzles, deaths, and win condition work

### 5c — Dialogue & Flavour in YAML ✅ DONE

- ✅ NPC dialogue lines already in YAML (done in 5a)
- ✅ Room description variants (troll clearing) in YAML (done in 5b)
- ✅ Item use flavour text in YAML (done in 5b)
- ✅ Moved banner / intro text to YAML game metadata
  - New `Meta` struct in `game_state.rs` (title, subtitle, tagline, banner, intro)
  - `banner` field uses YAML `|4` block scalar for ASCII art
  - `intro` uses `{bold:text}` template syntax for emphasis
  - `print_banner()` now reads from `state.meta` instead of hardcoded text
- ✅ Added `art` field to room YAML schema (file path to `.ans` art)
  - Engine loads art from file if `art` is set, falls back to built-in `art.rs`
  - Prepares for future image-to-ANSI pipeline

### 5d — Validation & Documentation

- [ ] YAML schema validation on load (missing rooms, broken exit references,
  missing items, etc.) with clear error messages
- [ ] Write a short `AUTHORING.md` guide: how to create a new game using
  the YAML format
- [ ] Create a minimal example game YAML (not Hobitty) to prove the engine
  is truly generic

---

## Iteration 6 — Image-to-ANSI Art Converter

**Goal:** A self-contained Rust tool that converts PNG/JPEG images into
half-block ANSI art `.ans` files. This allows an agent (or human author)
to provide source images for room illustrations and have them automatically
converted during game setup, rather than hand-crafting ANSI art.

### 6a — Core converter binary

- [ ] Add `image` crate as an optional dependency
- [ ] New binary: `cargo run --bin img2ans -- <input> <output> [options]`
- [ ] Load PNG/JPEG, resize to target dimensions (e.g. `--width 50 --height 11`)
- [ ] Render pixel pairs as Unicode half-block characters (▀▄█) with 256-colour
  or 24-bit ANSI foreground/background codes
- [ ] Write output as a plain text `.ans` file (ready to reference from `game.yaml`)
- [ ] Support `--palette` option: `truecolor` (default), `256`, `16` for
  different terminal compatibility levels

### 6b — Batch conversion & YAML integration

- [ ] Support converting all images in a directory:
  `cargo run --bin img2ans -- --batch art/source/ art/`
- [ ] Optional `art_image` field in room YAML schema — points to a source
  image; a build/setup step converts it to `.ans` automatically
- [ ] Setup script / `cargo run --bin setup` that scans `game.yaml` for
  `art_image` references and runs the converter for each one

### 6c — Quality tuning

- [ ] Dithering options (none, Floyd-Steinberg, ordered)
- [ ] Brightness / contrast adjustment
- [ ] Optional border/frame around art
- [ ] Preview mode: convert and display in terminal without writing a file

---

## Tech Notes

- Pure Rust, no external crates for iteration 1 (just `std`)
- `serde` + `serde_yaml` added in iteration 5
- `image` crate added in iteration 6 (optional, only for the converter binary)
- `cargo run` to play (loads `game.yaml` from working directory)
- Designed for easy extension — games defined as YAML data, not code
