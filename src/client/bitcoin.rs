use super::{Trezor, TrezorResponse};
use crate::error::Result;
use crate::flows::sign_tx::SignTxProgress;
use crate::protos;
use crate::utils;
use bitcoin::network::constants::Network;
use bitcoin::psbt;
use bitcoin::Address;
use bitcoin::{address::NetworkUnchecked, bip32};
use secp256k1::{self, ecdsa::RecoverableSignature};
use unicode_normalization::UnicodeNormalization;

pub use crate::protos::InputScriptType;

impl Trezor {
	pub fn get_public_key(
		&mut self,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
		show_display: bool,
	) -> Result<TrezorResponse<bip32::ExtendedPubKey, protos::PublicKey>> {
		let mut req = protos::GetPublicKey::new();
		req.address_n = utils::convert_path(path);
		req.set_show_display(show_display);
		req.set_coin_name(utils::coin_name(network)?);
		req.set_script_type(script_type);
		self.call(req, Box::new(|_, m| Ok(m.xpub().parse()?)))
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
		req.address_n = utils::convert_path(path);
		req.set_coin_name(utils::coin_name(network)?);
		req.set_show_display(show_display);
		req.set_script_type(script_type);
		self.call(req, Box::new(|_, m| parse_address(m.address())))
	}

	pub fn sign_tx(
		&mut self,
		psbt: &psbt::PartiallySignedTransaction,
		network: Network,
	) -> Result<TrezorResponse<SignTxProgress, protos::TxRequest>> {
		let tx = &psbt.unsigned_tx;
		let mut req = protos::SignTx::new();
		req.set_inputs_count(tx.input.len() as u32);
		req.set_outputs_count(tx.output.len() as u32);
		req.set_coin_name(utils::coin_name(network)?);
		req.set_version(tx.version as u32);
		req.set_lock_time(tx.lock_time.to_consensus_u32());
		self.call(req, Box::new(|c, m| Ok(SignTxProgress::new(c, m))))
	}

	pub fn sign_message(
		&mut self,
		message: String,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
	) -> Result<TrezorResponse<(Address, RecoverableSignature), protos::MessageSignature>> {
		let mut req = protos::SignMessage::new();
		req.address_n = utils::convert_path(path);
		// Normalize to Unicode NFC.
		let msg_bytes = message.nfc().collect::<String>().into_bytes();
		req.set_message(msg_bytes);
		req.set_coin_name(utils::coin_name(network)?);
		req.set_script_type(script_type);
		self.call(
			req,
			Box::new(|_, m| {
				let address = parse_address(m.address())?;
				let signature = utils::parse_recoverable_signature(m.signature())?;
				Ok((address, signature))
			}),
		)
	}
}

fn parse_address(s: &str) -> Result<Address> {
	let address = s.parse::<Address<NetworkUnchecked>>()?;
	Ok(address.assume_checked())
}
