use super::{CELL_EMPTY, COLOUR_MASK, HALF_PIECE_WIDTH, INDEX_MASK, INDEX_NULL, INDEX_WIDTH, TOP_MASK};

pub fn do_move(index_start: usize, index_end: usize, cells: &mut [u8; 45]) {
    if index_start != index_end {
        // Move the piece to the target cell
        cells[index_end] = cells[index_start];

        // Set the starting cell as empty
        cells[index_start] = CELL_EMPTY;
    }
}

pub fn do_stack(index_start: usize, index_end: usize, cells: &mut [u8; 45]) {
    let piece_start: u8 = cells[index_start];
    let piece_end: u8 = cells[index_end];

    // If the moving piece is already on top of a stack, leave the bottom piece in the starting cell
    cells[index_start] = piece_start >> HALF_PIECE_WIDTH;

    // Move the top piece to the target cell and set its new bottom piece
    cells[index_end] = (piece_start & TOP_MASK) + (piece_end << HALF_PIECE_WIDTH);
}

pub fn do_unstack(index_start: usize, index_end: usize, cells: &mut [u8; 45]) {
    let piece_start: u8 = cells[index_start];

    // Leave the bottom piece in the starting cell
    cells[index_start] = piece_start >> HALF_PIECE_WIDTH;

    // Remove the bottom piece from the moving piece
    // Move the top piece to the target cell
    // Will overwrite the eaten piece if there is one
    cells[index_end] = piece_start & TOP_MASK;
}

pub fn play(action: u64, cells: &mut [u8; 45]) {
    let index_start: usize = (action & INDEX_MASK) as usize;
    let index_mid: usize = ((action >> INDEX_WIDTH) & INDEX_MASK) as usize;
    let index_end: usize = ((action >> (2 * INDEX_WIDTH)) & INDEX_MASK) as usize;

    if index_start == INDEX_NULL {
        return;
    }

    let piece_start: u8 = cells[index_start];

    if piece_start != CELL_EMPTY {
        // If there is no intermediate move
        if index_mid == INDEX_NULL {
            // Simple move
            do_move(index_start, index_end, cells);
        } else {
            let piece_mid: u8 = cells[index_mid];
            let piece_end: u8 = cells[index_end];
            // The piece at the mid coordinates is an ally : stack and move
            if piece_mid != CELL_EMPTY
                && (piece_mid & COLOUR_MASK) == (piece_start & COLOUR_MASK)
                && (index_start != index_mid)
            {
                do_stack(index_start, index_mid, cells);
                do_move(index_mid, index_end, cells);
            }
            // The piece at the end coordinates is an ally : move and stack
            else if piece_end != CELL_EMPTY && (piece_end & COLOUR_MASK) == (piece_start & COLOUR_MASK) {
                do_move(index_start, index_mid, cells);
                do_stack(index_mid, index_end, cells);
            }
            // The end coordinates contain an enemy or no piece : move and unstack
            else {
                do_move(index_start, index_mid, cells);
                do_unstack(index_mid, index_end, cells)
            }
        }
    }
}