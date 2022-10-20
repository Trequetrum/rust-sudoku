
use arrayvec::ArrayVec;
use anyhow::Result;
use crate::{cell, Board};
use crate::group::Group;

// To print how how many of each tuple size there is
    // for i in 1..=9 {
    //     let mut count = 0;
    //     for j in 1..=511 {
    //         if cell::count_options(j) == i {
    //             count += 1;
    //         }
    //     }
    //     println!("{}: {}", i, count);
    // }

pub enum NTupleType {
    Gen, Naked, Hidden, Both
}
pub struct NTuple { 
    tuple_type: NTupleType,
    group: Group,
    options: u32,
    position: ArrayVec<u32, 9>
}


pub struct MetaBoard {
    // group / sub-group/ tupleSize / (openTuples, openIndices)
    tuple_state: ArrayVec<ArrayVec<ArrayVec<(ArrayVec<u32, 126>, ArrayVec<usize, 9>), 4>, 9>, 3>
}

impl MetaBoard {
    pub fn new() -> MetaBoard {
        MetaBoard { tuple_state: (0..3)
            .map(|group| (0..9)
                .map(|sub_group| (0..4 as u32)
                    .map(|tuple_size| (
                        (1..511 as u32).filter(|n| cell::cardinality(*n) == tuple_size + 1).collect(), 
                        Group::new(Group::from_meta_style(group), sub_group).indices
                    )).collect()
                ).collect()
            ).collect()
        }
    }
}

pub fn find_n_tuples(
    board: ArrayVec<u32, 81>, 
    metadata: MetaBoard, 
    size: u32, 
    group: Group
) -> Result<Vec<NTuple>> {
    Ok(vec![])
}


pub fn enforce_1_tuple(
    board: &mut Board,
    metadata: &mut MetaBoard
) -> () {

    for group in 0..3 {
        for sub_group in 0..9 {

            let mut drop_tuples = 0;
            let mut drop_indices = ArrayVec::<u32, 9>::new();

            let (open_tuples, open_indices) = &mut metadata.tuple_state[group][sub_group][1-1];

            for tuple in open_tuples.clone() {

                let mut not_disjoint_idxs = ArrayVec::<usize, 9>::new();
                let mut superset_idxs = ArrayVec::<usize, 9>::new();
                let curr_indices = open_indices.clone();
                for idx in &curr_indices {

                    let cell = board[*idx];
                    if cell::not_disjoint!(cell, tuple) {
                        not_disjoint_idxs.push(*idx);
                    }
                    if cell::is_superset!(cell, tuple) {
                        superset_idxs.push(*idx)
                    }

                }

                // if size tuple == size superset) then -- this is a naked tuple
                // if ... >= ... superset) then -- I'm not interested 
                // if ... < ... superset) then -- this board is not valid

                // if size tuple == size notDisjoint) then -- this is a hidden tuple
                // if ... > ... notDisjoint) then -- this board is not valid
                // if ... <= ... notDisjoint) then -- I'm not interested

                // Check if naked tuple
                if cell::cardinality(tuple) == superset_idxs.len() as u32 {
                    let complement = !tuple;
                    for i in curr_indices {
                        if !superset_idxs.contains(&i) {
                            board[i] = board[i] & complement
                        }
                    }
                }

                if cell::cardinality(tuple) == not_disjoint_idxs.len() as u32 {
                    for i in not_disjoint_idxs {
                        board[i] = board[i] & tuple
                    }
                }

            }
        }
    }

}
