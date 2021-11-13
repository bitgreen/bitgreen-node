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
  
key=="kyc" {"manager":"xxxaccountidxxx","supervisor":"xxxxaccountidxxxx","operators":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}
for example:  
{"manager":"5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY","supervisor":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","operators":["5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

key=="bondapproval" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...],"mandatoryunderwriting":"Y/N","mandatorycreditrating":"Y/N","mandatorylegalopinion":"Y/N"}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"],"mandatoryunderwriting":"Y","mandatorycreditrating":"Y","mandatorylegalopinion":"Y"}  

key=="underwriterssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

key=="insurerssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

key=="creditratingagencies" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

key=="lawyerssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

key=="collateralsverification" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

key=="fundapproval" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}  
for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}  

key=="infodocuments" {"documents:[{"document":"xxxxdescription"},{"document":"xxxxdescription"}]}  
for example: {"documents":[{"document":"Profit&Loss Previous year"},{"document":"Board Members/Director List"}]}  
        
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

{"name":"Smith and Wesson Inc","address":"103, Paris Boulevard","city":"London","zip":"00100","state":"England","country":"Great Britain","phone":"+441232322332","website":"https://www.smith.co.uk","ipfsdocs":[{"description":"Balance Sheet 2020","ipfsaddress":"42ff96731ce1f53aa014c55662a3964b61422c2c9c3f38c11b2cf3ee45440c7c"},{"description":"Revenue Report 2021","ipfsaddress":"b26707691ce34a738fa5dab526e800be831bcc63a199a7d83414f5d6b0a8836c"}]}


## Approve KYC
The KYC must be approved from a supervisors and the manager. The function for approval is:  
```rust
kycApprove(account:AccoundId)
```
where the account is the account id to be approved.
The KYC is finalized once both a supervisor and manager has approved.



TODO:
Deny approval for KYC




