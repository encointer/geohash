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
//! use std::convert::TryFrom;
//! use geohash::{GeoHash, Direction};
//! let lon = I64F64::from_num(112.5584);
//!   let lat = I64F64::from_num(37.8324f64);
//!
//!   // decode a geohash
//!   let (lon, lat, _, _) = GeoHash::try_from("ww8p1r4t8")?.try_as_coordinates()?;
//!
//!   // find a neighboring hash
//!   let sw = GeoHash::try_from("ww8p1r4t8")?.neighbor(Direction::SW)?;
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
use core::convert::TryFrom;
use codec::{Decode, Encode, MaxEncodedLen};
use fixed::types::I64F64;

pub use crate::error::GeohashError;
pub use crate::neighbors::{Direction, Neighbors};

#[derive(Debug)]
struct Coordinate
{
    pub lon: I64F64,
    pub lat: I64F64,
}

struct Rectangle
{
    min: Coordinate,
    max: Coordinate,
}

static BASE32_CODES: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(Encode, Decode, Eq, PartialEq, Clone, Debug, Ord, PartialOrd, scale_info::TypeInfo, MaxEncodedLen)]
pub struct GeoHash<const LEN: usize>(pub [u8; LEN]);

impl<const LEN: usize> Deref for GeoHash<LEN> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl<const LEN: usize> TryFrom<&str> for GeoHash<LEN> {
    type Error = GeohashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TryFrom::<&[u8]>::try_from(value.as_bytes())
    }
}

impl<const LEN: usize> TryFrom<[u8; LEN]> for GeoHash<LEN> {
    type Error = GeohashError;

    fn try_from(value: [u8; LEN]) -> Result<Self, Self::Error> {
        TryFrom::<&[u8]>::try_from(&value[..])
    }
}

impl<const LEN: usize> TryFrom<&[u8]> for GeoHash<LEN> {
    type Error = GeohashError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // `try_from` is only successful if the input is a valid base 32 encoded geo hash.

        if value.len() != LEN {
            return Err(GeohashError::InvalidLen);
        }

        for c in value.iter() {
            let _ = hash_value_of_char(*c as char)?;
        }

        let mut arr = [0u8; LEN];
        arr.clone_from_slice(value);

        Ok(GeoHash(arr))
    }
}

impl<const LEN: usize> GeoHash<LEN> {
    /// Internal function to encode a coordinate to a geohash with length `LEN`.
    ///
    /// ### Examples
    ///
    /// Encoding a coordinate to a length five geohash:
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    /// use fixed::types::I64F64;
    /// use geohash::GeoHash;
    /// let lon = I64F64::from_num(-120.6623);
    /// let lat = I64F64::from_num(35.3003);
    /// let geohash_string = GeoHash::try_from_params(lat, lon).expect("Invalid coordinate");
    /// assert_eq!(geohash_string, GeoHash::try_from("9q60y").unwrap());
    /// ```
    ///
    /// Encoding a coordinate to a length ten geohash:
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    /// use fixed::types::I64F64;
    /// use geohash::GeoHash;
    /// let lon = I64F64::from_num(-120.6623);
    /// let lat = I64F64::from_num(35.3003);
    /// let geohash_string = GeoHash::try_from_params(lat, lon).expect("Invalid coordinate");
    ///
    /// assert_eq!(geohash_string, GeoHash::try_from("9q60y60rhs").unwrap());
    /// ```
    pub fn try_from_params(lat: I64F64, lon: I64F64) -> Result<GeoHash<LEN>, GeohashError> {
        let mut out = [0u8; LEN];

        let mut bits_total: i8 = 0;
        let mut hash_value: usize = 0;
        let mut max_lat = I64F64::from_num(90);
        let mut min_lat = I64F64::from_num(-90);
        let mut max_lon = I64F64::from_num(180);
        let mut min_lon = I64F64::from_num(-180);

        if lon < min_lon || lon > max_lon || lat < min_lat || lat > max_lat {
            return Err(GeohashError::InvalidCoordinateRange(lon, lat));
        }

        let two = I64F64::from_num(2);
        for i in 0..out.len() {
            for _ in 0..5 {
                if bits_total % 2 == 0 {
                    let mid = (max_lon + min_lon) / two;
                    if lon > mid {
                        hash_value = (hash_value << 1) + 1usize;
                        min_lon = mid;
                    } else {
                        hash_value <<= 1;
                        max_lon = mid;
                    }
                } else {
                    let mid = (max_lat + min_lat) / two;
                    if lat > mid {
                        hash_value = (hash_value << 1) + 1usize;
                        min_lat = mid;
                    } else {
                        hash_value <<= 1;
                        max_lat = mid;
                    }
                }
                bits_total += 1;
            }

            let code: char = BASE32_CODES[hash_value];
            out[i] = code as u8;
            hash_value = 0;
        }
        Ok(GeoHash(out))
    }

