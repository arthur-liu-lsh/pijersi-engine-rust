use rayon::prelude::*;

use super::{
    actions::play_action,
    lookup::{NEIGHBOURS1, NEIGHBOURS2},
    movegen::available_player_actions,
    rules::{can_move1, can_move2, can_stack, can_unstack, is_action_win},
    translate::action_to_string,
    COLOUR_MASK, MAX_PLAYER_ACTIONS, STACK_THRESHOLD,
};

/// Returns the number of possible actions for a player.
///
/// Is used to speed up perft at depth=1 since it only needs the number of leaf nodes, not the moves.
fn count_player_actions(cells: &[u8; 45], current_player: u8) -> u64 {
    let mut player_action_count: u64 = 0u64;

    // Calculate possible actions
    for index in 0..45 {
        if cells[index] != 0 {
            // Choose pieces of the current player's colour
            if (cells[index] & COLOUR_MASK) == (current_player << 1) {
                player_action_count += count_piece_actions(cells, index);
            }
        }
    }
    player_action_count
}

/// Returns the number of possible actions for a specific piece.
///
/// Is used to speed up perft at depth=1 since it only needs the number of leaf nodes, not the moves.
#[inline]
fn count_piece_actions(cells: &[u8; 45], index_start: usize) -> u64 {
    let mut piece_action_count: u64 = 0u64;

    let piece_start: u8 = cells[index_start];

    // If the piece is not a stack
    if piece_start < STACK_THRESHOLD {
        // 1-range first action
        for &index_mid in NEIGHBOURS1
            .iter()
            .skip(7 * index_start + 1)
            .take(NEIGHBOURS1[7 * index_start])
        {
            // stack, [1/2-range move] optional
            if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in NEIGHBOURS2
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS2[7 * index_mid])
                {
                    if can_move2(cells, piece_start, index_mid, index_end)
                        || (index_start == ((index_mid + index_end) / 2)
                            && can_move1(cells, piece_start, index_end))
                    {
                        piece_action_count += 1;
                    }
                }

                // stack, 0/1-range move
                for &index_end in NEIGHBOURS1
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS1[7 * index_mid])
                {
                    if can_move1(cells, piece_start, index_end) || index_start == index_end {
                        piece_action_count += 1;
                    }
                }

                // stack only
                piece_action_count += 1;
            }
            // 1-range move
            else if can_move1(cells, piece_start, index_mid) {
                piece_action_count += 1;
            }
        }
    } else {
        // 2 range first action
        for &index_mid in NEIGHBOURS2
            .iter()
            .skip(7 * index_start + 1)
            .take(NEIGHBOURS2[7 * index_start])
        {
            if can_move2(cells, piece_start, index_start, index_mid) {
                // 2-range move, stack or unstack
                for &index_end in NEIGHBOURS1
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS1[7 * index_mid])
                {
                    // 2-range move, unstack or 2-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        piece_action_count += 1;
                    }
                }
                // 2-range move;
                piece_action_count += 1;
            }
        }
        // 1-range first action
        for &index_mid in NEIGHBOURS1
            .iter()
            .skip(7 * index_start + 1)
            .take(NEIGHBOURS1[7 * index_start])
        {
            // 1-range move, [stack or unstack] optional
            if can_move1(cells, piece_start, index_mid) {
                // 1-range move, stack or unstack
                for &index_end in NEIGHBOURS1
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS1[7 * index_mid])
                {
                    // 1-range move, unstack or 1-range move, stack
                    if can_unstack(cells, piece_start, index_end)
                        || can_stack(cells, piece_start, index_end)
                    {
                        piece_action_count += 1;
                    }
                }
                // 1-range move, unstack on starting position
                piece_action_count += 1;

                // 1-range move
                piece_action_count += 1;
            }
            // stack, [1/2-range move] optional
            else if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in NEIGHBOURS2
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS2[7 * index_mid])
                {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        piece_action_count += 1;
                    }
                }

                // stack, 1-range move
                for &index_end in NEIGHBOURS1
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS1[7 * index_mid])
                {
                    if can_move1(cells, piece_start, index_end) {
                        piece_action_count += 1;
                    }
                }

                // stack only
                piece_action_count += 1;
            }

            // unstack
            if can_unstack(cells, piece_start, index_mid) {
                // unstack only
                piece_action_count += 1;
            }
        }
    }
    piece_action_count
}

/// Debug function to measure the number of leaf nodes (possible moves) at a given depth.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// At depth 0, returns 1.
pub fn perft(cells: &[u8; 45], current_player: u8, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 | 2 => perft_iter(cells, current_player, depth),
        _ => {
            let available_actions: [u64; 512] = available_player_actions(cells, current_player);
            let n_actions: usize = available_actions[MAX_PLAYER_ACTIONS - 1] as usize;

            available_actions
                .par_iter()
                .take(n_actions)
                .filter(|&&action| !is_action_win(cells, action))
                .map(|&action| {
                    let mut new_cells: [u8; 45] = *cells;
                    play_action(&mut new_cells, action);
                    perft_iter(&new_cells, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}

/// Part of the perft debug function to measure the number of leaf nodes (possible moves) at a given depth. Is called by perft.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// At depth 0, returns 1.
pub fn perft_iter(cells: &[u8; 45], current_player: u8, depth: u64) -> u64 {
    match depth {
        0 => 1u64,
        1 => count_player_actions(cells, current_player),
        _ => {
            let available_actions: [u64; 512] = available_player_actions(cells, current_player);
            let n_actions: usize = available_actions[MAX_PLAYER_ACTIONS - 1] as usize;

            let mut new_cells: [u8; 45] = [0u8; 45];

            available_actions
                .iter()
                .take(n_actions)
                .filter(|&&action| !is_action_win(cells, action))
                .map(|&action| {
                    new_cells = *cells;
                    play_action(&mut new_cells, action);
                    perft_iter(&new_cells, 1 - current_player, depth - 1)
                })
                .sum()
        }
    }
}

/// Split Perft debug function to measure the number of leaf nodes (possible moves) at a given depth.
///
/// Recursively counts the number of leaf nodes at the chosen depth.
///
/// Separates the node count between all possible depth 1 moves and returns a vector of (action_string: String, action: u64, count: u64).
///
/// At depth 0, returns an empty vector.
pub fn perft_split(cells: &[u8; 45], current_player: u8, depth: u64) -> Vec<(String, u64, u64)> {
    match depth {
        0 => vec![],
        _ => {
            let available_actions: [u64; MAX_PLAYER_ACTIONS] =
                available_player_actions(cells, current_player);
            let n_actions: usize = available_actions[MAX_PLAYER_ACTIONS - 1] as usize;

            available_actions
                .par_iter()
                .take(n_actions)
                .filter(|&&action| !is_action_win(cells, action))
                .map(|&action| {
                    let mut new_cells = *cells;
                    play_action(&mut new_cells, action);
                    (
                        action_to_string(cells, action),
                        action,
                        perft_iter(&new_cells, 1 - current_player, depth - 1),
                    )
                })
                .collect()
        }
    }
}
