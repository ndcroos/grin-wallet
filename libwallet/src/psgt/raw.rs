// Copyright 2021 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! #

//use crate::psgt;
use crate::psgt::encode::{
	self, deserialize, serialize, Decodable, Encodable, VarInt, MAX_VEC_SIZE,
};
use crate::psgt::Error;
use core::fmt;

use std::io;

/// A PSGT key in its raw byte form.
#[derive(Debug, PartialEq, Hash, Eq, Clone, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Key {
	/// The type of this PSGT key.
	pub type_value: u8,
	/// The key itself in raw byte form.
	#[cfg_attr(feature = "serde", serde(with = "::serde_utils::hex_bytes"))]
	pub key: Vec<u8>,
}

/// A PSGT key-value pair in its raw byte form.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pair {
	/// The key of this key-value pair.
	pub key: Key,
	/// The value of this key-value pair in raw byte form.
	#[cfg_attr(feature = "serde", serde(with = "::serde_utils::hex_bytes"))]
	pub value: Vec<u8>,
}

impl fmt::Display for Key {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "type: {:#x}, key: ", self.type_value)?;
		hex::format_hex(&self.key[..], f)
	}
}

impl Decodable for Key {
	fn decode<D: io::Read>(mut d: D) -> Result<Self, encode::Error> {
		let VarInt(byte_size): VarInt = Decodable::decode(&mut d)?;

		if byte_size == 0 {
			return Err(Error::NoMorePairs.into());
		}

		let key_byte_size: u64 = byte_size - 1;

		if key_byte_size > MAX_VEC_SIZE as u64 {
			return Err(encode::Error::OversizedVectorAllocation {
				requested: key_byte_size as usize,
				max: MAX_VEC_SIZE,
			});
		}

		let type_value: u8 = Decodable::decode(&mut d)?;

		let mut key = Vec::with_capacity(key_byte_size as usize);
		for _ in 0..key_byte_size {
			key.push(Decodable::decode(&mut d)?);
		}

		Ok(Key {
			type_value: type_value,
			key: key,
		})
	}
}

impl Encodable for Key {
	fn encode<S: io::Write>(&self, mut s: S) -> Result<usize, io::Error> {
		let mut len = 0;
		len += VarInt((self.key.len() + 1) as u64).encode(&mut s)?;

		len += self.type_value.encode(&mut s)?;

		for key in &self.key {
			len += key.encode(&mut s)?
		}

		Ok(len)
	}
}

impl Encodable for Pair {
	fn encode<S: io::Write>(&self, mut s: S) -> Result<usize, io::Error> {
		let len = self.key.encode(&mut s)?;
		Ok(len + self.value.encode(s)?)
	}
}

impl Decodable for Pair {
	fn decode<D: io::Read>(mut d: D) -> Result<Self, encode::Error> {
		Ok(Pair {
			key: Decodable::decode(&mut d)?,
			value: Decodable::decode(d)?,
		})
	}
}
