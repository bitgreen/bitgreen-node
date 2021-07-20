# Utilities 
- Minter of BITG for Test Net
- Proxy Claim Server

# Bitgreen Minter for Test Net 
This programs is an https server where the user can require the transfer of 100 BITG for the TEST NET.

## Installation
- Install [Nodejs](https://nodejs.org)  
- Install the required libraries:  
```bash
npm install express
npm install readfile
yarn add @polkadot/keyring
yarn add @polkadot/api
```
## Run the server:
From the command line, execute:  
```bash
node bitg-transfer-testnet.js
```
## Connect from Client:

Point the browser to your server on port 8443, for example:
https://testnet.bitg.org:8443

Insert you account and click on "Submit". 
In a few seconds you will receive 100 BITG.


# Bitgreen Proxy Claiming Server
This programs is an https server that can be used to submit a claim to transfer the balance from the old blockchain to new one, without having a previous balance on the recipient account.

## Installation
- Install [Nodejs](https://nodejs.org)  
- Install the required libraries:  
```bash
npm install express
npm install readfile
npm install requests
yarn add @polkadot/keyring
yarn add @polkadot/api
```
## Run the server:
From the command line, execute:  
```bash
node bitg-claim-server.js
```
## Submit the Balance Claim:
You can use as a working example the python app [ecdsa_signing_https_post.py](../python/ecdsa_signing_https_post.py)



