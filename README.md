# Bitg

Implementation of a https://bitg.org node in Rust based on the Substrate framework.


## Installation

If you just wish to run a Bitg node without compiling it yourself, you may
either run the latest binary from our
[releases](https://github.com/bitgreen/bitg-node/releases) page, or install
Bitg from one of our package repositories.

### Connect to Bitg network

Connect to the global network by running:

```bash
./target/release/bitg --chain=bitg
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.polkadot.io/#list/Bitg
