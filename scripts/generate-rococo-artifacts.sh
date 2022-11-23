#!/usr/bin/env bash
set -e

echo "##GENERATING RAW CHAINSPEC FOR ROCOCO"
./target/release/bitgreen-parachain build-spec --raw --chain rococo --disable-default-bootnode > artifacts/bitgreen-rococo-raw.json

echo "##GENERATING GENESIS WASM FOR ROCOCO"
./target/release/bitgreen-parachain export-genesis-wasm --chain artifacts/bitgreen-rococo-raw.json > artifacts/bitgreen-rococo-wasm

echo "##GENERATING GENESIS STATE FOR ROCOCO"
./target/release/bitgreen-parachain export-genesis-state --chain artifacts/bitgreen-rococo-raw.json > artifacts/bitgreen-rococo-genesis
