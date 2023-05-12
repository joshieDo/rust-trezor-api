#![allow(ambiguous_glob_reexports, unknown_lints, rustdoc::all)]

// Common
pub mod messages;
pub mod messages_bootloader;
pub mod messages_common;
pub mod messages_crypto;
pub mod messages_debug;
pub mod messages_management;

pub use messages::*;
pub use messages_bootloader::*;
pub use messages_common::*;
pub use messages_crypto::*;
pub use messages_debug::*;
pub use messages_management::*;

macro_rules! features {
    ($($feature:literal => { $($item:item)+ })+) => {$(
        $(
            #[cfg(feature = $feature)]
            $item
        )+
    )+};
}

features! {
	"bitcoin" => {
		pub mod messages_bitcoin;
		pub use messages_bitcoin::*;
	}

	"ethereum" => {
		pub mod messages_ethereum;
		pub mod messages_ethereum_eip712;

		pub use messages_ethereum::*;
		pub use messages_ethereum_eip712::*;
	}

	"cardano" => {
		pub mod messages_cardano;
		pub use messages_cardano::*;
	}

	"lisk" => {
		pub mod messages_lisk;
		pub use messages_lisk::*;
	}

	"monero" => {
		pub mod messages_monero;
		pub use messages_monero::*;
	}

	"nem" => {
		pub mod messages_nem;
		pub use messages_nem::*;
	}

	"ontology" => {
		pub mod messages_ontology;
		pub use messages_ontology::*;
	}

	"ripple" => {
		pub mod messages_ripple;
		pub use messages_ripple::*;
	}

	"stellar" => {
		pub mod messages_stellar;
		pub use messages_stellar::*;
	}

	"tezos" => {
		pub mod messages_tezos;
		pub use messages_tezos::*;
	}

	"tron" => {
		pub mod messages_tron;
		pub use messages_tron::*;
	}
}
