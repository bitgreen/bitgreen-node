#/bin/bash
# This is as script to initialize some variable and launch the Oracle Server
# Change the variables below following your configuration:
export APIURL=https://api.your_oracle_domain_name.xxx/redeem?code=%CODE%
export TLSPORT=7443
export HTTPPORT=3003
## change with your secret seed that has the right on the ERC20 used
export SECRETSEED="join upgrade peasant must quantum verify beyond bullet velvet machine false replace"  
# Set the ERC20 token id as from "Assets" pallet
export TOKENID=1
# set the url for Blockchain Node:
export NODE="ws://localhost:9944"
# Set this variables to your certificate and key in pem format.
#export SSL_KEY=/etc/letsencrypt/live/testnode.bitg.org/privkey.pem
#export SSL_CERT=/etc/letsencrypt/live/testnode.bitg.org/fullchain.pem
# Launching the Oracle API Server. Nodejs should be in the path
node bitg-oracle-impact-actions-server.js



