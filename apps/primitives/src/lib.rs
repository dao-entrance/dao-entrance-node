#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]

use frame_support::{serde::Deserializer, Deserialize};

pub mod traits;
pub mod types;

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}
