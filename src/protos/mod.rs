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
        pub mod messages_ethereum_definitions;

        pub use messages_ethereum::*;
        pub use messages_ethereum_eip712::*;
        pub use messages_ethereum_definitions::*;
    }

    "binance" => {
        pub mod messages_binance;
        pub use messages_binance::*;
    }

    "cardano" => {
        pub mod messages_cardano;
        pub use messages_cardano::*;
    }

    "eos" => {
        pub mod messages_eos;
        pub use messages_eos::*;
    }

    "monero" => {
        pub mod messages_monero;
        pub use messages_monero::*;
    }

    "nem" => {
        pub mod messages_nem;
        pub use messages_nem::*;
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

    "webauthn" => {
        pub mod messages_webauthn;
        pub use messages_webauthn::*;
    }
}
