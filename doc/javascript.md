# Integration Guide for JavaScript (Bitgreen Blockchain)

This guide addresses the information required to integrate the BitGreen Blockchain in your app client using Javascript  
Bitgreen is based on the [Substrate framework](https://www.substrate.dev), the same framework used to build [Polkadot](https://polkadot.network).  
The same libraries used for Polkadot are working smootlhy with Bitgreen.

## JavaScript Library
The best and well maintained library for Javascript is [Polkadot.js](https://polkadot.js.org/docs/).
The libraries allow to create new accounts, submit transactions and read data from the blockchain.  
Here some references:

- [Install the Library](https://polkadot.js.org/docs/api/start/install)
- [Connect to a Bitgreen Node](https://polkadot.js.org/docs/api/examples/promise/simple-connect)
- [Create/Manage Accounts](https://polkadot.js.org/docs/ui-keyring)
- [Submit Transactions](https://polkadot.js.org/docs/api/examples/promise/make-transfer)
- [Query Transactions](#query-transactions)
- [Other Functions](https://polkadot.js.org/docs/)

## Endpoint 
You can [install a node](../README.md) in your machine or use our Testnet Servers: 
```
wss://testnet.bitg.org
```
## Data Types
BitGreen blockchain has some differences compared to Polkadot, some data types have been created for our specific modules.  
You should "inject" the datatypes during the connection to the node.  
The data types are in [/assets/types.json](../assets/types.json)  
Here you can find an example how to open a connection injecting the required data types:  
[/utilities/bitg-transfer-testnet.js](../utilities/bitg-transfer-testnet.js)  

## How to get BITG for Testnet
You can get 100 free BITG on Testnet using our free minter available at:  
[https://testnet.bitg.org:8443](https://testnet.bitg.org:8443)  

## Query Transactions
The blockchain itself has not great capacity of querying transactions so a cache engine is available for an easier integration.  
You can make https calls to the following endpoint:  
```
https://testnet.bitg.org:9443/transactions  
```
(eventually replace testnode.bitg.org with your domain name if you have your own node) parameters:  
- account - the SS58 address of the account.
-  dst - Date/time of starting the selection of the transactions.
- dse - Date/time of ending the selection of the transactions.
For example you can browse or call an https get: 
```
https://testnode.bitg.org:9443/transactions?account=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY&dts=2021-08-03+00:00:00&dte=2021-08-20+23:59:59
```
to obtain a json answer self-explained:
{
    "transactions": [{
        "id": 1,
        "blocknumber": 162388,
        "txhash": "0x91935fe79c429695eb34ca2a191ee2ec7e40b2734691ce7a8ba758c7e1f01e22",
        "sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "recipient": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
        "amount": 100000000000000000000,
        "dtblockchain": "Tue Aug 03 2021 13:16:50 GMT+0200 (Central European Summer Time)"
    }, {
        "id": 2,
        "blocknumber": 162390,
        "txhash": "0xd02b3a584ed96d5b5dff266112403a63aaa2ea43257a9ed482b0efd6080ecd64",
        "sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "recipient": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
        "amount": 54000000000000000000,
        "dtblockchain": "Tue Aug 03 2021 13:17:10 GMT+0200 (Central European Summer Time)"
    }, {
        "id": 3,
        "blocknumber": 162424,
        "txhash": "0x1b2495ac57c152f37aeb59ed28d833ccf7b7e6cf3b7cf345a1e194c0c5e562fd",
        "sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "recipient": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
        "amount": 21000000000000000000,
        "dtblockchain": "Tue Aug 03 2021 13:22:50 GMT+0200 (Central European Summer Time)"
    }, {
        "id": 4,
        "blocknumber": 162430,
        "txhash": "0xc1a74a4310fe6294a0fcdff58004009bfa48525d49a671c0a58834bce56e6cdb",
        "sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "recipient": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
        "amount": 21000000000000000000,
        "dtblockchain": "Tue Aug 03 2021 13:23:50 GMT+0200 (Central European Summer Time)"
    }]
}

## Further Information  
Please refer to the main [README.md](../README.md).  
