//! Trigger evaluation engine.
//!
//! Checks triggers against the current game state and executes effects.
//! Also handles riddle game logic and text template processing.

use crate::color;
use crate::game_state::{
    Condition, Direction, Effect, GameState, PendingRiddle,
};

// ── Text template processing ───────────────────────────────────────

/// Process template markers in text:
///   {npc:text}      → color::npc(text)
///   {item:text}     → color::item(text)
///   {dialogue:text} → color::dialogue(text)
///   {riddle:text}   → color::riddle(text)
///   {exits:text}    → color::exits(text)
///   {danger:text}   → color::danger(text)
///   {bell}          → terminal bell character
pub fn process_template(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut tag = String::new();
            let mut found_close = false;
            while let Some(&c) = chars.peek() {
                if c == '}' {
                    chars.next();
                    found_close = true;
                    break;
                }
                tag.push(c);
                chars.next();
            }

            if !found_close {
                // No closing brace — output literally
                result.push('{');
                result.push_str(&tag);
                continue;
            }

            if let Some((typ, content)) = tag.split_once(':') {
                match typ {
                    "bold" => {
                        result.push_str(color::BOLD);
                        result.push_str(content);
                        result.push_str(color::RESET);
                    }
                    "npc" => result.push_str(&color::npc(content)),
                    "item" => result.push_str(&color::item(content)),
                    "dialogue" => result.push_str(&color::dialogue(content)),
                    "riddle" => result.push_str(&color::riddle(content)),
                    "exits" => result.push_str(&color::exits(content)),
                    "danger" => result.push_str(&color::danger(content)),
                    _ => {
                        result.push('{');
                        result.push_str(&tag);
                        result.push('}');
                    }
                }
            } else if tag == "bell" {
                result.push_str(color::BELL);
            } else {
                result.push('{');
                result.push_str(&tag);
                result.push('}');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

// ── Condition evaluation ───────────────────────────────────────────

/// Check whether all conditions are met.
pub fn conditions_met(conditions: &[Condition], state: &GameState) -> bool {
    conditions.iter().all(|c| match c {
        Condition::InRoom(room) => state.current_room == *room,
        Condition::HasItem(item) => state.inventory.iter().any(|&id| id == *item),
        Condition::FlagSet(flag) => state.flags.contains(*flag),
        Condition::FlagUnset(flag) => !state.flags.contains(*flag),
    })
}

// ── Effect execution ───────────────────────────────────────────────

/// Execute a list of effects, returning the output text.
/// Some effects modify state, some produce text, some both.
/// Returns (output_text, should_stop) — should_stop is true for
/// game_over/win effects where we don't want further processing.
pub fn execute_effects(effects: &[Effect], state: &mut GameState) -> String {
    let mut output = String::new();

    for effect in effects {
        match effect {
            Effect::Print(text) => {
                let processed = process_template(text);
                if !output.is_empty() && !processed.is_empty() {
                    // Only add separator if both sides have content
                }
                output.push_str(&processed);
            }
            Effect::SetFlag(flag) => {
                state.flags.insert(flag);
            }
            Effect::Bell => {
                output.push_str(color::BELL);
            }
            Effect::AddItem { room, item } => {
                if let Some(r) = state.rooms.get_mut(room) {
                    r.items.push(item);
                }
            }
            Effect::OpenExit { room, direction, destination } => {
                if let Some(dir) = Direction::from_str(direction) {
                    if let Some(r) = state.rooms.get_mut(room) {
                        r.exits.insert(dir, destination);
                    }
                }
            }
            Effect::RemoveNpc(npc_id) => {
                if let Some(npc) = state.npcs.get_mut(npc_id) {
                    npc.current_room = leak_str("nowhere");
                }
            }
            Effect::GameOver { message, hint } => {
                state.running = false;
                let msg = process_template(message);
                let h = process_template(hint);
                output = format!(
                    "{}\n\n{}{}\n\n{}",
                    msg,
                    color::BELL,
                    color::danger("=== GAME OVER ==="),
                    h,
                );
            }
            Effect::Win(text) => {
                state.running = false;
                let processed = process_template(text);
                output = format!(
                    "{}\n{bell}{success}\n\
                     ================================================================\n\
                     {pad:>20}CONGRATULATIONS!\n\
                     \n\
                     {pad:>4}You have completed HOBITTY: An 80s Text Adventure!\n\
                     \n\
                     {pad:>4}Dobo Daggins, burglar extraordinaire, has reclaimed\n\
                     {pad:>4}the treasure of the Lonely Mountain.\n\
                     \n\
                     {pad:>4}Thank you for playing!\n\
                     ================================================================\n\
                     {reset}",
                    processed,
                    bell = color::BELL,
                    success = color::BRIGHT_YELLOW,
                    reset = color::RESET,
                    pad = "",
                );
            }
            Effect::StartRiddle(_) => {
                // Handled specially by the caller — see start_riddle()
            }
        }
    }

    output
}

// ── Trigger matching ───────────────────────────────────────────────

/// Check for a matching trigger by event type.
/// Returns the index of the first matching trigger, if any.
fn find_trigger(
    event: &str,
    target: Option<&str>,
    direction: Option<&str>,
    state: &GameState,
) -> Option<usize> {
    state.triggers.iter().position(|t| {
        if t.event != event {
            return false;
        }
        // Match target (substring match, like the original code)
        if let Some(trigger_target) = t.target {
            match target {
                Some(player_target) => {
                    if !player_target.contains(trigger_target) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        // Match direction
        if let Some(trigger_dir) = t.direction {
            match direction {
                Some(player_dir) => {
                    if trigger_dir != player_dir {
                        return false;
                    }
                }
                None => return false,
            }
        }
        // Check conditions
        conditions_met(&t.conditions, state)
    })
}

// ── Public API ─────────────────────────────────────────────────────

/// Check for a "go" trigger (blocked exit).
/// Returns Some(message) if movement should be blocked.
pub fn check_go(direction: &str, state: &GameState) -> Option<String> {
    let idx = find_trigger("go", None, Some(direction), state)?;
    let effects = &state.triggers[idx].effects;
    // Go triggers only produce print effects (no state changes)
    let mut output = String::new();
    for e in effects {
        if let Effect::Print(text) = e {
            output.push_str(&process_template(text));
        }
    }
    Some(output)
}

/// Check for a "wait" trigger.
/// Returns Some(message) if a trigger fired.
pub fn check_wait(state: &mut GameState) -> Option<String> {
    let idx = find_trigger("wait", None, None, state)?;
    let effects = state.triggers[idx].effects.clone();
    Some(execute_effects(&effects, state))
}

/// Check for a "use" trigger.
/// Returns Some(message) if a trigger handled the use command.
pub fn check_use(target: &str, state: &mut GameState) -> Option<String> {
    let idx = find_trigger("use", Some(target), None, state)?;
    let effects = state.triggers[idx].effects.clone();
    Some(execute_effects(&effects, state))
}

/// Check for an "attack" trigger.
/// Returns Some(message) if a trigger handled the attack.
pub fn check_attack(target: &str, state: &mut GameState) -> Option<String> {
    let idx = find_trigger("attack", Some(target), None, state)?;
    let effects = state.triggers[idx].effects.clone();
    Some(execute_effects(&effects, state))
}

/// Check for a "talk_to" trigger.
/// Returns Some(message) if a trigger handled the talk.
/// May return a StartRiddle effect that the caller must handle.
pub fn check_talk(target: &str, state: &mut GameState) -> Option<String> {
    let idx = find_trigger("talk_to", Some(target), None, state)?;
    let effects = &state.triggers[idx].effects;

    // Check for StartRiddle effect
    for e in effects {
        if let Effect::StartRiddle(riddle_id) = e {
            return Some(start_riddle(riddle_id, state));
        }
    }

    let effects = effects.to_vec();
    Some(execute_effects(&effects, state))
}

/// Check for a "pickup" trigger (extra effects after taking an item).
/// Returns additional output text, if any.
pub fn check_pickup(item_id: &str, state: &GameState) -> Option<String> {
    let idx = find_trigger("pickup", Some(&item_id), None, state)?;
    let effects = &state.triggers[idx].effects;
    let mut output = String::new();
    for e in effects {
        if let Effect::Bell = e {
            output.push_str(color::BELL);
        }
        if let Effect::Print(text) = e {
            output.push_str(&process_template(text));
        }
    }
    if output.is_empty() { None } else { Some(output) }
}

// ── Riddle game ────────────────────────────────────────────────────

/// Start a riddle game. Sets up pending_riddle state and returns intro text.
fn start_riddle(riddle_id: &'static str, state: &mut GameState) -> String {
    let riddle = match state.riddles.get(riddle_id) {
        Some(r) => r.clone(),
        None => return format!("(Error: riddle '{}' not found)", riddle_id),
    };

    state.pending_riddle = Some(PendingRiddle {
        riddle_id,
        question_index: 0,
    });

    let npc_name = state.npcs.get(riddle.npc)
        .map(|n| n.name)
        .unwrap_or(riddle.npc);

    format!(
        "{}\n\n{} asks:\n{}",
        process_template(riddle.intro),
        color::npc(npc_name),
        color::riddle(riddle.questions[0].question.trim()),
    )
}

/// Handle a riddle answer. Called from the main loop when a riddle is pending.
pub fn handle_riddle_answer(input: &str, state: &mut GameState) -> String {
    let pending = match &state.pending_riddle {
        Some(p) => p.clone(),
        None => return String::new(),
    };

    let answer = input.trim().to_lowercase();

    // Allow quitting during riddle game
    if answer == "quit" || answer == "q" {
        state.running = false;
        state.pending_riddle = None;
        return "Farewell, adventurer. Until next time!".to_string();
    }

    let riddle = match state.riddles.get(pending.riddle_id) {
        Some(r) => r.clone(),
        None => {
            state.pending_riddle = None;
            return "(Error: riddle definition missing)".to_string();
        }
    };

    let correct = answer.contains(riddle.questions[pending.question_index].answer);

    if correct {
        if pending.question_index + 1 < riddle.questions.len() {
            // Next riddle
            state.pending_riddle = Some(PendingRiddle {
                riddle_id: pending.riddle_id,
                question_index: pending.question_index + 1,
            });

            let npc_name = state.npcs.get(riddle.npc)
                .map(|n| n.name)
                .unwrap_or(riddle.npc);

            format!(
                "{}\n\n{} asks:\n{}",
                process_template(riddle.correct_response),
                color::npc(npc_name),
                color::riddle(riddle.questions[pending.question_index + 1].question.trim()),
            )
        } else {
            // Won all riddles!
            state.pending_riddle = None;
            let win_text = process_template(riddle.win_text);
            let effects_output = execute_effects(&riddle.win_effects, state);
            if effects_output.is_empty() {
                win_text
            } else {
                format!("{}\n\n{}", win_text, effects_output)
            }
        }
    } else {
        // Wrong answer — death
        state.running = false;
        state.pending_riddle = None;
        let lose_text = process_template(riddle.lose_text);
        let hint = process_template(riddle.lose_hint);
        format!(
            "{}\n\n{}{}\n\n{}",
            lose_text,
            color::BELL,
            color::danger("=== GAME OVER ==="),
            hint,
        )
    }
}

/// Get room description considering conditional overrides.
pub fn get_room_desc<'a>(state: &'a GameState) -> &'a str {
    let room = state.current_room();
    for cd in &room.conditional_descriptions {
        if conditions_met(&cd.conditions, state) {
            return cd.description;
        }
    }
    if room.items.is_empty() {
        room.description_when_empty.unwrap_or(room.description)
    } else {
        room.description
    }
}

/// Get short room description considering conditional overrides.
pub fn get_room_short_desc<'a>(state: &'a GameState) -> &'a str {
    let room = state.current_room();
    for cd in &room.conditional_descriptions {
        if conditions_met(&cd.conditions, state) {
            if let Some(short) = cd.short_description {
                return short;
            }
        }
    }
    room.short_description
}

/// Replay structural effects (open_exit, remove_npc) for triggers
/// whose conditions are currently met. Used when loading a save.
pub fn replay_structural_effects(state: &mut GameState) {
    let triggers = state.triggers.clone();
    for trigger in &triggers {
        if conditions_met(&trigger.conditions, state) {
            for effect in &trigger.effects {
                match effect {
                    Effect::OpenExit { room, direction, destination } => {
                        if let Some(dir) = Direction::from_str(direction) {
                            if let Some(r) = state.rooms.get_mut(room) {
                                r.exits.insert(dir, destination);
                            }
                        }
                    }
                    // NPC positions are restored from save data directly
                    _ => {}
                }
            }
        }
    }
}

// ── Internal helpers ───────────────────────────────────────────────

/// Leak a string to get &'static str (for "nowhere" and similar runtime strings).
fn leak_str(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}
