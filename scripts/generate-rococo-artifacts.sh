#!/usr/bin/env bash
set -e

echo "##GENERATING RAW CHAINSPEC FOR ROCOCO"
./target/release/bitgreen-parachain build-spec --raw --chain local-rococo --disable-default-bootnode > ./artifacts/testnet/bitgreen-testnet-raw.json

echo "##GENERATING GENESIS WASM FOR ROCOCO"
./target/release/bitgreen-parachain export-genesis-wasm --chain ./artifacts/testnet/bitgreen-testnet-raw.json > ./artifacts/testnet/bitgreen-testnet-wasm

echo "##GENERATING GENESIS STATE FOR ROCOCO"
./target/release/bitgreen-parachain export-genesis-state --chain ./artifacts/testnet/bitgreen-testnet-raw.json > ./artifacts/testnet/bitgreen-testnet-genesis