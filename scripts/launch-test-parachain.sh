#!/usr/bin/env bash
set -e

rm -rf ./collatorone

echo "##GENERATING RAW CHAINSPEC FOR dev"
./target/release/bitgreen-parachain build-spec --raw --chain rococo-local --disable-default-bootnode > artifacts/bitgreen-dev-raw.json

echo "##GENERATING GENESIS WASM FOR dev"
./target/release/bitgreen-parachain export-genesis-wasm --chain ./artifacts/bitgreen-dev-raw.json > artifacts/bitgreen-dev-wasm

echo "##GENERATING GENESIS STATE FOR dev"
./target/release/bitgreen-parachain export-genesis-state --chain ./artifacts/bitgreen-dev-raw.json > artifacts/bitgreen-dev-genesis

./target/release/bitgreen-parachain --collator --force-authoring --chain ./artifacts/bitgreen-dev-raw.json --port 40336 \
--ws-port 9946 --ws-external --base-path ./collatorone \
--rpc-port 9979 --rpc-cors all --discover-local \
--rpc-external --rpc-methods=unsafe -- --execution wasm --chain ../../polkadot/rococo-local-cfde.json --port 40334