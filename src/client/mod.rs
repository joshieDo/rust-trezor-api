#[cfg(feature = "f_bitcoin")]
mod bitcoin;
#[cfg(feature = "f_bitcoin")]
pub use self::bitcoin::*;
#[cfg(feature = "f_ethereum")]
mod ethereum;
#[cfg(feature = "f_ethereum")]
pub use self::ethereum::*;

pub mod common;
pub use self::common::*;

use super::Model;
use error::{Error, Result};

use messages::TrezorMessage;
use protos;
use protos::MessageType::*;
use transport::{ProtoMessage, Transport};

/// A Trezor client.
pub struct Trezor {
	model: Model,
	// Cached features for later inspection.
	features: Option<protos::Features>,
	transport: Box<dyn Transport>,
}

/// Create a new Trezor instance with the given transport.
pub fn trezor_with_transport(model: Model, transport: Box<dyn Transport>) -> Trezor {
	Trezor {
		model,
		transport,
		features: None,
	}
}

impl Trezor {
	/// Get the model of the Trezor device.
	pub fn model(&self) -> Model {
		self.model
	}

	/// Get the features of the Trezor device.
	pub fn features(&self) -> Option<&protos::Features> {
		self.features.as_ref()
	}

	/// Sends a message and returns the raw ProtoMessage struct that was responded by the device.
	/// This method is only exported for users that want to expand the features of this library
	/// f.e. for supporting additional coins etc.
	pub fn call_raw<S: TrezorMessage>(&mut self, message: S) -> Result<ProtoMessage> {
		let proto_msg = ProtoMessage(S::message_type(), message.write_to_bytes()?);
		self.transport.write_message(proto_msg).map_err(Error::TransportSendMessage)?;
		self.transport.read_message().map_err(Error::TransportReceiveMessage)
	}

