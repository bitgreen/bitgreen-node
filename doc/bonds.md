# Bonds

This pallet is the core runtime to manage the green bonds.

"Bonds" allows to:

- Configure the authorized accounts to different level of operations.
- Create Bonds
- Approve Bonds
- Create Legal Opinions
- Underwrite Bonds


The pallet is called "Bonds" and below you can find the "Extrinsics" and queries available, ordered by logic of use:  

## Create/Change Settings
This function allows to manage the different possibile configurations. It's accessible by SUDO only.
```rust
create_change_settings(key: Vec<u8>, configuration: Vec<u8>)
```
where:
- key is a string identifying a specific settings
- configuration is a json structure that can be different for each "key".

- This configuration allows to set the accounts of the operator enabled to submit and approve the KYC (Know Your Client Data).    
key=="kyc" {"manager":"xxxaccountidxxx","supervisor":"xxxxaccountidxxxx","operators":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}
for example:  
{"manager":"5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY","supervisor":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","operators":["5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

- This configuration allows to set the accounts of the operator enabled to approve a Bond. The Bond is submitted from the operator of the Fund.  
key=="bondapproval" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...],"mandatoryunderwriting":"Y/N","mandatorycreditrating":"Y/N","mandatorylegalopinion":"Y/N"}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"],"mandatoryunderwriting":"Y","mandatorycreditrating":"Y","mandatorylegalopinion":"Y"}  

- This configuration allows to set the accounts of the operator enabled to approve an underwriter.  
key=="underwriterssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

- This configuration allows to set the accounts of the operator enabled to submit the data for an insurer.  
key=="insurerssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

- This configuration allows to set the accounts of the operator enabled to submit the data for a credit agency.  
key=="creditratingagencies" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

- This configuration allows to set the accounts of the operator enabled to submit the data for a lawyer.  
key=="lawyerssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

- This configuration allows to set the accounts of the operator enabled to approve the collaterals data.  
key=="collateralsverification" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

- This configuration allows to set the accounts of the operator enabled to approve a new fund.
key=="fundapproval" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

- This configuration allows to configure the type of document requested (general configuration).  
key=="infodocuments" {"documents:[{"document":"xxxxdescription"},{"document":"xxxxdescription"}]}  
for example: {"documents":[{"document":"Profit&Loss Previous year"},{"document":"Board Members/Director List"}]}  

- This configuration allows to configure the minum reserve required for the insurer (general configuration) (no decimals).  
key=="insuranceminreserve" {"currency":"xxxx"},"reserve":"xxxamountxx"}]}  
for example:  {"currency":"USDC"},"reserve":"1000000"}]}  

        
## Query Settings
You can query the settings above calling the function:  
```rust
Settings(key: Vec<u8>)
```
where key is one the key used in the storage.


## Create/Change KYC
The following function allows to create/change the KYC (Know Your Client) data.  
Bond issuers as bond buyers shall be identified. Only the configured accounts are allowed.  
The function name is:  
```rust
create_change_kyc(accountid: T::AccountId, info: Vec<u8>)
```
Where:
- accountid is the Account Id on the blockchain of the identified entity. 
- info is a json structure with the KYC data as follows:
```json
{"name":"Smith and Wesson Inc","address":"103, Paris Boulevard","city":"London","zip":"00100","state":"England","country":"Great Britain","phone":"+441232322332","website":"https://www.smith.co.uk","ipfsdocs":[{"description":"Balance Sheet 2020","ipfsaddress":"42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"},{"description":"Revenue Report 2021","ipfsaddress":"b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"}]}  
```

## Approve KYC
The KYC must be approved from a supervisors and the manager. The function for approval is:  
```rust
kyc_approve(account:AccoundId)
```
where the account is the account id to be approved.
The KYC is finalized once both a supervisor and manager has signed the approval.
Till it's not finalized the KYC can be cancelled from the manager/supervisor

## Delete KYC
The KYC can be deleted from the manager of supervisors till it's not approved, calling this function:  
```rust
kyc_delete(account:AccoundId)
```
where the account is the account id of the KYC to be deleted.

