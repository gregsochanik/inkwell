# QA Test Results — Hobitty

---

## Iteration 1 — ALL FIXED

All 5 bugs and 9 feature improvements from Iteration 1 have been resolved.
See git history for details.

---

## Iteration 2 — NPCs & Dialogue

Tested: NPC display in rooms, TALK TO command, EXAMINE on NPCs, dialogue
cycling, idle behaviour, NPC movement AI, static NPCs, edge cases, aliases,
interaction with existing item/inventory features.

### Bugs

#### BUG-6: `take <npc>` says "I don't know what X is" instead of recognising the NPC

**Steps to reproduce:**
```
> take gandalf
I don't know what 'gandalf' is.

> take gollum
I don't know what 'gollum' is.
```

**Expected:** "You can't pick up Gandalf the Grey!" or similar — the game should
recognise the NPC is present but not takeable.

**Severity:** Medium — players will naturally try this and the response breaks
immersion by pretending the NPC doesn't exist.

---

#### BUG-7: `drop <npc>` says "You don't have X" — no NPC awareness

**Steps to reproduce:**
```
> drop gandalf
You don't have 'gandalf'.
```

**Expected:** A more flavourful response like "Gandalf isn't yours to drop!"
or at least an acknowledgement the name refers to a person, not an item.

**Severity:** Low — less likely than TAKE but still feels off.

---

#### BUG-8: `examine <npc>` when NPC is in another room gives generic item-missing message

**Steps to reproduce:**
```
(from Bag End, Thorin is on The Hill)
> examine thorin
You don't see 'thorin' here.
```

Compare with TALK TO which correctly says:
```
> talk to thorin
You don't see 'thorin' here right now.
```

**Expected:** EXAMINE should match TALK TO's behaviour — distinguish between
"NPC exists but isn't here" vs "no such thing". Currently EXAMINE falls through
to the item-not-found path because it only checks NPCs in the current room.

**Severity:** Low — cosmetic inconsistency.

---

### Feature Improvements

#### IMP-10: TAKE/DROP should check for NPCs and give flavour responses

When a player tries to TAKE or DROP a name that matches an NPC present in the
room, give a fun response instead of the item-not-found message.

e.g. `take gandalf` → "Gandalf raises an eyebrow. You can't pick up a wizard!"

---

#### IMP-11: EXAMINE should distinguish NPC-not-here from unknown-thing

When examining something that matches an NPC who exists but isn't in the room,
say "X isn't here right now" rather than the generic "You don't see X here."

---

#### IMP-12: NPC name aliases / keywords not supported

Players will naturally try names like "wizard", "dwarf", "creature", "old man"
to refer to NPCs. Currently only the NPC id (e.g. "gandalf") and full name
(e.g. "gandalf the grey") match.

**Suggestion:** Add a `keywords` field to the Npc struct containing aliases
like `["wizard", "old man"]` for Gandalf, `["dwarf", "king"]` for Thorin,
`["elf", "elf-lord"]` for Elrond, `["creature", "smeagol"]` for Gollum.

**Severity:** Medium — very natural player input that currently fails silently.

---

#### IMP-13: Idle text frequency varies by NPC due to pseudo-random seed

Gollum's idle text appeared only ~1 in 10 turns during testing, while Gandalf
and Thorin were more frequent. The LCG-based pseudo-random uses the NPC's
index as a seed, so different NPCs get different frequency distributions.

**Suggestion:** Either tune the RNG to be more uniform or use a different
seed strategy (e.g. hash of NPC id).

**Severity:** Very low — cosmetic, and some variability feels natural.

---

#### IMP-14: No newline before NPC idle/movement text

NPC idle text and arrival/departure messages appear immediately after the
command output without a blank line separator. This can make them visually
run together with the command response.

**Example:**
```
> Gandalf the Grey says: "Good morning! Or is it?..."

Gandalf puffs on his pipe, blowing a smoke ring shaped like a ship.
```

The blank line before the idle text is present here (good), but after
a LOOK command:

