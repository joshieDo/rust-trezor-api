//! # Trezor API library
//!
//! ## Connecting
//!
//! Use the public top-level methods `find_devices()` and `unique()` to find devices.  When using
//! `find_devices()`, a list of different available devices is returned.  To connect to one or more
//! of them, use their `connect()` method.
//!
//! ## Logging
//!
//! We use the log package interface, so any logger that supports log can be attached.
//! Please be aware that `trace` logging can contain sensitive data.
//!

#[cfg(feature = "f_bitcoin")]
extern crate bitcoin;
#[cfg(feature = "f_bitcoin")]
extern crate bitcoin_bech32;
#[cfg(feature = "f_bitcoin")]
extern crate bitcoin_hashes;
#[cfg(feature = "f_bitcoin")]
extern crate secp256k1;
#[cfg(feature = "f_bitcoin")]
extern crate unicode_normalization;

#[cfg(feature = "f_ethereum")]
extern crate primitive_types;

extern crate byteorder;
extern crate hex;
extern crate rusb;
#[macro_use]
extern crate log;
extern crate protobuf;

mod messages;
mod transport;

pub mod client;
pub mod error;
pub mod protos;
#[cfg(feature = "f_bitcoin")]
pub mod utils;

mod flows {
	#[cfg(feature = "f_bitcoin")]
	pub mod sign_tx;
}

pub use client::{
	ButtonRequest, ButtonRequestType, EntropyRequest, Features, InputScriptType, InteractionType,
	PassphraseRequest, PinMatrixRequest, PinMatrixRequestType, Trezor, TrezorResponse, WordCount,
};
pub use error::{Error, Result};
#[cfg(feature = "f_bitcoin")]
pub use flows::sign_tx::SignTxProgress;
pub use messages::TrezorMessage;

use std::fmt;

/// The different kind of Trezor device models.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Model {
	Trezor1,
	Trezor2,
	Trezor2Bl,
}

impl fmt::Display for Model {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(match self {
			Model::Trezor1 => "Trezor 1",
			Model::Trezor2 => "Trezor 2",
			Model::Trezor2Bl => "Trezor 2 Bootloader",
		})
	}
}

/// A device found by the `find_devices()` method.  It can be connected to using the `connect()`
/// method.
#[derive(Debug)]
pub struct AvailableDevice {
	pub model: Model,
	pub debug: bool,
	transport: transport::AvailableDeviceTransport,
}

impl fmt::Display for AvailableDevice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} (transport: {}) (debug: {})", self.model, &self.transport, self.debug)
	}
}

impl AvailableDevice {
	/// Connect to the device.
	pub fn connect(self) -> Result<Trezor> {
		let transport = transport::connect(&self).map_err(Error::TransportConnect)?;
		Ok(client::trezor_with_transport(self.model, transport))
	}
}

/// Search for all available devices.
/// Most devices will show up twice both either debugging enables or disabled.
pub fn find_devices(debug: bool) -> Result<Vec<AvailableDevice>> {
	let mut devices = Vec::new();
	use transport::webusb::WebUsbTransport;
	devices.extend(WebUsbTransport::find_devices(debug).map_err(Error::TransportConnect)?);
	Ok(devices)
}

/// Try to get a single device.  Optionally specify whether debug should be enabled or not.
/// Can error if there are multiple or no devices available.
/// For more fine-grained device selection, use `find_devices()`.
/// When using USB mode, the device will show up both with debug and without debug, so it's
/// necessary to specify the debug option in order to find a unique one.
pub fn unique(debug: bool) -> Result<Trezor> {
	let mut devices = find_devices(debug)?;
	match devices.len() {
		0 => Err(Error::NoDeviceFound),
		1 => Ok(devices.remove(0).connect()?),
		_ => {
			debug!("Trezor devices found: {:?}", devices);
			Err(Error::DeviceNotUnique)
		}
	}
}
