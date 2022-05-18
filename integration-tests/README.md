# Integration Tests

Integration tests for bitgreen pallets.

### How to run

1. Run `npm install`

2. Start node/parachain locally and ensure its accepting connections at port 9944

```
# start node
cargo b -p bitg-node
./target/debug/bitg-node --dev
```

3. Run `npm run` to begin various tests
