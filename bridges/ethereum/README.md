# Bitgreen Bridge
This is the smart contract to act as bridge for Bitgreen chain to EVM chain like Ethereum.

# Public Functions

"function deposit(bytes32 destination)" is the only public function available.  
The sender that wish to wrap ETH or ERC20 in Bitgreen Chain, should deposit the amount and the address of destination.

All the other functions are accessible to the "Keepers" of the bridge.

## Truffle setup

Refer to truffle oficial documentation at https://trufflesuite.com/docs/truffle/getting-started/running-migrations/

### Custom parameters for bitgreen

* Run yarn to bring dependencies
* Set your environment variable YOUR_SEED_PHRASE to contract owner mnemonic phrase
* Select network when running migration based on configuration from truffle-config.js 

Example:
```
yarn
export YOUR_SEED_PHRASE="affair leopard fever palace ..."
truffle migrate --network rinkeby_local

```
