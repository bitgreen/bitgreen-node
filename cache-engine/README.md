# Cache Engine for BitGreen BlockChain
The purpose of this module is to store the transactions  of the blockchain in mysql database and offer an API interface for the applications client.
The solution is divided in 2 modules: 
- A crawler that  store the transaction of the blockchain in a database
- A Web Api server to query the data stored in the database

## CRAWLER
The crawler has the mission to wait for new blocks and store the transactions in the database.
Written in Python 3, the library used is:   [https://github.com/polkascan/py-substrate-interface](https://github.com/polkascan/py-substrate-interface)

### Requirements:

You should have installed:  
- [Python 3.x](https://www.python.org)  
- [Python Package Index (pip)](https://pypi.org)  
- [Mariadb Server](https://mariadb.org)  

## Installation  
This instructions refers to an installation for LINUX operating system.  
Execute from command line:
```sh
pip3 install substrate-interface
pip3 install mysql-connector-python
```

## Create Database and grant access
Launch the mysql cli:  
```sh
mysql
```
and copy/paste:   
```
create database bitgreen;  
CREATE USER 'bitgreen'@'localhost' IDENTIFIED BY 'aszxqw1234';      // for example only, change with your password
GRANT ALL ON bitgreen.* TO bitgreen@'localhost';  
flush privileges;  
```
Customise the file:  
```sh
bitg-blockchain-crawler.sh
```
to reflect your database configuration.  

## Run

The Bitgreen node run locally.  
Execute from the command line:  
```sh
./bit-blockchain-crawler.sh
```
 
 To process the past blocks the node should run in "archive" mode by the additional the parameter:  
 ```sh
 --pruning archive
```
for example:
```sh
./target/release/bitg-node --chain assets/chain_spec_testnet_raw.json --port 30333 --name bitg-testnet1 --rpc-cors all --pruning archive
```
in the case the node was already starting in "non-archive mode" you will have to purge the chain before launching the command above:  
```sh
./target/release/bitg-node purge-chain --chain assets/chain_spec_testnet_raw.json
```

## API Server

The API server offers an https endpoint to query the transaction and received a json structure as answer.

## Requirements:
- [Nodejs >=14.x](https://nodejs.dev)

## Installation
You should install the required libraries using npn (part of nodejs package):  
```sh
npm install express
npm install mysql
```
Customize the starting script:
customise the script: bitg-cache-server.sh settings the variable to access the database and execute:  
```sh
bitg-cache-server.sh
```
Setting the environment variable accordingly your configuration.  

To enable HTTPS you should install the private key and certificate from a well recognised Certification Authority.  
In the example we used: [https://certbot.eff.org](https://certbot.eff.org).  
And you should set the accordingly environment variables in "bitg-cache-server.sh" to point to the correct file name.  

## Usage

From command line, execute:  
```sh
./bitg-cache-server.sh
```


Once active, You can make https calls to the following endpoint:  
## Query transactions
```
https://testnode.bitg.org:9443/transactions
```
where "testnode.bitg.org" should be replace with your node name or ip address.  

possible parameters:  
- account - the SS58 address of the account.   
- dst - Date/time of starting the selection of the  transactions.  
- dse - Date/time of ending the selection of the transactions.  

For example you can browse or make an https get:  

```sh
https://testnode.bitg.org:9443/transactions?account=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY&dts=2021-08-03+00:00:00&dte=2021-08-20+23:59:59
```
to obtain a json answer self-explained:  
```json
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
```

## Get Single transactions by txhash

You can query a single transaction searching by txthash, for example:  

```sh
https://testnode.bitg.org:9443/transaction?txhash=0x91935fe79c429695eb34ca2a191ee2ec7e40b2734691ce7a8ba758c7e1f01e22
```
to obtain a json answer like the following:  
```json
{
		"id": 1,
		"blocknumber": 162388,
		"txhash": "0x91935fe79c429695eb34ca2a191ee2ec7e40b2734691ce7a8ba758c7e1f01e22",
		"sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"recipient": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
		"amount": 100000000000000000000,
		"dtblockchain": "Tue Aug 03 2021 13:16:50 GMT+0200 (Central European Summer Time)"
}





