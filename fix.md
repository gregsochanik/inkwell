# QA Test Results — Hobitty Iteration 1

Tested: all commands, navigation, items, edge cases, aliases.

---

## Bugs — ALL FIXED

### BUG-1: Articles ("a", "the") not stripped — FIXED
Parser now strips leading "a", "an", "the" from noun targets.
`take the map`, `drop a sword`, `examine the ring` all work.

### BUG-2: `look at <item>` ignored the target — FIXED
`look at <item>` now works as an alias for `examine <item>`.

### BUG-3: `examine room` tried to find an item called "room" — FIXED
`examine room`, `examine around`, `examine here` etc. now act as LOOK.

### BUG-4: Exit ordering was non-deterministic — FIXED
Exits now always display in canonical N, S, E, W order.

### BUG-5: Same error message for "not here" and "doesn't exist" — FIXED
TAKE now gives three distinct responses:
- "You're already carrying that!"
- "You don't see X here." (exists but in another room)
- "I don't know what X is." (doesn't exist at all)

---

## Feature Improvements — ALL IMPLEMENTED

### IMP-1: Strip articles from input — DONE (merged with BUG-1)

### IMP-2: Support `look at <item>` as EXAMINE — DONE (merged with BUG-2)

### IMP-3: Support common verb synonyms — DONE
- `read` works as examine alias
- `open`, `use`, `eat`, `drink`, `wear`, `equip`, `put` return flavour messages

### IMP-4: Sort exits in consistent cardinal order — DONE (merged with BUG-4)

### IMP-5: Goblin Cave dynamic description — DONE
Room now shows "The floor is bare rock, slick with moisture" after ring is taken,
instead of "something glimmers faintly". Uses new `description_when_empty` field.

### IMP-6: Inventory capacity limit — DONE
Max 4 items. Shows count in inventory (e.g. "3/4"). "Your pockets are full!"
when trying to pick up a 5th item.

### IMP-7: Removed `leave` as DROP alias — DONE
Was ambiguous with "leave the room". Removed.

### IMP-8: Short description on revisit — DONE
First visit shows full prose. Revisit via navigation shows short description.
Explicit LOOK always shows full description.

### IMP-9: Startup prompt separator — DONE
"What will you do?" displayed after initial room description.

---

## Summary

| Category | Found | Fixed |
|----------|-------|-------|
| Bugs     | 5     | 5     |
| Feature improvements | 9 | 9 |

All issues resolved across 11 commits.
