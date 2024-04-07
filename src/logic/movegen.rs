use super::super::lookup::{NEIGHBOURS1, NEIGHBOURS2};

use super::actions::{copy_cells, play_action};
use super::rules::{can_move1, can_move2, can_stack, can_unstack, is_action_win};
use super::translate::action_to_string;
use super::{COLOUR_MASK, INDEX_NULL, INDEX_WIDTH, MAX_PLAYER_ACTIONS, STACK_THRESHOLD};

/// Concatenate three indices into a u64 action.
/// The first index is stored in the 8 least significant bits.
pub fn concatenate_action(index_start: usize, index_mid: usize, index_end: usize) -> u64 {
    (index_start | (index_mid << INDEX_WIDTH) | (index_end << (2 * INDEX_WIDTH))) as u64
}

/// Concatenate a half action and the last index into a u64 action.
/// The first index is stored in the 8 least significant bits.
pub fn concatenate_half_action(half_action: u64, index_end: usize) -> u64 {
    half_action | (index_end << (2 * INDEX_WIDTH)) as u64
}

/// Returns the possible moves for a player.
/// The result is a size MAX_PLAYER_ACTIONS array of u64 where the last element is the number of actions.
pub fn available_player_actions(current_player: u8, cells: &[u8; 45]) -> [u64; MAX_PLAYER_ACTIONS] {
    let mut player_actions: [u64; MAX_PLAYER_ACTIONS] = [0u64; MAX_PLAYER_ACTIONS];

    // Calculate possible player_actions
    for index in 0..45 {
        if cells[index] != 0 {
            // Choose pieces of the current player's colour
            if (cells[index] & COLOUR_MASK) == (current_player << 1) {
                available_piece_actions(cells, index, &mut player_actions);
            }
        }
    }
    player_actions
}

