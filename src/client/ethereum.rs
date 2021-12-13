use protos;

use super::{handle_interaction, Trezor};
use crate::error::Result;

use primitive_types::U256;

/// Access list item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessListItem {
	/// Accessed address
	pub address: String,
	/// Accessed storage keys
	pub storage_keys: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
/// An ECDSA signature
pub struct Signature {
	/// R value
	pub r: U256,
	/// S Value
	pub s: U256,
	/// V value in 'Electrum' notation.
	pub v: u64,
}

impl Trezor {
	// ETHEREUM
	pub fn ethereum_get_address(&mut self, path: Vec<u32>) -> Result<String> {
		let mut req = protos::EthereumGetAddress::new();
		req.set_address_n(path);

		let address = handle_interaction(
			self.call(req, Box::new(|_, m: protos::EthereumAddress| Ok(m.get_address().into())))?,
		)?;
		Ok(address)
	}

	pub fn ethereum_sign_message(&mut self, message: Vec<u8>, path: Vec<u32>) -> Result<Signature> {
		let mut req = protos::EthereumSignMessage::new();
		req.set_address_n(path);
		req.set_message(message);
		let signature = handle_interaction(self.call(
			req,
			Box::new(|_, m: protos::EthereumMessageSignature| {
				let signature = m.get_signature();

				// why are you in the end
				let v = signature[64] as u64;
				let r = U256::from_big_endian(&signature[0..32]);
				let s = U256::from_big_endian(&signature[32..64]);

				Ok(Signature {
					r,
					v,
					s,
				})
			}),
		)?)?;

		Ok(signature)
	}

	#[allow(clippy::too_many_arguments)]
	pub fn ethereum_sign_tx(
		&mut self,
		path: Vec<u32>,
		nonce: Vec<u8>,
		gas_price: Vec<u8>,
		gas_limit: Vec<u8>,
		to: String,
		value: Vec<u8>,
		_data: Vec<u8>,
		chain_id: u64,
	) -> Result<Signature> {
		let mut req = protos::EthereumSignTx::new();
		let mut data = _data;

		req.set_address_n(path);
		req.set_nonce(nonce);
		req.set_gas_price(gas_price);
		req.set_gas_limit(gas_limit);
		req.set_value(value);
		req.set_chain_id(chain_id);
		req.set_to(to);

		req.set_data_length(data.len() as u32);
		req.set_data_initial_chunk(data.splice(..std::cmp::min(1024, data.len()), []).collect());

		let mut resp =
			handle_interaction(self.call(req, Box::new(|_, m: protos::EthereumTxRequest| Ok(m)))?)?;

		while resp.get_data_length() > 0 {
			let mut ack = protos::EthereumTxAck::new();
			ack.set_data_chunk(data.splice(..std::cmp::min(1024, data.len()), []).collect());

			resp = self.call(ack, Box::new(|_, m: protos::EthereumTxRequest| Ok(m)))?.ok()?;
		}

		if resp.get_signature_v() <= 1 {
			resp.set_signature_v(resp.get_signature_v() + 2 * (chain_id as u32) + 35);
		}

		Ok(Signature {
			r: resp.get_signature_r().into(),
			v: resp.get_signature_v().into(),
			s: resp.get_signature_s().into(),
		})
	}

	#[allow(clippy::too_many_arguments)]
	pub fn ethereum_sign_eip1559_tx(
		&mut self,
		path: Vec<u32>,
		nonce: Vec<u8>,
		gas_limit: Vec<u8>,
		to: String,
		value: Vec<u8>,
		_data: Vec<u8>,
		chain_id: u64,
		max_gas_fee: Vec<u8>,
		max_priority_fee: Vec<u8>,
		access_list: Vec<AccessListItem>,
	) -> Result<Signature> {
		let mut req = protos::EthereumSignTxEIP1559::new();
		let mut data = _data;

		req.set_address_n(path);
		req.set_nonce(nonce);
		req.set_max_gas_fee(max_gas_fee);
		req.set_max_priority_fee(max_priority_fee);
		req.set_gas_limit(gas_limit);
		req.set_value(value);
		req.set_chain_id(chain_id);
		req.set_to(to);

		if !access_list.is_empty() {
			let mut list_access = Vec::new();

			for item in access_list {
				let mut access = protos::EthereumSignTxEIP1559_EthereumAccessList::new();

				access.set_address(item.address);
				access.set_storage_keys(protobuf::RepeatedField::from_vec(item.storage_keys));

				list_access.push(access)
			}

			req.set_access_list(protobuf::RepeatedField::from_vec(list_access.clone()));
		}

		req.set_data_length(data.len() as u32);
		req.set_data_initial_chunk(data.splice(..std::cmp::min(1024, data.len()), []).collect());

		let mut resp =
			handle_interaction(self.call(req, Box::new(|_, m: protos::EthereumTxRequest| Ok(m)))?)?;

		while resp.get_data_length() > 0 {
			let mut ack = protos::EthereumTxAck::new();
			ack.set_data_chunk(data.splice(..std::cmp::min(1024, data.len()), []).collect());

			resp = self.call(ack, Box::new(|_, m: protos::EthereumTxRequest| Ok(m)))?.ok()?
		}

		if resp.get_signature_v() <= 1 {
			resp.set_signature_v(resp.get_signature_v() + 2 * (chain_id as u32) + 35);
		}

		Ok(Signature {
			r: resp.get_signature_r().into(),
			v: resp.get_signature_v().into(),
			s: resp.get_signature_s().into(),
		})
	}
}
