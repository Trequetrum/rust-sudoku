
use arrayvec::ArrayVec;
use crate::*;
use GroupStyle::*;

#[derive(Debug, Clone, Copy)]
pub enum GroupStyle { Row, Column, Box }

#[derive(Debug, Clone)]
pub struct Group {
    pub style: GroupStyle,
    pub sub: u32, 
    pub indices: ArrayVec<usize, 9>
}

impl Group {

    pub fn meta_style(style: GroupStyle) -> u32 {
        match style {
            Row => 0,
            Column => 1,
            Box => 2
        }
    }

    pub fn from_meta_style(meta_style: u32) -> GroupStyle {
        match meta_style {
            0 => Row,
            1 => Column,
            2 => Box,
            _ => panic!("Group::meta_style doesn't support value: {}", meta_style)
        }
    }

    pub fn new(style: GroupStyle, sub: u32) -> Group {

        let indices = match &style {
            Row => Group::row_indices(sub),
            Column => Group::col_indices(sub),
            Box => Group::box_indices(sub)
        };

        Group { style, sub, indices }
    }

    fn row_indices(n: u32) -> ArrayVec::<usize, 9> {
        let mut res = ArrayVec::<usize, 9>::new();
    
        for i in 0..BOARD_SIZE {
            res.push((n * 9 + i) as usize); 
        }
    
        res
    }
    
    // indices n = coerce $ (_ * numOfOptions) >>> (_ + n) <$> 0 .. (numOfOptions - 1)
    
    fn col_indices(n: u32) -> ArrayVec::<usize, 9> {
        let mut res = ArrayVec::<usize, 9>::new();
    
        for i in 0..BOARD_SIZE {
            res.push((i * 9 + n) as usize);
        }
    
        res
    }
    
    // boxOffset :: Int
    // boxOffset = n / root * root * numOfOptions +
    // (n `mod` root) * root
    
    // boxIndex :: Int -> Int -> Int
    // boxIndex r c = boxOffset + numOfOptions * r + c
    fn box_indices(n: u32) -> ArrayVec::<usize, 9> {
        let mut res = ArrayVec::<usize, 9>::new();
    
        let box_offset = n / BOARD_ROOT * BOARD_ROOT * BOARD_SIZE + (n % BOARD_ROOT) * BOARD_ROOT;
    
        for i in 0..BOARD_SIZE {
            let r = i / BOARD_ROOT;
            let c = i * BOARD_ROOT;
            res.push((box_offset + BOARD_SIZE * r + c) as usize);
        }
    
        res
    }
}