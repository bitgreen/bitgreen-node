# VCU(Verified Carbon Units) Module

## Overview

Verra is a global leader helping to tackle the world’s most intractable environmental and social challenges by developing and managing standards that help the private sector, countries, and civil society achieve ambitious sustainable development and climate action goals.
Verra verifies projects and infrastructure that generate carbon credits and grants Verified Carbon Units (VCU).

The VCU data (serial number, project, amount of CO2 in tons, photos, videos, documentation) will  be stored off-chain on IPFS (www.ipfs.io). This module stores new VCUs on chain including IPFS hash. 



## Functions

### Create Proxy Settings  
This function allows to define some accounts with administrator's rights', keeping the super user key safer. 
It's accessible only to Super User (SUDO call):  
create_proxy_settings(accounts: Vec<u8>)  
the accounts field is a json structure: {"accounts": ["accountid1", "accountid2"] }  
For example:  
{"accounts":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

### Destroy Proxy Settings
This function allows to remove the proxy settings if already created.  
It's accessible only to Super User (SUDO call):  
destroy_proxy_settings()  

### Add Authorized Account
This function allows to authorize an account to manage AVG (Assets Generating VCU).
It's accessible only to Super User (SUDO call):  
add_authorized_account(account_id: T::AccountId, description: Vec<u8>);
- account_id should be the account to authorize
- description field is free text minimum 4 chars
If you call the function for an account already existing, the description is replaced.

### Destroy Authorized Account
This function allows to revoke the authorization to manage AVG (Assets Generating VCU).
It's accessible only to Super User (SUDO call):  
destroy_authorized_account(account_id: T::AccountId);
- account_id should be the account to be revoked.

## Create New Asset Generating VCU  (AGV)
This function creates a new AGV. Each AGV should have an id within the same account.  
The function can be called from SUDO or from an authorized account by "add_authorized_account()":  
create_asset_generating_vcu(avg_account_id: T::AccountId, avg_id: u32, content: Vec<u8>)
- avg_account_id is the owner account of the AVG
- avg_id is a unique id of the asset for the same owner account, usually starting from 1 for each account.
- contents is a json structure as follows:
{
    Description: Vec<u8> (max 64 bytes) (mandatory)
	ProofOwnership: ipfs link to a folder with the proof of ownership (mandatory)
	OtherDocuments: [{description:string,ipfs:hash},{....}], (Optional)
	ExpiringDateTime: DateTime, (YYYY-MM-DD hh:mm:ss) (optional)
	NumberofShares: Integer (maximum 10000 shares mandatory)
}
ex: {"description":"Description", "proofOwnership":"ipfslink", "numberOfShares":10000}

## Destroy Asset Generating VCU  (AGV)
This function destroy an AVG.  
The function can be called from SUDO or from an authorized account by "add_authorized_account()":  
destroy_asset_generating_vcu(avg_account_id: T::AccountId, avg_id: u32)  
- avg_account_id is the owner account of the AVG  
- avg_id is a unique id of the AVG to remove within the owner account  

## Mint Shares for AVG
This functions allow to mint new shares of an AVG for a recipient account.  
The function can be called from SUDO or from an authorized account by "add_authorized_account()":  
mint_shares_asset_generating_vcu(recipient: T::AccountId, avg_account: Vec<u8>, number_of_shares: u32)  
- "recipient" is the account that shall receive the new shares
- "avg_account" is the account id of the AVG followed by - avg_id for example: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1  
- "number_of_shares" is the number of shares to be minted and sent to the recipient

## Burn Shares for AVG
This functions allow to burn shares of an AVG for a recipient account.  
The function can be called from SUDO or from an authorized account by "add_authorized_account()":  
burn_shares_asset_generating_vcu(recipient: T::AccountId, avg_account: Vec<u8>, number_of_shares: u32)  
- "recipient" is the account that shall have the shared decreased for the amount burned
- "avg_account" is the account id of the AVG followed by - avg_id for example: 5Hdr4DQufkxmhFcymTR71jqYtTnfkfG5jTs6p6MSnsAcy5ui-1  
- "number_of_shares" is the number of shares to be burned