```
Exits: east

Gandalf taps his staff on the ground thoughtfully.
```

No blank line between "Exits: east" and the idle text.

**Severity:** Very low — cosmetic.

---

### What Works Well

- All 4 NPCs (Gandalf, Thorin, Elrond, Gollum) display correctly in rooms
- LOOK (full) and GLANCE (revisit) both show NPCs
- Multiple NPCs in the same room display correctly
- TALK TO cycles through all 5 dialogue lines per NPC and wraps correctly
- TALK TO aliases work: `speak to`, `chat with`, `talk <name>`
- Article stripping works: `talk to the gandalf`, `examine the gandalf`
- EXAMINE works on NPCs present in the room
- NPC idle text fires at reasonable intervals with varied lines
- NPC movement: Thorin and Gandalf wander between rooms with departure
  ("X wanders off toward Y") and arrival ("X arrives.") messages
- Static NPCs: Elrond never leaves Rivendell, Gollum never leaves Goblin Cave
  (tested 10+ turns each)
- Existing item features (take, drop, inventory, examine items) all still work
  correctly alongside NPCs
- HELP text updated with TALK TO command
- Edge cases handled: bare `talk to`, `talk`, nonexistent NPC name

---

---

## Iteration 3 — Puzzles & Win Condition

Tested: Troll encounter puzzle (alive/stone descriptions, north blocked, WAIT
solve, ATTACK death, key reveal, revisit), riddle game with Gollum (start,
all 3 riddles correct, wrong answer death, QUIT during riddle, post-win state,
Gollum removal), USE command (all 5 items with/without inventory, context at
Lonely Mountain), win condition (map+key=win, key without map, map without key),
death states (attack trolls, attack Gollum, wrong riddle), new rooms (Beorn's
Hall, Lake-town, Lonely Mountain — descriptions, exits, navigation back/forth),
new command aliases (WAIT/Z, ATTACK/FIGHT/KILL/HIT/STAB, RIDDLE, PLAY RIDDLE,
UNLOCK), HELP text, integration with existing items/NPCs/navigation.

### Bugs

#### BUG-9: `examine trolls` says "You don't see 'trolls' here" even though trolls are in the room

**Steps to reproduce:**
```
(in Troll Clearing, trolls alive)
> examine trolls
You don't see 'trolls' here.

> examine troll
You don't see 'troll' here.

(after trolls turned to stone)
> examine trolls
You don't see 'trolls' here.
```

