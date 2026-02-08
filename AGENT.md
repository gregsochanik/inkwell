# Agent Guide — Hobitty

## What this is

A data-driven text adventure engine in Rust. Games are defined in YAML; the
engine handles parsing, state, triggers, NPCs, riddles, save/load, and ANSI art.

## Project layout

```
src/
  main.rs          Entry point, CLI arg parsing, game loop
  game_state.rs    Core types: Room, Item, Npc, Trigger, Condition, Effect, etc.
  loader.rs        YAML parsing → GameState, plus schema validation
  triggers.rs      Condition evaluation, effect execution, riddle engine, templates
  commands.rs      Command dispatch (delegates puzzle logic to triggers)
  parser.rs        Text input → Command enum
  npc.rs           NPC idle behaviour and movement AI
  save.rs          Save/load game state to file
  art.rs           Built-in ANSI half-block room art (fallback)
  color.rs         ANSI escape code constants
  world.rs         Legacy hardcoded world builder (dead code, kept for reference)
examples/
  hobitty/         Main game — Hobbit-inspired adventure
  locked_tower/    Minimal 3-room demo game
PLAN.md            Roadmap and iteration history
AUTHORING.md       Guide for creating games in YAML
```

## Key conventions

- **Rust edition 2024**, deps: `serde`, `serde_yaml`.
- Game content lives in `<game-dir>/game.yaml`. Run with `cargo run -- <game-dir>`.
- All relative paths (art files, save files) resolve from the game directory.
- `&'static str` is used throughout `GameState` — YAML strings are `Box::leak`ed.
- YAML enums use `!tag` syntax (e.g. `!in_room`, `!set_flag`, `!print`).
- Trigger system: `event` + `target` + `direction` + `conditions` → `effects`.
- Text templates: `{bold:text}`, `{npc:text}`, `{item:text}`, `{dialogue:text}`,
  `{exits:text}`, `{bell}`.

## Build & run

```
cargo build
cargo run -- examples/hobitty
cargo run -- examples/locked_tower
```

## Testing a change

1. `cargo build` — must compile with zero errors.
2. `cargo run -- examples/hobitty` — play through: take sword, go south to
   trolls, wait (trolls turn to stone), go north, go north to goblin cave,
   talk gollum (answer: mountain, wind, dark), go east, take key, go east to
   lonely mountain, use key → win.
3. `cargo run -- examples/locked_tower` — take pin, use pin, go north, talk
   ghost (answer: moonlight), take key, use key, go north, use rope → win.

## Adding a new game

1. Create `examples/<name>/game.yaml` following `AUTHORING.md`.
2. Run `cargo run -- examples/<name>` — validation errors show on load.

## Common tasks

- **New trigger event type**: add variant to `Effect`/`Condition` enums in
  `game_state.rs`, handle in `triggers.rs`, add YAML deser in `loader.rs`.
- **New command**: add to `Command` enum in `game_state.rs`, parse in
  `parser.rs`, dispatch in `commands.rs`.
- **Changing game content**: edit `examples/<name>/game.yaml`, not Rust code.

## What NOT to do

- Don't hardcode game-specific logic in Rust — use YAML triggers.
- Don't modify `world.rs` — it's legacy dead code.
- Don't commit `save.dat` files.
