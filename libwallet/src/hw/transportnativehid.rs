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

//!  Wrapper for HID device.

use log::info;
use std::ffi::CStr;
use std::sync::{Arc, Mutex, Weak};

// Contains convenience methods for encoding and decoding numbers in Big-endian
use byteorder::{BigEndian, ReadBytesExt};
use cfg_if::cfg_if; // Macro, if Linux is used.
use futures::future;
use hidapi::HidDevice;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::io::Cursor;
use std::mem;
use trait_async::trait_async;

use nix::ioctl_read;

use crate::hw::apdu_types::*;
use crate::hw::ledger_error::*;
//use crate::hw::ledger_types::*;

const LEDGER_VID: u16 = 0x2c97; // Vendor ID
const LEDGER_USAGE_PAGE: u16 = 0xFFA0; //
const LEDGER_CHANNEL: u16 = 0x0101; //
const LEDGER_PACKET_SIZE: u8 = 64; // Size of the packet that is used to communicate with APDU packets.
const LEDGER_TIMEOUT: i32 = 10_000_000; //

///
unsafe impl Sync for TransportNativeHID {}

// Allow unused functions.
#[allow(dead_code)]
/// Wrapper for HID device
pub struct TransportNativeHID {
	api_mutex: Arc<Mutex<hidapi::HidApi>>,
	device: HidDevice,
	device_mutex: Mutex<i32>,
}

impl TransportNativeHID {
	#[cfg(not(target_os = "linux"))]
	fn find_ledger_device_path(api: &hidapi::HidApi) -> Result<&CStr, LedgerHIDError> {
		for device in api.device_list() {
			if device.vendor_id() == LEDGER_VID && device.usage_page() == LEDGER_USAGE_PAGE {
				return Ok(device.path());
			}
		}
		Err(LedgerHIDError::DeviceNotFound)
	}

	/// Find the Ledger device path.
	#[cfg(target_os = "linux")]
	fn find_ledger_device_path(api: &hidapi::HidApi) -> Result<&CStr, LedgerHIDError> {
		// look at all devices, find the one that matched the LEDGER_VID.
		for device in api.device_list() {
			if device.vendor_id() == LEDGER_VID {
				let usage_page = get_usage_page(&device.path())?;
				if usage_page == LEDGER_USAGE_PAGE {
					// If this all worked, return here.
					return Ok(device.path());
				}
			}
		}
		// If the method gets to this point, return with an error code.
		Err(LedgerHIDError::DeviceNotFound)
	}

	/// Create a new TransportNativeHID.
	pub fn new() -> Result<Self, LedgerHIDError> {
		let apiwrapper = HIDAPIWRAPPER.lock().expect("Could not lock api wrapper");
		let api_mutex = apiwrapper.get().expect("Error getting api_mutex");
		let api = api_mutex.lock().expect("Could not lock");

		// Find underlying device.
		let device_path = TransportNativeHID::find_ledger_device_path(&api)?;
		let device = api.open_path(&device_path)?;

		let ledger = TransportNativeHID {
			device,
			device_mutex: Mutex::new(0),
			api_mutex: api_mutex.clone(),
		};

		Ok(ledger)
	}

	/// Write a APDU command to the HID device
	fn write_apdu(&self, channel: u16, apdu_command: &[u8]) -> Result<i32, LedgerHIDError> {
		let command_length = apdu_command.len() as usize;
		let mut in_data = Vec::with_capacity(command_length + 2);
		in_data.push(((command_length >> 8) & 0xFF) as u8);
		in_data.push((command_length & 0xFF) as u8);
		// Appends all elements from apdu_command to in_data.
		in_data.extend_from_slice(&apdu_command);

		// Initialize buffer
		let mut buffer = vec![0u8; LEDGER_PACKET_SIZE as usize];
		buffer[0] = ((channel >> 8) & 0xFF) as u8; // channel big endian
		buffer[1] = (channel & 0xFF) as u8; // channel big endian
		buffer[2] = 0x05u8;

		for (sequence_idx, chunk) in in_data
			.chunks((LEDGER_PACKET_SIZE - 5) as usize)
			.enumerate()
		{
			buffer[3] = ((sequence_idx >> 8) & 0xFF) as u8; // sequence_idx big endian
			buffer[4] = (sequence_idx & 0xFF) as u8; // sequence_idx big endian
			buffer[5..5 + chunk.len()].copy_from_slice(chunk);

			info!("[{:3}] << {:}", buffer.len(), hex::encode(&buffer));

			let result = self.device.write(&buffer);

			match result {
				Ok(size) => {
					if size < buffer.len() {
						return Err(LedgerHIDError::Comm(
							"USB write error. Could not send whole message",
						));
					}
				}
				Err(x) => return Err(LedgerHIDError::Hid(x)),
			}
		}
		// If we get to here, return 1.
		Ok(1)
	}

