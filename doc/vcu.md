# VCU

Verra is a global leader helping to tackle the world’s most intractable environmental and social challenges by developing and managing standards that help the private sector, countries, and civil society achieve ambitious sustainable development and climate action goals.
Verra verifies projects and infrastructure that generate carbon credits and grants Verified Carbon Units (VCU).

This pallet is the core runtime to manage the VCU.

"VCU" allows to:

- Configure the authorized accounts to different level of operations.
- Create/Destroy proxy setting
- Manage AVG Shares
- Mint/Burn/transfer AVG Shares
- Schedule to generate VCU periodically


The pallet is called "VCU" and below you can find the "Extrinsics" and queries available, ordered by logic of use:  

## Create/Change Proxy Settings
We need the possibility to define some administrator accounts for the pallet VCU without using the super user account.

This function allows to create a proxy settings that allow to define some accounts with administrato rights on the pallet. It's accessible by SUDO only.
```rust
create_proxy_settings(accounts: Vec<u8>)
```
where:
- accounts are some administrator accounts for the pallet VCU

All accounts will be store with key: “admin”

{"accounts": ["accountid1", "accountid2"] }  
for example:  
{"accounts":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}

        
## Query Settings
You can query the settings above calling the function:  
```rust
Settings(key: Vec<u8>)
```
where key is one the key used in the storage.

## Destroy Proxy Settings  

This function destroys proxy settings with key:""admin. It's accessible by SUDO only.

```rust
destroy_proxy_settings()
```

## Add Authorized Account
The Assets Generating VCU (Verified Carbon Credit) can be stored/changed only from approved accounts. It's accessible by SUDO only.

This function allows to store the enabled Accounts on chain.
```rust
add_authorized_account(account_id: T::AccountId, description: Vec<u8>)
```
where:
- account_id is id of approved account
- description could be "Verra" for example

## Query Authorized Account
You can query the AuthorizedAccountsAGV above calling the function:
```rust
AuthorizedAccountsAGV(account_id: T::AccountId)
```
where key is the account id used in the storage.

## Destroy Authorized Account

This function removes authorized account from storage. It's accessible by SUDO only.

```rust
destroy_authorized_account(account_id: T::AccountId)
```

## Create/Change Assets Generating VCU
The Assets Generating VCU (Verified Carbon Credit) should be stored on chain from the authorized accounts.
Initially the authorized account can belong to BitGreen.  
This function allows to store/change the AssetsGeneratingVCU. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.
A change is possible from the same account that has stored or SUDO account.
```rust
create_asset_generating_vcu(authorized_account: T::AccountId, signer: u32, content: Vec<u8>)
```
where:
- authorized_account is approved account stored in `AuthorizedAccountsAGV`
- signer is unique id
- content is json structure as follows:
```json
  {
  Description: Vec<u8> (max 64 bytes) (mandatory)
  ProofOwnership: ipfs link to a folder with the proof of ownership (mandatory)
  OtherDocuments: [{description:string,ipfs:hash},{....}], (Optional)
  ExpiringDateTime: DateTime, (YYYY-MM-DD hh:mm:ss) (optional)
  NumberofShares: Integer (maximum 10000 shares mandatory)
  }
```

## Query Assets Generating VCU
You can query the AuthorizedAccountsAGV above calling the function:
```rust
AssetsGeneratingVCU(account_id: T::AccountId, signer: u32)
```
where keys are the account id used in the storage and signer.

## Destroy Assets Generating VCU

This function removes Assets Generating VCU from storage. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

```rust
destroy_asset_generating_vcu(account_id: T::AccountId, signer: u32)
```

## Mint Assets Generating VCU Shares

The AVG shares can be minted from the Authorized account up to the maximum number set in the AssetsGeneratingVCU. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

```rust
mint_shares_asset_generating_vcu(recipient: T::AccountId, agv_id: Vec<u8>, number_of_shares: u32)
```

This function also -
- Checks the existance of the AGVID
- Stores the new share
- updates the total shares minted in AssetsGeneratingVCUSharesMinted
  if the same recipient has already the same kind of share the state should be updated accordingly.

## Burn Assets Generating VCU Shares

The AVG shares can be burned from the Authorized account in the AssetsGeneratingVCU. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

```rust
burn_shares_asset_generating_vcu(recipient: T::AccountId, agv_id: Vec<u8>, number_of_shares: u32)
```

This function also -
- Checks the existance of the shares
- Updates the state for `AssetsGeneratingVCUSharesMinted` and `AssetsGeneratingVCUShares`

## Transfer Assets Generating VCU Shares

The owner of the share can transfer them to other account by a function called. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

```rust
transfer_shares_asset_generating_vcu(sender: T::AccountId, recipient: T::AccountId, agv_id: Vec<u8>, number_of_shares: u32)
```

This function also -
- checks the availability of the shares
- updates the state for both accounts accordingly to the transfer

## Create Asset Generating VCU Schedule

Some of the AVG may have schedule to generate VCU periodically. This is a case for example of a forest that every 6 months may
generate a certain amount of CO2.

The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

```rust
create_asset_generating_vcu_schedule(account_id: T::AccountId, signer: u32, period_days: u64, amount_vcu: Balance, token_id: u32)
```

## Destroy Asset Generating VCU Schedule

Some of the AVG may have schedule to generate VCU periodically. This is a case for example of a forest that every 6 months may
generate a certain amount of CO2.
This function allows to remove the schedule above. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

```rust
destroy_asset_generating_vcu_schedule(origin, account_id: T::AccountId, signer: u32)
```

## Minting of Scheduled VCU

The assets generating VCU may I have a schedule stored in: AssetsGeneratingVCUSchedule
This function allows the minting of the VCU periodically. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

This function checks if it’s time to mint new VCU based on the schedule and the previous generated VCU stored in AssetsGeneratingVCUGenerated or 
if it’s time to generate new VCU, it mints the scheduled “Assets” (see Assets pallets), and stores in AssetsGeneratingVCUGenerated  a json structure with the following fields:
```json
{
“timestamp”: u32  (epoch time in seconds)
“amountvcu”: i32,
}
```
The function must deny further minting once is done till the new schedule is expired.
For example with a schedule every year, the minting will be executed only one time every 365 days.

```rust
mint_scheduled_vcu(origin, account_id: T::AccountId, signer: u32)
```

where:
- authorized_account is approved account stored in `AuthorizedAccountsAGV`
- signer is unique id

## VCU Retirement

The owner of the “VCUs”  can decide anytime to “retire”, basically burning them. The function must be accessible only from SUDO account or one of the accounts stored in AuthorizedAccountsAGV.

```rust
retire_vcu(account_id: T::AccountId, signer: u32, asset_id: u32, amount: u128)
```

where:
- authorized_account is approved account stored in `AuthorizedAccountsAGV`
- signer is unique id
- asset_id in pallet Asset

This function also -
- Burn the amount of tokens from pallet “Assets” and
- Update the number of burned VCU for the signer in VCUsBurnedAccounts
- Update the total of burned VCU for vcu type (assetsid) in VCUsBurned