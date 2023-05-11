use std::fmt;

use hidapi_rusb as hid;

use crate::transport::error::Error;
use crate::transport::protocol::{Link, Protocol, ProtocolV1};
use crate::transport::{derive_model, AvailableDeviceTransport, ProtoMessage, Transport};
use crate::AvailableDevice;

mod constants {
	//! A collection of constants related to the HID protocol.
	pub use crate::transport::constants::*;

	pub const WIRELINK_USAGE: u16 = 0xFF00;
	pub const WIRELINK_INTERFACE: isize = 0;
	pub const DEBUGLINK_USAGE: u16 = 0xFF01;
	pub const DEBUGLINK_INTERFACE: isize = 1;
}

/// The chunk size for the serial protocol.
const CHUNK_SIZE: usize = 64;

/// The read timeout.
const READ_TIMEOUT_MS: u64 = 100000;

/// There are two different HID link protocol versions.
#[derive(Debug)]
enum HidVersion {
	V1,
	V2,
}

/// An available transport for connecting with a device.
#[derive(Debug)]
pub struct AvailableHidTransport {
	pub serial_nb: String,
}

impl fmt::Display for AvailableHidTransport {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "HID (serial nb: {})", &self.serial_nb)
	}
}

/// An actual serial HID USB link to a device over which bytes can be sent.
pub struct HidLink {
	hid_version: HidVersion,
	_hid_manager: hid::HidApi,
	handle: Option<hid::HidDevice>,
}

impl Drop for HidLink {
	fn drop(&mut self) {
		// Manually drop before manager is dropped.
		self.handle.take();
	}
}

impl Link for HidLink {
	fn write_chunk(&mut self, chunk: Vec<u8>) -> Result<(), Error> {
		debug_assert_eq!(CHUNK_SIZE, chunk.len());
		let payload = match self.hid_version {
			HidVersion::V1 => chunk,
			HidVersion::V2 => {
				let mut payload = vec![0];
				payload.extend(chunk);
				payload
			}
		};
		self.handle.as_mut().unwrap().write(payload.as_slice())?;
		Ok(())
	}

	fn read_chunk(&mut self) -> Result<Vec<u8>, Error> {
		let mut chunk = vec![0; 64];
		//TODO(stevenroose) have different timeouts for messages that do user input
		match self.handle.as_mut().unwrap().read_timeout(&mut chunk, READ_TIMEOUT_MS as i32) {
			Ok(64) => Ok(chunk),
			Err(_) => Err(Error::DeviceReadTimeout),
			Ok(chunk_size) => Err(Error::UnexpectedChunkSizeFromDevice(chunk_size)),
		}
	}
}

/// Derive from the HID device whether or not it is a debugable device or not.
/// It returns None for not-recognized devices.
fn derive_debug(dev: &hid::DeviceInfo) -> Option<bool> {
	if dev.usage_page() == constants::DEBUGLINK_USAGE
		|| dev.interface_number() == constants::DEBUGLINK_INTERFACE as i32
	{
		Some(true)
	} else if dev.usage_page() == constants::WIRELINK_USAGE
		|| dev.interface_number() == constants::WIRELINK_INTERFACE as i32
	{
		Some(false)
	} else {
		None
	}
}

/// Probe the HID version for a Trezor 1 device.
fn probe_hid_version(handle: &mut hid::HidDevice) -> Result<HidVersion, Error> {
	let mut w = vec![0xff; 65];
	w[0] = 0;
	w[1] = 63;
	if handle.write(w.as_slice())? == 65 {
		return Ok(HidVersion::V2);
	}
	let mut w = vec![0xff; 64];
	w[0] = 63;
	if handle.write(w.as_slice())? == 64 {
		return Ok(HidVersion::V1);
	}
	Err(Error::UnknownHidVersion)
}

/// An implementation of the Transport interface for HID devices.
pub struct HidTransport {
	protocol: ProtocolV1<HidLink>,
}

impl HidTransport {
	/// Find devices using the HID transport.
	pub fn find_devices(debug: bool) -> Result<Vec<AvailableDevice>, Error> {
		let hidman = hid::HidApi::new()?;
		let mut devices = Vec::new();
		for dev in hidman.device_list() {
			let dev_id = (dev.vendor_id(), dev.product_id());
			let model = match derive_model(dev_id) {
				Some(m) => m,
				None => continue,
			};
			if derive_debug(dev) != Some(debug) {
				continue;
			}
			let serial = match dev.serial_number() {
				Some(s) => s,
				None => continue,
			};

			devices.push(AvailableDevice {
				model,
				debug,
				transport: AvailableDeviceTransport::Hid(AvailableHidTransport {
					serial_nb: serial.into(),
				}),
			});
		}
		Ok(devices)
	}

	/// Connect to a device over the HID transport.
	pub fn connect(device: &AvailableDevice) -> Result<Box<dyn Transport>, Error> {
		let transport = match device.transport {
			AvailableDeviceTransport::Hid(ref t) => t,
			_ => panic!("passed wrong AvailableDevice in HidTransport::connect"),
		};

		// Traverse all actual devices again and find the matching one.
		let hidman = hid::HidApi::new()?;

		let mut handle = hidman
			.device_list()
			.find_map(|dev| {
				let dev_id = (dev.vendor_id(), dev.product_id());
				if derive_model(dev_id) == Some(device.model)
					&& derive_debug(dev) == Some(device.debug)
					&& dev.serial_number() == Some(transport.serial_nb.as_str())
				{
					Some(hidman.open(dev.vendor_id(), dev.product_id()))
				} else {
					None
				}
			})
			.ok_or(Error::DeviceNotFound)??;

		let hid_version = probe_hid_version(&mut handle)?;
		Ok(Box::new(HidTransport {
			protocol: ProtocolV1 {
				link: HidLink {
					_hid_manager: hidman,
					hid_version,
					handle: Some(handle),
				},
			},
		}))
	}
}

impl super::Transport for HidTransport {
	fn session_begin(&mut self) -> Result<(), Error> {
		self.protocol.session_begin()
	}
	fn session_end(&mut self) -> Result<(), Error> {
		self.protocol.session_end()
	}

	fn write_message(&mut self, message: ProtoMessage) -> Result<(), Error> {
		self.protocol.write(message)
	}
	fn read_message(&mut self) -> Result<ProtoMessage, Error> {
		self.protocol.read()
	}
}
