use bitcoin::network::constants::Network; //TODO(stevenroose) change after https://github.com/rust-bitcoin/rust-bitcoin/pull/181
use bitcoin::util::bip32;
use bitcoin::util::psbt;
use bitcoin::Address;
use flows::sign_tx::SignTxProgress;
use secp256k1;
use unicode_normalization::UnicodeNormalization;
use utils;

use protos;
pub use protos::InputScriptType;

use super::{Trezor, TrezorResponse};
use crate::error::Result;

impl Trezor {
	pub fn get_public_key(
		&mut self,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
		show_display: bool,
	) -> Result<TrezorResponse<bip32::ExtendedPubKey, protos::PublicKey>> {
		let mut req = protos::GetPublicKey::new();
		req.set_address_n(utils::convert_path(path));
		req.set_show_display(show_display);
		req.set_coin_name(utils::coin_name(network)?);
		req.set_script_type(script_type);
		self.call(req, Box::new(|_, m| Ok(m.get_xpub().parse()?)))
	}

	//TODO(stevenroose) multisig
	pub fn get_address(
		&mut self,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
		show_display: bool,
	) -> Result<TrezorResponse<Address, protos::Address>> {
		let mut req = protos::GetAddress::new();
		req.set_address_n(utils::convert_path(path));
		req.set_coin_name(utils::coin_name(network)?);
		req.set_show_display(show_display);
		req.set_script_type(script_type);
		self.call(req, Box::new(|_, m| Ok(m.get_address().parse()?)))
	}

	pub fn sign_tx(
		&mut self,
		psbt: &psbt::PartiallySignedTransaction,
		network: Network,
	) -> Result<TrezorResponse<SignTxProgress, protos::TxRequest>> {
		let tx = &psbt.global.unsigned_tx;
		let mut req = protos::SignTx::new();
		req.set_inputs_count(tx.input.len() as u32);
		req.set_outputs_count(tx.output.len() as u32);
		req.set_coin_name(utils::coin_name(network)?);
		req.set_version(tx.version);
		req.set_lock_time(tx.lock_time);
		self.call(req, Box::new(|c, m| Ok(SignTxProgress::new(c, m))))
	}

	pub fn sign_message(
		&mut self,
		message: String,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
	) -> Result<TrezorResponse<(Address, secp256k1::RecoverableSignature), protos::MessageSignature>>
	{
		let mut req = protos::SignMessage::new();
		req.set_address_n(utils::convert_path(path));
		// Normalize to Unicode NFC.
		let msg_bytes = message.nfc().collect::<String>().into_bytes();
		req.set_message(msg_bytes);
		req.set_coin_name(utils::coin_name(network)?);
		req.set_script_type(script_type);
		self.call(
			req,
			Box::new(|_, m| {
				let address = m.get_address().parse()?;
				let signature = utils::parse_recoverable_signature(m.get_signature())?;
				Ok((address, signature))
			}),
		)
	}
}
