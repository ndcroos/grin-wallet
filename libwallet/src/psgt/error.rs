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

use crate::psgt::{encode, raw};
use core::fmt;

/// Ways that a Partially Signed Transaction might fail.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Error {
	/// Magic bytes for a PSGT must be the ASCII for "psgt" serialized in most
	/// significant byte order.
	InvalidMagic,
	/// The separator for a PSGT must be `0xff`.
	InvalidSeparator,
	/// Known keys must be according to spec.
	InvalidKey(raw::Key),
	/// Non-proprietary key type found when proprietary key was expected
	InvalidProprietaryKey,
	/// Keys within key-value map should never be duplicated.
	DuplicateKey(raw::Key),
	/// Signals that there are no more key-value pairs in a key-value map.
	NoMorePairs,
	/// Data inconsistency/conflicting data during merge procedure
	MergeConflict(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::InvalidKey(ref rkey) => write!(f, "invalid key: {}", rkey),
			Error::InvalidProprietaryKey => write!(
				f,
				"non-proprietary key type found when proprietary key was expected"
			),
			Error::DuplicateKey(ref rkey) => write!(f, "duplicate key: {}", rkey),
			Error::InvalidMagic => f.write_str("invalid magic"),
			Error::InvalidSeparator => f.write_str("invalid separator"),
			Error::NoMorePairs => f.write_str("no more key-value pairs for this psbt map"),
			Error::MergeConflict(ref s) => {
				write!(f, "Merge conflict: {}", s)
			}
		}
	}
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {}
