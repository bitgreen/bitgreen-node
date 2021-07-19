
// Bitg server for claiming balance from the previous blockchain.
// The exstrinc submission is executed from a "proxy account" using the data signed from 
// the owner of the funds; to do not require a positive balances for the recipient account.
//*********************************************************************************************

// pulling required libraries
let express = require('express');
const https = require("https");
let fs = require('fs');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { decodeAddress, encodeAddress } = require('@polkadot/keyring');
const { hexToU8a, isHex } = require('@polkadot/util');


//***************************************************************************************************
// customization section - you can change the following constants upon your preferences
// this is the secret seed of the "proxy" account that should have balances for enough gas fees
// the gas fees are not charged so a minimum of 1 BITG is enough to keep in this account to mitigate
// any risk
const SECRETSEED="shaft crack below own paper myself crouch van deer excite raw people"; // for example:
//Secret phrase `shaft crack below own paper myself crouch van deer excite raw people` is account:
//Secret seed:       0xd33af02cfca629d941ffad26fd44df1bdbefed0d5ee27dacdaf2abe40a6866e3
//Public key (hex):  0xdab35bd076ca8be8d0068e8cf3060a54a46c6607aac35e15defe55b649832075
//Public key (SS58): 5H1TaQBgtVwPA3Bk7r9feEcKTFgLAHKHPu5ghZaMU8iqbdXu
//Account ID:        0xdab35bd076ca8be8d0068e8cf3060a54a46c6607aac35e15defe55b649832075
//SS58 Address:      5H1TaQBgtVwPA3Bk7r9feEcKTFgLAHKHPu5ghZaMU8iqbdXu
// end customizaton section
//***************************************************************************************************
console.log("[Info] - BITG Claiming Server - ver. 1.00 - Starting");
// execute main loop as async function because of "await" requirements that cannot be execute from the main body
mainloop();
async function mainloop(){
    //setup express (http server)
    let app = express(); 
    app.use(express.urlencoded({ extended: true })); // for parsing application/x-www-form-urlencoded
    //main form in  index.html
    app.get('/',function(req,res){             
        let v=read_file("index.html");
        res.send(v);
    });
    //mint
    app.post('/claim',async function(req, res) {
        console.log(req.body);
        oldaddress=req.body.oldaddress;
        oldpublickey=req.body.oldpublickey;
        signature=req.body.signature;
        recipient=req.body.recipient;
        let va= await isValidAccountAddress(recipient);
        if (va===false){
            w='{"answer":"KO","message":"Recipient account is not valid"}'
            res.send(w);
            return;
        }
        console.log("[Info] Claiming the balance for ",recipient);
        
        // generate key pair from Seed
        const keyring = new Keyring({ type: 'sr25519', ss58Format: 42 });
        const keyspair = keyring.createFromUri(SECRETSEED, { name: 'sr25519' });
        console.log(keyspair.address);
        //const wsProvider = new WsProvider('wss://testnode.bitg.org');  
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');  
        const api = await ApiPromise.create({ provider: wsProvider,"types" :{
        
                "CallOf": "Call",
                "DispatchTime": {
                    "_enum": {
                        "At": "BlockNumber",
                        "After": "BlockNumber"
                    }
                },
                "ScheduleTaskIndex": "u32",
                "DelayedOrigin": {
                    "delay": "BlockNumber",
                    "origin": "PalletsOrigin"
                },
                "StorageValue": "Vec<u8>",
                "GraduallyUpdate": {
                    "key": "StorageKey",
                    "targetValue": "StorageValue",
                    "perBlock": "StorageValue"
                },
                "StorageKeyBytes": "Vec<u8>",
                "StorageValueBytes": "Vec<u8>",
                "RpcDataProviderId": "Text",
                "OrderedSet": "Vec<AccountId>",
                "OrmlAccountData": {
                    "free": "Balance",
                    "frozen": "Balance",
                    "reserved": "Balance"
                },
                "OrmlBalanceLock": {
                    "amount": "Balance",
                    "id": "LockIdentifier"
                },
                "DelayedDispatchTime": {
                    "_enum": {
                        "At": "BlockNumber",
                        "After": "BlockNumber"
                    }
                },
                "DispatchId": "u32",
                "Price": "FixedU128",
                "OrmlVestingSchedule": {
                    "start": "BlockNumber",
                    "period": "BlockNumber",
                    "periodCount": "u32",
                    "perPeriod": "Compact<Balance>"
                },
                "VestingScheduleOf": "OrmlVestingSchedule",
                "PalletBalanceOf": "Balance",
                "ChangeBalance": {
                    "_enum": {
                        "NoChange": "Null",
                        "NewValue": "Balance"
                    }
                },
                "BalanceWrapper": {
                    "amount": "Balance"
                },
                "BalanceRequest": {
                    "amount": "Balance"
                },
                "EvmAccountInfo": {
                    "nonce": "Index",
                    "contractInfo": "Option<EvmContractInfo>",
                    "developerDeposit": "Option<Balance>"
                },
                "CodeInfo": {
                    "codeSize": "u32",
                    "refCount": "u32"
                },
                "EvmContractInfo": {
                    "codeHash": "H256",
                    "maintainer": "H160",
                    "deployed": "bool"
                },
                "EvmAddress": "H160",
                "CallRequest": {
                    "from": "Option<H160>",
                    "to": "Option<H160>",
                    "gasLimit": "Option<u32>",
                    "storageLimit": "Option<u32>",
                    "value": "Option<U128>",
                    "data": "Option<Bytes>"
                },
                "CID": "Vec<u8>",
                "ClassId": "u32",
                "ClassIdOf": "ClassId",
                "TokenId": "u64",
                "TokenIdOf": "TokenId",
                "TokenInfoOf": {
                    "metadata": "CID",
                    "owner": "AccountId",
                    "data": "TokenData"
                },
                "TokenData": {
                    "deposit": "Balance"
                },
                "Properties": {
                    "_set": {
                        "_bitLength": 8,
                        "Transferable": 1,
                        "Burnable": 2
                    }
                },
                "BondingLedger": {
                    "total": "Compact<Balance>",
                    "active": "Compact<Balance>",
                    "unlocking": "Vec<UnlockChunk>"
                },
                "Amount": "i128",
                "AmountOf": "Amount",
                "AuctionId": "u32",
                "AuctionIdOf": "AuctionId",
                "TokenSymbol": {
                    "_enum": {
                        "BITG": 0,
                        "USDG": 1
                    }
                },
                "CurrencyId": {
                    "_enum": {
                        "Token": "TokenSymbol",
                        "DEXShare": "(TokenSymbol, TokenSymbol)",
                        "ERC20": "EvmAddress"
                    }
                },
                "CurrencyIdOf": "CurrencyId",
                "AuthoritysOriginId": {
                    "_enum": [
                        "Root"
                    ]
                },
                "TradingPair": "(CurrencyId,  CurrencyId)",
                "AsOriginId": "AuthoritysOriginId",
                "SubAccountStatus": {
                    "bonded": "Balance",
                    "available": "Balance",
                    "unbonding": "Vec<(EraIndex,Balance)>",
                    "mockRewardRate": "Rate"
                },
                "Params": {
                    "targetMaxFreeUnbondedRatio": "Ratio",
                    "targetMinFreeUnbondedRatio": "Ratio",
                    "targetUnbondingToFreeRatio": "Ratio",
                    "unbondingToFreeAdjustment": "Ratio",
                    "baseFeeRate": "Rate"
                },
                "Ledger": {
                    "bonded": "Balance",
                    "unbondingToFree": "Balance",
                    "freePool": "Balance",
                    "toUnbondNextEra": "(Balance, Balance)"
                },
                "ChangeRate": {
                    "_enum": {
                        "NoChange": "Null",
                        "NewValue": "Rate"
                    }
                },
                "ChangeRatio": {
                    "_enum": {
                        "NoChange": "Null",
                        "NewValue": "Ratio"
                    }
                },
                "BalanceInfo": {
                    "amount": "Balance"
                },
                "Rate": "FixedU128",
                "Ratio": "FixedU128",
                "PublicKey": "[u8; 20]",
                "DestAddress": "Vec<u8>",
                "Keys": "SessionKeys2",
                "PalletsOrigin": {
                    "_enum": {
                        "System": "SystemOrigin",
                        "Timestamp": "Null",
                        "RandomnessCollectiveFlip": "Null",
                        "Balances": "Null",
                        "Accounts": "Null",
                        "Currencies": "Null",
                        "Tokens": "Null",
                        "Vesting": "Null",
                        "Utility": "Null",
                        "Multisig": "Null",
                        "Recovery": "Null",
                        "Proxy": "Null",
                        "Scheduler": "Null",
                        "Indices": "Null",
                        "GraduallyUpdate": "Null",
                        "Authorship": "Null",
                        "Babe": "Null",
                        "Grandpa": "Null",
                        "Staking": "Null",
                        "Session": "Null",
                        "Historical": "Null",
                        "Authority": "DelayedOrigin",
                        "ElectionsPhragmen": "Null",
                        "Contracts": "Null",
                        "EVM": "Null",
                        "Sudo": "Null",
                        "TransactionPayment": "Null"
                    }
                },
                "LockState": {
                    "_enum": {
                        "Committed": "None",
                        "Unbonding": "BlockNumber"
                    }
                },
                "LockDuration": {
                    "_enum": [
                        "OneMonth",
                        "OneYear",
                        "TenYears"
                    ]
                },
                "EraIndex": "u32",
                "Era": {
                    "index": "EraIndex",
                    "start": "BlockNumber"
                },
                "Commitment": {
                    "state": "LockState",
                    "duration": "LockDuration",
                    "amount": "Balance",
                    "candidate": "AccountId"
                },
                "CommitmentOf": "Commitment",
                "ClassInfoOf":"ClassId"
            }
        });
        console.log("[INFO] Connected to Bitgreen Node with genesis: ",api.genesisHash.toHex())
        //make transfer
            api.tx.claim.claimDeposit(oldaddress,oldpublickey,signature,recipient)
            .signAndSend(keyspair, ({ status, events, dispatchError }) => {
                try {
                    // status would still be set, but in the case of error we can shortcut
                    // to just check it (so an error would indicate InBlock or Finalized)
                    if (dispatchError) {
                    if (dispatchError.isModule) {
                        // for module errors, we have the section indexed, lookup
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        const { documentation, name, section } = decoded;
                        console.log(`${section}.${name}: ${documentation.join(' ')}`);
                    } else {
                        // Other, CannotLookup, BadOrigin, no extra info
                        console.log(dispatchError.toString());
                    }
                    }
                } catch{
                    console.log("[Error] Too many transactions in short time");
                }
                    
            });
            let v='{"answer":"ok"}';
            res.send(v);
    });
    
// listening to server port
console.log("[info] - listening for connections on port TCP/3001...");
let server=app.listen(3001,function() {});
/*
// loading certificate/key
const options = {
    key: fs.readFileSync("/etc/letsencrypt/live/testnet.bitg.org/privkey.pem"),
    cert: fs.readFileSync("/etc/letsencrypt/live/testnet.bitg.org/fullchain.pem")
};
// Https listening on port 8443 -> proxy to 3000
https.createServer(options, app).listen(8443);
*/
}

//function to return content of a file name
function read_file(name){
    const fs = require('fs');
    if(!fs.existsSync(name))
        return(undefined);
    try {
        const data = fs.readFileSync(name, 'utf8')
        return(data);
      } catch (err) {
        console.error(err);
        return(undefined);
      }
}
// function to check validity of an account address
async function isValidAccountAddress(address){
  try {
    encodeAddress(
      isHex(address)
        ? hexToU8a(address)
        : decodeAddress(address)
    );

    return true;
  } catch (error) {
    return false;
  }
}
