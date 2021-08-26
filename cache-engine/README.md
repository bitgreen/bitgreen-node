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
To syncronize the cache from first block written in the table "sync" of the database "bitgreen" you can launch:   
```sh
./bitg-cache-server.sh --sync
```

Once active, You can make https calls to the following endpoint:  
## Query transactions by Account
```
https://testnode.bitg.org:9443/transactions
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

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

## Query Single Transaction by Txhash

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
```

## Impact Actions - Query Impact Actions Configuration

You can query the configuration of impact actions in the system by the following GET:  

```sh
https://testnode.bitg.org:9443/impactactions
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

to obtain a json answer like the following:  
```json
{
		"impactactions": [{
		"id": 9,
		"blocknumber": 247,
		"txhash": "0x6fa43de51eaf89401d2e54129b3ef6063302c55ef56d7e9effad44c6e4660794",
		"sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"description": "Planting a tree",
		"categories": [1],
		"auditors": 1,
		"blockstart": 1,
		"blockend": 1000000,
		"rewardstoken": 1,
		"rewardsamount": 1000,
		"rewardsoracle": 0,
		"rewardauditors": 50,
		"slashingsauditors": 0,
		"maxerrorsauditor": 10,
		"fields": {},
		"dtblockchain": "Fri Aug 13 2021 04:34:30 GMT+0400 (Gulf Standard Time)"
	},{..}]
}	
```
where 
- categories is an array of categories of impact actions id;  
- auditors is the number od auditors required, it may be 0;  
- blockstart is the block number from when the configuration is valid;  
- blockend is the block number till when the configuration is valid;  
- rewardstoken is the token id  (assetid in Assets Pallet), of the token used as rewards, 0=BITG;  
- rewardsamount is the amount of token given as rewards to the operator of the impact action;  
- rewardsoracle is the amount of token given as rewards to the Oracle, if present;  
- rewardauditors is the amount of tokens givine as rewards to the auditors, in case of multiple auditors the amount will be split;  
- slashingsauditors is the amount of slashing token in case of verified error;  
- maxerrorsauditor is the maximum number of errors an auditor can do, after that he will be disable from further auditings;  
- fields is a configurable structure to have custom fields required in the approval request.  
 fields is a json structure as follows:  
 ```json

 [
	 {
	 "fieldname":"xxxxxx",
	 "fieldtype":"N/S"  (N=Numeric, S=String)
	 "mandatory":"Y/N"	(Y= yes is mandatory, N= Optional field)
 	},
 	{...}
 ]
 ```
 You can configure as many field you need withint the maximum of 8192 bytes.

 ## Impact Actions - Query Approval Requests (ALL)

 You can query all the approvals request by the following GET:  

```sh
https://testnode.bitg.org:9443/impactactions
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

to obtain a json answer like the following:  
```json
{
	"approvalrequests": [{
		"id": 2,
		"blocknumber": 470,
		"txhash": "0xd1aa2db676176c1564887cdd9b17520491fb8bf52c7c819d722ff2d166efd741",
		"signer": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
		"info": {
			"impactactionid": 1,
			"description": "Planted a new tree",
			"coordinates": "25.283294382,55.292989282",
			"ipfsphoto": "bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi"
		},
		"dtblockchain": "Mon Aug 16 2021 10:00:40 GMT+0400 (Gulf Standard Time)"
	},{...}]
}
```

## Impact Actions - Query Approval Request (Single)

 You can query all one approval request by the following GET:  

```sh
https://testnode.bitg.org:9443/impactactions?id=xx
```
where "testnode.bitg.org" should be replaced with your node name or ip address and id is the approval request id.  

to obtain a json answer like the following:  
```json
{
	"id": 2,
	"blocknumber": 470,
	"txhash": "0xd1aa2db676176c1564887cdd9b17520491fb8bf52c7c819d722ff2d166efd741",
	"signer": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
	"info": {
		"impactactionid": 1,
		"description": "Planted a new tree",
		"coordinates": "25.283294382,55.292989282",
		"ipfsphoto": "bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi"
	},
	"dtblockchain": "Mon Aug 16 2021 10:00:40 GMT+0400 (Gulf Standard Time)"
}
```

## Impact Actions - Query Votes of the Auditors on Approval Requests
You can query the votes expressed from each auditor on the approval request by the following GET:

