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

//! Functions and types for Ledger device

pub mod apdu_types;
pub mod ledger_error;
pub mod ledger_types;
pub mod ledgerdevice;
pub mod transportnativehid;

pub use self::apdu_types::*;
pub use self::ledger_error::*;
pub use self::ledger_types::*;
pub use self::ledgerdevice::*;
pub use self::transportnativehid::*;

/*
use cfg_if::cfg_if;

//
cfg_if::cfg_if! {
if #[cfg(target_os = "linux")] {
	#[macro_use]
	extern crate nix;
	extern crate libc;
	use std::mem;
} else {
	// Mock the type in other target_os
	mod nix {
		use thiserror::Error;
		#[derive(Clone, Debug, Error, Eq, PartialEq)]
		pub enum Error {}
	}
}}

*/
