# QA Test Results — Hobitty

## Iteration 1 — ALL FIXED (see git history)

All 5 bugs and 9 improvements from iteration 1 were resolved.

---

## Iteration 2 — NPCs & Dialogue

Tested: NPC room presence, TALK TO, EXAMINE NPCs, idle text, NPC movement,
edge cases, interaction with existing item/inventory features.

---

### Bugs

#### BUG-6: `take <npc>` says "I don't know what X is" instead of recognising the NPC

**Steps to reproduce:**
```
> take gandalf
I don't know what 'gandalf' is.

> take gollum
I don't know what 'gollum' is.
```

**Expected:** Should say something like "You can't pick up Gandalf the Grey!"
The TAKE command only checks items, not NPCs, so NPC names fall through to
the "doesn't exist in world" error path.

**Severity:** Medium — very natural thing to try, confusing response.

---

#### BUG-7: `drop <npc>` says "You don't have X" — no NPC awareness

**Steps to reproduce:**
```
> drop gandalf
You don't have 'gandalf'.
```

**Expected:** Could say "You can't drop a person!" or similar.

**Severity:** Low — less common than TAKE, but still confusing.

---

#### BUG-8: `examine <npc>` gives no "not here" distinction when NPC exists elsewhere

**Steps to reproduce:**
```
(from Bag End, Thorin is on The Hill)
> examine thorin
You don't see 'thorin' here.
```

**Expected:** Should say "Thorin Oakenshield isn't here right now." to match
the TALK TO command's behaviour which correctly distinguishes this case.

**Severity:** Low — inconsistency between EXAMINE and TALK TO error messages.

---

#### BUG-9: NPC idle text output missing trailing newline

**Steps to reproduce:**
NPC idle text is printed via `print!` (no newline) in main.rs, while all
other game output uses `println!`. When idle text fires on the same turn as
the next prompt, the `>` prompt can appear on the same line as the idle text.

Not consistently visible in piped testing since output buffers differently,
but in interactive play the prompt could concatenate with idle text.

**Severity:** Low — cosmetic.

---

### Feature Improvements

#### IMP-10: TAKE/DROP should recognise NPCs and give flavour responses

TAKE should check if the target is an NPC in the room before falling through
to the item-not-found path. Same for DROP checking if the name matches a
known NPC.

e.g. `take gandalf` → "Gandalf the Grey raises an eyebrow. You can't pick up a wizard!"

#### IMP-11: EXAMINE should distinguish "NPC not here" vs "nothing called that"

Like TALK TO already does, EXAMINE should check if the target matches a
known NPC who is elsewhere and give a more helpful message.

#### IMP-12: No keyword/alias support for NPC names

Players might type `talk to wizard`, `examine the dwarf`, `talk to creature`,
or `examine old man`. None of these work because NPC matching only checks the
`id` and `name` fields. Adding a `keywords` field to Npc (e.g. `["wizard",
"old man"]` for Gandalf) would make this much more natural.

#### IMP-13: Idle text frequency feels inconsistent across NPCs

Gollum seemed to fire idle text much less frequently than Gandalf or Elrond
over the same number of turns. This is because the pseudo-random function
uses the NPC's index as a seed, and certain index/turn combinations produce
less variety. Consider using a different hash mixing strategy or simply
tracking idle cooldowns per NPC.

#### IMP-14: No way to see who is in the room without using LOOK

If an NPC arrives or departs, you see a message. But there's no short command
like `WHO` to see who is present without re-displaying the full room. Minor,
since LOOK works, but a nice quality-of-life addition.

#### IMP-15: Multiple NPCs in same room don't interact with each other

When Gandalf and Thorin are both on The Hill, they don't acknowledge each
other. A future improvement could add NPC-to-NPC interaction flavour text
(e.g. "Gandalf and Thorin argue about the best route through the mountains.").

---

### Summary

| Category | Count |
|----------|-------|
| Bugs     | 4     |
| Feature improvements | 6 |

**Top 3 priorities for next fix pass:**
1. BUG-6 — TAKE should recognise NPCs and refuse nicely
2. IMP-12 — NPC keyword aliases for natural phrasing
3. BUG-8 — EXAMINE NPC consistency with TALK TO error messages

**What works well:**
- All 4 NPCs appear correctly in room descriptions (both full and short)
- TALK TO works with all aliases (speak, chat, with/to prepositions)
- Dialogue cycles correctly through all 5 lines and wraps
- EXAMINE works on all NPCs with partial name matching (gandalf, grey, elrond, gollum)
- Article stripping works with NPC names ("examine the gandalf")
- NPC idle behaviour fires with varied flavour text
- Thorin/Gandalf roam between adjacent allowed rooms, with arrival/departure messages
- Elrond stays fixed in Rivendell, Gollum stays fixed in Goblin Cave
- Multiple NPCs can co-exist in the same room
- All existing item/inventory/navigation features continue to work correctly
- HELP text includes the new TALK TO command
