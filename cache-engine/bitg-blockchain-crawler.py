# The App listening to new blocks written read the exstrincs and store the transactions in a mysql/mariadb database.
# the database must be created, the app will create the tables and indexes used.
# import libraries
# system packages
import sys
import os
import json
# Substrate module
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException
# base64 encoder/decoder
import base64
# base58 encoder/decoder
import base58
#import scale library to load data types
import scalecodec
# import mysql connector
import mysql.connector
currentime=""

# initialize default vars
try:
    DB_NAME=os.environ['DB_NAME']
    DB_USER=os.environ['DB_USER']
    DB_PWD=os.environ['DB_PWD']
    DB_HOST=os.environ['DB_HOST']
except NameError:
    print("System Variables have not been set")
    exit(1)


# function to load data types registry
def load_type_registry_file(file_path: str) -> dict:
    with open(os.path.abspath(file_path), 'r') as fp:
        data = fp.read()
    return json.loads(data)
# function to create tables required
def create_tables():
    cnx = mysql.connector.connect(user=DB_USER, password=DB_PWD,host=DB_HOST,database=DB_NAME)
    cursor = cnx.cursor()
    
    # use database
    try:
        cursor.execute("USE {}".format(DB_NAME))
    except mysql.connector.Error as err:
        print("Database {} does not exists.".format(DB_NAME))
        print(err)
        exit(1)
    # create tables
    createtx="CREATE TABLE `transactions` (`id` MEDIUMINT NOT NULL AUTO_INCREMENT,`blocknumber` INT(11) NOT NULL,`txhash` VARCHAR(66) NOT NULL,  `sender` VARCHAR(64) NOT NULL,  `recipient` VARCHAR(64) NOT NULL,  `amount` numeric(32,0) NOT NULL,  `dtblockchain` DATETIME NOT NULL, CONSTRAINT txhash_unique UNIQUE (txhash),PRIMARY KEY (id))"
    try:
        print("Creating table TRANSACTIONS...")
        cursor.execute(createtx)
    except mysql.connector.Error as err:
            if(err.msg!="Table 'transactions' already exists"):
                print(err.msg)
    else:
        print("OK")
    # create indexes
    createidxtx="CREATE INDEX txhash on transactions(txhash)"
    try:
        print("Creating index TXHASH on TRANSACTIONS...")
        cursor.execute(createidxtx)
    except mysql.connector.Error as err:
            if(err.msg!="Duplicate key name 'txhash'"):
                print(err.msg)
    else:
        print("OK")
    createidxtx="CREATE INDEX sender on transactions(sender)"
    try:
        print("Creating index SENDER on TRANSACTIONS...")
        cursor.execute(createidxtx)
    except mysql.connector.Error as err:
            if(err.msg!="Duplicate key name 'sender'"):
                print(err.msg)
    else:
        print("OK")
    createidxtx="CREATE INDEX recipient on transactions(recipient)"
    try:
        print("Creating index RECIPIENT on TRANSACTIONS...")
        cursor.execute(createidxtx)
    except mysql.connector.Error as err:
        if(err.msg!="Duplicate key name 'recipient'"):
            print(err.msg)
    else:
        print("OK")
    # creating sync table to keep  syncronisation info
    createsync="CREATE TABLE `sync` (`id` MEDIUMINT NOT NULL AUTO_INCREMENT,`lastblocknumberverified` INT(11) NOT NULL, PRIMARY KEY (id))"
    try:
        print("Creating table SYNC...")
        cursor.execute(createsync)
    except mysql.connector.Error as err:
            if(err.msg!="Table 'sync' already exists"):
                print(err.msg)
    else:
        print("OK")
    
    cursor.close()
    cnx.close()
