# Impact Actions

The mission of Bitgreen is to encourage a sustainable life safeguarding the environment.  
With this module, we create a tool to register and rewardas positive action with a real impact.  

"Impact Actions" allows to:

- Configure the Impact Actions and their rewards;
- Configure Oracles to validate the impact actions;
- Configure Auditors to validate manually the impact actions;
- Submit the Impact Actions for approval;
- Approve the Impact Actions, triggering the rewards.


The pallet is called "impactActions" and below you can find the "Exstrinsics" and queries available, ordered by logic of use:  

## Create Category  
The Super User (or the Technical Commitee) should create initially the categories for the  "Impact Actions", using "Sudo" calls.
The function to call is: 
```rust
createCategory(uid: u32,description: Vec<u8>)
```
where:  
- "uid" is a unique id of the category;  
- "description" is a free description of the category.  
  
The categories are used to match the competent auditors.  
  
## Destroy Category  

The Super User (or the Technical Commitee) can remove a category by  "Sudo" calls, calling the function:
```rust
destroyCategory(uid: u32)
```

where:  
- "uid" is a unique id of the category.  

## Query Category
You can query the state of the category on the blockchain calling the function:
```rust
impactActionsCategories(uid: u32) -> Vec<u8>
```
where uid is the unique id of the category and you will get the description or empty if it has been found.



## Create Impact Action Configuration  
The Super User (or the Technical Commitee) should create the impact action configuration by "Sudo" calls. 
This is the most complex data structure of this module.  
The function to call is: 
```rust
createImpactAction(uid: u32, configuration: Vec<u8>)
```
where:  
- "uid" is a unique id of the category;  
- "configuration" is a json structure with the following fields:
```json
{
    "description" : String              // The description of the impact action
    "categories" : Array of Integers    // the categories of this impact action, they must be pre-loaded
    "auditors" : Integer                // Number of auditors approvals required to trigger the rewards
    "blockstart" : Integer              // Activated from this block number
    "blockend": :  Integer               // Deactivated after this block number
    "rewardstoken" : Integer            // 0 = Native BITG, 1.. = unique id of the ERC 20 token as stored by "Assets" Pallet
    "rewardsamount" : Integer           // Amount of the rewards in the selected token for the user (mandatory)
    "rewardoracle" : Integer            // Amount of the rewards in the selected token for the Oracle (can be zero)
    "rewardsauditors" : Integer         // Amount of the rewards in the selected token for the Auditors (can be zero). Shared in equal parties between multiple auditors.
    "slashingauditors" : Integer        // Penalties to auditors for verified errors
    "maxerrorsauditor" : Integer        // Max number of errors before to remove auditor for excessive number reached
                                        // You can force the user to submit some custom fields with the approval request. Here you can define name, type and mandatory.
    "fields" : Array of json            // for example [{"fieldname":"namesurname","fieldtype":"S","mandatory":"Y"},{..}]
                                        // where "fieldname" is the name of the expected field, "fieldtype" can be "S" for string or "N" for number, mandatory can be Y/N.
}
```
The json should be one single line with no space between the fields, for example:
```json
{"description":"Planting a tree","categories":[1],"auditors":1,"blockstart":1,"blockend":1000000,"rewardstoken":0,"rewardsamount":1000,"rewardsoracle":0,"rewardsauditors":50,"slashingsauditors":0,"maxerrorsauditor":10}
```
this one is an example with custom fields:  
```json
{"description":"Planting a tree","categories":[1],"auditors":1,"blockstart":1,"blockend":1000000,"rewardstoken":0,"rewardsamount":1000,"rewardsoracle":0,"rewardsauditors":50,"slashingsauditors":0,"maxerrorsauditor":10,"fields":[{"fieldname":"namesurname","fieldtype":"S","mandatory":"Y"},{"fieldname":"phonenumber","fieldtype":"S","mandatory":"Y"}]}
```

## Destroy Impact Actions  
  
The Super User (or the Technical Commitee) can remove an impact action by  "Sudo" calls, using the function:  
```rust
destroyImpactAction(uid: u32)  
```
  
where:  
- "uid" is the unique id of the impact action.  


## Query Impact Action
You can query the state of the impact action on the blockchain calling the function:  
```rust
impactActionsC(uid: u32) -> Vec<u8>
```
where uid is the unique id of the impact action and you will get the json structure with the configuration, please check the "createImpactAction" for the meaning of tthe field received.  



