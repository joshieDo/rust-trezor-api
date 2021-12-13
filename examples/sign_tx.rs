extern crate bitcoin;
extern crate fern;
extern crate hex;
extern crate log;
extern crate trezor_client;

use std::collections::HashMap;
use std::io::{self, Write};

use bitcoin::{
	blockdata::script::Builder, consensus::encode::Decodable, network::constants::Network,
	util::bip32, util::hash::BitcoinHash, util::psbt, Address, Transaction, TxIn, TxOut,
};

use trezor_client::{Error, SignTxProgress, TrezorMessage, TrezorResponse};

fn setup_logger() {
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!("[{}][{}] {}", record.target(), record.level(), message))
		})
		.level(log::LevelFilter::Trace)
		.chain(std::io::stderr())
		.apply()
		.unwrap();
}

fn handle_interaction<T, R: TrezorMessage>(resp: TrezorResponse<T, R>) -> T {
	match resp {
		TrezorResponse::Ok(res) => res,
		TrezorResponse::Failure(_) => resp.ok().unwrap(), // assering ok() returns the failure error
		TrezorResponse::ButtonRequest(req) => handle_interaction(req.ack().unwrap()),
		TrezorResponse::PinMatrixRequest(req) => {
			println!("Enter PIN");
			let mut pin = String::new();
			if io::stdin().read_line(&mut pin).unwrap() != 5 {
				println!("must enter pin, received: {}", pin);
			}
			// trim newline
			handle_interaction(req.ack_pin(pin[..4].to_owned()).unwrap())
		}
		TrezorResponse::PassphraseRequest(req) => {
			println!("Enter passphrase");
			let mut pass = String::new();
			io::stdin().read_line(&mut pass).unwrap();
			// trim newline
			handle_interaction(req.ack_passphrase(pass[..pass.len() - 1].to_owned()).unwrap())
		}
	}
}

fn tx_progress(
	psbt: &mut psbt::PartiallySignedTransaction,
	progress: SignTxProgress,
	raw_tx: &mut Vec<u8>,
) -> Result<(), Error> {
	if let Some(part) = progress.get_serialized_tx_part() {
		raw_tx.write(part).unwrap();
	}

	if !progress.finished() {
		let progress = handle_interaction(progress.ack_psbt(&psbt, Network::Testnet).unwrap());
		tx_progress(psbt, progress, raw_tx)
	} else {
		Ok(())
	}
}

fn main() {
	setup_logger();
	// init with debugging
	let mut trezor = trezor_client::unique(false).unwrap();
	trezor.init_device(None).unwrap();

	let pubkey = handle_interaction(
		trezor
			.get_public_key(
				&vec![
					bip32::ChildNumber::from_hardened_idx(0).unwrap(),
					bip32::ChildNumber::from_hardened_idx(0).unwrap(),
					bip32::ChildNumber::from_hardened_idx(1).unwrap(),
				]
				.into(),
				trezor_client::protos::InputScriptType::SPENDADDRESS,
				Network::Testnet,
				true,
			)
			.unwrap(),
	);
	let addr = Address::p2pkh(&pubkey.public_key, Network::Testnet);
	println!("address: {}", addr);

	let mut psbt = psbt::PartiallySignedTransaction {
		global: psbt::Global {
			unsigned_tx: Transaction {
				version: 1,
				lock_time: 0,
				input: vec![TxIn {
					previous_output: "c5bdb27907b78ce03f94e4bf2e94f7a39697b9074b79470019e3dbc76a10ecb6:0".parse().unwrap(),
					sequence: 0xffffffff,
					script_sig: Builder::new().into_script(),
					witness: vec![],
				}],
				output: vec![TxOut {
					value: 14245301,
					script_pubkey: addr.script_pubkey(),
				}],
			},
			unknown: HashMap::new(),
		},
		inputs: vec![psbt::Input {
			non_witness_utxo: Some(Transaction::consensus_decode(&mut &hex::decode("020000000001011eb5a3e65946f88b00d67b321e5fd980b32a2316fb1fc9b712baa6a1033a04e30100000017160014f0f81ee77d552b4c81497451d1abf5c22ce8e352feffffff02b55dd900000000001976a9142c3cf5686f47c1de9cc90b4255cc2a1ef8c01b3188acfb0391ae6800000017a914a3a79e37ad366d9bf9471b28a9a8f64b50de0c968702483045022100c0aa7b262967fc2803c8a9f38f26682edba7cafb7d4870ebdc116040ad5338b502205dfebd08e993af2e6aa3118a438ad70ed9f6e09bc6abfd21f8f2957af936bc070121031f4e69fcf110bb31f019321834c0948b5487f2782489f370f66dc20f7ac767ca8bf81500").unwrap()[..]).unwrap()),
			..Default::default()
		}],
		outputs: vec![
			psbt::Output {
				..Default::default()
			},
		],
	};

	println!("psbt before: {:?}", psbt);
	println!("unsigned txid: {}", psbt.global.unsigned_tx.bitcoin_hash());
	println!(
		"unsigned tx: {}",
		hex::encode(bitcoin::consensus::encode::serialize(&psbt.global.unsigned_tx))
	);

	let mut raw_tx = Vec::new();
	let progress = handle_interaction(trezor.sign_tx(&psbt, Network::Testnet).unwrap());
	tx_progress(&mut psbt, progress, &mut raw_tx).unwrap();

	println!("signed tx: {}", hex::encode(raw_tx));
}