/// Calculates the possible moves for a player.
/// The result is stored in a size MAX_PLAYER_ACTIONS array of u64 where the last element is the number of actions.
/// This array is passed in parameter and modified by this function.
fn available_piece_actions(
    cells: &[u8; 45],
    index_start: usize,
    player_actions: &mut [u64; MAX_PLAYER_ACTIONS],
) {
    let mut index_actions: usize = player_actions[MAX_PLAYER_ACTIONS - 1] as usize;

    let piece_start: u8 = cells[index_start];

    // If the piece is not a stack
    if piece_start < STACK_THRESHOLD {
        // 1-range first action
        for &index_mid in NEIGHBOURS1.iter().take(7 * index_start + NEIGHBOURS1[7 * index_start] + 1).skip(7 * index_start + 1)
        {
            let half_action: u64 = (index_start | (index_mid << INDEX_WIDTH)) as u64;
            // stack, [1/2-range move] optional
            if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in NEIGHBOURS2.iter().take(7 * index_mid + NEIGHBOURS2[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    if can_move2(cells, piece_start, index_mid, index_end)
                        || (index_start == ((index_mid + index_end) / 2)
                            && can_move1(cells, piece_start, index_end))
                    {
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }

                // stack, 0/1-range move
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    if can_move1(cells, piece_start, index_end) || index_start == index_end {
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }

                // stack only
                player_actions[index_actions] =
                    concatenate_action(index_start, index_start, index_mid);
                index_actions += 1;
            }
            // 1-range move
            else if can_move1(cells, piece_start, index_mid) {
                player_actions[index_actions] =
                    concatenate_action(index_start, 0x000000FF, index_mid);
                index_actions += 1;
            }
        }
    } else {
        // 2 range first action
        for &index_mid in NEIGHBOURS2.iter().take(7 * index_start + NEIGHBOURS2[7 * index_start] + 1).skip(7 * index_start + 1)
        {
            let half_action: u64 = (index_start | (index_mid << INDEX_WIDTH)) as u64;
            if can_move2(cells, piece_start, index_start, index_mid) {
                // 2-range move, stack or unstack
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    // 2-range move, unstack or 2-range move, stack
                    if can_unstack(cells, piece_start, index_end) || can_stack(cells, piece_start, index_end) {
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }
                // 2-range move
                player_actions[index_actions] =
                    concatenate_action(index_start, INDEX_NULL, index_mid);
                index_actions += 1;
            }
        }
        // 1-range first action
        for &index_mid in NEIGHBOURS1.iter().take(7 * index_start + NEIGHBOURS1[7 * index_start] + 1).skip(7 * index_start + 1)
        {
            let half_action: u64 = (index_start | (index_mid << INDEX_WIDTH)) as u64;
            // 1-range move, [stack or unstack] optional
            if can_move1(cells, piece_start, index_mid) {
                // 1-range move, stack or unstack
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    // 1-range move, unstack or 1-range move, stack
                    if can_unstack(cells, piece_start, index_end) || can_stack(cells, piece_start, index_end) {
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }
                // 1-range move, unstack on starting position
                player_actions[index_actions] =
                    concatenate_action(index_start, index_mid, index_start);
                index_actions += 1;

                // 1-range move
                player_actions[index_actions] =
                    concatenate_action(index_start, INDEX_NULL, index_mid);
                index_actions += 1;
            }
            // stack, [1/2-range move] optional
            else if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in NEIGHBOURS2.iter().take(7 * index_mid + NEIGHBOURS2[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }

                // stack, 1-range move
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    if can_move1(cells, piece_start, index_end) {
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }

                // stack only
                player_actions[index_actions] =
                    concatenate_action(index_start, index_start, index_mid);
                index_actions += 1;
            }

            // unstack
            if can_unstack(cells, piece_start, index_mid) {
                // unstack only
                player_actions[index_actions] =
                    concatenate_action(index_start, index_start, index_mid);
                index_actions += 1;
            }
        }
    }

    player_actions[MAX_PLAYER_ACTIONS - 1] = index_actions as u64;
}

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
fn count_piece_actions(cells: &[u8; 45], index_start: usize) -> u64 {
    let mut piece_action_count: u64 = 0u64;

    let piece_start: u8 = cells[index_start];

    // If the piece is not a stack
    if piece_start < STACK_THRESHOLD {
        // 1-range first action
        for &index_mid in NEIGHBOURS1.iter().take(7 * index_start + NEIGHBOURS1[7 * index_start] + 1).skip(7 * index_start + 1)
        {
            // stack, [1/2-range move] optional
            if can_stack(cells, piece_start, index_mid) {
                // stack, 2-range move
                for &index_end in NEIGHBOURS2.iter().take(7 * index_mid + NEIGHBOURS2[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    if can_move2(cells, piece_start, index_mid, index_end)
                        || (index_start == ((index_mid + index_end) / 2)
                            && can_move1(cells, piece_start, index_end))
                    {
                        piece_action_count += 1;
                    }
                }

                // stack, 0/1-range move
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
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
        for &index_mid in NEIGHBOURS2.iter().take(7 * index_start + NEIGHBOURS2[7 * index_start] + 1).skip(7 * index_start + 1)
        {
            if can_move2(cells, piece_start, index_start, index_mid) {
                // 2-range move, stack or unstack
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    // 2-range move, unstack or 2-range move, stack
                    if can_unstack(cells, piece_start, index_end) || can_stack(cells, piece_start, index_end) {
                        piece_action_count += 1;
                    }
                }
                // 2-range move;
                piece_action_count += 1;
            }
        }
        // 1-range first action
        for &index_mid in NEIGHBOURS1.iter().take(7 * index_start + NEIGHBOURS1[7 * index_start] + 1).skip(7 * index_start + 1)
        {
            // 1-range move, [stack or unstack] optional
            if can_move1(cells, piece_start, index_mid) {
                // 1-range move, stack or unstack
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    // 1-range move, unstack or 1-range move, stack
                    if can_unstack(cells, piece_start, index_end) || can_stack(cells, piece_start, index_end) {
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
                for &index_end in NEIGHBOURS2.iter().take(7 * index_mid + NEIGHBOURS2[7 * index_mid] + 1).skip(7 * index_mid + 1)
                {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        piece_action_count += 1;
                    }
                }

                // stack, 1-range move
                for &index_end in NEIGHBOURS1.iter().take(7 * index_mid + NEIGHBOURS1[7 * index_mid] + 1).skip(7 * index_mid + 1)
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

/// Perft debug function to measure the number of leaf nodes (possible moves) at a given depth.
/// 
/// Recursively counts the number of leaf nodes at the chosen depth.
/// 
/// At depth 0, returns 1.
pub fn perft(cells: &[u8; 45], current_player: u8, depth: u64) -> u64
{
    if depth == 0 {
        return 1u64;
    } else if depth == 1 {
        return count_player_actions(cells, current_player);
    }

    let mut count: u64 = 0u64;

    let available_actions: [u64; 512] = available_player_actions(current_player, cells);
    let n_actions: usize = available_actions[MAX_PLAYER_ACTIONS - 1] as usize;

    let mut new_cells: [u8; 45] = [0u8; 45];

    for &action in available_actions.iter().take(n_actions) {
        if !is_action_win(cells, action) {
            copy_cells(cells, &mut new_cells);
            play_action(&mut new_cells, action);
            count += perft(&new_cells, 1 - current_player, depth - 1);
        }
    }
    count
}

/// Split Perft debug function to measure the number of leaf nodes (possible moves) at a given depth.
/// 
/// Recursively counts the number of leaf nodes at the chosen depth.
/// 
/// Separates the node count between all possible depth 1 moves and returns a vector of (action_string: String, action: u64, count: u64).
/// 
/// At depth 0, returns an empty vector.
pub fn perft_split(cells: &[u8; 45], current_player: u8, depth: u64) -> Vec<(String, u64, u64)> {
    let mut results: Vec<(String, u64, u64)> = Vec::with_capacity(256);

    if depth == 0 {
        return results;
    }

    let available_actions: [u64; MAX_PLAYER_ACTIONS] = available_player_actions(current_player, cells);
    let n_actions: usize = available_actions[MAX_PLAYER_ACTIONS - 1] as usize;
    
    let mut new_cells: [u8; 45] = [0u8; 45];

    for &action in available_actions.iter().take(n_actions) {
        if !is_action_win(cells, action) {
            copy_cells(cells, &mut new_cells);
            play_action(&mut new_cells, action);
            results.push((action_to_string(cells, action), action, perft(&new_cells, 1 - current_player, depth - 1)));
        }
    }

    results
}