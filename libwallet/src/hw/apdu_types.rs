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

//! Types used with the APDU standard, from communicating with the Ledger device.

use crate::ledger_error::*;
use trait_async::trait_async;

#[derive(Debug)]
/// Commands follow the ISO/IEC 7816-4 smartcard protocol.
pub struct APDUCommand {
	/// Protocol version
	pub cla: u8,
	/// Instruction command
	pub ins: u8,
	/// Subcommand
	pub p1: u8,
	/// Command/Subcommand counter
	pub p2: u8,
	/// options, additional data
	pub data: Vec<u8>,
}

impl APDUCommand {
	///
	pub fn serialize(&self) -> Vec<u8> {
		let mut v = vec![self.cla, self.ins, self.p1, self.p2, self.data.len() as u8];
		v.extend(&self.data);
		v
	}
}

#[derive(Debug)]
/// Answer packet from Ledger device.
pub struct APDUAnswer {
	///
	pub data: Vec<u8>,
	///
	pub retcode: u16,
}

impl APDUAnswer {
	///
	pub fn from_answer(answer: Vec<u8>) -> APDUAnswer {
		let apdu_retcode =
			(u16::from(answer[answer.len() - 2]) << 8) + u16::from(answer[answer.len() - 1]);
		let apdu_data = &answer[..answer.len() - 2];

		APDUAnswer {
			data: apdu_data.to_vec(),
			retcode: apdu_retcode,
		}
	}
}

/// Transport struct
pub struct APDUTransport {
	/// Native rust transport
	pub transport_wrapper: Box<dyn Exchange>,
}

impl APDUTransport {
	/// Creates a native rust transport
	pub fn new(wrapper: impl Exchange + 'static) -> Self {
		Self {
			transport_wrapper: Box::new(wrapper),
		}
	}

	/// Use to talk to the ledger device
	pub async fn exchange(&self, command: &APDUCommand) -> Result<APDUAnswer, TransportError> {
		self.transport_wrapper.exchange(command).await
	}
}

/// Use this method to communicate with the ledger device
#[trait_async]
pub trait Exchange: Send + Sync {
	/// Use to talk to the ledger device
	async fn exchange(&self, command: &APDUCommand) -> Result<APDUAnswer, TransportError>;
}