	///
	fn read_apdu(&self, _channel: u16, apdu_answer: &mut Vec<u8>) -> Result<usize, LedgerHIDError> {
		let mut buffer = vec![0u8; LEDGER_PACKET_SIZE as usize];
		let mut sequence_idx = 0u16;
		let mut expected_apdu_len = 0usize;

		// Infinite loop.
		loop {
			//
			let res = self.device.read_timeout(&mut buffer, LEDGER_TIMEOUT)?;

			if (sequence_idx == 0 && res < 7) || res < 5 {
				return Err(LedgerHIDError::Comm("Read error. Incomplete header"));
			}

			// Create a new cursor, wrapping an in-memory buffer.
			// Allows to use Read and/or Write on them.
			let mut rdr = Cursor::new(&buffer);

			let _rcv_channel = rdr.read_u16::<BigEndian>()?;
			let _rcv_tag = rdr.read_u8()?;
			let rcv_seq_idx = rdr.read_u16::<BigEndian>()?;

			// TODO: Check why windows returns a different channel/tag
			//        if rcv_channel != channel {
			//            return Err(Box::from(format!("Invalid channel: {}!={}", rcv_channel, channel )));
			//        }
			//        if rcv_tag != 0x05u8 {
			//            return Err(Box::from("Invalid tag"));
			//        }

			if rcv_seq_idx != sequence_idx {
				return Err(LedgerHIDError::Comm("Invalid sequence idx"));
			}

			if rcv_seq_idx == 0 {
				expected_apdu_len = rdr.read_u16::<BigEndian>()? as usize;
			}

			let available: usize = buffer.len() - rdr.position() as usize;
			let missing: usize = expected_apdu_len - apdu_answer.len();
			let end_p = rdr.position() as usize + std::cmp::min(available, missing);

			let new_chunk = &buffer[rdr.position() as usize..end_p];

			info!("[{:3}] << {:}", new_chunk.len(), hex::encode(&new_chunk));

			apdu_answer.extend_from_slice(new_chunk);

			if apdu_answer.len() >= expected_apdu_len {
				return Ok(apdu_answer.len());
			}

			sequence_idx += 1;
		}
	}

	///
	pub fn exchange(&self, command: &APDUCommand) -> Result<APDUAnswer, LedgerHIDError> {
		println!("TransportNativeHID exchange");
		let _guard = self.device_mutex.lock().unwrap();

		self.write_apdu(LEDGER_CHANNEL, &command.serialize())?;

		let mut answer: Vec<u8> = Vec::with_capacity(256);
		let res = self.read_apdu(LEDGER_CHANNEL, &mut answer)?;

		if res < 2 {
			return Err(LedgerHIDError::Comm("response was too short"));
		}

		Ok(APDUAnswer::from_answer(answer))
	}

	///
	pub fn close() {
		extern crate hidapi;
	}
}

