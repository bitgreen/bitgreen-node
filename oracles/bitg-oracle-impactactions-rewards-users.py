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
print("bitg-oracle-impactactions-rewards-users v. 1.00- This program assign rewards in ERC20 to users")

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
# search for the last proceessed approval request
querytx="select id,signer,info from impactactionsapprovalrequests where dtapproved IS NULL and dtrefused IS NULL"
cursor = cnx.cursor()
cursor.execute(querytx)
lar=(0,)   
#process the approval request to check if they are finally approved or refused
for (approvalrequestid,account,info) in cursor:
    infoj=json.loads(info)
    impactactionid=infoj['impactactionid']
    # get number of auditors required
    cursori = cnx.cursor()
    querytx="select auditors,rewardstoken,rewardsamount from impactactions where id=%s"
    datatx=(impactactionid,)
    try:
        cursori.execute(querytx,datatx)
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
    auditorsv=0
    rewardsamountv=0
    for (auditors,rewardstoken,rewardsamount) in cursor:
        auditorsv=auditors
        rewardsamountv=rewardsamount
        rewardstokenv=rewardstoken
    # check if auditors are required and the rewards if set
    if (auditorsv==0 or rewardsamountv==0):
        continue
    #close cursor
    cursori.close()
    # check for vote from auditors
    yesv=0
    nov=0
    # yes votes
    cursorv = cnx.cursor()
    querytx="select count(*) as yesvotes from impactactionsapprovalrequestauditorvotes where approvalrequestid=%s and vote='Y'"
    datatx=(approvalrequestid,)
    try:
        cursorv.execute(querytx,datatx)
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
    for (yesvotes) in cursor:
        yesv=yesvotes
    cursorv.close()
    # no votes
    cursorv = cnx.cursor()
    querytx="select count(*) as novotes from impactactionsapprovalrequestauditorvotes where approvalrequestid=%s and vote='N'"
    datatx=(approvalrequestid,)
    try:
        cursorv.execute(querytx,datatx)
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
    for (novotes) in cursor:
        nov=novotes
    cursorv.close()
    # check if all the auditors have voted
    if (nov+yes<auditorsv):
        continue
    # mint the rewards
    call = substrate.compose_call(
        call_module='Assets',
        call_function='mint',
        call_params={
            'id': rewardstoken,
            'beneficiary': account,
            'amount': rewardsamountv
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
    
    # set dtapproved if # yes > # no
    if (yesv>nov):
        cursorv = cnx.cursor()
        querytx="update impactactionsapprovalrequests set dtapproved=now() where id=%s"
        datatx=(approvalrequestid,)
        try:
            cursorv.execute(querytx,datatx)
            cursorv.commit()
        except mysql.connector.Error as err:
            print("[Error] ",err.msg)
            cursorv.close()
            continue    
        cursorv.close()
    # set dtapproved if # yes < # no
    if (yesv<nov):
        cursorv = cnx.cursor()
        querytx="update impactactionsapprovalrequests set dtrefused=now() where id=%s"
        datatx=(approvalrequestid,)
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