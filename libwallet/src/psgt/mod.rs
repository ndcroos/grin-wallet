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

//use prelude::*;
//use io;

mod error;
pub use self::error::Error;

pub mod raw;

#[macro_use]
mod macros;

pub mod serialize;

pub mod encode;

mod map;
pub use self::map::{Global, Input, Map, Output};

use crate::grin_core::core::transaction::Transaction;

/// A Partially Signed Transaction.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PartiallySignedTransaction {
	/// The key-value pairs for all global data.
	//pub global: Global,
	/// The corresponding key-value map for each input in the unsigned
	/// transaction.
	pub inputs: Vec<Input>,
	/// The corresponding key-value map for each output in the unsigned
	/// transaction.
	pub outputs: Vec<Output>,
}

impl PartiallySignedTransaction {
	/// Create a PartiallySignedTransaction from an unsigned transaction, error
	/// if not unsigned
	pub fn from_unsigned_tx(tx: Transaction) -> Result<Self, self::Error> {
		Ok(PartiallySignedTransaction {
			inputs: vec![Default::default(); tx.input.len()],
			outputs: vec![Default::default(); tx.output.len()],
			global: Global::from_unsigned_tx(tx)?,
		})
	}

	/// Extract the Transaction from a PartiallySignedTransaction by filling in
	/// the available signature information in place.
	pub fn extract_tx(self) -> Transaction {
		let mut tx: Transaction = self.global.unsigned_tx;

		tx
	}

