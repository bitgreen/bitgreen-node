//******************************************************************************************************
// This is an Oracle that receive a redeem code over https, check the validity of such code contacting
// an API url and eventually mints a fungible assets for the amount received back
//******************************************************************************************************

// pulling required libraries
let express = require('express');
const https = require("https");
let fs = require('fs');
let mysql= require('mysql');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { decodeAddress, encodeAddress } = require('@polkadot/keyring');
const { hexToU8a, isHex } = require('@polkadot/util');

console.log("[Info] - BitGreen - Oracle for Impact Actions v. 1.10 - Starting");
console.log("This is an Oracle that receive a redeem code over https, check the validity of such code contacting an API url and eventually mints a fungible assets for the amount received in the answer.")

// read the environment varianles required
const APIURL=process.env.APIURL
const SSL_CERT=process.env.SSL_CERT
const SSL_KEY=process.env.SSL_KEY
const TLSPORT=process.env.TLSPORT
const HTTPPORT=process.env.HTTPPORT
const SECRETSEED=process.env.SECRETSEED
const TOKENID=process.env.TOKENID
const NODE=process.env.NODE


// verify mandatory environment variables
if (typeof APIURL === 'undefined'){
    console.log("[Error] the environment variable APIURL is not set.");
    process.exit(1);
}
if (typeof SECRETSEED === 'undefined'){
    console.log("[Info] the environment variable SECRETSEED is not set");
    process.exit(1);
}
if (typeof TOKENID === 'undefined'){
    console.log("[Info] the environment variable TOKENID is not set");
    process.exit(1);
}
if (typeof NODE === 'undefined'){
    console.log("[Info] the environment variable TOKENID is not set");
    process.exit(1);
}
// read other variables, setting default if not configured
if (typeof TLSPORT === 'undefined'){
    console.log("[Info] the environment variable APIURL is not set, setting default to 7443");
    TLSPORT="7443"
}
if (typeof HTTPPORT === 'undefined'){
    console.log("[Info] the environment variable HTTPPORT is not set, setting default to 3003");
    HTTPPORT="3003"
}

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
    //get transactions in the date/time limits
    app.get('/redeem',async function(req, res) {
        if(typeof req.query.code ==='undefined') {
            res.send('{"answer":"KO","message":"code parameter has not been received');
            
        }else {
            if(typeof req.query.account ==='undefined') {
                res.send('{"answer":"KO","message":"account parameter has not been received');
            }else {
                if ( await isValidAccountAddress(req.query.account)==false) {
                    res.send('{"answer":"KO","message":"account parameter is not valid');
                }else {
                    code=req.query.code;
                    account=req.query.account;
                    console.log("Checking Redeem Code: ",code," Account: ",account);
                    redeem_code(res,code,account);
                }
            }
        }
    });
    // listening to server port
    console.log("[Info] - Listening for HTTP connections on port TCP/",HTTPPORT);
    let server=app.listen(HTTPPORT,function() {});
    if(typeof SSL_CERT!=='undefined' && SSL_KEY!=='undefined'){
        // loading certificate/key
        const options = {
            key: fs.readFileSync(SSL_KEY),
            cert: fs.readFileSync(SSL_CERT)
        };
        console.log("[Info] - Listening for TLS connections on port TCP/",TLSPORT);
        // Https listening on port 9443 -> proxy to 3002
        https.createServer(options, app).listen(TLSPORT);
    }
}

// function to verify the redeem code and eventually mint the ERC20
async function redeem_code(res,code,account){
   let url=HTTPAPI.replace("%CODE%",code);
   fetch(url)
   .then(data => {
       let r=JSON.parse(data);
       tokens=1 // default to 1 token
       if(r.answer=="OK"){
           // we try to get the amount to mint
           if(typeof r.tokens !='undefined'){
               tokens=r.tokens;
           }
           // minting the tokens earned (TODO)
           mint_tokens(res,tokens);
       } else {
        // in case of error we send back the answer received
        res.send(data);
        return;
       }
       
    })
   .then(res =>{console.log(res)})
}
// function to mint tokens (Assets pallet)
async function mint_tokens(res,tokens,account){
    // generate key pair from Seed
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 42 });
    
    const wsProvider = new WsProvider(NODE);  
    console.log("Node: ",NODE," Seed: ",SECRETSEED);
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
                    "TransactionPayment": "Null",
                    "Assets": "Null",
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
    const keyspair = keyring.createFromUri(SECRETSEED, { name: 'sr25519' });
    //Mint the token 
    api.tx.assets.mint(TOKENID,account,tokens)
    .signAndSend(keyspair, ({ status, events, dispatchError }) => {
       // status would still be set, but in the case of error we can shortcut
       // to just check it (so an error would indicate InBlock or Finalized)
       if (dispatchError) {
       if (dispatchError.isModule) {
           // for module errors, we have the section indexed, lookup
           const decoded = api.registry.findMetaError(dispatchError.asModule);
           console.log(decoded);
           const { docs, name, section } = decoded;
           console.log(`${section}.${name}: ${docs.join(' ')}`);
           res.send('{"answer":"KO","message":"'+dispatchError.toString()+'"}');
       } else {
           // Other, CannotLookup, BadOrigin, no extra info
           console.log(dispatchError.toString());
           res.send('{"answer":"KO","message":"'+dispatchError.toString()+'"}');

       } 
       }
   else {
           console.log('{"answer":"OK","message":"Redeem accepted"}');
           res.send('{"answer":"OK","message":"Redeem accepted"}');
       }             
    });
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
