# Hobitty

An 80s-style text adventure engine in Rust, loosely inspired by the classic
1982 Melbourne House *The Hobbit*.

Games are defined entirely in YAML — the Rust engine handles parsing, room
navigation, inventory, NPCs, puzzles, riddles, save/load, and ANSI art.

## Requirements

- **Rust** (edition 2024) — install via [rustup](https://rustup.rs/)

No other dependencies required. Everything is pure Rust + `serde`/`serde_yaml`.

## Quick start

```bash
# Clone and build
git clone <repo-url>
cd hobitty
cargo build

# Play the main adventure
cargo run -- examples/hobitty

# Play the demo escape game
cargo run -- examples/locked_tower
```

## How to play

Type commands at the `>` prompt. The parser understands simple verb-noun input:

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

### Hobitty (`examples/hobitty/`)

A ~10 room adventure where you play as Dobo Daggins. Collect items, outwit
trolls, solve Gollum's riddles, and find the treasure of the Lonely Mountain.

### The Locked Tower (`examples/locked_tower/`)

A minimal 3-room escape game demonstrating the engine's generic capabilities.
Pick a lock, answer a ghost's riddle, and climb to freedom.

## Creating your own game

Games are self-contained directories with a `game.yaml` file:

```
examples/my_game/
  game.yaml       # game definition (required)
  art/            # room art .ans files (optional)
```

See [AUTHORING.md](AUTHORING.md) for the full YAML schema reference, including
rooms, items, NPCs, triggers, riddles, and text templates.

Run your game with:

```bash
cargo run -- examples/my_game
```

The engine validates your YAML on load and reports errors and warnings.

## Project structure

```
src/
  main.rs          Game loop and CLI
  game_state.rs    Core data types
  loader.rs        YAML loader and validator
  triggers.rs      Event/condition/effect engine
  commands.rs      Command dispatch
  parser.rs        Text parser
  npc.rs           NPC behaviour
  save.rs          Save/load
  art.rs           Built-in ANSI room art
  color.rs         Terminal colour helpers
examples/          Game directories
PLAN.md            Development roadmap
AUTHORING.md       Game authoring guide
```

## Contributing

1. Check [PLAN.md](PLAN.md) for the roadmap and open iterations.
2. Game content changes go in YAML, not Rust code.
3. Engine changes should remain generic — no game-specific logic in Rust.
4. After changes, verify both example games still play through to completion.

## Licence

This is a personal prototype / learning project.
