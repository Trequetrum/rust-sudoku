use std::cmp;
use std::convert::TryInto;

const MAX_CARDINALITY: u8 = 16;

// Wrapper around a 16 bit integer that lets us think
// about it as a set with elements (0 - 15)
#[derive(Debug, Clone, Copy)]
pub struct U16Set(u16);

//-----------------------------------------------------------------
// Constructors for sets
//-----------------------------------------------------------------

pub fn empty() -> U16Set {
    U16Set(0)
}

pub fn universal() -> U16Set {
    // U16Set((1 << MAX_CARDINALITY) - 1)
    U16Set(std::u16::MAX)
}

pub fn singleton(v: u8) -> U16Set {
    U16Set(1 << cmp::min(MAX_CARDINALITY - 1, v))
}

//-----------------------------------------------------------------
// Destructuring sets
//-----------------------------------------------------------------

pub fn pick_one(a: U16Set) -> Option<u8> {
    let U16Set(mut set) = a;

    for e in 0..MAX_CARDINALITY {
        if set & 1 != 0 {
            return Some(e);
        }
        if set != 0 {
            set = set >> 1;
        } else {
            break;
        }
    }

    None
}

pub fn pick_all(a: U16Set) -> Vec<u8> {
    let mut elems = vec![];
    let U16Set(mut set) = a;

    for e in 0..MAX_CARDINALITY {
        if set & 1 != 0 {
            elems.push(e);
        }
        if set != 0 {
            set = set >> 1;
        } else {
            break;
        }
    }

    elems
}

pub fn pick_singleton(a: U16Set) -> Option<u8> {
    let mut elems = pick_all(a);
    if elems.len() == 1 {
        return elems.pop();
    }
    None
}

//-----------------------------------------------------------------
// Operations for sets
//-----------------------------------------------------------------

pub fn cardinality(u_set: U16Set) -> u8 {
    u_set.0.count_ones().try_into().unwrap()
}

// Set union is the same as pairwise or
pub fn union(a: U16Set, b: U16Set) -> U16Set {
    U16Set(a.0 | b.0)
}

// Set union is the same as pairwise and
pub fn intersection(a: U16Set, b: U16Set) -> U16Set {
    U16Set(a.0 & b.0)
}

pub fn complement(a: U16Set) -> U16Set {
    U16Set(!a.0)
}

pub fn subtraction(a: U16Set, b: U16Set) -> U16Set {
    U16Set(a.0 & !b.0)
}

// Pairwise XOR (this is the union of differences)
// Written as: ( (A - B) U (B - A) )
pub fn pairwise_xor(a: U16Set, b: U16Set) -> U16Set {
    U16Set(a.0 ^ b.0)
}

//-----------------------------------------------------------------
// Operations for elements
//-----------------------------------------------------------------

pub fn union_elem(a: U16Set, e: u8) -> U16Set {
    union(a, singleton(e))
}

pub fn subtract_elem(a: U16Set, e: u8) -> U16Set {
    subtraction(a, singleton(e))
}

// Toggle an element in a set
pub fn exclusive_elem(a: U16Set, e: u8) -> U16Set {
    pairwise_xor(a, singleton(e))
}

//-----------------------------------------------------------------
// Predicates
//-----------------------------------------------------------------

pub fn is_empty(a: U16Set) -> bool {
    a.0 == 0
}

pub fn is_singleton(a: U16Set) -> bool {
    cardinality(a) == 1
}

pub fn contains(u_set: U16Set, v: u8) -> bool {
    (v < MAX_CARDINALITY) && (((u_set.0 >> v) & 1) != 0)
}

// Check if the union of two sets is inhabited
pub fn not_disjoint(a: U16Set, b: U16Set) -> bool {
    a.0 & b.0 != 0
}

// Check if the first set is a superset of the second set
// This is not strict, so (a `isSuperset` a) == true
pub fn is_superset(a: U16Set, b: U16Set) -> bool {
    a.0 | b.0 == a.0
}

// Check if the first set is a subset of the second set
// This is not strict, so (a `isSubset` a) == true
pub fn is_subset(a: U16Set, b: U16Set) -> bool {
    b.0 | a.0 == b.0
}