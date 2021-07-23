# Impact Actions

The mission of Bitgreen is to encourage a sustainable life safeguarding the environment.  
With this module, we create a tool to register and rewardas positive action with a real impact.  

"Impact Actions" allows to:

- Configure the Impact Actions and their rewards;
- Configure Oracles to validate the impact actions;
- Configure Auditors to validate manually the impact actions;
- Submit the Impact Actions for approval;
- Approve the Impact Actions, triggering the rewards.


The pallet is called "impactActions".

## Create Category  
The Super User (or the Technical Commitee) should create the categories for the  "Impact Actions", using "Sudo" calls.
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

