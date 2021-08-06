# ORACLES


## Oracle for Impact Actions


## Requirements:
- [Nodejs >=14.x](https://nodejs.dev)

## Installation
You should install the required libraries using npn (part of nodejs package):  
```sh
npm install express
```
Customize the starting script:
customise the script: bitg-oracle-impact-action.sh settings the variables to access as from your configuration and execute:  
```sh
bitg-oracle-impact-actions.sh
```
Setting the environment variable accordingly your configuration.  

To enable HTTPS you should install the private key and certificate from a well recognised Certification Authority.  
In the example we used: [https://certbot.eff.org](https://certbot.eff.org).  
And you should set the accordingly environment variables in "bitg-cache-server.sh" to point to the correct file name.  