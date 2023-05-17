#!/bin/sh

# Generates src/protos/
# Requires the `protoc-gen-rust` binary (`cargo install protoc-gen-rust`).
# Overwrites src/protos/mod.rs, but the change should not be committed, and
# instead should be handled manually.

root="$(dirname "$(dirname "$0")")"
out_dir="$root/src/protos"
proto_dir="$root/trezor-common/protob"

protoc \
    --proto_path "$proto_dir" \
    --rust_out "$out_dir" \
    "$proto_dir"/*.proto
