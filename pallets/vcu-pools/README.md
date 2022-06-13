 ## VCU Pools Pallet

 The VCU Pools pallet lets users create and manage vcu pools. A vcu pool is a collection of vcu tokens of different types represented by a
 common pool token. A user holding any vcu tokens (subject to the VCU pool config) can deposit vcu tokens to the pool and receive equivalent
 pool tokens in return. These pool tokens can be transferred freely and can be retired. When retire function is called, the underlying vcu credits
 are retired starting from the oldest in the pool.

 ### Pool Config
 A pool creator can setup configs, these configs determine which type of tokens are accepted into the pool. Currently the owner can setup two configs for a pool
 1. Registry List : This limits the pool to accept vcu's issued by the given registry's only
 2. Project List : This limits the pool to accepts vcu's issued by specific project's only

 ## Interface

 ### Permissionless Functions

 * `create`: Creates a new pool with given config
 * `deposit`: Deposit some vcu tokens to generate pool tokens
 * `retire`: Burn a specified amount of pool tokens
