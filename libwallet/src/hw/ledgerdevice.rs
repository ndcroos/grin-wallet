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

//! Wrapper for Ledger device.

use bincode;

use std::str;
use std::collections::BTreeMap;

use ed25519_dalek::PublicKey as DalekPublicKey;
use ed25519_dalek::Signature as DalekSignature;

use crate::types::Context;

use crate::grin_core::core::transaction::Transaction;
use crate::grin_core::core::{Inputs, Output, TxKernel, FeeFields};

use crate::hw::apdu_types::*;
use crate::hw::ledger_error::{Error, LedgerAppError};
use crate::hw::ledger_types::*;
use crate::hw::transportnativehid::*;

use crate::grin_util::secp::key::{PublicKey, SecretKey};
use crate::grin_util::secp::pedersen::Commitment;
use crate::grin_util::secp::Signature;
use crate::grin_util::{secp, static_secp_instance};
use crate::grin_keychain::{BlindSum, BlindingFactor, Keychain, Identifier, SwitchCommitmentType};

use crate::grin_keychain::Identifier;
use crate::keykeeper::SenderInputParams;
use crate::keykeeper_types::TransactionData;
use crate::slate::{PaymentInfo, Slate};

// Different instructions
// TODO

const INS_GET_VERSION: u8 = 0x03;
const INS_GET_APP_NAME: u8 = 0x04;
const INS_DEVICE_RESET: u8 = 0x05;
//const INS_PUT_KEY: u8 = 0x06;
//const INS_APP_INFO: u8 = 0x07;
const INS_GET_NUM_SLOTS: u8 = 0x08;
//const INS_GEN_KEY_DERIVATION: u8 = 0x00;
//const INS_GENERATE_KEYPAIR: u8 = 0x00;
//const INS_RESET: u8 = 0x00;
//const INS_GET_KEY: u8 = 0x00;

const INS_SEND: u8 = 0x0B;
const INS_RECEIVE: u8 = 0x0C; // TODO
const INS_GET_RANGEPROOF: u8 = 0x0D; // TODO

// Constants
const PROTOCOL_VERSION: u8 = 4;

const CLA_APP_INFO: u8 = 0xb0; //

const CLA_DEVICE_INFO: u8 = 0xe0; //
const INS_DEVICE_INFO: u8 = 0x01;

const USER_MESSAGE_CHUNK_SIZE: usize = 250; //

/// Definition of a LedgerDevice.
/// This will be used to access a Ledger hardware wallet.
pub struct LedgerDevice {
	/// The underlying HID device
	_ledger: TransportNativeHID,
}

impl LedgerDevice {
	/// Get the underlying HID device.
	pub fn new() -> LedgerDevice {
		LedgerDevice {
			_ledger: TransportNativeHID::new().expect("Could not get a device"),
		}
	}

	///
	pub fn init(&mut self) -> Result<(), Error> {
		self._ledger = TransportNativeHID::new().expect("Could not get a device");
		Ok(())
	}

	///
	fn connect(&mut self) -> Result<(), Error> {
		LedgerDevice::disconnect(self);
		//connect();
		LedgerDevice::reset(self);
		LedgerDevice::get_secret_keys(self);
		Ok(())
	}

	///
	fn connected(&mut self) -> bool {
		return false;
	}

	///
	fn disconnect(&mut self) -> Result<(), Error> {
		Ok(())
	}

	///
	pub fn get_secret_keys(&mut self) -> Result<(), Error> {
		println!("get_secret_keys");
		//LedgerDevice::send_simple(self, INS_GET_KEY, 0x02);
		Ok(())
	}

	///
	pub fn reset(&mut self) -> Result<(), Error> {
		//let cmd = LedgerDevice::set_command_header_noopt(self, INS_RESET, 0x00, 0x00);
		//self._ledger.exchange(&cmd);
		Ok(())
	}

	///
	pub fn send_simple(&mut self, ins: u8, p1: u8) -> () {
		let cmd = LedgerDevice::set_command_header_noopt(self, ins, p1, 0x00);
		self._ledger.exchange(&cmd);
	}

