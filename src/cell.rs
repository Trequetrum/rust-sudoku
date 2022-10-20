
pub fn pick_one(cell: u32) -> u32 {
    let mut cell = cell;

    for option in 1..=9 {
        if cell % 2 != 0 { 
            return option;
        }
        cell = cell >> 1;
    }

    0
}

// A cell represents a set of options, so we'll use set terminology.
pub fn cardinality(cell: u32) -> u32 {
    let mut cell = cell;
    let mut count = 0;

    for _ in 1..=9 {
        if cell % 2 != 0 { 
            count += 1;
        }
        cell = cell >> 1;
    }

    count
}


macro_rules! not_disjoint {
    ($a:expr,$b:expr) => {
        $a & $b != 0
    } 
}

macro_rules! is_superset {
    ($a:expr,$b:expr)=>{
        $a | $b == $a
    }
}

pub(crate) use not_disjoint;
pub(crate) use is_superset;
