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

//! Keykeeper interface for Ledger hardware wallet.

use crate::grin_keychain::{BlindSum, BlindingFactor, Keychain};
use crate::hw::LedgerDevice;
use crate::keykeeper_types::{KeyKeeper, SenderInputParams, TransactionData};
use crate::slate::Slate;
use crate::types::Context;
use crate::Error;

pub struct LedgerKeyKeeper {
	ledger: LedgerDevice,
}

impl KeyKeeper for LedgerKeyKeeper {
	fn get_num_slots(&mut self) -> Result<(), Error> {
		let slotsCount = self.ledger.get_num_slots();
		Ok(())
	}

	fn get_rangeproof(&mut self) -> Result<(), Error> {
		self.ledger.get_rangeproof();
		Ok(())
	}
}

impl LedgerKeyKeeper {
	pub fn new() -> LedgerKeyKeeper {
		LedgerKeyKeeper {
			ledger: LedgerDevice::new(),
		}
	}

	// fee: from estimate_send_tx
	pub fn sign_sender<K: Keychain>(
		&mut self,
		keychain: &K,
		slate: &Slate,
		height: u64,
	) -> Result<(), Error> {
		let keychain = w.keychain(keychain_mask)?;

		// Get inputs and outputs
		let tx = slate.tx.as_ref().expect("Error getting transaction body.");
		let tx_body = tx.body;
		let inputs = tx_body.inputs;
		let outputs = tx_body.outputs;
		let kernels = tx_body.kernels;
		let data = TransactionData {
			inputs: inputs,
			outputs: outputs,
			kernels: kernels,
			tko: None,
			proof_sig: None,
		};

		/*
				let mut inputs_outputs : InputsOutputs = InputsOutputs {
					inputs: match slate { tx.body.inputs?,
					outputs: slate.tx.body.outputs?
				};
		*/

		//let mut offset = slate.tx.offset?;
		let fee = slate.fee_fields;
		//let height = ;
		let payment_proof = &slate.payment_proof;
		let sender_input_params = SenderInputParams {};
		self.ledger.sign_sender(keychain, data, sender_input_params);

		Ok(())
	}

	pub fn sign_receiver(&mut self, slate: &Slate) -> Result<(), Error> {
		let keychain = w.keychain(keychain_mask)?;

		let tx = slate.tx.as_ref().expect("Error getting transaction body.");
		let tx_body = tx.body;
		let inputs = tx_body.inputs;
		let outputs = tx_body.outputs;
		let kernels = tx_body.kernels;
		let data = TransactionData {
			inputs: inputs,
			outputs: outputs,
			kernels: kernels,
			tko: None,
			proof_sig: None,
		};
		self.ledger.sign_receiver(keychain, data);

		Ok(())
	}

	pub fn sign_finalize(&mut self, slate: &Slate) -> Result<(), Error> {
		let keychain = w.keychain(keychain_mask)?;

		let tx = slate.tx.as_ref().expect("Error getting transaction body.");
		let tx_body = tx.body;
		let inputs = tx_body.inputs;
		let outputs = tx_body.outputs;
		let kernels = tx_body.kernels;
		//slate
		let data = TransactionData {
			inputs: inputs,
			outputs: outputs,
			kernels: kernels,
			tko: None,
			proof_sig: None,
		};
		self.ledger.sign_finalize(keychain, data);

		Ok(())
	}

	pub fn get_commitment(&mut self,) -> ()
	{


	}

	pub fn adjust_offset(&mut self,) -> ()
	{
		let offset = ;
		self.ledger.adjust_offset();
	}

	pub fn get_payment_proof(&mut self,) -> ()
	{
		let account = ;	
		let value = ;
		let commitment = ;
		let sender_address
		let data = ;
		self.ledger.get_payment_proof();
	}

	pub fn select_input(&mut self,) -> ()
	{
		let id = ;
		let value = ;
		let switch_commitment_type = ;
		let data = ;
		self.ledger.select_input();
	}

	pub fn select_output(&mut self,) -> ()
	{
		let account = ;
		let data = ;
		self.ledger.select_output();
	}

	pub fn get_private_nonce(&mut self,) -> ()
	{
		let account = ;
		let private_nonce = ;
		let data = ;
		self.ledger.get_private_nonce();
	}


	pub fn get_pubkey(&mut self,) -> ()
	{
		let secp256k1_compressed_key = ;
		self.ledger.get_pubkey();
	}

	pub fn get_account_pubkey(&mut self,) -> ()
	{
		let account = ;
		let data = ;
		self.ledger.get_account_pubkey();
	}

	pub fn get_aes_key(&mut self,) -> ()
	{

		let data;
		self.ledger.get_aes_key();
	}

	pub fn get_blindingfactor_pubkey(&mut self,) -> ()
	{

		self.ledger.get_blindingfactor_pubkey();
	}

	pub fn get_kernel(&mut self,) -> ()
	{
		let sec_nonce = ;
		let pub_nonce = ;
		let secp256k1_compressed_key = ;
		let kernel_type = ;
		let lock_height = ;
		let relative_height = ;
		let commitment = ;
		let receiver_sig = ;
		let data = ;
		self.ledger.get_kernel();
	}

}