	/// Sends a message and returns a TrezorResponse with either the expected response message,
	/// a failure or an interaction request.
	/// This method is only exported for users that want to expand the features of this library
	/// f.e. for supporting additional coins etc.
	pub fn call<'a, T, S: TrezorMessage, R: TrezorMessage>(
		&'a mut self,
		message: S,
		result_handler: Box<ResultHandler<'a, T, R>>,
	) -> Result<TrezorResponse<'a, T, R>> {
		trace!("Sending {:?} msg: {:?}", S::message_type(), message);
		let resp = self.call_raw(message)?;
		if resp.message_type() == R::message_type() {
			let resp_msg = resp.into_message()?;
			trace!("Received {:?} msg: {:?}", R::message_type(), resp_msg);
			Ok(TrezorResponse::Ok(result_handler(self, resp_msg)?))
		} else {
			match resp.message_type() {
				MessageType_Failure => {
					let fail_msg = resp.into_message()?;
					debug!("Received failure: {:?}", fail_msg);
					Ok(TrezorResponse::Failure(fail_msg))
				}
				MessageType_ButtonRequest => {
					let req_msg = resp.into_message()?;
					trace!("Received ButtonRequest: {:?}", req_msg);
					Ok(TrezorResponse::ButtonRequest(ButtonRequest {
						message: req_msg,
						client: self,
						result_handler,
					}))
				}
				MessageType_PinMatrixRequest => {
					let req_msg = resp.into_message()?;
					trace!("Received PinMatrixRequest: {:?}", req_msg);
					Ok(TrezorResponse::PinMatrixRequest(PinMatrixRequest {
						message: req_msg,
						client: self,
						result_handler,
					}))
				}
				MessageType_PassphraseRequest => {
					let req_msg = resp.into_message()?;
					trace!("Received PassphraseRequest: {:?}", req_msg);
					Ok(TrezorResponse::PassphraseRequest(PassphraseRequest {
						message: req_msg,
						client: self,
						result_handler,
					}))
				}
				mtype => {
					debug!(
						"Received unexpected msg type: {:?}; raw msg: {}",
						mtype,
						hex::encode(resp.into_payload())
					);
					Err(Error::UnexpectedMessageType(mtype))
				}
			}
		}
	}

	pub fn init_device(&mut self, session_id: Option<Vec<u8>>) -> Result<()> {
		let features = self.initialize(session_id)?.ok()?;
		self.features = Some(features);
		Ok(())
	}

	pub fn initialize(
		&mut self,
		session_id: Option<Vec<u8>>,
	) -> Result<TrezorResponse<Features, Features>> {
		let mut req = protos::Initialize::new();
		if let Some(session_id) = session_id {
			req.set_session_id(session_id);
		}
		self.call(req, Box::new(|_, m| Ok(m)))
	}

	pub fn ping(&mut self, message: &str) -> Result<TrezorResponse<(), protos::Success>> {
		let mut req = protos::Ping::new();
		req.set_message(message.to_owned());
		self.call(req, Box::new(|_, _| Ok(())))
	}

	pub fn change_pin(&mut self, remove: bool) -> Result<TrezorResponse<(), protos::Success>> {
		let mut req = protos::ChangePin::new();
		req.set_remove(remove);
		self.call(req, Box::new(|_, _| Ok(())))
	}

	pub fn wipe_device(&mut self) -> Result<TrezorResponse<(), protos::Success>> {
		let req = protos::WipeDevice::new();
		self.call(req, Box::new(|_, _| Ok(())))
	}

	pub fn recover_device(
		&mut self,
		word_count: WordCount,
		passphrase_protection: bool,
		pin_protection: bool,
		label: String,
		dry_run: bool,
	) -> Result<TrezorResponse<(), protos::Success>> {
		let mut req = protos::RecoveryDevice::new();
		req.set_word_count(word_count as u32);
		req.set_passphrase_protection(passphrase_protection);
		req.set_pin_protection(pin_protection);
		req.set_label(label);
		req.set_enforce_wordlist(true);
		req.set_dry_run(dry_run);
		req.set_field_type(
			protos::RecoveryDevice_RecoveryDeviceType::RecoveryDeviceType_ScrambledWords,
		);
		//TODO(stevenroose) support languages
		req.set_language("english".to_owned());
		self.call(req, Box::new(|_, _| Ok(())))
	}

	#[allow(clippy::too_many_arguments)]
	pub fn reset_device(
		&mut self,
		display_random: bool,
		strength: usize,
		passphrase_protection: bool,
		pin_protection: bool,
		label: String,
		skip_backup: bool,
		no_backup: bool,
	) -> Result<TrezorResponse<EntropyRequest, protos::EntropyRequest>> {
		let mut req = protos::ResetDevice::new();
		req.set_display_random(display_random);
		req.set_strength(strength as u32);
		req.set_passphrase_protection(passphrase_protection);
		req.set_pin_protection(pin_protection);
		req.set_label(label);
		req.set_skip_backup(skip_backup);
		req.set_no_backup(no_backup);
		self.call(
			req,
			Box::new(|c, _| {
				Ok(EntropyRequest {
					client: c,
				})
			}),
		)
	}

	pub fn backup(&mut self) -> Result<TrezorResponse<(), protos::Success>> {
		let req = protos::BackupDevice::new();
		self.call(req, Box::new(|_, _| Ok(())))
	}

	//TODO(stevenroose) support U2F stuff? currently ignored all

	pub fn apply_settings(
		&mut self,
		label: Option<String>,
		use_passphrase: Option<bool>,
		homescreen: Option<Vec<u8>>,
		auto_lock_delay_ms: Option<usize>,
	) -> Result<TrezorResponse<(), protos::Success>> {
		let mut req = protos::ApplySettings::new();
		if let Some(label) = label {
			req.set_label(label);
		}
		if let Some(use_passphrase) = use_passphrase {
			req.set_use_passphrase(use_passphrase);
		}
		if let Some(homescreen) = homescreen {
			req.set_homescreen(homescreen);
		}
		if let Some(auto_lock_delay_ms) = auto_lock_delay_ms {
			req.set_auto_lock_delay_ms(auto_lock_delay_ms as u32);
		}
		self.call(req, Box::new(|_, _| Ok(())))
	}

	pub fn sign_identity(
		&mut self,
		identity: protos::IdentityType,
		digest: Vec<u8>,
		curve: String,
	) -> Result<TrezorResponse<Vec<u8>, protos::SignedIdentity>> {
		let mut req = protos::SignIdentity::new();
		req.set_identity(identity);
		req.set_challenge_hidden(digest);
		req.set_challenge_visual("".to_owned());
		req.set_ecdsa_curve_name(curve);
		self.call(req, Box::new(|_, m| Ok(m.get_signature().to_owned())))
	}
}
