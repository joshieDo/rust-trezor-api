//! # Error Handling

use std::error;
use std::fmt;

use hidapi_rusb as hid;
use rusb;

/// Trezor error.
#[derive(Debug)]
pub enum Error {
	/// Error from hidapi.
	Hid(hid::HidError),
	/// Error from ruusb.
	Usb(rusb::Error),
	/// Error from io
	IO(std::io::Error),
	/// The device to connect to was not found.
	DeviceNotFound,
	/// The device is no longer available.
	DeviceDisconnected,
	/// The HID version supported by the device was unknown.
	UnknownHidVersion,
	/// The device produced a data chunk of unexpected size.
	UnexpectedChunkSizeFromDevice(usize),
	/// Timeout expired while reading from device.
	DeviceReadTimeout,
	/// The device sent a chunk with a wrong magic value.
	DeviceBadMagic,
	/// The device sent a message with a wrong session id.
	DeviceBadSessionId,
	/// The device sent an unexpected sequence number.
	DeviceUnexpectedSequenceNumber,
	/// Received a non-existing message type from the device.
	InvalidMessageType(u32),
	/// Unable to determine device serial number.
	NoDeviceSerial,
}

impl From<hid::HidError> for Error {
	fn from(e: hid::HidError) -> Error {
		Error::Hid(e)
	}
}

impl From<rusb::Error> for Error {
	fn from(e: rusb::Error) -> Error {
		Error::Usb(e)
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error::IO(e)
	}
}

impl error::Error for Error {
	fn cause(&self) -> Option<&dyn error::Error> {
		match *self {
			Error::Hid(ref e) => Some(e),
			Error::Usb(ref e) => Some(e),
			_ => None,
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Hid(ref e) => fmt::Display::fmt(e, f),
			Error::Usb(ref e) => fmt::Display::fmt(e, f),
			Error::IO(ref e) => fmt::Display::fmt(e, f),
			Error::DeviceNotFound => write!(f, "the device to connect to was not found"),
			Error::DeviceDisconnected => write!(f, "the device is no longer available"),
			Error::UnknownHidVersion => write!(f, "HID version of the device unknown"),
			Error::UnexpectedChunkSizeFromDevice(_) => {
				write!(f, "the device produced a data chunk of unexpected size")
			}
			Error::DeviceReadTimeout => write!(f, "timeout expired while reading from device"),
			Error::DeviceBadMagic => write!(f, "the device sent chunk with wrong magic value"),
			Error::DeviceBadSessionId => {
				write!(f, "the device sent a message with a wrong session id")
			}
			Error::DeviceUnexpectedSequenceNumber => {
				write!(f, "the device sent an unexpected sequence number")
			}
			Error::InvalidMessageType(_) => {
				write!(f, "received a non-existing message type from the device")
			}
			Error::NoDeviceSerial => write!(f, "unable to determine device serial number"),
		}
	}
}