## Investment Fund - Create/Change
The KYC operators are enabled to create/change an investment fund using the following function:
```rust
fund_create_change(accountid: T::AccountId, info: Vec<u8>)
```
Where:  
- accountid is the account of the fund manager.
- info is  json structure with the fund data:  
```json
{
	"name": "Smith and Wesson Investments Inc",
	"address": "103, Paris Boulevard",
	"city": "London",
	"zip": "00100",
	"state": "England",
	"country": "Great Britain",
	"phone": "+441232322332",
	"website": "https://www.smith.co.uk",
	"ipfsdocs": [{
		"description": "Balance Sheet 2020",
		"ipfsaddress": "42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"
	}, {
		"description": "Revenue Report 2021",
		"ipfsaddress": "b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"
	}],
	"initialfees": "100",
	"yearlyfees": "75",
	"performancefees": "50",
	"fundtype": "H",
	"depositaccountid": "5GhRnzRTohd8f4bLvozc9u7qqDy9whnoZMF7hzaFVQBRsMxG",
	"fundmanagers": ["5H1TaQBgtVwPA3Bk7r9feEcKTFgLAHKHPu5ghZaMU8iqbdXu", "5DoHzjMEKoWfjbmSEYDHU7RWkiJFMZwUfiqENomJgGfysQbk"]
}
```
for copy/paste:
```json
{"name":"Smith and Wesson Inc","address":"103, Paris Boulevard","city":"London","zip":"00100","state":"England","country":"Great Britain","phone":"+441232322332","website":"https://www.smith.co.uk","ipfsdocs":[{"description":"Balance Sheet 2020","ipfsaddress":"42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"},{"description":"Revenue Report 2021","ipfsaddress":"b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"}],"initialfees":"100","yearlyfees":"75","performancefees":"50","fundtype":"H","depositaccountid":"5GhRnzRTohd8f4bLvozc9u7qqDy9whnoZMF7hzaFVQBRsMxG","fundmanagers":["5H1TaQBgtVwPA3Bk7r9feEcKTFgLAHKHPu5ghZaMU8iqbdXu","5DoHzjMEKoWfjbmSEYDHU7RWkiJFMZwUfiqENomJgGfysQbk"]}  
```
Where:
- "ipfsdocs" are the public documents published on IPFS part of the information set.  
- "initialfees" - is the pergentage of fees to pay at the subscription on the invested capital (2 decimals as integer so 100 = 1%).  
- "yearlyfees" - is the percentage of annual fees computed on the invested capital (2 decimals as integer so 100 = 1%).  
- "performancefees" - is the percentage of performance fees computed on the capital gain (2 decimals as integer so 100 = 1%).  
- "fundtype" - can be "H" for Hedge Fund or "E" for enterprise fund.  
- "depositaccountid" - is the account where to received the funds.  
- "fundmanagers" - are the accounts enabled to manage the fund operations.

## Investment Fund - Approval

The investment fund is subject to approval from the supervisor and manager of KYC.
The Manager and the Supervisor can use this function to approve the submitted fund:
```rust
fund_approve(accountid: T::AccountId)
```
where account id is the account id used to identify the fund.
Once both Manager and Supervisors has signed the approval, a new state in "FundsApproved" will be stored.

