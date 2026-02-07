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

## Iteration 3 — Puzzles & Win Condition

- Troll encounter puzzle (use items / wait for dawn)
- Riddle game with Gollum in the Goblin Cave
- Locked doors / key usage
- Win condition: reach the Lonely Mountain with the right items
- Death / game-over states

## Iteration 4 — Polish & Atmosphere

- ~~Randomised NPC flavour text~~ (done in iteration 2)
- ASCII art title screen and room illustrations
- Save / load game state
- Colour output (ANSI codes)
- More rooms extending the map toward the Lonely Mountain
- Sound effects via terminal bell (just for fun)

---

## Tech Notes

- Pure Rust, no external crates for iteration 1 (just `std`)
- `cargo run` to play
- Designed for easy extension — rooms and items defined as data, not code
