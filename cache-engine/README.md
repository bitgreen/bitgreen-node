# Cache Engine for BitGreen BlockChain
Bitgreen is a parachain on the Polkadot Network. This repository will manage a caching engine that will (1) download and store blocks to a Postgresql DB, (2) calculate derived data from on-chain sources, which is also stored in the DB, and (3) make this data available via public API.

The solution is managed by 2 shell script:  

- bitgreen-cache-server-install.sh will install and configure a clean Debian install as a (1) Bitgreen node, (2) Posstgresql database for cached chain data and calculated values, and (3) an API server to make public endpoint available on the server
- bitgreen-cache-server-manager.sh can be used to check the status of services and launch all required programs after a reboot. 

The install script will handle install and configure your node.

## CRAWLER
The crawler has the mission to wait for new blocks and store the transactions in the database.
Written in Python 3, the library used is:   [https://github.com/polkascan/py-substrate-interface](https://github.com/polkascan/py-substrate-interface)

### Requirements:

You should have installed:  
- A clean Debian install

## Installation and Setup  
This instructions refers to an installation for LINUX operating system.  
Execute from command line:
```sh
chmod +x bitgreen-cache-server-install.sh
./bitgreen-cache-server-install.sh
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


## Query transactions by Account
You can query the transactions done on BITG tokens by https calls to the following endpoint:  

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
		"gasfees": 12400000000000,
		"dtblockchain": "Tue Aug 03 2021 13:16:50 GMT+0200 (Central European Summer Time)"
	}, {
		"id": 2,
		"blocknumber": 162390,
		"txhash": "0xd02b3a584ed96d5b5dff266112403a63aaa2ea43257a9ed482b0efd6080ecd64",
		"sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"recipient": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
		"amount": 54000000000000000000,
		"gasfees": 12400000000000,
		"dtblockchain": "Tue Aug 03 2021 13:17:10 GMT+0200 (Central European Summer Time)"
	}, {
		"id": 3,
		"blocknumber": 162424,
		"txhash": "0x1b2495ac57c152f37aeb59ed28d833ccf7b7e6cf3b7cf345a1e194c0c5e562fd",
		"sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"recipient": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
		"amount": 21000000000000000000,
		"gasfees": 12400000000000,
		"dtblockchain": "Tue Aug 03 2021 13:22:50 GMT+0200 (Central European Summer Time)"
	}, {
		"id": 4,
		"blocknumber": 162430,
		"txhash": "0xc1a74a4310fe6294a0fcdff58004009bfa48525d49a671c0a58834bce56e6cdb",
		"sender": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"recipient": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
		"amount": 21000000000000000000,
		"gasfees": 12400000000000,
		"dtblockchain": "Tue Aug 03 2021 13:23:50 GMT+0200 (Central European Summer Time)"
	}]
}
```

## Query Single Transaction by Txhash

You can query a single transaction on BITG tokens searching by txthash, for example:  

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
		"gasfees": 12400000000000,
		"dtblockchain": "Tue Aug 03 2021 13:16:50 GMT+0200 (Central European Summer Time)"
}
```

## Query  Assets List (ERC20)
You can query the list of ERC20s token by https calls to the following endpoint:  

```
https://testnode.bitg.org:9443/assets
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

to obtain a json answer self-explained:  
```json

```

## Query transactions on Assets (ERC20) by Account
You can query the transactions done on ERC20 token by https calls to the following endpoint:  

```
https://testnode.bitg.org:9443/transactions
```
where "testnode.bitg.org" should be replaced with your node name or ip address.  

possible parameters:  
- account - the SS58 address of the account.   
- assetid - is the asset id of the ERC20 token
- dst - Date/time of starting the selection of the  transactions.  
- dse - Date/time of ending the selection of the transactions.  

For example you can browse or make an https get:  

```sh
https://testnode.bitg.org:9443/assetstransactions?account=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY&assetid=1&dts=2021-08-03+00:00:00&dte=2021-08-20+23:59:59
```
to obtain a json answer self-explained:  
```json
{
	"assetstransactions": [{
		"id": 1,
		"blocknumber": 1500,
		"txhash": "0x821fdff7e1848f7be7db746eff2ba2f1e749cbc5bf71eb67c9a92bbfc7b29c56",
		"signer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"sender": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"recipient": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"assetid": "1",
		"category": "Minted",
		"amount": 100000,
		"dtblockchain": "Fri Aug 27 2021 11:53:30 GMT+0400 (Gulf Standard Time)"
	}, {
		"id": 2,
		"blocknumber": 1585,
		"txhash": "0xe57fd032e9f1b54d147a338425881e86a0af2b65536cb8a4ca20d9b34099e869",
		"signer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"sender": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"recipient": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"assetid": "1",
		"category": "Minted",
		"amount": 100000,
		"dtblockchain": "Fri Aug 27 2021 12:07:40 GMT+0400 (Gulf Standard Time)"
	}, {
		"id": 3,
		"blocknumber": 1587,
		"txhash": "0x8f22591a74def67c2f576b3571aa534c525bdeea97fd59eded8e7177b6c725d4",
		"signer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"sender": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"recipient": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
		"assetid": "1",
		"category": "Transfer",
		"amount": 100,
		"dtblockchain": "Fri Aug 27 2021 12:08:00 GMT+0400 (Gulf Standard Time)"
	}, {
		"id": 4,
		"blocknumber": 1755,
		"txhash": "0xc77ba67f181d95ee6a14b91d22a6aeea7c3ef54dfe14b3ebed716c9efa344a5d",
		"signer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"sender": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"recipient": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"assetid": "1",
		"category": "Burnt",
		"amount": 1,
		"dtblockchain": "Fri Aug 27 2021 12:36:00 GMT+0400 (Gulf Standard Time)"
	}, {
		"id": 5,
		"blocknumber": 82,
		"txhash": "0x6e442c2e01ea5289757b40c0183016144f7db08bc8a12ef4b1d68150c749ec43",
		"signer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"sender": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
		"recipient": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"assetid": "1",
		"category": "Minted",
		"amount": 1000000000,
		"dtblockchain": "Mon Aug 30 2021 06:59:30 GMT+0400 (Gulf Standard Time)"
	}]
}
```

## Query Single Transaction on Assets (ERC20) by Txhash

You can query a single transaction on BITG tokens searching by txthash, for example:  

```sh
https://testnode.bitg.org:9443/assetstransaction?txhash=0x91935fe79c429695eb34ca2a191ee2ec7e40b2734691ce7a8ba758c7e1f01e22
```
to obtain a json answer like the following:  
```json
{
	"id": 1,
	"blocknumber": 1500,
	"txhash": "0x821fdff7e1848f7be7db746eff2ba2f1e749cbc5bf71eb67c9a92bbfc7b29c56",
	"sender": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
	"recipient": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
	"assetid": "1",
	"category": "Minted",
	"amount": 100000,
	"dtblockchain": "Fri Aug 27 2021 11:53:30 GMT+0400 (Gulf Standard Time)"
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











