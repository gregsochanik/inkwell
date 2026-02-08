# Authoring a Game

This guide explains how to create a text adventure game using the engine's
YAML format. The Rust engine is generic — it reads a `game.yaml` file and
plays whatever game is defined there.

## Directory Convention

Each game lives in its own directory with at minimum a `game.yaml` file:

```
examples/my_game/
  game.yaml          # required — the game definition
  art/               # optional — room art (.ans files)
    dungeon.ans
```

Run a game with:

```
cargo run -- examples/my_game
```

## Quick Start

1. Create a directory for your game (e.g. `examples/my_game/`).
2. Create a `game.yaml` file inside it (see `examples/locked_tower/` for a
   minimal template).
3. Run `cargo run -- examples/my_game`.
4. The engine validates your YAML on load and reports errors/warnings.

## File Structure

```yaml
game:       # metadata & intro
rooms:      # locations the player can visit
items:      # objects that can be picked up / used
npcs:       # non-player characters
triggers:   # event → condition → effect rules
riddles:    # multi-turn riddle games
```

---

## Game Metadata

```yaml
game:
  title: "My Game"
  subtitle: "A Text Adventure"
  tagline: "By Author Name"
  start_room: starting_room_id
  banner: |4
      ASCII art for the title screen goes here
  intro: >-
    Introductory text shown when the game starts.
    Use {bold:text} for emphasis.
```

| Field        | Required | Description                              |
|--------------|----------|------------------------------------------|
| `title`      | yes      | Game title                               |
| `start_room` | yes      | ID of the room the player starts in      |
| `subtitle`   | no       | Shown below the banner in yellow          |
| `tagline`    | no       | Shown below the subtitle in dim text      |
| `banner`     | no       | ASCII art for the title screen            |
| `intro`      | no       | Opening text (supports templates)         |

---

## Rooms

```yaml
rooms:
  kitchen:
    name: "The Kitchen"
    description: >-
      A warm kitchen with copper pots hanging from the ceiling.
    short_description: "You are in the kitchen."
    description_when_empty: >-
      A warm kitchen. The shelves are bare.
    exits:
      north: hallway
      east: garden
    items:
      - bread
    art: art/kitchen.ans
    conditional_descriptions:
      - conditions:
          - !flag_unset lights_on
        description: "The kitchen is dark. You can barely see."
        short_description: "You are in a dark kitchen."
```

| Field                      | Required | Description                                |
|----------------------------|----------|--------------------------------------------|
| `name`                     | yes      | Display name                               |
| `description`              | yes      | Full description (first visit / LOOK)       |
| `short_description`        | no       | Shown on revisit                           |
| `description_when_empty`   | no       | Shown when all items have been taken        |
| `exits`                    | no       | Map of direction → room_id                  |
| `items`                    | no       | List of item IDs placed here at start       |
| `art`                      | no       | Path to an `.ans` ANSI art file             |
| `conditional_descriptions` | no       | Description overrides based on game flags   |

**Directions:** `north`, `south`, `east`, `west`

---

## Items

```yaml
items:
  bread:
    name: "a loaf of bread"
    description: "A fresh loaf of bread. It smells wonderful."
```

| Field         | Required | Description                             |
|---------------|----------|-----------------------------------------|
| `name`        | yes      | Display name (shown in inventory/room)   |
| `description` | yes      | Shown when the player EXAMINEs the item  |

Items must be placed in a room's `items` list or added by a trigger's
`add_item` effect to be accessible.

---

## NPCs

```yaml
npcs:
  guard:
    name: "The Guard"
    description: "A burly guard in chainmail."
    start_room: gate
    allowed_rooms:
      - gate
      - courtyard
    dialogue:
      - '"Halt! State your business."'
      - '"Move along."'
    idle_texts:
      - "The guard shifts his weight from foot to foot."
```

| Field           | Required | Description                                    |
|-----------------|----------|------------------------------------------------|
| `name`          | yes      | Display name                                   |
| `description`   | yes      | Shown when the player EXAMINEs the NPC         |
| `start_room`    | yes      | Room ID where the NPC starts                   |
| `allowed_rooms` | no       | Rooms the NPC can wander into (empty = stays)  |
| `dialogue`      | no       | Lines cycled through with TALK TO               |
| `idle_texts`    | no       | Random flavour text when NPC is nearby          |

---

## Triggers

Triggers are the core game logic system. Each trigger has an event type,
optional target/direction, conditions, and effects. Triggers are checked
**in order** — the first matching trigger wins.

```yaml
triggers:
  - id: door_locked
    event: go
    direction: north
    conditions:
      - !in_room hallway
      - !flag_unset has_key
    effects:
      - !print "The door is locked."

  - id: use_key_on_door
    event: use
    target: key
    conditions:
      - !in_room hallway
    effects:
      - !set_flag has_key
      - !print "You unlock the door."
```

### Event Types

