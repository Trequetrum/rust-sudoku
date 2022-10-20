// mod u16_set;
// mod sudoku_indices;

use arrayvec::ArrayVec;
use anyhow::Result;
use crate::group::Group;

const BOARD_SIZE: u32 = 9;
const BOARD_ROOT: u32 = 3;

type Board = ArrayVec<u32, 81>;

mod sudoku_utils;
mod cell;
mod group;
mod n_tuple_strategy;

fn main() {
    // let full = u16_set::universal();
    // let full2 = u16_set::universal();
    // let empty = u16_set::empty();
    // println!("{:#?} {} {}", full, u16_set::contains(full2, 0), u16_set::contains(empty, 0));
    println!("Hello, world!");


    // let mut tmp = ArrayVec::<_, 16>::new();
    // tmp.push(1);
    // tmp.push(2);

    // println!("{:#?}", tmp);
    // println!("{:#?}", tmp.);

    sudoku_utils::dbg_display_board(ArrayVec::from(
        [ 511,511,8,32,64,2,511,511,511
        , 16,511,511,128,511,511,511,256,32
        , 511,32,4,511,8,511,511,511,128
        , 4,128,2,1,511,511,256,32,511
        , 8,64,16,511,511,511,1,511,511
        , 256,1,511,2,511,8,16,511,511
        , 511,511,511,511,511,511,511,2,256
        , 511,511,1,511,511,511,64,8,4
        , 2,511,511,511,32,4,128,511,1
        ]
    ));
}

