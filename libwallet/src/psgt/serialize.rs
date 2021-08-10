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

//! # PSGT Serialization
//!
//! Defines traits used for (de)serializing PSGT values into/from raw
//! bytes in PSGT key-value pairs.

use crate::psgt;
use crate::psgt::encode::{self, serialize, Decodable};

/// A trait for serializing a value as raw data for insertion into PSGT
/// key-value pairs.
pub trait Serialize {
	/// Serialize a value as raw data.
	fn serialize(&self) -> Vec<u8>;
}

/// A trait for deserializing a value from raw data in PSGT key-value pairs.
pub trait Deserialize: Sized {
	/// Deserialize a value from raw data.
	fn deserialize(bytes: &[u8]) -> Result<Self, encode::Error>;
}
