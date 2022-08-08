# Carbon Credits Pallet

The Carbon Credits pallet manages the creation and retirement of Carbon Credits units for the bitgreen runtime.

### Background on Carbon Credits

Each Carbon Credits represents a reduction or removal of one tonne of carbon dioxide equivalent (CO2e) achieved by a project. Carbon Creditss are characterized by a number of quality assurance principles which are confirmed through the project validation and verification process. Carbon Credits are ultimately purchased and retired by an end user as a means of offsetting their emissions.

### OnChain Representation

 Credits in a project are represented in terms of batches, these batches are usually seperated in terms of 'vintages'. The vintage
 refers to the `age` of the credit. So a batch could hold 500credits with 2020 vintage.
 We use `issuance_year` to represent the vintage of the credit, this is important in minting and retirement options since in a project
 with multiple vintages we always mint/retire tokens from the oldest vintage.

 When a project is created, we take the total supply of the credits available (entire supply in the registry), then as the originator
 chooses, tokens can be minted for each credit at once or in a staggered manner. In every mint, the `minted` count is incremented and
 when credit is retired, the `retired` count is incremented.

```
 Conditions :
    - `minted` is always less than or equal to `total_supply`
    - `retired` is always less than or equal to `minted`
    
```

  Example : For a project that has a supply of 100 tokens, minted and retired 100 tokens, the struct will look as : `Batch {..., total_supply : 100, minted : 100, retired : 100 }`

``` rust
/// Onchain Representation of a single batch
pub struct Batch<T: pallet::Config> {
     /// Descriptive name for this batch of credits
    pub name: ShortStringOf<T>,
    /// UUID for this batch, usually provided by the registry
    pub uuid: ShortStringOf<T>,
    /// The year the associated credits were issued
    pub issuance_year: u32,
    /// start date for multi year batch
    pub start_date: u32,
    /// end date for multi year batch
    pub end_date: u32,
    /// The total_supply of the credits - this represents the total supply of the
    /// credits in the registry.
    pub total_supply: T::Balance,
    /// The amount of tokens minted for this Carbon Credits
    pub minted: T::Balance,
    /// The amount of tokens minted for this Carbon Credits
    pub retired: T::Balance,
}
```

A project can represent Carbon Creditss from multiple batches. For example a project can have 100 tokens of 2019 vintage and 200 tokens of 2020 vintage. In this case the project can package these two vintages to create a Carbon Credits token that has a supply of 300 tokens. These vintages can be represented inside a batchgroup, in this case, it is important to remember that the minting and retirement always gives priority to the oldest vintage.
Example : in the above case of 300 tokens, when the originator mints 100 tokens, we first mint the oldest (2019) credits and only once the supply is exhausted we move on the next vintage, same for retirement.
### Asset Handler

The Carbon Credits pallet depends a fungible asset handler that implements the fungibles trait like pallet-assets. The Carbon Credits pallet creates an AssetClass for each `Carbon Credits_id` and mints the amount of tokens to the respective account. The `asset_id` in the Carbon CreditsDetail represents the asset created by the Asset Handler.

We also rely on the Asset Handler to help the user manage these tokens, currently the user can only transfer these tokens, the other functions like burn/mint are gated to only be performed by the Carbon Credits pallet, this is to ensure the retired and supply count is always updated.


### Extrinsics

* `create`: Creates a new project onchain with details of batches of credits
* `mint`: Mint a specified amount of token credits
* `retire`: Burn a specified amount of token credits
### Permissioned Functions
* `force_add_authorized_account`: Adds a new_authorized_account to the list
* `force_remove_authorized_account`: Removes an authorized_account from the list
* `force_set_next_asset_id`: Set the NextAssetId in storage
* `approve_project`: Set the project status to approved so minting can be executed
