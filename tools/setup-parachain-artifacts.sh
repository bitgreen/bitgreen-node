PARACHAIN_ID=2000

FILENAME_PREFIX="bitg-parachain-${PARACHAIN_ID}"

rm -rf /tmp/parachain/alice

../target/release/bitg-parachain build-spec --chain dev --raw --disable-default-bootnode > "${FILENAME_PREFIX}"-raw.json

../target/release/bitg-parachain export-genesis-wasm --chain "$FILENAME_PREFIX"-raw.json > "$FILENAME_PREFIX"-wasm

../target/release/bitg-parachain export-genesis-state --chain "$FILENAME_PREFIX"-raw.json > "$FILENAME_PREFIX"-genesis