# function to syncronise the blockchain reading the old blocks if not yet loaded
def sync_blockchain(substrate):
    # we get the the last block from the blockchain
    r=substrate.rpc_request(method='chain_getHeader',params=[],result_handler=None)
    rs=r.get('result')
    lastblockhex=rs.get('number')
    lastblocknumber=int(lastblockhex,16)
    print("[Info] Last Block: ",lastblocknumber)
    # we check the last block reconcilied
    cnx = mysql.connector.connect(user=DB_USER, password=DB_PWD,host=DB_HOST,database=DB_NAME)
    cursor = cnx.cursor(dictionary=True)
    lastblocknumberverified=0
    query="select * from sync where id=1"
    try:
        cursor.execute(query)
        for row in cursor:
            lastblocknumberverified=row['lastblocknumberverified']
        #lastblocknumberverified=row.get('lastblocknumberverified')
    except mysql.connector.Error as err:
        print(err.msg)
        lastblocknumberverified=0
    
    print("[INFO] Last block number verified:",lastblocknumberverified)
    # loop the new block number to find gaps and fill them in case
    x=lastblocknumberverified+1
    cursorb = cnx.cursor()
    while x<=lastblocknumber:
        # get block data
        print("Processing block # ",x)
        result = substrate.get_block(block_number=x)
        for extrinsic in result['extrinsics']:
            if extrinsic.address:
                signed_by_address = extrinsic.address.value
            else:
                signed_by_address = None
            print('\nPallet: {}\nCall: {}\nSigned by: {}'.format(
                extrinsic.call_module.name,
                extrinsic.call.name,
                signed_by_address
            ))
            if extrinsic.call_module.name=="Timestamp" and extrinsic.call.name=="set":
                currentime=extrinsic.params[0]['value']
            if extrinsic.call_module.name=="Balances" and ( extrinsic.call.name=="transfer" or extrinsic.call.name=="transfer_keep_alive"):
                ## store the transaction in the database
                store_transaction(x,'0x'+extrinsic.extrinsic_hash,extrinsic.address.value,extrinsic.params[0]['value'],extrinsic.params[1]['value'],currentime)
        # update sync
        sqlst=""
        if(lastblocknumberverified==0):
            sqlst="insert into sync set lastblocknumberverified="+str(x)
        else:
            sqlst="update sync set lastblocknumberverified="+str(x)+" where id=1"
        try:
            print("sqlst: ",sqlst)
            r=cursorb.execute(sqlst)
            print(r)
        except mysql.connector.Error as err:
            print(err.msg)
            
        lastblocknumberverified=x
        # increase block number
        x=x+1
    #end while loop
    cursor.close()
    cursorb.close()
    cnx.close()



# function to store a new transaction
def store_transaction(blocknumber,txhash,sender,recipient,amount,currenttime):
    cnx = mysql.connector.connect(user=DB_USER, password=DB_PWD,host=DB_HOST,database=DB_NAME)
    print("Storing New Transaction")
    print("TxHash: ",currentime)
    print("Current time: ",currentime)
    print("Sender: ",sender)
    print("Recipient: ",recipient)
    print("Amount: ",amount)
    cursor = cnx.cursor()
    dtblockchain=currenttime.replace("T"," ")
    dtblockchain=dtblockchain[0:19]
    addtx="insert into transactions set blocknumber=%s,txhash=%s,sender=%s,recipient=%s,amount=%s,dtblockchain=%s"
    datatx=(blocknumber,txhash,sender,recipient,amount,dtblockchain)
    try:
        cursor.execute(addtx,datatx)
    except mysql.connector.Error as err:
                print(err.msg)
    cnx.commit()
    cursor.close()
    cnx.close()
    
# subscription handler for new blocks written
def subscription_handler(obj, update_nr, subscription_id):

    print(f"New block #{obj['header']['number']} produced by {obj['author']} hash: {obj['header']['hash']}")
    # Retrieve extrinsics in block
    result = substrate.get_block(block_number=obj['header']['number'])
    for extrinsic in result['extrinsics']:
        if extrinsic.address:
            signed_by_address = extrinsic.address.value
        else:
            signed_by_address = None
        print(extrinsic)

        print('\nPallet: {}\nCall: {}\nSigned by: {}'.format(
            extrinsic.call_module.name,
            extrinsic.call.name,
            signed_by_address
        ))
        if extrinsic.call_module.name=="Timestamp" and extrinsic.call.name=="set":
            currentime=extrinsic.params[0]['value']
        if extrinsic.call_module.name=="Balances" and ( extrinsic.call.name=="transfer" or extrinsic.call.name=="transfer_keep_alive"):
            ## store the transaction in the database
            store_transaction(obj['header']['number'],'0x'+extrinsic.extrinsic_hash,extrinsic.address.value,extrinsic.params[0]['value'],extrinsic.params[1]['value'],currentime)
        # Loop through call params
        for param in extrinsic.params:

            if param['type'] == 'Compact<Balance>':
                param['value'] = '{} {}'.format(param['value'] / 10 ** substrate.token_decimals, substrate.token_symbol)

            print("Param '{}': {}".format(param['name'], param['value']))

## MAIN 

# load custom data types
custom_type_registry = load_type_registry_file("../assets/types.json")

# define connection parameters
substrate = SubstrateInterface(
    #url="wss://testnode.bitg.org",
    url="ws://127.0.0.1:9944",
    ss58_format=42,
    type_registry_preset='default',
    type_registry=custom_type_registry

)
# create database tables
create_tables()
# syncronise the blockchain
sync_blockchain(substrate)
exit(0)
# subscribe to new block writing
result = substrate.subscribe_block_headers(subscription_handler, include_author=True)
print(result)

