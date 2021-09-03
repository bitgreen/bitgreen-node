#/bin/bash
# This is as script to initialize some variables and launch the crawler.
# Change the variables below following your configuration, these are only examples:
export DB_NAME=bitgreen
export DB_USER=bitgreen
export DB_HOST=127.0.0.1
export DB_PWD=aszxqw1234
export NODE=ws://127.0.0.1:9944
# set you SEED this an example only
export SEED='episode together nose spoon dose oil faculty zoo ankle evoke admit walnut'
#export NODE=wss://testnode.bitg.org
# launching the crawler, python3 should be in the path
python3 bitg-oracle-impactactions-assign-auditors.py $1


