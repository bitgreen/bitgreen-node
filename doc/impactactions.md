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
Categories(uid: u32) -> Vec<u8>
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
- "uid" is a unique id of the Impact Action;  
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
impactActions(uid: u32) -> Vec<u8>
```
where uid is the unique id of the impact action and you will get the json structure with the configuration, please check the function "createImpactAction" for the meaning of the fields received.  

## Create Oracle Configuration  
A blockchain Oracle is a third-party service that provides the blockchain with information from the outside world. 
It is the layer that queries, verifies, and authenticates external data sources, usually via trusted APIs and then relays that information.  
The Super User (or the Technical Commitee) should create an Oracle configuration by "Sudo" calls.   
The function to call is: 
```rust
createOracle(uid: u32, configuration: Vec<u8>)
```
where:  
- "uid" is a unique id of the Oracle;  
- "configuration" is a json structure with the following fields:
```json
{
    "description" : String              // The description of the Oracle
    "account" : String                  // The account of the Oracle to check the signature
    "otherinfo" : String                // an IPFS address for additional info about the Oracle
}
```
The json should be one single line with no space between the fields, for example:
```json
{"description":"Plastic Recycling Verification","account":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","otherinfo":"bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi"}
```

## Destroy Oracle
  
The Super User (or the Technical Commitee) can remove an Oracle by  "Sudo" calls, using the function:  
```rust
destroyOracle(uid: u32)  
```
  
where:  
- "uid" is the unique id of the Oracle to delete.  

## Query Oracle
You can query the state of the Oracle on the blockchain calling the function:  
```rust
oracles(uid: u32) -> Vec<u8>
```
where uid is the unique id of the Oracle,  you will get the json structure with the configuration, please check the "createOracle" function for the meaning of the fields received.  

## Create Auditor Configuration  
An Auditor is an human operator enabled to verify the approval requests and expresses a vote to approval/refuse the request.  
The Super User (or the Technical Commitee) should create an Auditor configuration by "Sudo" calls.   
The function to call is: 
```rust
createAuditor(uid: u32, account: AccountId, configuration: Vec<u8>)
```
where:  
- "uid" is a unique id of the Auditor;  
- "account" is the account of the auditor;
- "configuration" is a json structure with the following fields:
```json
{
    "description" : String              // The description of the Oracle
    "account" : String                  // The account of the Oracle to check the signature
    "categories" : Array                // The categories of competence
    "area" : String                     // Area of actions if delimited by coordindates of a center point and a border point.
    "otherinfo" : String                // an IPFS address for additional info about the Oracle
}
```
The json should be one single line with no space between the fields, for example:
```json
{"description":"John Smith","categories":[1,2],"area":"41.40338,2.17403","otherinfo":"bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi","stakesmin":0}
```

## Destroy Auditor
  
The Super User (or the Technical Commitee) can remove an Oracle by  "Sudo" calls, using the function:  
```rust
destroyAuditor(uid: u32)  
```
  
where:  
- "uid" is the unique id of the Auditor to delete.  

## Query Auditor
You can query the state of the Auditor on the blockchain calling the function:  
```rust
oracles(uid: u32) -> Vec<u8>
```
where uid is the unique id of the Auditor,  you will get the json structure with the configuration, please check the "createAuditor" function for the meaning of the fields received.  


## Create Proxy Account (for assigning auditors)
The auditors should be assigned to the approval request from an off-chain process. The assignment shall be signed from the delegated/proxy account.   
The Super User (or the Technical Commitee) should setup the proxy account in the initial configuration.  
The function to call by "Sudo" is:  
```rust
createProxy(origin, uid: u32, proxy: AccountId)
```
where:  
- "uid" is a unique id of the proxy account, set it to 0;  
- "proxy" is the Account id that will sign the assignment on chain.

## Destroy Proxy Account
  
The Super User (or the Technical Commitee) can remove a Proxy by  "Sudo" calls, using the function:  
```rust
destroyProxy(uid: u32)  
```
where:  
- "uid" is the unique id of the proy account to be deleted.  


## Request Approval Impact Action
Any user can submit a request approval of the impact action.
The function to call is: 
```rust
requestApproval(uid: u32, info: Vec<u8>)
```
where:  
- "uid" is a unique id of the request approval;  
- info" is a json structure with the following fields:
```json
{
    "impactactionid": Number         // the impact action id sas from created Impact Actions
    //.. other fields name and type as defined in Impact Action configuration
    //.. other free fields. Total maximum lenght is 8192 bytes
}
```
The json should be one single line with no space between the fields, for example using a request with free fields:
```json
{"impactactionid":1,"description":"Planted a new tree","coordinates":"25.283294382,55.292989282","ipfsphoto":"bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi"}
```
Using the custom fields defined with:  
```json
{"description":"Planting a tree","categories":[1],"auditors":1,"blockstart":1,"blockend":1000000,"rewardstoken":0,"rewardsamount":1000,"rewardsoracle":0,"rewardsauditors":50,"slashingsauditors":0,"maxerrorsauditor":10,"fields":[{"fieldname":"namesurname","fieldtype":"S","mandatory":"Y"},{"fieldname":"phonenumber","fieldtype":"S","mandatory":"Y"}]}

```
you can submit a request approval like this one:  
```json
{"impactactionid":2,"description":"Planted a new tree","namesurname":"Jhone Smith","phonenumber":"+13489383845"}

```

## Assigning an  Auditor to a Request Approval

Some  "request approval" may requires the assignment of a human auditor to review and approve or refuse the impact action submitted from an  user.  
The function can be submitted  ONLY from a "Proxy Account", (see "createProxy()" above).  
The function to call as extrisinc is:  
```rust
assignAuditor(approvalid: u32, auditor: AccountId, maxdays: u32)
```
where:  
- "approvalid" is the id of the request approval submitted from a regular user;  
- "auditor" is the account of the auditor to assign to the approval request;  
- "maxdays" are the maximum number of days allowed for the auditor to make the review, after such time the approval request will be re-assigned and the ranking of the auditor decreased.  


## Voting a Request Approval (Auditors only)

The function to call as extrisinc is:  
```rust
voteApprovalRequest(approvalid: u32, vote: Vec<u8>)
```
where:  
- "approvalid" is the id of the request approval for an impact action, submitted from any regular user;  
- "vote" is the a json structure with the following fields:  

```json
{
    "vote" : String (Y/N)               // Y= Approve the submitted request, N = refuse the submitted request
    "otherinfo" : String                // an IPFS address for additional info about the auditing process
}
```

for example, to vote Y (approved):  
```json
{"vote":"Y","otherinfo":"bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi"}
```
for example to vote N (refused):  
```json
{"vote":"Y","otherinfo":"bafybeigdyrzt5sfp7udm7hu76uh7y27nf3efuylqabf3oclgtqy55fbzdi"}
```
