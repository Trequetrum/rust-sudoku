use arrayvec::ArrayVec;

use crate::{cell, Board};
// Indices laid out for visual reference
//     0, 1, 2,   3, 4, 5,   6, 7, 8,
//     9, 10,11,  12,13,14,  15,16,17,
//     18,19,20,  21,22,23,  24,25,26,
//     ---------  ---------  ---------
//     27,28,29,  30,31,32,  33,34,35,
//     36,37,38,  39,40,41,  42,43,44,
//     45,46,47,  48,49,50,  51,52,53,
//     ---------  ---------  ---------
//     54,55,56,  57,58,59,  60,61,62,
//     63,64,65,  66,67,68,  69,70,71,
//     72,73,74,  75,76,77,  78,79,80
//

// startingBoard :: Board
// startingBoard = fromMaybe unconstrainedBoard $ fromCells $ coerce
//   [ 511,511,8,32,64,2,511,511,511
//   , 16,511,511,128,511,511,511,256,32
//   , 511,32,4,511,8,511,511,511,128
//   , 4,128,2,1,511,511,256,32,511
//   , 8,64,16,511,511,511,1,511,511
//   , 256,1,511,2,511,8,16,511,511
//   , 511,511,511,511,511,511,511,2,256
//   , 511,511,1,511,511,511,64,8,4
//   , 2,511,511,511,32,4,128,511,1
//   ]
//
// startingBoardSolved :: Board
// startingBoardSolved = fromMaybe unconstrainedBoard $ fromCells $ coerce
//   [ 128,256,8,32,64,2,4,1,16
//   , 16,2,64,128,4,1,8,256,32
//   , 1,32,4,16,8,256,2,64,128
//   , 4,128,2,1,16,64,256,32,8
//   , 8,64,16,4,256,32,1,128,2
//   , 256,1,32,2,128,8,16,4,64
//   , 64,4,128,8,1,16,32,2,256
//   , 32,16,1,256,2,128,64,8,4
//   , 2,8,256,64,32,4,128,16,1
//   ]

// """
//   004 | 672 | 000
//   500 | 800 | 096
//   063 | 040 | 008
//   ----+-----+----
//   382 | 100 | 960
//   475 | 000 | 100
//   910 | 204 | 500
//   ----+-----+----
//   000 | 000 | 029
//   001 | 000 | 743
//   200 | 063 | 801
// """
pub fn dbg_display_board(brd: ArrayVec<u32, 81>) {
    for d in 1..=11 {
        for r in 1..=11 {
            if d % 4 == 0 && r % 4 == 0 {
                print!("—+—");
            } else if d % 4 == 0 {
                print!("—");
            } else if r % 4 == 0 {
                print!(" | ");
            } else {
                let current_row = r - (r / 4);
                let current_column = d - (d / 4);
                let current_index = current_row + (9 * (current_column - 1)) - 1;

                let int_repr = brd[current_index];

                if int_repr.count_ones() == 1 {
                    print!("{}", cell::pick_one(int_repr));
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

pub fn dbg_display_board_hints(brd: ArrayVec<u32, 81>) {
    for d in 0..37 {
        for r in 0..37 {
            if d % 12 == 0 && r == 0 {
                print!("  =");
            } else if d % 12 == 0 {
                print!("==");
            } else if d % 4 == 0 && r == 0 {
                print!("  —");
            } else if d % 4 == 0 {
                print!("——");
            } else if r % 12 == 0 {
                print!("  ‖  ");
            } else if r % 4 == 0 {
                print!("  |  ");
            } else {
                let current_row = r / 4;
                let current_column = d / 4;
                let current_index = current_row + (9 * current_column);

                // Binary representaiton of what's at this index
                let int_repr = brd[current_index];

                // Which option 1..=9 are we checking?
                let checking_val = r % 4 + 3 * (d % 4 - 1);
                // Represent the option as bit in an int
                let checking_val_repr = 1 << (checking_val - 1);

                if int_repr & checking_val_repr == checking_val_repr {
                    print!("{}", checking_val);
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

pub fn parse_sudoku_board(input: &str) -> Option<Board> {
    let board: ArrayVec<u32, 81> = input
        .chars()
        .map(|c| if c == '.' { '0' } else { c })
        .filter_map(|c| c.to_digit(10))
        .map(|n| if n == 0 { 511 } else { 1 << (n - 1) })
        .collect();

    if board.len() == 81 {
        Some(board)
    } else {
        None
    }
}
