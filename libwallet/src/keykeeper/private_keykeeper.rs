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

//! General interface that should by implemented by a software keykeeper, or an interface that interacts with a hardware wallet.

use crate::grin_keychain::{BlindSum, BlindingFactor, Keychain};
use crate::slate::Slate;
//use crate::{Error, ErrorKind};
use crate::keykeeper_types::{KeyKeeper, SenderInputParams, TransactionData};
use crate::Error;

pub trait PrivateKeyKeeper {
	//
	fn sign_sender(&mut self, slate: &Slate, data: TransactionData) -> Result<(), Error>;

	//
	fn sign_receiver(&mut self, slate: &Slate, data: TransactionData) -> Result<(), Error>;

	//
	fn sign_finalize(&mut self, slate: &Slate, data: TransactionData) -> Result<(), Error>;
}
