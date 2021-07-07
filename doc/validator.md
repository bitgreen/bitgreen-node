# Run a Validator - BitGreen

## Preface
Running a validator on a live network is a lot of responsibility. You will be accountable for not 
only your own stake, but also the stake of your current nominators.  
If you make a mistake and get slashed, your money and your reputation will be at risk.  
However, running a validator can also be very rewarding, knowing that you contribute to the security 
of Bitgreen network while growing your stash.
  
It is highly recommended that you have significant system administration experience before attempting 
to run your own validator. You must be able to handle technical issues and anomalies with your node 
which you must be able to tackle yourself. Being a validator involves more than just executing the 
Bitgreen Node.  
Since security is so important to running a successful validator, you should take a look at the secure 
validator information to make sure you understand the factors to consider when constructing your infrastructure. 

Warning:  
Any BITG that you stake for your validator is liable to be slashed, meaning that an insecure or improper 
setup may result in loss of BITG tokens!   
If you are not confident in your ability to run a validator node, it is recommended to nominate your BITG 
to a trusted validator node instead.  



## Standard Hardware
The most common way for a beginner to run a validator is on a cloud server running Linux.  
You may choose whatever VPS provider that your prefer, and whatever operating system you are comfortable with.  
For this guide we will be using Debian 10, but the instructions should be similar for other platforms.  
  
The transactions weights in Bitgreen were benchmarked on standard hardware. It is recommended that validators 
run at least the standard hardware in order to ensure they are able to process all blocks in time. 
The following are not minimum requirements but if you decide to run with less than this beware that you might 
have performance issue.  
For the full details of the standard hardware please see here:  

- CPU - Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz
- Storage - A NVMe solid state drive. Should be reasonably sized to deal with blockchain growth. 
Starting around 80GB - 160GB will be okay, but will need to be re-evaluated every six months.
- Memory - 64GB.

## Install Rust Compiler

- [Install Rust Language Compiler](doc/rust-setup.md)  
- Update Rust to last version:  
```sh
rustup update
```
- Install Git:  
```sh
apt-get install git
```  

## Install NTP
NTP is a networking protocol designed to synchronize the clocks of computers over a network.   
NTP allows you to synchronize the clocks of all the systems within the network.  
Currently it is required that validators' local clocks stay reasonably in sync, so you should be running NTP or a similar service.  
You can check whether you have the NTP client by running:  
```sh
timedatectl
```
If NTP is installed and running, you should see System clock synchronized: yes (or a similar message). If you do not see it, you can install it by executing:  
```sh
apt-get install ntp
```
ntpd will be started automatically after install. You can query ntpd for status information to verify that everything is working:  
```sh
ntpq -p
```
Warning:  
Skipping this can result in the validator node missing block authorship opportunities.  
If the clock is out of sync (even by a small amount), the blocks the validator produces may not get accepted by the network.  
This will result in ImOnline heartbeats making it on chain, but zero allocated blocks making it on chain.  

## Install the Node:
  
- Clone the node from our public repository:  
```sh
git clone https://github.com/bitgreen/bitg-node
cd bitg-node
```
- Build the node with:  
```sh
cargo build --release
```

## Launch the node in TESTNET
```sh
/target/release/bitg-node --chain assets/chain_spec_testnet_raw.json --port 30333 --name yournodename --validator --rpc-cors all
```