	/// Attempt to merge with another `PartiallySignedTransaction`.
	pub fn merge(&mut self, other: Self) -> Result<(), self::Error> {
		self.global.merge(other.global)?;

		for (self_input, other_input) in self.inputs.iter_mut().zip(other.inputs.into_iter()) {
			self_input.merge(other_input)?;
		}

		for (self_output, other_output) in self.outputs.iter_mut().zip(other.outputs.into_iter()) {
			self_output.merge(other_output)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use secp256k1::Secp256k1;

	use crate::psgt::map::{Global, Input, Output};
	use crate::psgt::raw;

	use psgt::raw::ProprietaryKey;
	use std::collections::BTreeMap;

	#[test]
	fn trivial_psgt() {
		let psgt = PartiallySignedTransaction {
			global: Global {
				unsigned_tx: Transaction {
					version: 2,
					input: vec![],
					output: vec![],
				},
				version: 0,
				unknown: BTreeMap::new(),
			},
			inputs: vec![],
			outputs: vec![],
		};
		assert_eq!(
			serialize_hex(&psgt),
			"70736274ff01000a0200000000000000000000"
		);
	}

	#[test]
	fn serialize_then_deserialize_output() {
		let secp = &Secp256k1::new();
	}

	#[test]
	fn serialize_then_deserialize_global() {
		let expected = Global {
			unsigned_tx: Transaction {
				version: 2,
				input: vec![TxIn {
					previous_output: OutPoint {
						txid: Txid::from_hex(
							"f61b1742ca13176464adb3cb66050c00787bb3a4eead37e985f2df1e37718126",
						)
						.unwrap(),
						vout: 0,
					},
				}],
				output: vec![TxOut { value: 99999699 }, TxOut { value: 100000000 }],
			},
			version: 0,
			unknown: Default::default(),
		};

		let actual: Global = deserialize(&serialize(&expected)).unwrap();

		assert_eq!(expected, actual);
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_serde_psgt() {
		//! Create a full PSGT value with various fields filled and make sure it can be JSONized.
		use crate::psgt::map::Input;

		// create some values to use in the PSBT
		let tx = Transaction {
			version: 1,
			input: vec![TxIn {
				previous_output: OutPoint {
					txid: Txid::from_hex(
						"e567952fb6cc33857f392efa3a46c995a28f69cca4bb1b37e0204dab1ec7a389",
					)
					.unwrap(),
					vout: 1,
				},
			}],
			output: vec![TxOut {
				value: 190303501938,
				script_pubkey: hex_script!("a914339725ba21efd62ac753a9bcd067d6c7a6a39d0587"),
			}],
		};

		let psbt = PartiallySignedTransaction {
			global: Global {
				version: 0,
				unknown: unknown.clone(),
			},
			inputs: vec![Input {
				partial_sigs: vec![(
					"0339880dc92394b7355e3d0439fa283c31de7590812ea011c4245c0674a685e883"
						.parse()
						.unwrap(),
					vec![8, 5, 4],
				)]
				.into_iter()
				.collect(),
				unknown: unknown.clone(),
				..Default::default()
			}],
			outputs: vec![Output {
				bip32_derivation: keypaths.clone(),
				proprietary: proprietary.clone(),
				unknown: unknown.clone(),
				..Default::default()
			}],
		};

		let encoded = ::serde_json::to_string(&psgt).unwrap();
		let decoded: PartiallySignedTransaction = ::serde_json::from_str(&encoded).unwrap();

		assert_eq!(psbt, decoded);
	}

	mod bip_vectors {

		#[cfg(feature = "base64")]
		use std::str::FromStr;

		use crate::psgt::map::{Global, Input, Map, Output};
		use crate::psgt::raw;
		use crate::psgt::{Error, PartiallySignedTransaction};
		use std::collections::BTreeMap;

		#[test]
		#[should_panic(expected = "InvalidMagic")]
		fn invalid_vector_1() {
			hex_psgt!("0200000001268171371edff285e937adeea4b37b78000c0566cbb3ad64641713ca42171bf6000000006a473044022070b2245123e6bf474d60c5b50c043d4c691a5d2435f09a34a7662a9dc251790a022001329ca9dacf280bdf30740ec0390422422c81cb45839457aeb76fc12edd95b3012102657d118d3357b8e0f4c2cd46db7b39f6d9c38d9a70abcb9b2de5dc8dbfe4ce31feffffff02d3dff505000000001976a914d0c59903c5bac2868760e90fd521a4665aa7652088ac00e1f5050000000017a9143545e6e33b832c47050f24d3eeb93c9c03948bc787b32e1300").unwrap();
		}

		#[test]
		#[should_panic(expected = "DuplicateKey(Key { type_value: 0, key: [] })")]
		fn invalid_vector_5() {
			hex_psgt!("70736274ff0100750200000001268171371edff285e937adeea4b37b78000c0566cbb3ad64641713ca42171bf60000000000feffffff02d3dff505000000001976a914d0c59903c5bac2868760e90fd521a4665aa7652088ac00e1f5050000000017a9143545e6e33b832c47050f24d3eeb93c9c03948bc787b32e1300000100fda5010100000000010289a3c71eab4d20e0371bbba4cc698fa295c9463afa2e397f8533ccb62f9567e50100000017160014be18d152a9b012039daf3da7de4f53349eecb985ffffffff86f8aa43a71dff1448893a530a7237ef6b4608bbb2dd2d0171e63aec6a4890b40100000017160014fe3e9ef1a745e974d902c4355943abcb34bd5353ffffffff0200c2eb0b000000001976a91485cff1097fd9e008bb34af709c62197b38978a4888ac72fef84e2c00000017a914339725ba21efd62ac753a9bcd067d6c7a6a39d05870247304402202712be22e0270f394f568311dc7ca9a68970b8025fdd3b240229f07f8a5f3a240220018b38d7dcd314e734c9276bd6fb40f673325bc4baa144c800d2f2f02db2765c012103d2e15674941bad4a996372cb87e1856d3652606d98562fe39c5e9e7e413f210502483045022100d12b852d85dcd961d2f5f4ab660654df6eedcc794c0c33ce5cc309ffb5fce58d022067338a8e0e1725c197fb1a88af59f51e44e4255b20167c8684031c05d1f2592a01210223b72beef0965d10be0778efecd61fcac6f79a4ea169393380734464f84f2ab30000000001003f0200000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000ffffffff010000000000000000036a010000000000000000").unwrap();
		}

		#[test]
		fn valid_vector_1() {
			let unserialized = PartiallySignedTransaction {
                global: Global {
                    unsigned_tx: Transaction {
                        version: 2,
                        input: vec![TxIn {
                            previous_output: OutPoint {
                                txid: Txid::from_hex(
                                    "f61b1742ca13176464adb3cb66050c00787bb3a4eead37e985f2df1e37718126",
                                ).unwrap(),
                                vout: 0,
                            },
                        }],
                        output: vec![
                            TxOut {
                                value: 99999699,
                                script_pubkey: hex_script!("76a914d0c59903c5bac2868760e90fd521a4665aa7652088ac"),
                            },
                            TxOut {
                                value: 100000000,
                                script_pubkey: hex_script!("a9143545e6e33b832c47050f24d3eeb93c9c03948bc787"),
                            },
                        ],
                    },
                    version: 0,
                    unknown: BTreeMap::new(),
                },
                inputs: vec![Input {
                    non_witness_utxo: Some(Transaction {
                        version: 1,
                        input: vec![TxIn {
                            previous_output: OutPoint {
                                txid: Txid::from_hex(
                                    "e567952fb6cc33857f392efa3a46c995a28f69cca4bb1b37e0204dab1ec7a389",
                                ).unwrap(),
                                vout: 1,
                            },
                        },
                        TxIn {
                            previous_output: OutPoint {
                                txid: Txid::from_hex(
                                    "b490486aec3ae671012dddb2bb08466bef37720a533a894814ff1da743aaf886",
                                ).unwrap(),
                                vout: 1,
                            },
                        }],
                        output: vec![
                            TxOut {
                                value: 200000000,
                                script_pubkey: hex_script!("76a91485cff1097fd9e008bb34af709c62197b38978a4888ac"),
                            },
                            TxOut {
                                value: 190303501938,
                                script_pubkey: hex_script!("a914339725ba21efd62ac753a9bcd067d6c7a6a39d0587"),
                            },
                        ],
                    }),
                    ..Default::default()
                },],
                outputs: vec![
                    Output {
                        ..Default::default()
                    },
                    Output {
                        ..Default::default()
                    },
                ],
            };

			let base16str = "70736274ff0100750200000001268171371edff285e937adeea4b37b78000c0566cbb3ad64641713ca42171bf60000000000feffffff02d3dff505000000001976a914d0c59903c5bac2868760e90fd521a4665aa7652088ac00e1f5050000000017a9143545e6e33b832c47050f24d3eeb93c9c03948bc787b32e1300000100fda5010100000000010289a3c71eab4d20e0371bbba4cc698fa295c9463afa2e397f8533ccb62f9567e50100000017160014be18d152a9b012039daf3da7de4f53349eecb985ffffffff86f8aa43a71dff1448893a530a7237ef6b4608bbb2dd2d0171e63aec6a4890b40100000017160014fe3e9ef1a745e974d902c4355943abcb34bd5353ffffffff0200c2eb0b000000001976a91485cff1097fd9e008bb34af709c62197b38978a4888ac72fef84e2c00000017a914339725ba21efd62ac753a9bcd067d6c7a6a39d05870247304402202712be22e0270f394f568311dc7ca9a68970b8025fdd3b240229f07f8a5f3a240220018b38d7dcd314e734c9276bd6fb40f673325bc4baa144c800d2f2f02db2765c012103d2e15674941bad4a996372cb87e1856d3652606d98562fe39c5e9e7e413f210502483045022100d12b852d85dcd961d2f5f4ab660654df6eedcc794c0c33ce5cc309ffb5fce58d022067338a8e0e1725c197fb1a88af59f51e44e4255b20167c8684031c05d1f2592a01210223b72beef0965d10be0778efecd61fcac6f79a4ea169393380734464f84f2ab300000000000000";

			assert_eq!(serialize_hex(&unserialized), base16str);
			assert_eq!(unserialized, hex_psbt!(base16str).unwrap());

			#[cfg(feature = "base64")]
			{
				let base64str = "cHNidP8BAHUCAAAAASaBcTce3/KF6Tet7qSze3gADAVmy7OtZGQXE8pCFxv2AAAAAAD+////AtPf9QUAAAAAGXapFNDFmQPFusKGh2DpD9UhpGZap2UgiKwA4fUFAAAAABepFDVF5uM7gyxHBQ8k0+65PJwDlIvHh7MuEwAAAQD9pQEBAAAAAAECiaPHHqtNIOA3G7ukzGmPopXJRjr6Ljl/hTPMti+VZ+UBAAAAFxYAFL4Y0VKpsBIDna89p95PUzSe7LmF/////4b4qkOnHf8USIk6UwpyN+9rRgi7st0tAXHmOuxqSJC0AQAAABcWABT+Pp7xp0XpdNkCxDVZQ6vLNL1TU/////8CAMLrCwAAAAAZdqkUhc/xCX/Z4Ai7NK9wnGIZeziXikiIrHL++E4sAAAAF6kUM5cluiHv1irHU6m80GfWx6ajnQWHAkcwRAIgJxK+IuAnDzlPVoMR3HyppolwuAJf3TskAinwf4pfOiQCIAGLONfc0xTnNMkna9b7QPZzMlvEuqFEyADS8vAtsnZcASED0uFWdJQbrUqZY3LLh+GFbTZSYG2YVi/jnF6efkE/IQUCSDBFAiEA0SuFLYXc2WHS9fSrZgZU327tzHlMDDPOXMMJ/7X85Y0CIGczio4OFyXBl/saiK9Z9R5E5CVbIBZ8hoQDHAXR8lkqASECI7cr7vCWXRC+B3jv7NYfysb3mk6haTkzgHNEZPhPKrMAAAAAAAAA";
				assert_eq!(
					PartiallySignedTransaction::from_str(base64str).unwrap(),
					unserialized
				);
				assert_eq!(base64str, unserialized.to_string());
				assert_eq!(
					PartiallySignedTransaction::from_str(base64str).unwrap(),
					hex_psbt!(base16str).unwrap()
				);
			}
		}

		#[test]
		fn valid_vector_2() {}

		#[test]
		fn valid_vector_3() {}
	}
}
