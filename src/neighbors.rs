use fixed::types::I64F64;
use crate::GeoHash;

#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors<const T: usize> {
    pub sw: GeoHash<T>,
    pub s: GeoHash<T>,
    pub se: GeoHash<T>,
    pub w: GeoHash<T>,
    pub e: GeoHash<T>,
    pub nw: GeoHash<T>,
    pub n: GeoHash<T>,
    pub ne: GeoHash<T>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// North
    N,
    /// North-east
    NE,
    /// Eeast
    E,
    /// South-east
    SE,
    /// South
    S,
    /// South-west
    SW,
    /// West
    W,
    /// North-west
    NW,
}

impl Direction {
    pub fn to_tuple(self) -> (I64F64, I64F64) {
        let minus_one = I64F64::from_num(-1);
        let zero = I64F64::from_num(0);
        let one = I64F64::from_num(1);
        match self {
            Direction::SW => (minus_one, minus_one),
            Direction::S => (minus_one, zero),
            Direction::SE => (minus_one, one),
            Direction::W => (zero, minus_one),
            Direction::E => (zero, one),
            Direction::NW => (one, minus_one),
            Direction::N => (one, zero),
            Direction::NE => (one, one),
        }
    }
}
