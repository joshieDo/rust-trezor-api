# Rust Trezor API

![Github Actions](https://github.com/joshieDo/rust-trezor-api/workflows/Build/badge.svg)
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/trezor-client.svg
[crates-url]: https://crates.io/crates/trezor-client

A fork of a [fork](https://github.com/romanz/rust-trezor-api) of a [library](https://github.com/stevenroose/rust-trezor-api) that provides a way to communicate with a Trezor T device from a Rust project.

Previous iterations were focused on bitcoin-only, **this one focuses on providing an ethereum interface**, for use in [ethers-rs](https://github.com/gakonst/ethers-rs/).

## Requirements

**MSRV: 1.60**

Make sure you have Trezor [udev](https://wiki.trezor.io/Udev_rules) rules installed.

Last tested with firmware v2.4.2

## Examples / Tests

`cargo run --example features`

## Features

-   `bitcoin` and `ethereum`: client implementation and full support;
-   `cardano`, `lisk`, `monero`, `nem`, `ontology`, `ripple`, `stellar`, `tezos`, and`tron`: only protobuf bindings.

## Future

At the moment, not looking into expanding more than what's necessary to maintain compatability/usability with ethers-rs.

## Credits

-   [Trezor](https://github.com/trezor/trezor-firmware)
-   [stevenroose](https://github.com/stevenroose)
-   [romanz](https://github.com/romanz)
