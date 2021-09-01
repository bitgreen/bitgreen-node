# The App scan for pending approval requests and assign an auditor based on the expertise
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

except NameError:
    print("System Variables have not been set")
    exit(1)

# function to load data types registry
def load_type_registry_file(file_path: str) -> dict:
    with open(os.path.abspath(file_path), 'r') as fp:
        data = fp.read()
    return json.loads(data)

## MAIN 
print("bitf-oracle-impactactsion-assign-auditors v. 1.00- This program assign an auditor to the pending approval requests")

# load custom data types
custom_type_registry = load_type_registry_file("../assets/types.json")
# define connection parameters
substrate = SubstrateInterface(
    url=NODE,
    ss58_format=42,
    type_registry_preset='default',
    type_registry=custom_type_registry
)
# open database
cnx = mysql.connector.connect(user=DB_USER, password=DB_PWD,host=DB_HOST,database=DB_NAME)
# search for the last proceessed approval request
query="select impactactionsapprovalrequests from sync limit 1"
cursor = cnx.cursor()
cursor.execute(query)
lar=(0,)   
for (lastapprovalrequestprocessed) in cursor:
    lar=lastapprovalrequestprocessed
cursor.close()
# search for pending approval requests and check if need to assign an auditor
cursor = cnx.cursor()
querytx="select id,info from impactactionsapprovalrequests where id>%s"
try:
    cursor.execute(querytx,lar)
except mysql.connector.Error as err:
    print("[Error] ",err.msg)
for (id,info) in cursor:
    print("id: ",id,"info: ",info)
    # TODO - filter those requiring and auditor
    # search a possible auditor
    # select the one with less tasks
    # assign the approval request
    # update impactactionsapprovalrequests

# close database
cursor.close()
cnx.close()
print("Regular end....")


