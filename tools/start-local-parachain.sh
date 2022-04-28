
PARACHAIN_ID=2000

FILENAME_PREFIX="bitg-parachain-${PARACHAIN_ID}"

rm -rf /tmp/parachain/alice

../target/release/bitg-parachain build-spec --chain dev --raw --disable-default-bootnode > "${FILENAME_PREFIX}"-raw.json

../target/release/bitg-parachain export-genesis-wasm --chain "$FILENAME_PREFIX"-raw.json > "$FILENAME_PREFIX"-wasm

../target/release/bitg-parachain export-genesis-state --chain "$FILENAME_PREFIX"-raw.json > "$FILENAME_PREFIX"-genesis

../target/release/bitg-parachain \
--alice \
--collator \
--force-authoring \
--chain "$FILENAME_PREFIX"-raw.json \
--base-path /tmp/parachain/alice \
--port 40336 \
--ws-port 9946 \
--rpc-port 9989 --rpc-cors all --discover-local \
--rpc-external --rpc-methods=unsafe \
-- \
--execution wasm \
--chain ../../polkadot/rococo-local-cfde.json