## Bond - Create/Change
This function allows a fund manager to submit a Bond structure for further approval.
The bond data is changeble till the approval process is not started.  
The function to call is the following:  
```rust
bond_create_change(id: u32,info: Vec<u8>)
```
Where:
- id is a unique number that must be different for each Bond.
- info is a json structure with a huge set of fields as follows:
```json
{
	"owner": "5GhRnzRTohd8f4bLvozc9u7qqDy9whnoZMF7hzaFVQBRsMxG",
	"totalamount": "10000000",
	"currency": "USDC",
	"country": "US",
	"interestrate": "100",
	"interestype": "X",
	"maturity": "36",
	"instalments": "1",
	"graceperiod": "0",
	"acceptedcurrencies": ["USDC", "USDT", "DAI"],
	"subordinated": "N",
	"putoption": "Y",
	"callption": "N",
	"ipfsdocs": [{
		"description": "Balance Sheet 2020",
		"ipfsaddress": "42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"
	}, {
		"description": "Revenue Report 2021",
		"ipfsaddress": "b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"
	}]
}
```
for copy/paste:  
```json
{"owner":"5GhRnzRTohd8f4bLvozc9u7qqDy9whnoZMF7hzaFVQBRsMxG","totalamount":"10000000","currency":"USDC","country":"US","interestrate":"100","interestype":"X","maturity":"36","instalments":"1","graceperiod":"0","acceptedcurrencies":["USDC","USDT","DAI"],"subordinated":"N","putoption":"Y","callption":"N","ipfsdocs":[{"description":"Balance Sheet 2020","ipfsaddress":"42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"},{"description":"Revenue Report 2021","ipfsaddress":"b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"}]}
```
where:
- "owner" is the account of the owner and signer.  
- "totalamount" is the amount of the bond in the designated currency with 0 decimals.  
- "currency" is the designated currency usuallya  stable coing.  
- "country" is the country of reference od the bond. It's used specially for indexed interest rates.  
- "interestrate" is the base interest rate of the bond as integer considering 2 decimals, for example 100 = 1%.  
- "interestype" is the type of interest that can be: X= FiXed Rate, F=Floating Rate, Z= Zero Interest Rate, I=Inflation Linked Rate.  
- "maturity" is the number of months till the natural end of the bond.  
- "instalments" is the number of instalment for the pay back till the natural end of the bond.  
- "graceperiod" the months of grace period, where no interested are accrued.  
- "acceptedcurrencies" is an array of acceptec currency to buy shares of the bond.  
- "subordinated" Y/N for subordinated bond.  
- "putoption" Y/N if there is a "PUT option.  
- "putvestingperiod" is the number of months of vesting for the Put Option.  
- "calloption" Y/N if there is a "CALL" option.  
- "callconvertibleoption" Y/N for convertible option for the CALL.  
- "ipfsdocs" are the documents annexed to the bond.  

## Bonds - Approve
The delegated accounts for the approval process must sign an approval before a bond can be operational.  
The function to approve a bond, is the following:  
```rust
    bond_approve(bondid: u32)
```
where:  
- "bondid" is the unique id of the bond to be approved.
Once the approval signature has been executed from one member of the commitee and the manager, the bond is approved and an additional state is stored on chain.

## Credit Rating - Agency Create
The delegated accounts as from setttings,  can create a credit rating agency on chain calling this function:  
```rust
credit_rating_agency_create_change(accountid: T::AccountId, info: Vec<u8>)
```
where:  
- "accountid" is the account of the credit rating agency;
- info is a json structure as follows:
```json
{
	"name": "xxxxxxx",
	"website": "xxxxxxxx",
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```
for copy/paste:
```json
{"name":"xxxxxxx","website":"xxxxxxxx","ipfsdocs":[{"description":"xxxxxxxx","ipfsaddress":"zzzzzzzzzz"},{"description":"xxxxxxxxx","ipfsaddress":"zzzzz"}]} 
```

## Credit Rating - Agency Destroy
The delegated manager as from settings, can delete a credit rating agency on chain calling this function:  
```rust
credit_rating_agency_destroy(accountid: T::AccountId)
```
where:  
- "accountid" is the account of the credit rating agency.



## Credit Rating - Create
The credit rating agency can publish on chain his rating. The submission is irrevocable and it can be done calling the following function:  
```rust
credit_rating_create(bondid: u32, info: Vec<u8>) -> dispatch::DispatchResult {
```
where:  
- "bondid" is the unique id of the bond the rating is referring to.  
- "info" is a json structure as follows:  
```json
{
	"description": "xxxxxxx",
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```
## Collaterals
The bond emitter/owner can submit collaterals guarantees that a are subject to approval.

