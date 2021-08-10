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

use crate::psgt;
use crate::psgt::encode;
use crate::psgt::raw;

/// A trait that describes a PSGT key-value map.
pub trait Map {
	/// Attempt to insert a key-value pair.
	fn insert_pair(&mut self, pair: raw::Pair) -> Result<(), encode::Error>;

	/// Attempt to get all key-value pairs.
	fn get_pairs(&self) -> Result<Vec<raw::Pair>, io::Error>;

	/// Attempt to merge with another key-value map of the same type.
	fn merge(&mut self, other: Self) -> Result<(), psgt::Error>;
}

mod global;
mod input;
mod output;
mod payment;

pub use self::global::Global;
pub use self::input::Input;
pub use self::output::Output;
pub use self::payment::Payment;
