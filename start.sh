rm -rf /tmp/parachain/alice

./target/release/bitg-parachain build-spec --chain rococo --raw --disable-default-bootnode > rococo-bitgreen-3029-raw.json

./target/release/bitg-parachain export-genesis-wasm --chain rococo-bitgreen-3029-raw.json > rococo-bitgreen-3029-wasm

./target/release/bitg-parachain export-genesis-state --chain rococo-bitgreen-3029-raw.json > rococo-bitgreen-3029-genesis

./target/release/bitg-parachain \
--alice \
--collator \
--force-authoring \
--chain rococo-bitgreen-3029-raw.json \
--base-path /tmp/parachain/alice \
--port 40333 \
--ws-port 9944 \
--rpc-port 9979 --rpc-cors all --discover-local \
--rpc-external --rpc-methods=unsafe \
-- \
--execution wasm \
--chain rococo.json