## Collaterals - Create
The owner of the bond, can submit a collateral guarantee using the following function:  
```rust
collaterals_create(bondid: u32, collateralid: u32, info: Vec<u8>)
```
where:  
- "bondid" is the unique id of the bond the collateral is referring to.  
- "collateralid" is the unique id of the collateral guarantee.
- "info" is a json structure as follows:  
```json
{
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```
## Collaterals - Approve
The delegated accounts as configured in the settings can approve the collateral calling the following function:  
```rust
collaterals_approve(bondid: u32, collateralid: u32, info: Vec<u8>)
```
where: 
- "bondid" is the unique id of the bond the collateral is referring to.  
- "collateralid" is the unique id of the collateral guarantee.
- "info" is a json structure as follows:  
```json
{
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```
## Underwriters 
It's possible to store information about underwriters of the bond. 

## Underwriters  - Create
The delegated accounts can create a new underwriter calling the following function:  
```rust
undwerwriter_create(underwriter_account: T::AccountId, info: Vec<u8>)
```
where:  
- "underwriter_account" is the account of the under writer.
- "info" is a json structure as follows:
```json
{
	"name": "xxxxxxx",
	"website": "xxxxxxxx",
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```

## Underwriters  - Destroy
The delegated accounts can delete an underwriter calling the following function:  
```rust
undwerwriter_destroy(underwriter_account: T::AccountId)
```
where:  
- "underwriter_account" is the account of the under writer to be deleted.

## Insurers 
It's possible to store information about insurers on the risk of no-payment for the bond. 

## Insurers  - Create
The delegated accounts can create a new Insurers calling the following function:  
```rust
insurer_create(insurer_account: T::AccountId, info: Vec<u8>)
```
where:  
- "insurer_account" is the account of the Insurer.
- "info" is a json structure as follows:
```json
{
	"name": "xxxxxxx",
	"website": "xxxxxxxx",
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```

## Insurers  - Destroy
The delegated accounts can delete an Insurers calling the following function:  
```rust
undwerwriter_destroy(insurer_account: T::AccountId)
```
where:  
- "insurer_account" is the account of the Insurer to be deleted.

## Insurance - Create
An approved insurer can create a new insurance within the reserve has staken. The function to call is:  
```rust
insurance_create(uid: u32, info: Vec<u8>) 
```
where:  
- "uid" is a unique id of the insurance for the signing insurer.
- "info" is a json structure as follows:  
```json
{
	"bondid": "xxx",
	"maxcoverage": "xxxxxxxxx",			//the max amount of coverage
	"payer":"xxxxxxxxxxxxxx",			// is the payer account supposed to sign the insurance
	"beneficiary": "xxxxxxxxxxxx",		// is the account of the beneficiary of the insurance (can be the same of the payer or not)
	"premium":"xxxxxxx"					// amount of the premium to pay in native tokens
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```
## Insurance - Sign & Pay
The payer can proceed to sign the insurance and pay the premium requested.The function to call is:  
```rust
insurance_sign(insurer_account: T::AccountId,uid: u32)
```
where:  
- "insurer_account" is the account of the insurere.
- "uid" is the unique id of the insurance to sign and pay.


## Insurance - Destroy
An approved insurer can destroy an insurance if it has not yet been signed and paid. The function to call is:  
```rust
insurance_destroy(uid: u32) 
```
where:  
- "uid" is a unique id of the insurance for the signing insurer.

## Lawyer 
It's possible to store on blockchain the data of approved lawyers. Only the delegate accounts can mamage the lawyer registraton and cancellation.  

## Lawyer - Create
The delegated accounts can create a new lawyer calling the following function:  
```rust
lawyer_create(lawyer_account: T::AccountId, info: Vec<u8>)
```
where:  
- "lawyeraccount" is the account of the Lawyer.
- "info" is a json structure as follows:
```json
{
	"name": "xxxxxxx",
	"website": "xxxxxxxx",
	"ipfsdocs": [{
		"description": "xxxxxxxx",
		"ipfsaddress": "zzzzzzzzzz"
	}, {
		"description": "xxxxxxxxx",
		"ipfsaddress": "zzzzz"
	}]
}
```
## Lawyer - Destroy
The delegated accounts can removed new lawyer calling the following function:  
```rust
lawyer_destroy(lawyer_account: T::AccountId)
```

