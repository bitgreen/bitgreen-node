rm -rf /tmp/parachain/alice

./target/release/bitg-parachain build-spec --chain dev --raw --disable-default-bootnode > rococo-local-parachain-2000-raw.json

./target/release/bitg-parachain export-genesis-wasm --chain rococo-local-parachain-2000-raw.json > para-2000-wasm

./target/release/bitg-parachain export-genesis-state --chain rococo-local-parachain-2000-raw.json > para-2000-genesis

./target/release/bitg-parachain \
--alice \
--collator \
--force-authoring \
--chain rococo-local-parachain-2000-raw.json \
--base-path /tmp/parachain/alice \
--port 40333 \
--ws-port 8844 \
-- \
--execution wasm \
--chain ../polkadot/rococo-local-cfde.json \
--port 30343 \
--ws-port 9977
