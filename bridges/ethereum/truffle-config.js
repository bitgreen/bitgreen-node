const HDWalletProvider = require('@truffle/hdwallet-provider');
const YOUR_SEED_PHRASE = process.env.YOUR_SEED_PHRASE || '';

module.exports = {
  // Uncommenting the defaults below 
  // provides for an easier quick-start with Ganache.
  // You can also follow this format for other networks;
  // see <http://truffleframework.com/docs/advanced/configuration>
  // for more details on how to specify configuration options!
  //
  networks: {
   development: {
     host: "127.0.0.1",
     port: 8545,
     network_id: "*"
   },
   rinkeby_infura: {
    provider: () => new HDWalletProvider(YOUR_SEED_PHRASE, `https://rinkeby.infura.io/v3/9b3b7f2e81174b0eb6e1db45ab1c9284`),
    network_id: 4,       // Ropsten's id
    gas: 5500000,        // Ropsten has a lower block limit than mainnet
    confirmations: 2,    // # of confs to wait between deployments. (default: 0)
    timeoutBlocks: 200,  // # of blocks before a deployment times out  (minimum/default: 50)
    skipDryRun: true     // Skip dry run before migrations? (default: false for public nets )
  },
  rinkeby_local: {
    provider: () => new HDWalletProvider(YOUR_SEED_PHRASE, `http://127.0.0.1:8545`),
    network_id: 4,       // Ropsten's id
    gas: 5500000,        // Ropsten has a lower block limit than mainnet
    confirmations: 2,    // # of confs to wait between deployments. (default: 0)
    timeoutBlocks: 200,  // # of blocks before a deployment times out  (minimum/default: 50)
    skipDryRun: true     // Skip dry run before migrations? (default: false for public nets )
  },
  goerli_local: {
    provider: () => new HDWalletProvider(YOUR_SEED_PHRASE, `http://127.0.0.1:8545`),
    network_id: 5,       // Ropsten's id
    gas: 5500000,        // Ropsten has a lower block limit than mainnet
    confirmations: 1,    // # of confs to wait between deployments. (default: 0)
    timeoutBlocks: 200,  // # of blocks before a deployment times out  (minimum/default: 50)
    skipDryRun: true     // Skip dry run before migrations? (default: false for public nets )
  }
  },
  //
  // Truffle DB is currently disabled by default; to enable it, change enabled:
  // false to enabled: true. The default storage location can also be
  // overridden by specifying the adapter settings, as shown in the commented code below.
  //
  // NOTE: It is not possible to migrate your contracts to truffle DB and you should
  // make a backup of your artifacts to a safe location before enabling this feature.
  //
  // After you backed up your artifacts you can utilize db by running migrate as follows: 
  // $ truffle migrate --reset --compile-all
  //
  // db: {
    // enabled: false,
    // host: "127.0.0.1",
    // adapter: {
    //   name: "sqlite",
    //   settings: {
    //     directory: ".db"
    //   }
    // }
  // }
  compilers: {
    solc: {
      version: "^0.8.11", // A version or constraint - Ex. "^0.5.0"
                         // Can be set to "native" to use a native solc or
                         // "pragma" which attempts to autodetect compiler versions
      docker: false, // Use a version obtained through docker
      parser: "solcjs",  // Leverages solc-js purely for speedy parsing
      settings: {
        optimizer: {
          enabled: true,
          runs: 1   // Optimize for how many times you intend to run the code
        },

      },
      // modelCheckerSettings: {
      //   // contains options for SMTChecker
      // }
    }
  }  
};