## Interbank Rates
Some bonds are linked to the interbank rates, so we have some function to store on chain the interbanks rates. Such functions are accessible only to the super user (Sudo Call).  

## Interbank Rates - Create
The super user can create a new interbank rate calling the following function:  
```rust
interbankrate_create(country_code: Vec<u8>, date: Vec<u8>, rate: u32)
```
where:  
- "country_code" is the ISo country code of the country of reference for the rate.
- "date" is the date of validity of the interbank rate in the format YYYY-MM-DD.
- "rate" is the interbank rate stored as integered considering 2 decimals. For example 320 = 3.2%.

## Interbank Rates - Destroy
The super user can remove an interbank rate calling the following function:  
```rust
interbankrate_destroy(country_code: Vec<u8>, date: Vec<u8>)
```
where:  
- "country_code" is the ISo country code of the country of reference for the rate.
- "date" is the date of validity of the interbank rate in the format YYYY-MM-DD.

## Inflation Rates
Some bonds are linked to the Inflation rates, so we have some function to store on chain the Inflation rates. Such functions are accessible only to the super user (Sudo Call).  

## Inflation Rates - Create
The super user can create a new Inflation rate calling the following function:  
```rust
inflationrate_create(country_code: Vec<u8>, date: Vec<u8>, rate: u32)
```
where:  
- "country_code" is the ISo country code of the country of reference for the rate.
- "date" is the date of validity of the Inflation rate in the format YYYY-MM-DD.
- "rate" is the Inflation rate stored as integered considering 2 decimals. For example 320 = 3.2%.

## Inflation Rates - Destroy
The super user can remove an Inflation rate calling the following function:  
```rust
inflationrate_destroy(country_code: Vec<u8>, date: Vec<u8>)
```
where:  
- "country_code" is the ISo country code of the country of reference for the rate.
- "date" is the date of validity of the Inflation rate in the format YYYY-MM-DD.


## Countries Codes
Many functions in the pallet need a country code. The super user (by SUDO call), is enabled to load and update a table of country codes and their english name.

## Countries Codes - Create
The function to load a new country code can be called as follows:  
```rust
iso_country_create(countrycode: Vec<u8>, countryname: Vec<u8>)
```
where:  
- "contrycode" should be the ISO standard country code;
- "countryname" should be the country name in English as published from ISO.

## Countries Codes - Create
The function to remove an existing country code can be called as follows:  
```rust
iso_country_destroy(countrycode: Vec<u8>)
```
where:  
- "contrycode" should be the ISO standard country code.

## Currencies
The functions are allowing only a certain set of crypto currencies. There are functions to configure them.

## Currencies - Create
The super user by SUDO calls, can add a new currecies between those acceped as BOND currency. The function to call is:  
```rust
currency_create(currencycode: Vec<u8>, info: Vec<u8>)
```
where:  
- "currencycode" is a unique short code for the crypto currency.  
- "info" is a json structure as follows:  
```json
 {
 	"name": "xxxxxxx",
 	"category": "c(rypto)/f(iat)",
 	"country": "countryisocode",
 	"blockchain": "Ethereum(...)",
 	"address": "xxxfor_crypto_currencyxxx"
 }
```
for example:  
```json
{"name":"Bitcoin","category":"c","country":"AE","blockchain":"Bitcoin","address":"not applicable"}
```
or
```json
{"name":"American Dollars","category":"f","country":"US","blockchain":"not applicable","address":"not applicable"}
```

## Currencies - Destroy
The super user by SUDO calls, can removecurrecies from those acceped as BOND currency. The function to call is:  
```rust
currency_destroy(currencycode: Vec<u8>)
```
where:  
- "currencycode" is a unique short code for the crypto currency.  








TODO:
- To add function to delete FUNDS (till not approved)
- Add function to freeze the fund from further operations
- Deny approval for KYC ?
- Possibility to lock bond for getting additional subscriber ?
- review weights
- clippy on the pallet
- testing suite
- Oracle to get Inflation rate and store periodically on Chain.






