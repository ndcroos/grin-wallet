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

use crate::grin_core::core::{Input, Output, TxKernel};
use crate::grin_keychain::BlindingFactor;
use crate::slate::PaymentInfo;
//use crate::hw::ledger_error::{Error};
use crate::{Error, ErrorKind};

pub trait KeyKeeper {
	// Send instruction for getting the number of slots
	fn get_num_slots(&mut self) -> Result<(), Error>;

	//
	fn get_rangeproof(&mut self) -> Result<(), Error>;
}

pub struct Slot {}

/// Store inputs and outputs
pub struct InputsOutputs {
	/// Identifier, mmr_index (if known), amount
	//input_ids: Vec<(Identifier, Option<u64>, u64)>,
	//output_ids: Vec<(Identifier, Option<u64>, u64)>,
	inputs: Vec<Input>,
	outputs: Vec<Output>,
}

// todo: put somewhere else
pub struct SenderInputParams {
	i_slot: i8,
}

pub struct SlateData {
	//kernel: ,
	//offset: BlindingFactor,
	pay_sig: Option<PaymentInfo>,
}

pub struct TxCommon {
	// Kernel consists of rangeproofs for the outputs, transaction excess and kernel signature
	// Usually a single kernel.
	kernel: Vec<TxKernel>,
	tko: BlindingFactor, // Transaction kernel offset
}

/*
pub struct TxMutual {
	paymentProofSignature: Option<PaymentInfo>,
}
*/
