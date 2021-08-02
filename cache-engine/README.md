# Cache Engine for BitGreen BlockChain
The purpose of this module is to store the transactions  of the blockchain in mysql database and offer an API interface for the applications client.
The solution is divided in 2 modules: 
- A crawler that  store the transaction of the blockchain in a database
- A Web Api server to query the data stored in the database

## CRAWLER
The crawler has the mission to wait for new blocks and store the transactions in the database.
Written in Python 3, the library used is:   [https://github.com/polkascan/py-substrate-interface](https://github.com/polkascan/py-substrate-interface)

### Requirements:

You should have installed:  
- [Python 3.x](https://www.python.org)  
- [Python Package Index (pip)](https://pypi.org)  
- [Mariadb Server](https://mariadb.org)  

## Installation  
This instructions refers to an installation for LINUX operating system.  
Execute from command line:
```sh
pip3 install substrate-interface
pip3 install mysql-connector-python
```

## Create Database and grant access
Launch the mysql cli:  
```sh
mysql
```
and copy/paste:   
```
create database bitgreen;  
CREATE USER 'bitgreen'@'localhost' IDENTIFIED BY 'aszxqw1234';      // for example only, change with your password
GRANT ALL ON bitgreen.* TO bitgreen@'localhost';  
flush privileges;  
```
Customise the file:  
```sh
bitg-blockchain-crawler.sh
```
to reflect your database configuration.  

## Run

The Bitgreen node run locally.  
Execute from the command line:  
```sh
./bit-blockchain-crawler.sh
```


## API Server



