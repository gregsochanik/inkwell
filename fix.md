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

## Summary

| Iteration | Category | Found | Fixed |
|-----------|----------|-------|-------|
| 1 | Bugs | 5 | 5 |
| 1 | Improvements | 9 | 9 |
| 2 | Bugs | 3 | 0 |
| 2 | Improvements | 5 | 0 |

**Top 3 priorities for next fix pass:**
1. BUG-6 / IMP-10 — TAKE/DROP should recognise NPCs and give flavour responses
2. IMP-12 — NPC keyword aliases (wizard, dwarf, creature, etc.)
3. BUG-8 / IMP-11 — EXAMINE NPC-not-here vs unknown-thing consistency
