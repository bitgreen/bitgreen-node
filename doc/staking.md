# Staking Module  

The Staking module is used to manage funds at stake by network maintainers, the validators.  

## Overview  

The Staking module is the means by which a set of network maintainers (known validators) are chosen based upon those who voluntarily place funds under deposit.   
Under deposit, those funds are rewarded under normal operation but are held at pain of slash (expropriation) should the staked maintainer be found not to be discharging its duties properly.  

## Terminology  

- Staking: The process of locking up funds for some time, placing them at risk of slashing (loss) in order to become a rewarded maintainer of the network.  
- Validating: The process of running a node to actively maintain the network, either by producing blocks or guaranteeing finality of the chain.  
- Nominating: The process of placing staked funds behind one or more validators in order to share in any reward, and punishment, they take. 
- Stash account: The account holding an owner's funds used for staking.  
- Controller account: The account that controls an owner's funds for staking.  
- Era: A (whole) number of sessions, which is the period that the validator set (and each validator's active nominator set) is recalculated and where rewards are paid out.   
- Slash: The punishment of a staker by reducing its funds.  

## Goals  

The staking system in BitGreen Proof of Stake is designed to make the following possible:

- Stake funds that are controlled by a cold wallet.  
- Withdraw some, or deposit more, funds without interrupting the role of an entity.  
- Switch between roles (nominator, validator, idle) with minimal overhead.  

## Scenarios  

### Staking  

Almost any interaction with the Staking module requires a process of bonding (also known as being a staker).   
To become bonded, a fund-holding account known as the stash account, which holds some or all of the funds that become frozen in place as part of the staking process, is paired with an active controller account, which issues instructions on how they shall be used.  
  
An account pair can become bonded using the bond call.  
  
Stash accounts can change their associated controller using the set_controller call.  

There are three possible roles that any staked account pair can be in: Validator, Nominator and Idle (defined in StakerStatus). There are three corresponding instructions to change between roles, namely: validate, nominate, and chill.  

### Validating

A validator takes the role of either validating blocks or ensuring their finality, maintaining the veracity of the network.  
A validator should avoid both any sort of malicious misbehavior and going offline. Bonded accounts that state interest in being a validator do NOT get immediately chosen as a validator. Instead, they are declared as a candidate and they might get elected at the next era as a validator. The result of the election is determined by nominators and their votes.  
  
An account can become a validator candidate via the validate call.  
  
### Nomination  

A nominator does not take any direct role in maintaining the network, instead, it votes on a set of validators to be elected. Once interest in nomination is stated by an account, it takes effect at the next election round. The funds in the nominator's stash account indicate the weight of its vote.  
Both the rewards and any punishment that a validator earns are shared between the validator and its nominators. This rule incentivizes the nominators to NOT vote for the misbehaving/offline validators as much as possible, simply because the nominators will also lose funds if they vote poorly.  
  
An account can become a nominator via the nominate call.  
  
### Rewards and Slash  

The reward and slashing procedure is the core of the Staking module, attempting to embrace valid behavior while punishing any misbehavior or lack of availability.  

Rewards must be claimed for each era before it gets too old by $HISTORY_DEPTH using the payout_stakers call. 
Any account can call payout_stakers, which pays the reward to the validator as well as its nominators. Only the [Config::MaxNominatorRewardedPerValidator] biggest stakers can claim their reward.  
This is to limit the i/o cost to mutate storage for each nominator's account.  
  
Slashing can occur at any point in time, once misbehavior is reported. Once slashing is determined, a value is deducted from the balance of the validator and all the nominators who voted for this validator (values are deducted from the stash account of the slashed entity).  
  
Slashing logic is further described in the documentation of the slashing module.  
  
Similar to slashing, rewards are also shared among a validator and its associated nominators. Yet, the reward funds are not always transferred to the stash account and can be configured. See Reward Calculation for more details.  
  
### Chilling  

Finally, any of the roles above can choose to step back temporarily and just chill for a while. This means that if they are a nominator, they will not be considered as voters anymore and if they are validators, they will no longer be a candidate for the next election.  
  
An account can step back via the chill call.  
  
### Session managing  
  
The module implement the trait SessionManager. Which is the only API to query new validator set and allowing these validator set to be rewarded once their era is ended.  
  
## Interface  

### Dispatchable Functions  

The dispatchable functions of the Staking module enable the steps needed for entities to accept and change their role, alongside some helper functions to get/set the metadata of the module.

### Public Functions  

The Staking module contains many public storage items and (im)mutable functions.  
Further details can be [found here.](https://docs.rs/pallet-staking/3.0.0/pallet_staking/)  