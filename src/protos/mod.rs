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
	"f_bitcoin" => {
		pub mod messages_bitcoin;
		pub use messages_bitcoin::*;
	}

	"f_ethereum" => {
		pub mod messages_ethereum;
		pub mod messages_ethereum_eip712;

		pub use messages_ethereum::*;
		pub use messages_ethereum_eip712::*;
	}

	"f_cardano" => {
		pub mod messages_cardano;
		pub use messages_cardano::*;
	}

	"f_lisk" => {
		pub mod messages_lisk;
		pub use messages_lisk::*;
	}

	"f_monero" => {
		pub mod messages_monero;
		pub use messages_monero::*;
	}

	"f_nem" => {
		pub mod messages_nem;
		pub use messages_nem::*;
	}

	"f_ontology" => {
		pub mod messages_ontology;
		pub use messages_ontology::*;
	}

	"f_ripple" => {
		pub mod messages_ripple;
		pub use messages_ripple::*;
	}

	"f_stellar" => {
		pub mod messages_stellar;
		pub use messages_stellar::*;
	}

	"f_tezos" => {
		pub mod messages_tezos;
		pub use messages_tezos::*;
	}

	"f_tron" => {
		pub mod messages_tron;
		pub use messages_tron::*;
	}
}
