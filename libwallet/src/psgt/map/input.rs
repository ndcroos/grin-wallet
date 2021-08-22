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

use std::io;

use crate::grin_core::core::transaction::OutputFeatures;
use crate::grin_core::libtx::secp_ser;
use crate::grin_util::secp::pedersen;

use crate::psgt;
use crate::psgt::encode;
use crate::psgt::map::Map;
use crate::psgt::raw;
use crate::psgt::serialize::Deserialize;
use crate::psgt::{error, Error};

const PSGT_OUTPUT_FEATURES: u8 = 0x00;
const PSGT_COMMIT: u8 = 0x00;

/// A key-value map for an input of the corresponding index in the unsigned
/// transaction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
	/// The features of the output being spent.
	pub features: OutputFeatures,
	/// The commit referencing the output being spent.
	#[serde(
		serialize_with = "secp_ser::as_hex",
		deserialize_with = "secp_ser::commitment_from_hex"
	)]
	pub commit: pedersen::Commitment,
}

impl Map for Input {
	fn insert_pair(&mut self, pair: raw::Pair) -> Result<(), encode::Error> {
		let raw::Pair {
			key: raw_key,
			value: raw_value,
		} = pair;

		match raw_key.type_value {
			PSGT_OUTPUT_FEATURES => {
				impl_psgt_insert_pair! {
					self.features <= <raw_key: _>|<raw_value: OutputFeatures>
				}
			}
			PSGT_COMMIT => {
				impl_psgt_insert_pair! {
					self.commit <= <raw_key: _>|<raw_value: pedersen::Commitment>
				}
			}
		}
		Ok(())
	}

	fn get_pairs(&self) -> Result<Vec<raw::Pair>, io::Error> {
		let mut rv: Vec<raw::Pair> = Default::default();

		// TODO

		for (key, value) in self.unknown.iter() {
			rv.push(raw::Pair {
				key: key.clone(),
				value: value.clone(),
			});
		}

		Ok(rv)
	}

	fn merge(&mut self, other: Self) -> Result<(), psgt::Error> {
		// TODO
		Ok(())
	}
}