	///
	pub fn send_secret(&mut self, secret: String, offset: u8) -> () {
		//assert!(offset + 32 <= BUFFER_SEND_SIZE, "send_secret: out of bounds.");
	}

	///
	pub fn receive_secret(&mut self /*, offset : u8*/) -> () {
		//assert!(offset + 32 <= BUFFER_RECV_SIZE, "send_secret: out of bounds.");
	}

	/// Set command with optional data.
	fn set_command_header(
		&mut self,
		instruction: u8,
		p1: u8,
		p2: u8,
		data: Vec<u8>,
	) -> APDUCommand {
		let cmd = APDUCommand {
			cla: PROTOCOL_VERSION,
			ins: instruction,
			p1: p1,
			p2: p2,
			data: data,
		};
		return cmd;
	}

	/// Set command with no optional data.
	fn set_command_header_noopt(&mut self, instruction: const u8, p1: const u8, p2: const u8) -> APDUCommand {
		let cmd = APDUCommand {
			cla: PROTOCOL_VERSION,
			ins: instruction,
			p1: p1,
			p2: p2,
			data: Vec::new(),
		};
		return cmd;
	}

	///
	pub fn generate_keys(&mut self, /*pub, sec, recoveryKey,*/ recover: bool) -> () {
		//LedgerDevice::send_simple(self, INS_GENERATE_KEYPAIR, 0x00);

		LedgerDevice::receive_secret(self);
	}

	///
	pub fn generate_key_derivation(&mut self /*pub, sec, derivation*/) -> () {
		println!("Generate key derivation.");
		//let cmd = LedgerDevice::set_command_header_noopt(self, INS_GEN_KEY_DERIVATION, 0x00, 0x00);

		LedgerDevice::send_secret(self, "".to_string(), 0);

		//self._ledger.exchange(&cmd);

		LedgerDevice::receive_secret(self);
	}

	/// Get the firmware's version.
	pub fn get_version(&mut self) -> () {
		println!("get_version");
		let cmd = LedgerDevice::set_command_header_noopt(self, INS_GET_VERSION, 0x00, 0x00);
		println!("cmd: {:?}", cmd);
		let result = self._ledger.exchange(&cmd).expect("Error during exchange");
		println!("{:?}", result);
	}

	/*
			/// Get the app name.
			fn get_app_name(&mut self) -> () {
				println!("get_app_name");
				let cmd = LedgerDevice::set_command_header_noopt(self, INS_GET_APP_NAME, 0x00, 0x00);
				println!("cmd: {:?}", cmd);
				let result = self._ledger.exchange(&cmd).expect("Error during exchange");
				println!("{:?}", result);
			}
	*/
	///
	pub fn get_public_key(&mut self) -> () {}

	///
	pub async fn get_app_name(&mut self) -> Result<(), LedgerAppError> {
		let _ledger = TransportNativeHID::new().expect("Could not get a device");
		let apdu_transport = APDUTransport::new(_ledger);
		//let cmd = LedgerDevice::set_command_header_noopt(self, INS_GET_APP_NAME, 0x00, 0x00);
		let cmd = APDUCommand {
			cla: 0xE0,
			ins: INS_GET_APP_NAME,
			p1: 0x00,
			p2: 0x00,
			data: Vec::new(),
		};
		println!("cmd: {:?}", cmd);
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		println!("description: {:?}", description);
		println!("response: {:?}", response);

		let app_name_bytes = &response.data[0..4];
		println!("app_name_bytes: {:?}", app_name_bytes);

		let app_name = str::from_utf8(app_name_bytes).map_err(|_e| LedgerAppError::Utf8)?;
		println!("app_name: {:?}", app_name);
		Ok(())
	}

