use crate::group::Group;
use crate::{cell, Board};
use anyhow::Result;
use arrayvec::ArrayVec;

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

#[derive(Debug, Clone)]
pub struct MetaBoard {
    // group / sub-group/ tupleSize / (openTuples, openIndices)
    tuple_state: ArrayVec<ArrayVec<ArrayVec<(ArrayVec<u32, 126>, ArrayVec<usize, 9>), 4>, 9>, 3>,
}

impl MetaBoard {
    // Allocates 14580 ints on the stack, that's a lot but it's still managable.
    #[rustfmt::skip]
    pub fn new() -> MetaBoard {
        MetaBoard {
            tuple_state: (0..3).map(
                |group| (0..9).map(
                    |sub_group| (0..4 as u32).map(
                        |tuple_size| (
                            (1..511 as u32)
                                .filter(|n| n.count_ones() == tuple_size + 1)
                                .collect(),
                            Group::new(Group::from_meta_style(group), sub_group).indices,
                        )
                    ).collect()
                ).collect()
            ).collect()
        }
    }
}

pub fn enforce_n_tuple(board: &mut Board, metadata: &mut MetaBoard) -> bool {
    let mut update = false;

    for group in 0..3 {
        for sub_group in 0..9 {
            for tuple_size in 0..4 {
                update = update
                    || enforce_n_tuple_helper(
                        &mut *board,
                        &mut *metadata,
                        group,
                        sub_group,
                        tuple_size,
                    );
            }
        }
    }

    update
}

pub fn enforce_n_tuple_helper(
    board: &mut Board,
    metadata: &mut MetaBoard,
    group: usize,
    sub_group: usize,
    tuple_size: usize,
) -> bool {
    let mut update = false;

    let mut drop_tuples = 0;
    let mut drop_indices = ArrayVec::<usize, 9>::new();

    let (open_tuples, open_indices) = &metadata.tuple_state[group][sub_group][tuple_size];

    for tuple in open_tuples {
        let mut not_disjoint_idxs = ArrayVec::<usize, 9>::new();
        let mut superset_idxs = ArrayVec::<usize, 9>::new();

        for idx in &*open_indices {
            let cell = board[*idx];
            if cell::not_disjoint!(*tuple, cell) {
                not_disjoint_idxs.push(*idx);
            }
            if cell::is_superset!(*tuple, cell) {
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
        if tuple.count_ones() == superset_idxs.len() as u32 {
            update = true;
            drop_tuples = drop_tuples | *tuple;
            let complement = !*tuple;

            for i in &*superset_idxs {
                drop_indices.push(*i);
            }

            for i in &*open_indices {
                if !superset_idxs.contains(&i) {
                    board[*i] = board[*i] & complement
                }
            }

        // Check if hidden tuple
        } else if tuple.count_ones() == not_disjoint_idxs.len() as u32 {
            update = true;
            drop_tuples = drop_tuples | *tuple;
            for i in not_disjoint_idxs {
                drop_indices.push(i);
                board[i] = board[i] & *tuple
            }
        }
    }

    for tuple_sizes in tuple_size..4 {
        let open_tuples = &mut metadata.tuple_state[group][sub_group][tuple_sizes].0;
        *open_tuples = open_tuples
            .iter()
            .filter(|x| cell::is_superset!(!drop_tuples, **x))
            .cloned()
            .collect();

        let open_indices = &mut metadata.tuple_state[group][sub_group][tuple_sizes].1;
        *open_indices = open_indices
            .iter()
            .filter(|x| !drop_indices.contains(*x))
            .cloned()
            .collect();
    }

    update
}
