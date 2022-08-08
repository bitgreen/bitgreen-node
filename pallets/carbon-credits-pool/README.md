 ## Carbon Credits Pool Pallet

 The Carbon Credits Pool pallet lets users create and manage Carbon Credits pools. A Carbon Credits pool is a collection of Carbon Credits tokens of different types represented by a
 common pool token. A user holding any Carbon Credits tokens (subject to the Carbon Credits pool config) can deposit Carbon Credits tokens to the pool and receive equivalent
 pool tokens in return. These pool tokens can be transferred freely and can be retired. When retire function is called, the underlying Carbon Credits credits
 are retired starting from the oldest in the pool.

 ### Pool Config
 A pool creator can setup configs, these configs determine which type of tokens are accepted into the pool. Currently the owner can setup two configs for a pool
 1. Registry List : This limits the pool to accept Carbon Credits's issued by the given registry's only
 2. Project List : This limits the pool to accepts Carbon Credits's issued by specific project's only

 ## Interface

 ### Permissionless Functions

 * `create`: Creates a new pool with given config
 * `deposit`: Deposit some Carbon Credits tokens to generate pool tokens
 * `retire`: Burn a specified amount of pool tokens
