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
print("bitg-oracle-impactactsion-assign-auditors v. 1.00- This program assign an auditor to the pending approval requests")

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
query="select lastapprovalrequestprocessed from sync limit 1"
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
for (approvalrequestid,info) in cursor:
    print("approvalrequestid: ",approvalrequestid,"info: ",info)
    # filter those requiring and auditor
    ar=json.loads(info)
    print(ar)
    print("Impact Action id:",ar['impactactionid'])
    # get impact action configuration
    cursoria = cnx.cursor()
    querytx="select id,auditors,category from impactactions where id=%s"
    datatx=(ar['impactactionid'],)
    try:
        cursoria.execute(querytx,datatx)
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
    auditorsn=0
    for (id,auditors,category) in cursoria:
        auditorsn=auditors
        categoryimpactaction=category
    cursoria.close()
    print("Number of Auditors required: ",auditorsn)
    # search for assigned auditors
    cursoraa = cnx.cursor()
    querytx="select count(*) as auditorsnr from impactactionsapprovalrequestsauditors where approvalrequestid=%s"
    datatx=(approvalrequestid,)
    try:
        cursoraa.execute(querytx,datatx)
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
        
    for (auditorsnr) in cursoraa:
        auditorsn=auditorsnr
        break
    cursoraa.close()
    print("Auditors already assigned: ",auditorsn[0])
    # search a possible auditor
    cursora = cnx.cursor()
    querytx="select id,account,description,categories,area from impactactionsauditors order by id desc"
    try:
        cursora.execute(querytx)
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
    # TODO - assign the auditor within the area (a function to calculate the area is required, we need also to force a correct area with 2 gps valid points in the pallet)
    # TODO - assign the auditor with less tasks open (add field to order the search by number of tasks)    
    for (id,account,description,categories,area) in cursora:
        ct=json.loads(categories)
        print("Auditor:",account, description)
        print("Categories:",ct)
        for c in ct:
            if(c==categoryimpactaction):
                #assign the auditor to the approval request
                print("Auditor found:",account,description,categories,area)
                # assign the approval request

        #repeat until the number of auditors is reached - TODO
    #close the cursor
    cursoraa.close()
    # update impactactionsapprovalrequests
    cursorc = cnx.cursor()
    querytx="update sync set lastapprovalrequestprocessed=%s"
    datatx=(approvalrequestid,)
    try:
#       cursora.execute(querytx,datatx)
        print("execute last update")
    except mysql.connector.Error as err:
        print("[Error] ",err.msg)
        continue
    #close the cursor
    cursorc.close()
# close cursor and database
cursor.close()
cnx.close()
print("Regular end....")


