//! # Error Handling

use std::error;
use std::fmt;
use std::result;

#[cfg(feature = "f_bitcoin")]
use bitcoin;
#[cfg(feature = "f_bitcoin")]
use bitcoin::util::base58;
#[cfg(feature = "f_bitcoin")]
use bitcoin_hashes::sha256d;
use protobuf::error::ProtobufError;
#[cfg(feature = "f_bitcoin")]
use secp256k1;

use client::InteractionType;
use protos;
use transport;

/// Trezor error.
#[derive(Debug)]
pub enum Error {
	/// Less than one device was plugged in.
	NoDeviceFound,
	/// More than one device was plugged in.
	DeviceNotUnique,
	/// Transport error connecting to device.
	TransportConnect(transport::error::Error),
	/// Transport error while beginning a session.
	TransportBeginSession(transport::error::Error),
	/// Transport error while ending a session.
	TransportEndSession(transport::error::Error),
	/// Transport error while sending a message.
	TransportSendMessage(transport::error::Error),
	/// Transport error while receiving a message.
	TransportReceiveMessage(transport::error::Error),
	/// Received an unexpected message type from the device.
	UnexpectedMessageType(protos::MessageType), //TODO(stevenroose) type alias
	/// Error reading or writing protobuf messages.
	Protobuf(ProtobufError),
	/// A failure message was returned by the device.
	FailureResponse(protos::Failure),
	/// An unexpected interaction request was returned by the device.
	UnexpectedInteractionRequest(InteractionType),
	/// Error in Base58 decoding
	#[cfg(feature = "f_bitcoin")]
	Base58(base58::Error),
	/// The given Bitcoin network is not supported.
	UnsupportedNetwork,
	/// Provided entropy is not 32 bytes.
	InvalidEntropy,
	/// The device referenced a non-existing input or output index.
	TxRequestInvalidIndex(usize),
	/// The device referenced an unknown TXID.
	#[cfg(feature = "f_bitcoin")]
	TxRequestUnknownTxid(sha256d::Hash),
	/// The PSBT is missing the full tx for given input.
	#[cfg(feature = "f_bitcoin")]
	PsbtMissingInputTx(sha256d::Hash),
	/// Device produced invalid TxRequest message.
	MalformedTxRequest(protos::TxRequest),
	/// User provided invalid PSBT.
	InvalidPsbt(String),
	/// Error encoding/decoding a Bitcoin data structure.
	#[cfg(feature = "f_bitcoin")]
	BitcoinEncode(bitcoin::consensus::encode::Error),
	/// Elliptic curve crypto error.
	#[cfg(feature = "f_bitcoin")]
	Secp256k1(secp256k1::Error),
}

impl From<ProtobufError> for Error {
	fn from(e: ProtobufError) -> Error {
		Error::Protobuf(e)
	}
}

#[cfg(feature = "f_bitcoin")]
impl From<base58::Error> for Error {
	fn from(e: base58::Error) -> Error {
		Error::Base58(e)
	}
}

#[cfg(feature = "f_bitcoin")]
impl From<bitcoin::consensus::encode::Error> for Error {
	fn from(e: bitcoin::consensus::encode::Error) -> Error {
		Error::BitcoinEncode(e)
	}
}

#[cfg(feature = "f_bitcoin")]
impl From<secp256k1::Error> for Error {
	fn from(e: secp256k1::Error) -> Error {
		Error::Secp256k1(e)
	}
}

impl error::Error for Error {
	fn cause(&self) -> Option<&dyn error::Error> {
		match *self {
			Error::TransportConnect(ref e) => Some(e),
			Error::TransportBeginSession(ref e) => Some(e),
			Error::TransportEndSession(ref e) => Some(e),
			Error::TransportSendMessage(ref e) => Some(e),
			Error::TransportReceiveMessage(ref e) => Some(e),
			#[cfg(feature = "f_bitcoin")]
			Error::Base58(ref e) => Some(e),
			_ => None,
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::NoDeviceFound => write!(f, "Trezor device not found"),
			Error::DeviceNotUnique => write!(f, "multiple Trezor devices found"),
			Error::UnsupportedNetwork => write!(f, "given network is not supported"),
			Error::InvalidEntropy => write!(f, "provided entropy is not 32 bytes"),
			Error::TransportConnect(ref e) => write!(f, "transport connect: {}", e),
			Error::TransportBeginSession(ref e) => write!(f, "transport beginning session: {}", e),
			Error::TransportEndSession(ref e) => write!(f, "transport ending session: {}", e),
			Error::TransportSendMessage(ref e) => write!(f, "transport sending message: {}", e),
			Error::TransportReceiveMessage(ref e) => {
				write!(f, "transport receiving message: {}", e)
			}
			Error::UnexpectedMessageType(ref t) => {
				write!(f, "received unexpected message type: {:?}", t)
			}
			Error::Protobuf(ref e) => write!(f, "protobuf: {}", e),
			Error::FailureResponse(ref e) => write!(
				f,
				r#"failure received: code={:?} message="{}""#,
				e.get_code(),
				e.get_message()
			),
			Error::UnexpectedInteractionRequest(ref r) => {
				write!(f, "unexpected interaction request: {:?}", r)
			}
			#[cfg(feature = "f_bitcoin")]
			Error::Base58(ref e) => fmt::Display::fmt(e, f),
			Error::TxRequestInvalidIndex(ref i) => {
				write!(f, "device referenced non-existing input or output index: {}", i)
			}
			#[cfg(feature = "f_bitcoin")]
			Error::TxRequestUnknownTxid(ref txid) => {
				write!(f, "device referenced unknown TXID: {}", txid)
			}
			#[cfg(feature = "f_bitcoin")]
			Error::PsbtMissingInputTx(ref txid) => write!(f, "PSBT missing input tx: {}", txid),
			Error::MalformedTxRequest(ref m) => write!(f, "malformed TxRequest: {:?}", m),
			Error::InvalidPsbt(ref m) => write!(f, "invalid PSBT: {}", m),
			#[cfg(feature = "f_bitcoin")]
			Error::BitcoinEncode(ref e) => write!(f, "bitcoin encoding error: {}", e),
			#[cfg(feature = "f_bitcoin")]
			Error::Secp256k1(ref e) => write!(f, "ECDSA signature error: {}", e),
		}
	}
}

/// Result type used in this crate.
pub type Result<T> = result::Result<T, Error>;
