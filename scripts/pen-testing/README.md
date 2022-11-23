# BITGREEN Node - Penetration Testing Tool

This is a penetration tool against a BitGreen Node.

## Hacking Attempts:

- Transfer funds from an empty wallet.
- Transfer funds to the same wallet.

## Installation  
  
Install nodejs, please refer to: nodejs documentation.    
Install yarn, please refer to yarn documentation and installation guides.  
Install Polkadot Javascript library:  
```sh
yarn add @polkadot/api  
```  
  
## Run  

Execute:    
```sh
node bitgreen-pentesting.js xxxxxxxxxxx
```
where xxxxxxxxxxx is the domain name of your node. The tool will connect over secure web socket on standard port 9944.  

 
