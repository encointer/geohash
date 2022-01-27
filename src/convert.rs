use crate::{hash_value_of_char, GeoHash, GeohashError};
use alloc::string::String;
use core::{convert::TryFrom, ops::Deref};

impl<const LEN: usize> Deref for GeoHash<LEN> {
    type Target = [u8; LEN];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const LEN: usize> TryFrom<&str> for GeoHash<LEN> {
    type Error = GeohashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TryFrom::<&[u8]>::try_from(value.as_bytes())
    }
}

impl<const LEN: usize> TryFrom<String> for GeoHash<LEN> {
    type Error = GeohashError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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

impl<const LEN: usize> Into<[u8; LEN]> for GeoHash<LEN> {
    fn into(self) -> [u8; LEN] {
        self.0
    }
}

impl<const LEN: usize> Into<String> for GeoHash<LEN> {
    fn into(self) -> String {
        String::from_utf8(self.0.to_vec())
            .expect("Geohash can only be constructed from a subset of utf8-strings; qed")
    }
}