**Expected:** A description of the trolls — alive ("Three enormous trolls
argue around a fire...") or stone ("Three stone shapes, frozen mid-argument").
Trolls are described in the room text but can't be examined because they
aren't items or NPCs.

**Severity:** Medium — the trolls are the central feature of the room and
players will naturally try to examine them.

---

#### BUG-10: Generic ATTACK response has hardcoded "the" before target name

**Code:**
```rust
format!("You wave your sword at the {}. Nothing much happens.", target)
```

For `attack gandalf` this produces: "You wave your sword at the gandalf."
The hardcoded "the" is grammatically wrong for proper nouns.

**Expected:** Either drop "the" or use context-aware grammar.

**Severity:** Low — only triggers for non-lethal attacks with a sword.

---

#### BUG-11: Goblin Cave description still mentions "Strange eyes" after Gollum leaves

**Steps to reproduce:**
```
(in Goblin Cave, after winning riddle game — Gollum has left)
> take ring
> look
A damp, dark cave... Strange eyes glint in the darkness.
The floor is bare rock, slick with moisture.
```

**Expected:** After Gollum leaves, the "Strange eyes" reference should be
removed. The `description_when_empty` text still contains this phrase even
though it refers to Gollum.

**Severity:** Low — cosmetic inconsistency, though a sharp player will notice.

---

### Feature Improvements

#### IMP-15: Room scenery cannot be examined

Players can only examine items and NPCs. Room elements described in the text
(trolls, fire, stone shapes, mountain, waterfalls, bees, etc.) return
"You don't see X here."

**Suggestion:** Add a `scenery` HashMap to Room containing examinable keywords
and their descriptions. e.g. `"troll" → "Three stone shapes, frozen forever
mid-argument."`, `"mountain" → "The great peak of Erebor towers above you."`

**Severity:** Medium — natural player behaviour that currently breaks immersion.

---

#### IMP-16: Non-lethal ATTACK on NPCs gives generic response

Attacking an NPC who isn't a puzzle target (e.g. Gandalf, Thorin, Elrond)
gives a bland "You wave your sword" or "no weapon" response. These should
be NPC-specific.

**Suggestion:** e.g. `attack gandalf` → "Gandalf gives you a withering look.
'I would not advise that, my dear hobbit.'"

**Severity:** Low — would add flavour.

---

#### IMP-17: New rooms (Beorn's Hall, Lake-town, Lonely Mountain) have no items or NPCs

These rooms are atmospheric but lack interactive content. Adding Beorn as
an NPC in his hall or scenery items would improve the second half of the game.

**Severity:** Low — the rooms work and have good descriptions, this is polish.

---

#### IMP-18: NPC idle text fires during active troll encounter

If a wandering NPC (Gandalf, Thorin) is in the Troll Clearing while the
trolls are alive, their idle text fires normally. This feels immersion-
breaking — Gandalf casually puffing his pipe while trolls threaten you.

**Suggestion:** Suppress NPC idle/movement in rooms with active encounters,
or have NPCs react to the trolls instead.

**Severity:** Very low — unlikely to occur and somewhat amusing if it does.

---

### What Works Well

- **Troll puzzle**: Alive description on first visit, north correctly blocked,
  WAIT solves with dramatic narrative, key appears, revisit shows stone trolls,
  description transitions cleanly between alive/stone states
- **Riddle game**: All 3 riddles work with exact and partial answers ("a mountain",
  "the teeth"), wrong answer = death with hint, QUIT during riddle works,
  Gollum removed after win, east exit dynamically added, re-RIDDLE after
  win handled ("nobody here to play riddles with")
- **USE command**: All 5 items have flavour text, context-sensitive at Lonely
  Mountain (map hints at door, key hints at needing map), items without
  inventory correctly rejected, unknown items get generic response
- **Win condition**: Map+key = full win sequence, key without map = hint,
  USE MAP at mountain = hint about needing key — all paths covered
- **Death states**: Attack trolls, attack Gollum, wrong riddle answer all
  give themed GAME OVER with hints about correct approach
- **New rooms**: Beorn's Hall, Lake-town, Lonely Mountain all have atmospheric
  descriptions, correct exits, proper first-visit/revisit behaviour
- **Command aliases**: WAIT/Z, ATTACK/FIGHT/KILL/HIT/STAB, RIDDLE, PLAY RIDDLE,
  PLAY RIDDLES, UNLOCK all work correctly
- **Edge cases**: Double WAIT (second gives "Time passes..."), RIDDLE with no
  Gollum, PLAY CARDS rejected, bare ATTACK gives "Attack what?", NPC ticks
  suppressed during riddle game
- **Integration**: All existing features (items, NPCs, navigation, inventory,
  examine, talk to) work correctly alongside new puzzle mechanics
- **HELP text**: Updated with all new commands

---

## Summary

| Iteration | Category | Found | Fixed |
|-----------|----------|-------|-------|
| 1 | Bugs | 5 | 5 |
| 1 | Improvements | 9 | 9 |
| 2 | Bugs | 3 | 0 |
| 2 | Improvements | 5 | 0 |
| 3 | Bugs | 3 | 0 |
| 3 | Improvements | 4 | 0 |

**Top priorities for next fix pass:**
1. BUG-9 / IMP-15 — Scenery examination (trolls, mountain, etc.)
2. BUG-6 / IMP-10 — TAKE/DROP should recognise NPCs (from Iteration 2)
3. BUG-10 — Fix "the" grammar in generic attack response
4. BUG-11 — Remove "Strange eyes" from Goblin Cave post-Gollum description
5. IMP-12 — NPC keyword aliases (from Iteration 2)
