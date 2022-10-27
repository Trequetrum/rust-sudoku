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

macro_rules! not_disjoint {
    ($a:expr,$b:expr) => {
        $a & $b != 0
    };
}

macro_rules! is_superset {
    ($a:expr,$b:expr) => {
        $a | $b == $a
    };
}

pub(crate) use is_superset;
pub(crate) use not_disjoint;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_superset() {
        for i in 1..=511 {
            assert!(is_superset!(511_u32, i));
        }
    }

    #[test]
    fn never_superset() {
        for i in 1..=511 {
            assert!(!is_superset!(0_u32, i));
        }
    }

    #[test]
    fn is_superset() {
        assert!(is_superset!(0b_101101001_u32, 0b_101101000_u32));
        assert!(is_superset!(0b_101101001_u32, 0b_101100001_u32));
        assert!(is_superset!(0b_101101001_u32, 0b_101001001_u32));
        assert!(is_superset!(0b_101101001_u32, 0b_100101001_u32));
        assert!(is_superset!(0b_101101001_u32, 0b_001101001_u32));
    }

    #[test]
    fn is_not_superset() {
        assert!(!not_disjoint!(0b_101101001_u32, 0b_010010110_u32));

        assert!(not_disjoint!(0b_101101001_u32, 0b_010010111_u32));
        assert!(not_disjoint!(0b_101101001_u32, 0b_010011110_u32));
        assert!(not_disjoint!(0b_101101001_u32, 0b_010110110_u32));
        assert!(not_disjoint!(0b_101101001_u32, 0b_011010110_u32));
        assert!(not_disjoint!(0b_101101001_u32, 0b_110010110_u32));
    }
}