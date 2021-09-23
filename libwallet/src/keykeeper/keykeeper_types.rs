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

//! Types associated with keykeeper

use crate::grin_core::core::{Input, Inputs, Output, TxKernel};
use crate::grin_keychain::BlindingFactor;
use crate::slate::PaymentInfo;
//use crate::hw::ledger_error::{Error};
use crate::{Error, ErrorKind};

pub trait KeyKeeper {
	// Send instruction for getting the number of slots
	//fn get_num_slots(&mut self) -> Result<(), Error>;

	//
	//fn get_rangeproof(&mut self) -> Result<(), Error>;

	/*
	/// Initiate tx as sender
	fn init_send_tx<'a, T: ?Sized, C, K>(
		w: &mut T,
		keychain_mask: Option<&SecretKey>,
		args: InitTxArgs,
		use_test_rng: bool,
	) -> Result<Slate, Error>
	where
		T: WalletBackend<'a, C, K>,
		C: NodeClient + 'a,
		K: Keychain + 'a,


		// Receive a tx as recipient
	fn receive_tx<'a, T: ?Sized, C, K>(
		w: &mut T,
		keychain_mask: Option<&SecretKey>,
		slate: &Slate,
		dest_acct_name: Option<&str>,
		use_test_rng: bool,
		hardware: bool,
	) -> Result<Slate, Error>
	where
		T: WalletBackend<'a, C, K>,
		C: NodeClient + 'a,
		K: Keychain + 'a


		fn finalize_tx<'a, T: ?Sized, C, K>(
		w: &mut T,
		keychain_mask: Option<&SecretKey>,
		slate: &Slate,
		post_automatically: bool,
		hardware: bool,
	) -> Result<Slate, Error>
	where
		T: WalletBackend<'a, C, K>,
		C: NodeClient + 'a,
		K: Keychain + 'a,

	*/
}

/// Different methods of serializing the packets needed for the hardware wallets.
pub enum HwSerializeFormat {
	/// Raw APDU packets, used in Ledger hardware wallet.
	APDU,
	/// Device independent format.
	PSGT,
}

pub struct Slot {}

/// Store inputs and outputs
/*
pub struct InputsOutputs {
	/// Identifier, mmr_index (if known), amount
	//input_ids: Vec<(Identifier, Option<u64>, u64)>,
	//output_ids: Vec<(Identifier, Option<u64>, u64)>,
	inputs: Vec<Input>,
	outputs: Vec<Output>,
}
*/

pub struct TransactionData {
	pub inputs: Inputs,
	pub outputs: Vec<Output>,
	pub kernels: Vec<TxKernel>,
	pub tko: BlindingFactor, // Transaction kernel offset
	pub proof_sig: Option<PaymentInfo>,
}

// todo: put somewhere else
pub struct SenderInputParams {
	//i_slot: i8,
}

pub struct SlateData {
	//kernel: ,
	//offset: BlindingFactor,
	pay_sig: Option<PaymentInfo>,
}

/*
pub struct TxCommon {
	// Kernel consists of rangeproofs for the outputs, transaction excess and kernel signature
	// Usually a single kernel.
	kernel: Vec<TxKernel>,
	tko: BlindingFactor, // Transaction kernel offset
}
*/

/*
pub struct TxMutual {
	paymentProofSignature: Option<PaymentInfo>,
}
*/
