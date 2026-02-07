use crate::color;
use crate::game_state::{GameState, NpcId, RoomId};

/// Simple pseudo-random number based on turn and a seed value.
/// We avoid external crates — this is a basic LCG hash.
fn pseudo_random(turn: u32, seed: u32) -> u32 {
    let n = turn.wrapping_mul(2654435761).wrapping_add(seed);
    n ^ (n >> 16)
}

/// Run one tick of NPC behaviour. Returns text to display to the player
/// (only events that happen in the player's current room).
pub fn tick(state: &mut GameState) -> String {
    let mut output = String::new();
    let player_room = state.current_room;
    let turn = state.turn;

    // Collect NPC ids so we can mutate state in the loop
    let npc_ids: Vec<NpcId> = state.npcs.keys().copied().collect();

    for (idx, npc_id) in npc_ids.iter().enumerate() {
        let seed = idx as u32;
        let rng = pseudo_random(turn, seed);

        // --- Movement (roughly 1 in 4 chance per turn) ---
        let should_move = (rng % 4) == 0;
        let npc_room_before: RoomId;
        let npc_name: &str;
        let allowed: Vec<RoomId>;

        {
            let npc = state.npcs.get(*npc_id).expect("npc must exist");
            npc_room_before = npc.current_room;
            npc_name = npc.name;
            allowed = npc.allowed_rooms.clone();
        }

        if should_move && allowed.len() > 1 {
            // Pick a destination from allowed rooms (not current room)
            let destinations: Vec<RoomId> = allowed
                .iter()
                .filter(|&&r| r != npc_room_before)
                .copied()
                .collect();

            if !destinations.is_empty() {
                let dest_idx = (rng / 4) as usize % destinations.len();
                let destination = destinations[dest_idx];

                // Only move if there's actually an exit path (adjacent room)
                let is_adjacent = state
                    .rooms
                    .get(npc_room_before)
                    .map(|r| r.exits.values().any(|&exit| exit == destination))
                    .unwrap_or(false);

                if is_adjacent {
                    // Move the NPC
                    state.npcs.get_mut(*npc_id).expect("npc must exist").current_room = destination;

                    // Report if player can see the departure or arrival
                    if npc_room_before == player_room {
                        let dest_name = state
                            .rooms
                            .get(destination)
                            .map(|r| r.name)
                            .unwrap_or("somewhere");
                        output.push_str(&format!(
                            "\n{} wanders off toward {}.",
                            color::npc(npc_name), dest_name
                        ));
                    } else if destination == player_room {
                        output.push_str(&format!("\n{} arrives.", color::npc(npc_name)));
                    }

                    continue; // Skip idle text if we moved
                }
            }
        }

        // --- Idle text (roughly 1 in 3 chance, only if in player's room) ---
        let npc = state.npcs.get(*npc_id).expect("npc must exist");
        if npc.current_room == player_room && !npc.idle_texts.is_empty() {
            let should_idle = (rng % 3) == 0;
            if should_idle {
                let idle_idx = (rng / 3) as usize % npc.idle_texts.len();
                output.push_str(&format!("\n{}", npc.idle_texts[idle_idx]));
            }
        }
    }

    output
}