| Event     | Target        | Direction | When it fires                    |
|-----------|---------------|-----------|----------------------------------|
| `go`      | —             | yes       | Player tries to move              |
| `wait`    | —             | —         | Player uses WAIT command          |
| `use`     | item keyword  | —         | Player uses USE <item>            |
| `attack`  | target keyword| —         | Player uses ATTACK <target>       |
| `talk_to` | NPC keyword   | —         | Player uses TALK TO <target>      |
| `pickup`  | item ID       | —         | After player successfully TAKEs   |

### Conditions

Conditions use YAML tag syntax. All conditions must be true for the trigger
to fire.

| Condition             | Meaning                               |
|-----------------------|---------------------------------------|
| `!in_room room_id`   | Player is in the specified room       |
| `!has_item item_id`  | Player has the item in inventory      |
| `!flag_set flag`     | A game flag is set                    |
| `!flag_unset flag`   | A game flag is NOT set                |

### Effects

| Effect                            | Description                              |
|-----------------------------------|------------------------------------------|
| `!print "text"`                   | Display text to the player               |
| `!set_flag flag_name`             | Set a game flag                          |
| `!bell ~`                         | Play terminal bell sound                 |
| `!add_item {room: r, item: i}`   | Place an item in a room                  |
| `!open_exit {room, direction, destination}` | Add a new exit to a room     |
| `!remove_npc npc_id`             | Remove an NPC from the game              |
| `!game_over {message, hint}`     | End the game (death)                     |
| `!win "text"`                    | End the game (victory)                   |
| `!start_riddle riddle_id`        | Begin a riddle game sequence             |

**Trigger order matters!** More specific triggers (with more conditions)
should come before less specific ones. For example, `use key` with
`in_room: vault` should appear before a generic `use key` trigger.

---

## Riddles

Riddle games are multi-turn interactions where the player must answer
a series of questions correctly.

```yaml
riddles:
  sphinx_riddle:
    npc: sphinx
    intro: >-
      The sphinx blocks your path.
      {dialogue:"Answer my riddles three, or be devoured!"}
    questions:
      - question: |
          "What walks on four legs in the morning,
            two at noon, and three in the evening?"
        answer: man
      - question: |
          "What can you catch but not throw?"
        answer: cold
    correct_response: >-
      {dialogue:"Correct... but can you answer THIS?"}
    win_text: >-
      The sphinx steps aside, defeated.
    win_effects:
      - !set_flag sphinx_defeated
      - !open_exit
        room: temple_entrance
        direction: east
        destination: inner_temple
      - !remove_npc sphinx
    lose_text: >-
      {dialogue:"WRONG!"} The sphinx pounces.
    lose_hint: "You have been eaten by the sphinx."
```

A riddle is triggered via `!start_riddle riddle_id` in a trigger's effects.
A typical pattern is a `talk_to` trigger that starts the riddle when
talking to the NPC.

---

## Text Templates

Use these templates in any text field (intro, descriptions, trigger
print text, riddle text):

| Template           | Renders as              |
|--------------------|-------------------------|
| `{bold:text}`      | **Bold** text            |
| `{npc:text}`       | NPC-coloured text        |
| `{item:text}`      | Item-coloured text       |
| `{dialogue:text}`  | Dialogue-coloured text   |
| `{riddle:text}`    | Riddle-coloured text     |
| `{exits:text}`     | Exit-coloured text       |
| `{danger:text}`    | Danger-coloured text     |
| `{bell}`           | Terminal bell sound      |

---

## Room Art

Rooms can display ANSI art when the player enters or LOOKs. Two options:

1. **External file:** Set `art: path/to/file.ans` on the room. The engine
   loads the file at display time.
2. **Built-in art:** The engine has a few built-in illustrations in `src/art.rs`
   that are used as a fallback when no `art` file is specified.

Art files are plain text containing ANSI escape codes (e.g. generated by
`chafa`, or the future `img2ans` tool from Iteration 6).

---

## Validation

The engine validates your YAML on load and reports:

**Errors** (prevent the game from starting):
- Exit destinations pointing to non-existent rooms
- Room items referencing non-existent item IDs
- NPC start rooms that don't exist
- Trigger effects referencing non-existent rooms/items/NPCs
- Riddle start references to non-existent riddle IDs
- Riddles with no questions

**Warnings** (printed but game still loads):
- NPC allowed rooms that don't exist
- Art files that can't be found on disk
- NPCs with no dialogue lines
- Unknown trigger event types
- Items defined but never placed anywhere

---

## Tips

- Use `>-` for flowing paragraphs (newlines become spaces, no trailing newline).
- Use `|` for text where newlines matter (poetry, formatted text).
- Use `|4` when content has leading spaces (like ASCII art banners).
- Flags are arbitrary strings — use descriptive names like `door_unlocked`.
- Test your game by running `cargo run -- <your-game-dir>` — validation errors
  appear immediately.
- See `examples/hobitty/` and `examples/locked_tower/` for working examples.
