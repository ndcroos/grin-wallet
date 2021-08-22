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

use std::collections::btree_map;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::io::{self, Cursor, Read};

use core::cmp;

use crate::psgt;
use crate::psgt::encode;
use crate::psgt::encode::{Decodable};
use crate::psgt::map::Map;
use crate::psgt::raw;
use crate::psgt::Error;

const PSGT_VERSION: u8 = 0x00;

/// A key-value map for global data.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Global {
	/// The version number of this PSGT. If omitted, the version number is 0.
	pub version: u32,
	/// Unknown global key-value pairs.
	#[cfg_attr(
		feature = "serde",
		serde(with = "::serde_utils::btreemap_as_seq_byte_values")
	)]
	pub unknown: BTreeMap<raw::Key, Vec<u8>>,
}

impl Global {}

impl Map for Global {
	fn insert_pair(&mut self, pair: raw::Pair) -> Result<(), encode::Error> {
		let raw::Pair {
			key: raw_key,
			value: raw_value,
		} = pair;

		match raw_key.type_value {
			_ => match self.unknown.entry(raw_key) {
				btree_map::Entry::Vacant(empty_key) => {
					empty_key.insert(raw_value);
				}
				btree_map::Entry::Occupied(k) => {
					return Err(Error::DuplicateKey(k.key().clone()).into())
				}
			},
		}

		Ok(())
	}

        /
	fn get_pairs(&self) -> Result<Vec<raw::Pair>, io::Error> {
		let mut rv: Vec<raw::Pair> = Default::default();

		rv.push(raw::Pair {
			key: raw::Key {
				type_value: PSGT_GLOBAL_UNSIGNED_TX,
				key: vec![],
			},
			value: {
				// Manually serialized to ensure 0-input txs are serialized
				// without witnesses.
				let mut ret = Vec::new();
				self.unsigned_tx.version.encode(&mut ret)?;
				self.unsigned_tx.input.encode(&mut ret)?;
				self.unsigned_tx.output.encode(&mut ret)?;
				ret
			},
		});

		// Serializing version only for non-default value; otherwise test vectors fail
		if self.version > 0 {
			rv.push(raw::Pair {
				key: raw::Key {
					type_value: PSGT_GLOBAL_VERSION,
					key: vec![],
				},
				value: u32_to_array_le(self.version).to_vec(),
			});
		}

		for (key, value) in self.unknown.iter() {
			rv.push(raw::Pair {
				key: key.clone(),
				value: value.clone(),
			});
		}

		Ok(rv)
	}

	// TODO
	// Keep in mind that according to BIP 174 this function must be commutative, i.e.
	// A.merge(B) == B.merge(A)
	fn merge(&mut self, other: Self) -> Result<(), psgt::Error> {
		// BIP 174: The Combiner must remove any duplicate key-value pairs, in accordance with
		//          the specification. It can pick arbitrarily when conflicts occur.

		// Keeping the highest version
		self.version = cmp::max(self.version, other.version);
		Ok(())
	}
}

impl Decodable for Global {
	fn decode<D: io::Read>(d: D) -> Result<Self, encode::Error> {
		loop {
			match raw::Pair::decode(&mut d) {
				Ok(pair) => {}
			}
		}
	}
}