    /// Decode geohash string into latitude, longitude
    ///
    /// Parameters:
    /// Geohash encoded `&str`
    ///
    /// Returns:
    /// A four-element tuple describs a bound box:
    /// * min_lat
    /// * max_lat
    /// * min_lon
    /// * max_lon
    fn decode_bbox(&self) -> Result<Rectangle, GeohashError> {
        let mut is_lon = true;
        let mut max_lat = I64F64::from_num(90);
        let mut min_lat = I64F64::from_num(-90);
        let mut max_lon = I64F64::from_num(180);
        let mut min_lon = I64F64::from_num(-180);
        let mut mid: I64F64;
        let mut hash_value: usize;

        let two = I64F64::from_num(2);

        for c in self.iter() {
            hash_value = hash_value_of_char(*c as char)?;

            for bs in 0..5 {
                let bit = (hash_value >> (4 - bs)) & 1usize;
                if is_lon {
                    mid = (max_lon + min_lon) / two;

                    if bit == 1 {
                        min_lon = mid;
                    } else {
                        max_lon = mid;
                    }
                } else {
                    mid = (max_lat + min_lat) / two;

                    if bit == 1 {
                        min_lat = mid;
                    } else {
                        max_lat = mid;
                    }
                }
                is_lon = !is_lon;
            }
        }

        Ok(Rectangle {
            min: Coordinate {
                lon: min_lon,
                lat: min_lat,
            },
            max: Coordinate {
                lon: max_lon,
                lat: max_lat,
            },
        })
    }

    /// Internal function to decode a geohash into a longitude/latitude pair with some
    /// longitude/latitude error. The return value is
    /// `(<longitude>, <latitude>, <longitude error>, <latitude error>)`.
    ///
    /// ### Examples
    ///
    /// Decoding a length five geohash:
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    /// use fixed::types::I64F64;
    /// use geohash::GeoHash;
    /// let geohash_str = GeoHash::try_from("9q60y").unwrap();
    /// let decoded = geohash_str.try_as_coordinates().expect("Invalid hash string");
    /// assert_eq!(
    ///     decoded,
    ///     (
    ///         I64F64::from_num(-120.65185546875),
    ///         I64F64::from_num(35.31005859375),
    ///         I64F64::from_num(0.02197265625),
    ///         I64F64::from_num(0.02197265625),
    ///     ),
    /// );
    /// ```
    ///
    /// Decoding a length ten geohash:
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    /// use fixed::types::I64F64;
    /// use geohash::GeoHash;
    /// let geohash_str = GeoHash::try_from("9q60y60rhs").unwrap();
    /// let decoded = geohash_str.try_as_coordinates().expect("Invalid hash string");
    /// assert_eq!(
    ///     decoded,
    ///     (
    ///         I64F64::from_num(-120.66229999065399),
    ///         I64F64::from_num(35.300298035144806),
    ///         I64F64::from_num(0.000005364418029785156),
    ///         I64F64::from_num(0.000002682209014892578),
    ///     ),
    /// );
    /// ```
    pub fn try_as_coordinates(&self) -> Result<(I64F64, I64F64, I64F64, I64F64), GeohashError> {
        let rect = self.decode_bbox()?;
        let c0 = rect.min;
        let c1 = rect.max;
        let two = I64F64::from_num(2);
        Ok((
            (c0.lon + c1.lon) / two, // longitude
            (c0.lat + c1.lat) / two, // latitude
            (c1.lon - c0.lon) / two, // longitude error
            (c1.lat - c0.lat) / two, // latitude error
        ))
    }

    /// Find neighboring geohashes for the given geohash and direction.
    pub fn neighbor(&self, direction: Direction) -> Result<GeoHash<LEN>, GeohashError> {
        let (lon, lat, lon_err, lat_err) = self.try_as_coordinates()?;
        let (dlat, dlng) = direction.to_tuple();
        let two = I64F64::from_num(2);
        let neighbor_lon = lon + two * lon_err.abs() * dlng;
        let neighbor_lat = lat + two * lat_err.abs() * dlat;
        GeoHash::try_from_params(neighbor_lat, neighbor_lon)
    }

    /// Find all neighboring geohashes for the given geohash.
    ///
    /// ### Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use geohash::GeoHash;
    /// let geohash_str = GeoHash::try_from("9q60y60rhs").unwrap();
    ///
    /// let neighbors = geohash_str.neighbors().expect("Invalid hash string");
    ///
    /// assert_eq!(
    ///     neighbors,
    ///     geohash::Neighbors {
    ///         n: GeoHash::try_from("9q60y60rht").unwrap(),
    ///         ne: GeoHash::try_from("9q60y60rhv").unwrap(),
    ///         e: GeoHash::try_from("9q60y60rhu").unwrap(),
    ///         se: GeoHash::try_from("9q60y60rhg").unwrap(),
    ///         s: GeoHash::try_from("9q60y60rhe").unwrap(),
    ///         sw: GeoHash::try_from("9q60y60rh7").unwrap(),
    ///         w: GeoHash::try_from("9q60y60rhk").unwrap(),
    ///         nw: GeoHash::try_from("9q60y60rhm").unwrap(),
    ///     }
    /// );
    /// ```
    pub fn neighbors(&self) -> Result<Neighbors<LEN>, GeohashError> {
        Ok(Neighbors {
            sw: self.neighbor(Direction::SW)?,
            s: self.neighbor( Direction::S)?,
            se: self.neighbor( Direction::SE)?,
            w: self.neighbor(Direction::W)?,
            e: self.neighbor(Direction::E)?,
            nw: self.neighbor(Direction::NW)?,
            n: self.neighbor(Direction::N)?,
            ne: self.neighbor(Direction::NE)?,
        })
    }
}

fn hash_value_of_char(c: char) -> Result<usize, GeohashError> {
    let ord = c as usize;
    if (48..=57).contains(&ord) {
        return Ok(ord - 48);
    } else if (98..=104).contains(&ord) {
        return Ok(ord - 88);
    } else if (106..=107).contains(&ord) {
        return Ok(ord - 89);
    } else if (109..=110).contains(&ord) {
        return Ok(ord - 90);
    } else if (112..=122).contains(&ord) {
        return Ok(ord - 91);
    }
    Err(GeohashError::InvalidHashCharacter(c))
}

mod error;
mod neighbors;
