PARACHAIN_ID=3000

FILENAME_PREFIX="bitg-parachain-rococo-${PARACHAIN_ID}"

rm -rf /tmp/parachain/alice

../target/release/bitg-parachain build-spec --chain rococo --raw --disable-default-bootnode > "${FILENAME_PREFIX}"-raw.json

../target/release/bitg-parachain export-genesis-wasm --chain "$FILENAME_PREFIX"-raw.json > "$FILENAME_PREFIX"-wasm

../target/release/bitg-parachain export-genesis-state --chain "$FILENAME_PREFIX"-raw.json > "$FILENAME_PREFIX"-genesis
