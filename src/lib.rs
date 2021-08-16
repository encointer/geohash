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
//! use geohash::{encode, decode, neighbor, Direction};
//! use fixed::types::I64F64;
//!
//! fn main() -> Result<(), Box<geohash::GeohashError>> {
//!   use geohash::GeoHash;
//! let lon = I64F64::from_num(112.5584);
//!   let lat = I64F64::from_num(37.8324f64);
//!
//!   // decode a geohash
//!   let (lon, lat, _, _) = decode(&GeoHash("ww8p1r4t8".as_bytes().to_vec()))?;
//!
//!   // find a neighboring hash
//!   let sw = neighbor(&GeoHash("ww8p1r4t8".as_bytes().to_vec()), Direction::SW)?;
//!
//!   Ok(())
//! }
//! ```
//!
//!
//!
#![no_std]
use codec::{Decode, Encode};
use ::core::ops::Deref;
extern crate alloc;


#[derive(Encode, Decode, Eq, PartialEq, Clone, Debug)]
pub struct GeoHash(pub Vec<u8>);

impl Deref for GeoHash {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


mod core;
mod error;
mod neighbors;

pub use crate::core::{decode, encode, neighbor, neighbors};
pub use crate::error::GeohashError;
pub use crate::neighbors::{Direction, Neighbors};
use alloc::vec::Vec;
