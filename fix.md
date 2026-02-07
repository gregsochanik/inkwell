# QA Test Results — Hobitty Iteration 1

Tested: all commands, navigation, items, edge cases, aliases.

---

## Bugs

### BUG-1: Articles ("a", "the") not stripped — TAKE/DROP/EXAMINE fail with natural phrasing

**Steps to reproduce:**
```
> take the map
You don't see 'the map' here.

> take a weathered map
You pick up a weathered map.    <-- works because "a weathered" is in the display name
                                    but this is accidental, not intentional
> take the weathered map
You don't see 'the weathered map' here.
```

**Expected:** Parser should strip leading articles ("a", "an", "the") before matching.

**Severity:** Medium — this is one of the most natural things a player will type.

---

### BUG-2: `look at <item>` is parsed as LOOK, ignoring the target

**Steps to reproduce:**
```
> look at map
```
Outputs the full room description (treated as plain `LOOK`). The "at map" part is silently discarded.

**Expected:** `look at <item>` should behave like `examine <item>`, or at minimum say "I don't understand 'look at map'."

**Severity:** Medium — very common natural phrasing.

---

### BUG-3: `examine room` tries to find an item called "room"

**Steps to reproduce:**
```
> examine room
You don't see 'room' here.
```

**Expected:** Could be an alias for LOOK, or at least a friendlier message like "Try LOOK to see the room."

**Severity:** Low.

---

### BUG-4: Exit ordering is non-deterministic

Exits are stored in a `HashMap<Direction, RoomId>`, so the order displayed changes between runs. For example, The Hill sometimes shows "Exits: south, west, east" and other times "Exits: east, south, west".

**Expected:** Exits should display in a consistent, predictable order (e.g. always N, S, E, W).

**Severity:** Low — cosmetic, but noticeable and feels untidy.

---

### BUG-5: `get`, `grab`, `pick up` aliases only work for first word — not after item is already taken

This is actually correct behaviour (can't take what's already taken), but the error message says "You don't see 'map' here" which is the same message as for nonexistent items. It would be better to distinguish between "not here" and "already taken" (though technically the item isn't in the room at all once taken, so this is more of a UX nit).

**Severity:** Very low.

---

## Feature Improvements

### IMP-1: Strip articles from input

The parser should strip leading "a", "an", "the" from noun targets in TAKE, DROP, EXAMINE commands. This is the single highest-impact improvement.

### IMP-2: Support `look at <item>` as EXAMINE

`look at sword` should work the same as `examine sword`. This is extremely natural 80s text adventure phrasing.

### IMP-3: Support common verb synonyms that players will try

The following verbs are not recognised but players will naturally try them:
- `open` (e.g. `open door`)
- `use` (e.g. `use key`)
- `read` (e.g. `read map`) — should behave like EXAMINE
- `eat` (e.g. `eat bread`)
- `wear` / `put on` (e.g. `wear ring`)

For iteration 1, these could return a flavour message like "You can't do that... yet." rather than "I don't understand".

### IMP-4: Sort exits in consistent cardinal order

Display exits in N, S, E, W order. Use a `BTreeMap` or sort the keys before display.

### IMP-5: Goblin Cave description mentions "something glimmers" even after ring is taken

The static room description says "On the floor, something glimmers faintly." This text remains even after the ring has been picked up.

**Suggestion:** Either make descriptions dynamic, or split into a base description and a conditional detail.

### IMP-6: No inventory capacity limit

The player can carry all 5 items at once. Classic text adventures often had a carry limit. Not essential for iteration 1, but worth noting for flavour.

### IMP-7: `leave` as a DROP alias is confusing

`leave` could also mean "leave the room" (i.e. a movement command). This ambiguity may confuse players. Consider removing it or at least supporting `leave room` as a synonym for a directional exit.

### IMP-8: No "you have already visited" or short description on revisit

Classic 80s adventures showed a short description on revisit and the full description only on first visit (or with LOOK). Currently every room entry shows the full description.

### IMP-9: Startup shows room twice effectively

On launch, the banner is followed by the full room description automatically. If the player then types `look`, they see the exact same text again. Consider a small separator or prompt like "What will you do?" between the auto-look and the first prompt.

---

## Summary

| Category | Count |
|----------|-------|
| Bugs     | 5     |
| Feature improvements | 9 |

**Top 3 priorities for next fix pass:**
1. BUG-1 — Strip articles (highest player impact)
2. BUG-2 / IMP-2 — Support `look at <item>`
3. BUG-4 / IMP-4 — Consistent exit ordering

All core features (LOOK, GO, TAKE, DROP, EXAMINE, INVENTORY, HELP, QUIT) work correctly for their happy paths. Navigation is fully two-way and all 8 rooms are reachable and returnable. All 5 items can be picked up, dropped, examined, and carried across rooms. Command aliases (L, I, N/S/E/W, X, ?, Q) all work.
