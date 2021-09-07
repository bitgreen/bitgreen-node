# This App compute the rewards for the user and mint the ERC20 accordingly.
# import libraries
# system packages
import sys
import os
import json
# Substrate module
from substrateinterface import SubstrateInterface, Keypair,ExtrinsicReceipt
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
# read environment variables
try:
    DB_NAME=os.environ['DB_NAME']
    DB_USER=os.environ['DB_USER']
    DB_PWD=os.environ['DB_PWD']
    DB_HOST=os.environ['DB_HOST']
    NODE=os.environ['NODE']
    SEED=os.environ['SEED']

except NameError:
    print("System Variables have not been set")
    exit(1)

# function to load data types registry
def load_type_registry_file(file_path: str) -> dict:
    with open(os.path.abspath(file_path), 'r') as fp:
        data = fp.read()
    return json.loads(data)

## MAIN 
print("bitg-oracle-impactactions-rewards-auditors v. 1.00- This program assign rewards in ERC20 to users")

# load custom data types
custom_type_registry = load_type_registry_file("../assets/types.json")
# define connection parameters
substrate = SubstrateInterface(
    url=NODE,
    ss58_format=42,
    type_registry_preset='default',
    type_registry=custom_type_registry
)
# generate keys pair
keyspair = Keypair.create_from_mnemonic(SEED)
# open database
cnx = mysql.connector.connect(user=DB_USER, password=DB_PWD,host=DB_HOST,database=DB_NAME)
# search for the votes not yet rewarded
querytx="select id as idvote,signer,approvalrequestid from impactactionsapprovalrequestauditorvotes where dtrewards IS NULL"
cursor = cnx.cursor()
cursor.execute(querytx)
lar=(0,)   
#process the approval request to check if they are finally approved or refused
for (idvote,account,approvalrequestid) in cursor:
    cursorr = cnx.cursor()
    querytx="select id as idapproval,signer,info from impactactionsapprovalrequests where id=%s"
    datatx=(approvalrequestid,)
    try:
        cursorr.execute(querytx,datatx)
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
    for (idapproval,signer,info) in cursorr:
        infoj=json.loads(info)
        infoj=json.loads(info)
        impactactionid=infoj['impactactionid']
        # get number of auditors required
        cursori = cnx.cursor()
        querytx="select auditors,rewardstoken,rewardauditors from impactactions where id=%s"
        datatx=(impactactionid,)
        try:
            cursori.execute(querytx,datatx)
        except mysql.connector.Error as err:
            print("[Error] ",err.msg)
            continue
        auditorsv=0
        rewardsamountv=0
        for (auditors,rewardstoken,rewardauditors) in cursor:
            auditorsv=auditors
            rewardsamountv=rewardauditors
            rewardstokenv=rewardstoken
        
        #close cursor
        cursori.close()
        # mint the rewardsm for auditors shared between all the auditors required.
        call = substrate.compose_call(
            call_module='Assets',
            call_function='mint',
            call_params={
                'id': rewardstoken,
                'beneficiary': account,
                'amount': rewardsamountv/auditorsv
            }
        )
        extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keyspair)
        try:
            receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
            print("Rewards assigned:",account,rewardstoken,rewardsamountv)
            print("Extrinsic '{}' sent and included in block '{}'".format(receipt.extrinsic_hash, receipt.block_hash))
        except SubstrateRequestException as e:
            print("Failed to send: {}".format(e))
            exit(1)
        
        # set dtrewards
        cursorv = cnx.cursor()
        querytx="update impactactionsapprovalrequestauditorvotes set dtrewards=now() where id=%s"
        datatx=(idvote,)
        try:
            cursorv.execute(querytx,datatx)
            cursorv.commit()
        except mysql.connector.Error as err:
            print("[Error] ",err.msg)
            cursorv.close()
            continue    
        cursorv.close()
# close cursor and database
cursor.close()
cnx.close()
print("Regular end....")