	///
	pub async fn get_num_slots(&mut self) -> Result<(), LedgerAppError> {
		let _ledger = TransportNativeHID::new().expect("Could not get a device");
		let apdu_transport = APDUTransport::new(_ledger);
		//let cmd = LedgerDevice::set_command_header_noopt(self, INS_GET_NUM_SLOTS, 0x00, 0x00);
		let cmd = APDUCommand {
			cla: 0xE0,
			ins: INS_GET_NUM_SLOTS,
			p1: 0x00,
			p2: 0x00,
			data: Vec::new(),
		};
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		let num_slots_bytes = &response.data[0..4]; // TODO
		let num_slots = str::from_utf8(num_slots_bytes).map_err(|_e| LedgerAppError::Utf8)?;
		println!("num_slots_bytes: {:?}", num_slots_bytes);
		println!("num_slots: {:?}", num_slots);
		Ok(())
	}

	pub fn start_send_tx(self, 
			account: &str, 
			output: const Output, 
			input: const Input, 
			fee: const FeeFields, 
			receiver_address: const DalekPublicKey) 
			-> Result<(), Error>
	{
		let fee_amount = fee.f;
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok(())
	}

	pub fn get_pubkey(self) -> Result< , Error>
	{
		let secp256k1_compressed_key = ;
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	pub fn get_account_pubkey(self, 
		account: const &str) 
		-> Result<(), Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	pub fn select_input(self, 
		id: const Identifier, 
		value: const u64, 
		switch_commitment_type: const SwitchCommitmentType) 
		-> Result<(), Error>
	{

		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok(())
	}

	pub fn select_output(self, account: const &str) -> Result<(), Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok(())
	}

