#![doc(html_root_url = "https://docs.rs/geohash/")]
//! # Geohash
//!
//! Geohash algorithm implementation in Rust. It encodes/decodes a
//! longitude-latitude tuple into/from a hashed string. You can find
//! more about geohash algorithm on [Wikipedia](https://en.wikipedia.org/wiki/Geohash)
//!
//! ## Usage
//! ```rust
//! extern crate geohash;
//!
//! use fixed::types::I64F64;
//!
//! fn main() -> Result<(), Box<geohash::GeohashError>> {
//! use geohash::{GeoHash, Direction};
//! let lon = I64F64::from_num(112.5584);
//!   let lat = I64F64::from_num(37.8324f64);
//!
//!   // decode a geohash
//!   let (lon, lat, _, _) = GeoHash("ww8p1r4t8".as_bytes().to_vec()).try_as_coordinates()?;
//!
//!   // find a neighboring hash
//!   let sw = GeoHash("ww8p1r4t8".as_bytes().to_vec()).neighbor(Direction::SW)?;
//!
//!   Ok(())
//! }
//! ```
//!
//!
//!
#![no_std]

extern crate alloc;

use ::core::ops::Deref;
use alloc::vec::Vec;

use codec::{Decode, Encode};
use fixed::types::I64F64;

use crate::core::{decode, encode, neighbor, neighbors};
pub use crate::error::GeohashError;
pub use crate::neighbors::{Direction, Neighbors};


#[derive(Encode, Decode, Eq, PartialEq, Clone, Debug)]
pub struct GeoHash(pub Vec<u8>);

impl Deref for GeoHash {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GeoHash {
    pub fn try_from_params(lat: I64F64, lon: I64F64, len: usize) -> Result<GeoHash, GeohashError> {
        encode(lat, lon, len)
    }
    pub fn try_as_coordinates(&self) -> Result<(I64F64, I64F64, I64F64, I64F64), GeohashError> {
        decode(self)
    }
    pub fn neighbors(&self) -> Result<Neighbors, GeohashError> {
        neighbors(self)
    }
    pub fn neighbor(&self, direction: Direction) -> Result<GeoHash, GeohashError> {
        neighbor(self, direction)
    }
}

mod core;
mod error;
mod neighbors;
