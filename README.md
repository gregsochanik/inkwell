# Inkwell — A Text Adventure Engine in Rust

A data-driven text adventure engine where games are defined entirely in YAML.
The Rust engine handles text parsing, room navigation, inventory, NPCs,
puzzles, riddles, save/load, and ANSI terminal art.

Originally inspired by the classic 1982 Melbourne House *The Hobbit*, the
project has evolved into a generic engine that can run any text adventure
game described in a YAML file.

## Install

### Pre-built binaries

Download the latest release from the
[Releases page](https://github.com/gregsochanik/inkwell/releases).
Extract the binary and run it:

```bash
./inkwell examples/hobitty
```

### Build from source

Requires **Rust** (edition 2024) — install via [rustup](https://rustup.rs/).

```bash
git clone https://github.com/gregsochanik/inkwell.git
cd inkwell
cargo build --release
./target/release/inkwell examples/hobitty
```

## How to play

Type commands at the `>` prompt:

| Command | Examples |
|---------|----------|
| Move | `go north`, `n`, `south` |
| Look around | `look`, `l` |
| Take items | `take sword`, `get bread` |
| Use items | `use key`, `use map` |
| Talk to NPCs | `talk gandalf`, `talk gollum` |
| Check inventory | `inventory`, `i` |
| Attack | `attack troll` |
| Wait | `wait` |
| Save / Load | `save`, `load` |
| Quit | `quit`, `q` |

## Included games

### Hobitty (`examples/hobitty/`) — the original game

A ~10 room adventure loosely based on The Hobbit. Collect items, outwit
trolls, solve riddles, and find the treasure of the Lonely Mountain.

### The Locked Tower (`examples/locked_tower/`)

A minimal 3-room escape game demonstrating the engine's generic capabilities.
Pick a lock, answer a ghost's riddle, and climb to freedom.

## Creating your own game

Each game is a directory containing a `game.yaml` file:

```
examples/my_game/
  game.yaml       # game definition (required)
  art/            # room art .ans files (optional)
```

See [AUTHORING.md](AUTHORING.md) for the full YAML schema reference covering
rooms, items, NPCs, triggers, conditions, effects, riddles, and text templates.

Run your game with:

```bash
cargo run -- examples/my_game
```

The engine validates your YAML on load and reports errors and warnings.

## Project structure

```
src/
  main.rs          Entry point, CLI, game loop
  game_state.rs    Core data types (Room, Item, Npc, Trigger, etc.)
  loader.rs        YAML parsing, state building, validation
  triggers.rs      Condition/effect engine, riddle system, text templates
  commands.rs      Command dispatch
  parser.rs        Text input parser
  npc.rs           NPC idle behaviour and movement
  save.rs          Save/load game state
  color.rs         Terminal colour constants
examples/          Game directories
PLAN.md            Development roadmap
AUTHORING.md       Game authoring guide
AGENT.md           Quick reference for AI agents working on this project
```

## Contributing

1. Check [PLAN.md](PLAN.md) for the roadmap and open iterations.
2. Game content goes in YAML — engine code should stay generic.
3. After changes, verify both example games play through to completion.

## Licence

This is a personal prototype / learning project.
