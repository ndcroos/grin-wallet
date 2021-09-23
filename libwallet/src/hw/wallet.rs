
pub trait DeviceCallback {


}

pub struct WalletDeviceCallback {
	pub wallet : Wallet;
}

impl WalletDeviceCallback {

	pub fn on_button_request(code: u64) -> Result<(), Error>{
		wall.on_device_button_request(code);
	}

	pub fn on_button_pressed() -> Result<(), Error>{
		wallet.on_device_button_pressed();
	}

	pub fn on_pin_request() -> Result<String, Error>{
		wallet.on_device_pin_request();
	}

	pub fn on_passphrase_request() -> Result<String, Error>{
		wallet.on_device_passphrase_request(on_device);
	}

	pub fn on_progress(event) -> Result<(), Error>{
		wallet.on_device_progress(event);
	}

}

impl Wallet {

	pub fn get_device_callback(self ) -> WalletDeviceCallback {
		if(!device_callback)
			//make new callback
		return self.device_callback
	}
	
	pub fn on_device_button_request(self, code: u64) -> Result<(), Error>{
		self.callback.on_device_button_request(code);
	}

	pub fn on_device_button_pressed(self) -> Result<(), Error>{
		// if not null
		self.callback.on_device_button_pressed();
	}

	pub fn on_device_passphrase_request(self) -> Result<(), Error>{
		// if not null
		self.callback.on_device_passphrase_request();
	}

	pub fn on_device_progress(self) -> Result<(), Error>{
		// if not null
		self.callback.on_device_progress();
	}

}

// wallet 2

pub fn device_name_option() -> Result<(), Error> {


}

pub fn device_derivation_path_option() -> Result<(), Error> {


}

pub fn restore(){
	lookup_device(device_name);
	hwdev.set_name(device_name);
	hwdev.set_derivation_path(device_derivation_path);
	hwdev.set_callback();
	// Create account from device
	
}

pub fn reconnect_device(){
	lookup_device(device_name);
	hwdev.set_name(device_name);
	hwdev.set_derivation_path(device_derivation_path);
	hwdev.set_callback();
}

/*

	if device_type == LEDGER || device_type = TREZOR
		// Initialize device
*/

pub fn lookup_device(device_description) -> {
	if(!devices_registered)
	{
		devices_registered = true;
		register_devices();
	}
	hw::get_device(device_descriptor);
}