	pub fn get_payment_proof(self, 
		account: const &str, 
		value: const u64, 
		commitment: const Commitment, 
		sender_address: const DalekPublicKey) 
		-> Result<(), Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		// Deserialize Ed25519 signature.
		Ok()
	}

	pub fn get_commitment(self, 
			account: const &str, 
			id: const Identifier, 
			value: const u64, 
			switch_commitment_type: const SwitchCommitmentType) 
			-> Result<Commitment, Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		// Deserialize commitment
		let commitment : Commitment = ;
		Ok(commitment)
	}

	pub fn get_private_nonce(self, 
			account: const &str, 
			private_nonce: const ) 
			-> Result<(), Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	pub fn adjust_offset(self, offset: const BlindingFactor) -> Result<(), Error>
	{
		offset = [u8; SECRET_KEY_SIZE];
		let data = ;
		// No response
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok(())
	}

	pub fn get_aes_key(self) -> Result<(), Error>
	{
		let data;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	pub fn get_blindingfactor_pubkey(self) -> Result<(), Error>
	{
		// No data.

		// Response

		// Secp256k1 compressed key
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	pub fn get_kernel(
			self,
			sec_nonce: const &SecretKey, 
			pub_nonce: const &PublicKey, 
			secp256k1_compressed_key: const ,
			kernel_type: const KernelFeatures,
			lock_height: const u64,
			relative_height: const ,
			receiver_sig: const DalekSignature,
			) 
			-> Result< , Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		// Deserialize signer signature
		Ok()
	}

	pub fn get_tor_pub_key(self, account: const &str) -> Result<(), Error>
	{
		let data = account;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	pub fn get_tor_tx_sig(
		account: const &str, 
		value: const u64,
		commitment: const Commitment,
		sender_address: const DalekPublicKey,
	/// Receiver address,
		data:
		) -> Result< , Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	pub fn get_random_nonce(self) -> Result< , Error>
	{
		let data = ;
		let response = apdu_transport.exchange(&cmd).await?;
		let description = self.map_apdu_error_description(response.retcode);
		Ok()
	}

	/* Round 1*/
	///
	pub async fn sign_sender<K: Keychain>(
		&mut self,
		keychain: &K,
		context: &Context,
		data: TransactionData,
		sender_input_params: SenderInputParams,
	) -> Result<(), LedgerAppError> {
		// Convert data to binary, before sending to Ledger device.

		//let psgt = self.create_psgt(data);
		// serialize PSGT
		//serialize_hex(&psgt);

		// Set slate as data.
		//let xs: Vec<u8> = bincode::serialize(&slate).unwrap();
		//let cmd = LedgerDevice::set_command_header(self, INS_SEND, 0x00, 0x00, xs);

		// Return pub_nonce and commitment, generated from secret nonce on device.
		//let pub_once =
		//let commitment =

		// TODO
		/*
				let response = apdu_transport.exchange(&cmd).await?;
				if response.retcode != APDUErrorCodes::NoError as u16 {
					return Err(LedgerAppError::TransportError(
						TransportError::APDUExchangeError,
					));
				}
		*/

		// Send

		// Get hex
		//let hex = "70736274ff010"; // TODO
						   // deserialize PSGT
		//let deserialized = deserialize(hex);

		Ok(())
	}

	///
	pub async fn sign_sender_round2(&mut self) -> Result<(), LedgerAppError> {
		//let cmd = LedgerDevice::LedgerDevice::set_command_header_noopt(self, INS_SEND, 0x00, 0x00);

		// Verify receiver part
		// Verify PaymentProof signed by receiver.
		// Ask permission to transfer funds to receiver

		Ok(())
	}

	/// Returns payment nonce, proof signature,
	pub async fn sign_receiver<K: Keychain>(
		&mut self,
		keychain: &K,
		context: &Context,
		data: TransactionData,
	) -> Result<(), LedgerAppError> {
		//let cmd = LedgerDevice::set_command_header_noopt(self, INS_RECEIVE, 0x00, 0x00);

		// Set data
		let tx_info = Vec::new();
		//let psgt = self.create_psgt(data);

		// serialize PSGT
		//serialize_hex(&psgt);

		let cmd = APDUCommand {
			cla: 0xE0,
			ins: INS_RECEIVE,
			p1: 0x00,
			p2: 0x00,
			data: tx_info,
		};

		/*
				let response = apdu_transport.exchange(&cmd).await?;
				if response.retcode != APDUErrorCodes::NoError as u16 {
					return Err(LedgerAppError::TransportError(
						TransportError::APDUExchangeError,
					));
				}
		*/

		// Convert response data to information we need

		// Get hex
		//let hex = "70736274ff010"; // TODO

		// deserialize PSGT
		//let deserialized = deserialize(hex);

		// Get payment proof signature from response data.
		//let paymentProofSignature : DalekSignature =

		//kernel_commitment
		//pubnonce

		Ok(())
	}

	pub async fn sign_finalize<K: Keychain>(
		&mut self,
		keychain: &K,
		context: &Context,
		data: TransactionData,
	) -> Result<(), LedgerAppError> {
		//let psgt = self.create_psgt(data);
		// serialize PSGT
		//serialize_hex(&psgt);

		// Get hex
		//let hex = "70736274ff010"; // TODO

		// Convert response data to information we need
		// deserialize PSGT
		//let deserialized = deserialize(hex);

		Ok(())
	}

	/// Returns payment nonce, proof signature,
	pub async fn get_rangeproof(&mut self) -> Result<(), LedgerAppError> {
		let tx_info = Vec::new();
		let cmd = APDUCommand {
			cla: 0xE0,
			ins: INS_GET_RANGEPROOF,
			p1: 0x00,
			p2: 0x00,
			data: tx_info,
		};

		/*
				let response = apdu_transport.exchange(&cmd).await?;
				if response.retcode != APDUErrorCodes::NoError as u16 {
					return Err(LedgerAppError::TransportError(
						TransportError::APDUExchangeError,
					));
				}
		*/
		//let psgt = self.create_psgt(&inputs, &outputs, &kernels);
		// Send

		//serialize_hex(&psgt), base16str);

		//let deserialized = deserialize();

		Ok(())
	}

	/// Translate a retcode into an error message.
	pub fn map_apdu_error_description(&mut self, retcode: u16) -> &'static str {
		match retcode {
			0x6400 => "APDU_CODE_EXECUTION_ERROR - No information given (NV-Ram not changed)",
			0x6700 => "APDU_CODE_WRONG_LENGTH - Wrong length",
			0x6982 => "APDU_CODE_EMPTY_BUFFER",
			0x6983 => "APDU_CODE_OUTPUT_BUFFER_TOO_SMALL - ",
			0x6984 => "APDU_CODE_DATA_INVALID - data reversibly blocked (invalidated)",
			0x6985 => "APDU_CODE_CONDITIONS_NOT_SATISFIED - Conditions of use not satisfied",
			0x6986 => "APDU_CODE_COMMAND_NOT_ALLOWED - Command not allowed (no current EF)",
			0x6A80 => "APDU_CODE_BAD_KEY_HANDLE - The parameters in the data field are incorrect",
			0x6B00 => "APDU_CODE_INVALIDP1P2 - Wrong parameter(s) P1-P2",
			0x6D00 => "APDU_CODE_INS_NOT_SUPPORTED - Instruction code not supported or invalid",
			0x6E00 => "APDU_CODE_CLA_NOT_SUPPORTED - Class not supported",
			0x6F00 => "APDU_CODE_UNKNOWN - ",
			0x6F01 => "APDU_CODE_SIGN_VERIFY_ERROR - ",
			_ => "[APDU_ERROR] Unknown",
		}
	}

	/// Stream a long request in chunks
	pub async fn send_chunks(
		&mut self,
		apdu_transport: &APDUTransport,
		start_command: &APDUCommand,
		message: &[u8],
	) -> Result<APDUAnswer, LedgerAppError> {
		// Returns an iterator over a slice in chunks, with the given size.
		let chunks = message.chunks(USER_MESSAGE_CHUNK_SIZE);
		// If length is 0, empty message
		// If length is > 255, invalid message
		match chunks.len() {
			0 => return Err(LedgerAppError::InvalidEmptyMessage),
			n if n > 255 => return Err(LedgerAppError::InvalidMessageSize),
			_ => (),
		}

		//
		if start_command.p1 != ChunkPayloadType::Init as u8 {
			return Err(LedgerAppError::InvalidChunkPayloadType);
		}

		// If retcode isn't OK, map to error description.
		let mut response = apdu_transport.exchange(start_command).await?;
		if response.retcode != 0x9000 {
			return Err(LedgerAppError::AppSpecific(
				response.retcode,
				self.map_apdu_error_description(response.retcode)
					.to_string(),
			));
		}

		// Send message chunks
		let last_chunk_index = chunks.len() - 1;
		for (packet_idx, chunk) in chunks.enumerate() {
			//
			let mut p1 = ChunkPayloadType::Add as u8;
			// If the packet ID is equal to the last_chunck_index,
			// change p1 type as to be the last one
			if packet_idx == last_chunk_index {
				p1 = ChunkPayloadType::Last as u8
			}

			let command = APDUCommand {
				cla: start_command.cla,
				ins: start_command.ins,
				p1,
				p2: 0,
				data: chunk.to_vec(),
			};

			// response is of type APDUAnswer
			response = apdu_transport.exchange(&command).await?;
			if response.retcode != 0x9000 {
				return Err(LedgerAppError::AppSpecific(
					response.retcode,
					self.map_apdu_error_description(response.retcode)
						.to_string(),
				));
			}
		}

		// If we get to here, return the response.
		Ok(response)
	}
}

/// Only used for testing purposes. Set specific key on device.
fn put_keys() -> () {
	/*
		let command = APDUCommand {
			cla: PROTOCOL_VERSION,
			ins: INS_PUT_KEY,
			p1: 0x00,
			p2: 0x00,
			data: Vec::new(),
		};
	*/

	// exchange
}

/* Restart application. Check client and app compatibility. */
/*
fn reset() -> bool {
	let command = APDUCommand {
		cla: PROTOCOL_VERSION,
		ins: INS_DEVICE_RESET,
		p1: 0x00,
		p2: 0x00,
		data: Vec::new(),
	};

	// exchange

	return true;
}
*/
