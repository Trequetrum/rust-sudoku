// mod u16_set;
// mod sudoku_indices;

use std::time::Instant;

use anyhow::Result;
use arrayvec::ArrayVec;

use crate::n_tuple_strategy::MetaBoard;

const BOARD_SIZE: u32 = 9;
const BOARD_ROOT: u32 = 3;

type Board = ArrayVec<u32, 81>;

mod cell;
mod group;
mod n_tuple_strategy;
mod sudoku_utils;

fn main() {
    println!("Hello, world!");

    let asset = [
        "85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.",
        "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..",
        "12..4......5.69.1...9...5.........7.7...52.9..3......2.9.6...5.4..9..8.1..3...9.4",
        "...57..3.1......2.7...234......8...4..7..4...49....6.5.42...3.....7..9....18.....",
        "7..1523........92....3.....1....47.8.......6............9...5.6.4.9.7...8....6.1.",
        "1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..4......7..7...3..",
        "1...34.8....8..5....4.6..21.18......3..1.2..6......81.52..7.9....6..9....9.64...2",
        "...92......68.3...19..7...623..4.1....1...7....8.3..297...8..91...5.72......64...",
        ".6.5.4.3.1...9...8.........9...5...6.4.6.2.7.7...4...5.........4...8...1.5.2.3.4.",
        "7.....4...2..7..8...3..8.799..5..3...6..2..9...1.97..6...3..9...3..4..6...9..1.35",
        "....7..2.8.......6.1.2.5...9.54....8.........3....85.1...3.2.8.4.......9.7..6....",
    ];

    let mut boards: ArrayVec<Board, 11> = asset
        .iter()
        .filter_map(|str| sudoku_utils::parse_sudoku_board(str))
        .collect();

    // #[rustfmt::skip]
    // let mut board = ArrayVec::from(
    //     [ 511u32,511,8,32,64,2,511,511,511
    //     , 16,511,511,128,511,511,511,256,32
    //     , 511,32,4,511,8,511,511,511,128
    //     , 4,128,2,1,511,511,256,32,511
    //     , 8,64,16,511,511,511,1,511,511
    //     , 256,1,511,2,511,8,16,511,511
    //     , 511,511,511,511,511,511,511,2,256
    //     , 511,511,1,511,511,511,64,8,4
    //     , 2,511,511,511,32,4,128,511,1
    //     ]
    // );

    let mut board = &mut boards[10];

    sudoku_utils::dbg_display_board_hints(board.clone());

    let mut meta_board = MetaBoard::new();

    let now = Instant::now();

    let mut count = 0;
    while n_tuple_strategy::enforce_n_tuple(&mut board, &mut meta_board) {
        count += 1;
    }
    println!("Time: {}", now.elapsed().as_micros());

    println!("Updated times: {}", count);

    sudoku_utils::dbg_display_board_hints(board.clone());

    //---------------------------------------------------------------------

    let now_2 = Instant::now();

    count = 0;
    while n_tuple_strategy::enforce_n_tuple(&mut board, &mut meta_board) {
        count += 1;
    }
    println!("Time: {}", now_2.elapsed().as_micros());

    println!("Updated times: {}", count);

    sudoku_utils::dbg_display_board_hints(board.clone());
}