cfg_if! {
if #[cfg(target_os = "linux")] {
	const HID_MAX_DESCRIPTOR_SIZE: usize = 4096; //

	/// Use the data layount representation that C uses: order, size, alignment of fields.
	#[repr(C)]
	pub struct HidrawReportDescriptor {
		size: u32,
		value: [u8; HID_MAX_DESCRIPTOR_SIZE],
	}

	///
	fn get_usage_page(device_path: &CStr) -> Result<u16, LedgerHIDError>
	{
		// #define HIDIOCGRDESCSIZE	_IOR('H', 0x01, int)
		// #define HIDIOCGRDESC		_IOR('H', 0x02, struct HidrawReportDescriptor)

		//
		ioctl_read!(hid_read_descr_size, b'H', 0x01, libc::c_int);
		ioctl_read!(hid_read_descr, b'H', 0x02, HidrawReportDescriptor);

		use std::os::unix::{fs::OpenOptionsExt, io::AsRawFd};
		use std::fs::OpenOptions;

		let file_name = device_path.to_str()?;
		let file = OpenOptions::new()
			.read(true)
			.write(true)
			.custom_flags(libc::O_NONBLOCK)
			.open(file_name)?;

		let mut desc_size = 0;

		unsafe {
			let fd = file.as_raw_fd();

			hid_read_descr_size(fd, &mut desc_size)?;
			let mut desc_raw_uninit = mem::MaybeUninit::<HidrawReportDescriptor>::new(HidrawReportDescriptor {
				size: desc_size as u32,
				value: [0u8; 4096]
			});
			hid_read_descr(fd, desc_raw_uninit.as_mut_ptr())?;
			let desc_raw = desc_raw_uninit.assume_init();

			let data = &desc_raw.value[..desc_raw.size as usize];

			let mut data_len;
			let mut key_size;
			let mut i = 0 as usize;

			while i < desc_size as usize {
				let key = data[i];
				let key_cmd = key & 0xFC;

				if key & 0xF0 == 0xF0 {
					data_len = 0;
					key_size = 3;
					if i + 1 < desc_size as usize {
						data_len = data[i + 1];
					}
				} else {
					key_size = 1;
					data_len = key & 0x03;
					if data_len == 3 {
						data_len = 4;
					}
				}

				if key_cmd == 0x04 {
					let usage_page = match data_len {
						1 => u16::from(data[i + 1]),
						2 => (u16::from(data[i + 2] )* 256 + u16::from(data[i + 1])),
						_ => 0 as u16
					};

					return Ok(usage_page);
				}

				i += (data_len + key_size) as usize;
			}
		}
		Ok(0)
	}
}}

#[trait_async]
impl Exchange for TransportNativeHID {
	async fn exchange(&self, command: &APDUCommand) -> Result<APDUAnswer, TransportError> {
		println!("exchange");
		let call = self
			.exchange(command)
			.map_err(|_| TransportError::APDUExchangeError)?;
		future::ready(Ok(call)).await
	}
}

/////

lazy_static! {
	static ref HIDAPIWRAPPER: Arc<Mutex<HidApiWrapper>> =
		Arc::new(Mutex::new(HidApiWrapper::new()));
}

struct HidApiWrapper {
	_api: RefCell<Weak<Mutex<hidapi::HidApi>>>,
}

unsafe impl Send for HidApiWrapper {}

impl HidApiWrapper {
	///
	fn new() -> Self {
		HidApiWrapper {
			_api: RefCell::new(Weak::new()),
		}
	}

	///
	fn get(&self) -> Result<Arc<Mutex<hidapi::HidApi>>, LedgerHIDError> {
		let tmp = self._api.borrow().upgrade();

		if let Some(api_mutex) = tmp {
			return Ok(api_mutex);
		}

		let hidapi = hidapi::HidApi::new()?;
		let tmp = Arc::new(Mutex::new(hidapi));
		self._api.replace(Arc::downgrade(&tmp));
		Ok(tmp)
	}
}

///
pub fn list_all_devices() -> () {
	println!("list_all_devices");
	let apiwrapper = HIDAPIWRAPPER.lock().expect("Could not lock api wrapper");
	let api_mutex = apiwrapper.get().expect("Error getting api_mutex");
	let api = api_mutex.lock().expect("Could not lock");
	println!("Listing all devices: ");
	for device_info in api.device_list() {
		println!(
			"{:#?} - {:#x}/{:#x}/{:#x}/{:#x} {:#} {:#}",
			device_info.path(),
			device_info.vendor_id(),
			device_info.product_id(),
			device_info.usage_page(),
			device_info.interface_number(),
			device_info.manufacturer_string().unwrap_or_default(),
			device_info.product_string().unwrap_or_default()
		);
	}
}

///
pub fn ledger_device_path() -> () {
	let apiwrapper = HIDAPIWRAPPER.lock().expect("Could not lock api wrapper");
	let api_mutex = apiwrapper.get().expect("Error getting api_mutex");
	let api = api_mutex.lock().expect("Could not lock");
	// TODO: Extend to discover two devices
	let ledger_path =
		TransportNativeHID::find_ledger_device_path(&api).expect("Could not find a device");
	println!("{:?}", ledger_path);
}
