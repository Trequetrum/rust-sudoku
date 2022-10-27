use crate::*;
use arrayvec::ArrayVec;
use GroupStyle::*;

#[derive(Debug, Clone, Copy)]
pub enum GroupStyle {
    Row,
    Column,
    Box,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub style: GroupStyle,
    pub sub: u32,
    pub indices: ArrayVec<usize, 9>,
}

impl Group {
    pub fn meta_style(style: GroupStyle) -> u32 {
        match style {
            Row => 0,
            Column => 1,
            Box => 2,
        }
    }

    pub fn from_meta_style(meta_style: u32) -> GroupStyle {
        match meta_style {
            0 => Row,
            1 => Column,
            2 => Box,
            _ => panic!("Group::meta_style doesn't support value: {}", meta_style),
        }
    }

    pub fn new(style: GroupStyle, sub: u32) -> Group {
        let indices = match &style {
            Row => Group::row_indices(sub),
            Column => Group::col_indices(sub),
            Box => Group::box_indices(sub),
        };

        Group {
            style,
            sub,
            indices,
        }
    }

    fn row_indices(n: u32) -> ArrayVec<usize, 9> {
        (0..BOARD_SIZE).map(|i| (n * 9 + i) as usize).collect()
    }

    // indices n = coerce $ (_ * numOfOptions) >>> (_ + n) <$> 0 .. (numOfOptions - 1)

    fn col_indices(n: u32) -> ArrayVec<usize, 9> {
        (0..BOARD_SIZE).map(|i| (i * 9 + n) as usize).collect()
    }

    fn box_indices(n: u32) -> ArrayVec<usize, 9> {

        // box_offset is the first index in the board that belong to the nth box
        let box_offset = n / BOARD_ROOT * BOARD_ROOT * BOARD_SIZE + (n % BOARD_ROOT) * BOARD_ROOT;

        (0..BOARD_ROOT)
            .flat_map(|r| (0..BOARD_ROOT).map(move |c| (box_offset + BOARD_SIZE * r + c) as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_row() {
        assert_eq!(
            Group::row_indices(0),
            ArrayVec::from([0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[test]
    fn fifth_row() {
        assert_eq!(
            Group::row_indices(4),
            ArrayVec::from([36, 37, 38, 39, 40, 41, 42, 43, 44])
        );
    }

    #[test]
    fn last_row() {
        assert_eq!(
            Group::row_indices(8),
            ArrayVec::from([72, 73, 74, 75, 76, 77, 78, 79, 80])
        );
    }

    #[test]
    fn first_column() {
        assert_eq!(
            Group::col_indices(0),
            ArrayVec::from([0, 9, 18, 27, 36, 45, 54, 63, 72])
        );
    }

    #[test]
    fn fifth_column() {
        assert_eq!(
            Group::col_indices(4),
            ArrayVec::from([4, 13, 22, 31, 40, 49, 58, 67, 76])
        );
    }

    #[test]
    fn last_column() {
        assert_eq!(
            Group::col_indices(8),
            ArrayVec::from([8, 17, 26, 35, 44, 53, 62, 71, 80])
        );
    }

    #[test]
    fn first_box() {
        assert_eq!(
            Group::box_indices(0),
            ArrayVec::from([0, 1, 2, 9, 10, 11, 18, 19, 20])
        );
    }

    #[test]
    fn fifth_box() {
        assert_eq!(
            Group::box_indices(4),
            ArrayVec::from([30, 31, 32, 39, 40, 41, 48, 49, 50])
        );
    }

    #[test]
    fn last_box() {
        assert_eq!(
            Group::box_indices(8),
            ArrayVec::from([60, 61, 62, 69, 70, 71, 78, 79, 80])
        );
    }
}
