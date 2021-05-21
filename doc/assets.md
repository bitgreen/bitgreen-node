# Assets Module   

The Assets module can be used to manage fungible assets tokens (ERC20)  
  
## Overview  

The Assets module provides functionality for asset management of fungible asset classes with a fixed supply, including:  
- Asset Issuance (Minting)  
- Asset Transferal  
- Asset Freezing  
- Asset Destruction (Burning)  

## Terminology  

- Admin: An account ID uniquely privileged to be able to unfreeze (thaw) an account and it's assets, as well as forcibly transfer a particular class of assets between arbitrary accounts and reduce the balance of a particular class of assets of arbitrary accounts.  
- Asset issuance/minting: The creation of a new asset, whose total supply will belong to the account that issues the asset. This is a privileged operation.  
- Asset transfer: The reduction of the balance of an asset of one account with the corresponding increase in the balance of another.  
- Asset destruction: The process of reduce the balance of an asset of one account. This is a privileged operation.  
- Fungible asset: An asset whose units are interchangeable.  
- Issuer: An account ID uniquely privileged to be able to mint a particular class of assets.  
- Freezer: An account ID uniquely privileged to be able to freeze an account from transferring a particular class of assets.  
- Freezing: Removing the possibility of an unpermissioned transfer of an asset from a particular account.  
- Non-fungible asset: An asset for which each unit has unique characteristics.  
- Owner: An account ID uniquely privileged to be able to destroy a particular asset class, or to set the Issuer, Freezer or Admin of that asset class.  
- Zombie: An account which has a balance of some assets in this pallet, but no other footprint on-chain, in particular no account managed in the frame_system pallet.  

## Goals  

The assets system in BitGreen  is designed to make the following possible:  
  
- Issue a new assets in a permissioned or permissionless way, if permissionless, then with a deposit required.  
- Allow accounts to hold these assets without otherwise existing on-chain (zombies).  
- Move assets between accounts.  
- Update the asset's total supply.  
- Allow administrative activities by specially privileged accounts including freezing account balances and minting/burning assets.  
  
## Interface  
  
### Permissionless Functions  
- create: Creates a new asset class, taking the required deposit.  
- transfer: Transfer sender's assets to another account.  

### Permissioned Functions  
- force_create: Creates a new asset class without taking any deposit.  
- force_destroy: Destroys an asset class. 
   
### Privileged Functions  
- destroy: Destroys an entire asset class; called by the asset class's Owner.  
- mint: Increases the asset balance of an account; called by the asset class's Issuer.  
- burn: Decreases the asset balance of an account; called by the asset class's Admin.  
- force_transfer: Transfers between arbitrary accounts; called by the asset class's Admin.  
- freeze: Disallows further transfers from an account; called by the asset class's Freezer.  
- thaw: Allows further transfers from an account; called by the asset class's Admin.  
- transfer_ownership: Changes an asset class's Owner; called by the asset class's Owner.  
- set_team: Changes an asset class's Admin, Freezer and Issuer; called by the asset class's Owner.  
  
### Public Functions  
- balance - Get the asset id balance of who.  
- total_supply - Get the total supply of an asset id.  
  
## Additional Documentation  
Please refer to the [module technical documentation for further information.](https://substrate.dev/rustdocs/v3.0.0/pallet_assets/index.html)  
