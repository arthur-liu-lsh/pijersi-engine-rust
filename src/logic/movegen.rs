use super::actions::{copy_cells, play_action};
use super::lookup::{NEIGHBOURS1, NEIGHBOURS2};
use super::rules::{can_move1, can_move2, can_stack, can_unstack, is_action_win};
use super::translate::action_to_string;
use super::{COLOUR_MASK, INDEX_NULL, INDEX_WIDTH, MAX_PLAYER_ACTIONS, STACK_THRESHOLD};

/// Concatenate three indices into a u64 action.
/// The first index is stored in the 8 least significant bits.
#[inline]
pub fn concatenate_action(index_start: usize, index_mid: usize, index_end: usize) -> u64 {
    (index_start | (index_mid << INDEX_WIDTH) | (index_end << (2 * INDEX_WIDTH))) as u64
}

/// Concatenate a half action and the last index into a u64 action.
/// The first index is stored in the 8 least significant bits.
#[inline]
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
#[inline]
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
        for &index_mid in NEIGHBOURS1
            .iter()
            .skip(7 * index_start + 1)
            .take(NEIGHBOURS1[7 * index_start])
        {
            let half_action: u64 = (index_start | (index_mid << INDEX_WIDTH)) as u64;
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
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }

                // stack, 0/1-range move
                for &index_end in NEIGHBOURS1
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS1[7 * index_mid])
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
        for &index_mid in NEIGHBOURS2
            .iter()
            .skip(7 * index_start + 1)
            .take(NEIGHBOURS2[7 * index_start])
        {
            let half_action: u64 = (index_start | (index_mid << INDEX_WIDTH)) as u64;
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
        for &index_mid in NEIGHBOURS1
            .iter()
            .skip(7 * index_start + 1)
            .take(NEIGHBOURS1[7 * index_start])
        {
            let half_action: u64 = (index_start | (index_mid << INDEX_WIDTH)) as u64;
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
                for &index_end in NEIGHBOURS2
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS2[7 * index_mid])
                {
                    if can_move2(cells, piece_start, index_mid, index_end) {
                        player_actions[index_actions] =
                            concatenate_half_action(half_action, index_end);
                        index_actions += 1;
                    }
                }

                // stack, 1-range move
                for &index_end in NEIGHBOURS1
                    .iter()
                    .skip(7 * index_mid + 1)
                    .take(NEIGHBOURS1[7 * index_mid])
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
