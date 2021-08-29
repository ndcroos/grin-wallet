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

//! Payment proof voor PSGT

use crate::psgt;
use crate::psgt::encode;
use crate::psgt::map::Map;
use crate::psgt::raw;
use std::io;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Payment {}

impl Map for Payment {
	fn insert_pair(&mut self, pair: raw::Pair) -> Result<(), encode::Error> {
		Ok(())
	}

	fn get_pairs(&self) -> Result<Vec<raw::Pair>, io::Error> {
		let mut rv: Vec<raw::Pair> = Default::default();

		// TODO
		/*
		for (key, value) in self.unknown.iter() {
			rv.push(raw::Pair {
				key: key.clone(),
				value: value.clone(),
			});
		}
				*/
		Ok(rv)
	}

	fn merge(&mut self, other: Self) -> Result<(), psgt::Error> {
		// TODO
		Ok(())
	}
}
