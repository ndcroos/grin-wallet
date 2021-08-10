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

//use prelude::*;
use crate::psgt;
use crate::psgt::encode::{self, deserialize, serialize, Decodable, Encodable};
use crate::psgt::Error;
use core::fmt;

//use io;

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
