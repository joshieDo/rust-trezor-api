#!/bin/sh

# Requires the `protoc-gen-rust` binary (`cargo install protoc-gen-rust`)
# Overwrites src/protos/mod.rs, but the change should not be committed.
protoc --proto_path ./protos --rust_out ./src/protos protos/*.proto