```sh
https://testnode.bitg.org:9443/impactactionsapprovalrequestauditorvotes?id=xx
```
where "testnode.bitg.org" should be replaced with your node name or ip address and id is the approval request id.  

to obtain a json answer like the following:  
```json
{
	"id": 2,
	"blocknumber": 470,
	"txhash": "0xd1aa2db676176c1564887cdd9b17520491fb8bf52c7c819d722ff2d166efd741",
	"signer": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
	"info": {
		"impactactionid": 1,
		"description": "Planted a new tree",
		"coordinates": "25.283294382,55.292989282",
		"ipfsphoto": "bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi"
	},
	"dtblockchain": "Mon Aug 16 2021 10:00:40 GMT+0400 (Gulf Standard Time)"
}
```

## Impact Actions - Query Auditors Assigned to Approval Requests  

You can query the auditors assigned to an approval request by the following GET:  

```sh
https://testnode.bitg.org:9443/impactactionsapprovalrequestsauditors?id=xx
```
where "testnode.bitg.org" should be replaced with your node name or ip address and id is the approval request id.  

to obtain a json answer like the following:  
```json
{
		"approvalrequestsauditors": [{
		"id": 1,
		"blocknumber": 679,
		"txhash": "0xc0c60e4741a879b26729137c8ccac5ae3610866d2a5340a03121b07cba969ba1",
		"signer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"approvalrequestid": 2,
		"auditor": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"maxdays": 60,
		"dtblockchain": "Mon Aug 16 2021 10:35:30 GMT+0400 (Gulf Standard Time)"
	}]
}
```

## Impact Actions - Query Auditors

You can query all the auditors registered using the following GET:

```sh
https://testnode.bitg.org:9443/impactactionauditors
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

to obtain a json answer like the following:  
```json
{
		"auditors": [{
		"id": 1,
		"blocknumber": 679,
		"txhash": "0xc0c60e4741a879b26729137c8ccac5ae3610866d2a5340a03121b07cba969ba1",
		"signer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"description": "Oracle to verify CO2 abosrbed from Solar Panels",
		"account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"categories": [1,2],
		"area": "55.255111,45.438981",
		"otherinfo":"bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi",
		"dtblockchain": "Mon Aug 16 2021 10:35:30 GMT+0400 (Gulf Standard Time)"
	}]
}
```
## Impact Actions - Query Oracles

You can query all the Oracles registered using the following GET:

```sh
https://testnode.bitg.org:9443/impactactionoracles
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

to obtain a json answer like the following:  
```json
{
		"oracles": [{
		"id": 1,
		"blocknumber": 679,
		"txhash": "0xc0c60e4741a879b26729137c8ccac5ae3610866d2a5340a03121b07cba969ba1",
		"signer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"description": "Oracle to verify CO2 abosrbed from Solar Panels",
		"account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"otherinfo":"bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi",
		"dtblockchain": "Mon Aug 16 2021 10:35:30 GMT+0400 (Gulf Standard Time)"
	}]
}
```

## Impact Actions - Categories

You can query all the Oracles registered using the following GET:

```sh
https://testnode.bitg.org:9443/impactactioncategories
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

to obtain a json answer like the following:  
```json
{
		"oracles": [{
		"id": 1,
		"blocknumber": 679,
		"txhash": "0xc0c60e4741a879b26729137c8ccac5ae3610866d2a5340a03121b07cba969ba1",
		"signer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"description": "Oracle to verify CO2 abosrbed from Solar Panels",
		"dtblockchain": "Mon Aug 16 2021 10:35:30 GMT+0400 (Gulf Standard Time)"
	}]
}
```
## Impact Actions - Proxies

You can query all the proxies registered using the following GET:

```sh
https://testnode.bitg.org:9443/impactactioncategories
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

to obtain a json answer like the following:  
```json
{
	"proxies": [{
		"id": 0,
		"blocknumber": 770,
		"txhash": "0x2f123ad6341a4fbcfad2cb4815bc1122cf5952c6be82eedcfd377d55ba27ce09",
		"sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"account": "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
		"dtblockchain": "Mon Aug 16 2021 10:50:40 GMT+0400 (Gulf Standard Time)"
	}]
}
```











