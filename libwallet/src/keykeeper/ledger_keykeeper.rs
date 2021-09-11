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

use crate::keykeeper_types::{KeyKeeper};
use crate::hw::LedgerDevice;
use crate::slate::Slate;
use crate::{Error, ErrorKind};

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
	pub fn sign_sender(&mut self, slate: &Slate, height: u64) -> Result<(), Error> {
		// Get inputs and outputs

		let tx = slate.tx.as_ref().expect("Error getting transaction body.");
                let tx_body = tx.body;
		let inputs = tx_body.inputs;
		let outputs = tx_body.outputs;
                let kernels = tx_body.kernels;
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
                let sender_input_params = new SenderInputParams;
		self.ledger.sign_sender(inputs, outputs, kernels, sender_input_params);
		Ok(())
	}

	pub fn sign_receiver(&mut self, slate: &Slate) -> Result<(), Error> {
                let tx = slate.tx.as_ref().expect("Error getting transaction body.");
                let tx_body = tx.body;
		let inputs = tx_body.inputs;
		let outputs = tx_body.outputs;
                let kernels = tx_body.kernels;
		self.ledger.sign_receiver(inputs, outputs, kernels);
		Ok(())
	}

	pub fn sign_finalize(&mut self, slate: &Slate) -> Result<(), Error> {
                //slate
		self.ledger.sign_finalize();
		Ok(())
	}

}
