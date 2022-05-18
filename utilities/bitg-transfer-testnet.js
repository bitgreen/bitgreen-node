//****************************************
// BITG Minting for Test Net
//****************************************
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
const SECRETSEED=""; // for example "join upgrade peasant must quantum verify beyond bullet velvet machine false replace"
// end customizaton section
//***************************************************************************************************
console.log("[Info] - BBB for TESTNET - ver. 1.00 - Starting");
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
    app.post('/mint',async function(req, res) {
        console.log(req.body);
        account=req.body.Account;
        let va= await isValidAccountAddress(account);
        if (va===false){
            let w=read_file("wrongaccount.html");
            res.send(w);
            return;
        }
        console.log("[Info] Transferring 100 BBB for ",account);
        
        // generate key pair from Seed
        const keyring = new Keyring({ type: 'sr25519', ss58Format: 42 });
        const keyspair = keyring.createFromUri(SECRETSEED, { name: 'sr25519' });
        const wsProvider = new WsProvider('wss://testnet.bitgreen.org');  
        const api = await ApiPromise.create({ provider: wsProvider,"types" :
            {
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
                    "BBB": 0,
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
                "AssetDetails": {
                    "owner": "AccountId",
                    "issuer": "AccountId",
                    "admin": "AccountId",
                    "freezer": "AccountId",
                    "supply": "Balance",
                    "deposit": "DepositBalance",
                    "max_zombies": "u32",
                    "min_balance":"Balance",
                    "zombies":"u32",
                    "accounts":"u32",
                    "is_frozen":"bool"
                },
                "AssetMetadata": {
                    "deposit":"DepositBalance",
                    "name": "Vec<u8>",
                    "symbol": "Vec<u8>",
                    "decimals":"u8"
                },
                "AssetBalance" : {
                    "balance":"Balance",
                    "is_frozen":"bool",
                    "is_zombie":"bool"
                },
                "AssetId":"u32",
                "BalanceOf":"Balance",
                "VCU": {
                "serial_number": "i32",
                "project": "Vec<u8>",
                "amount_co2": "Balance",
                "ipfs_hash": "Vec<u8>"
                },
                "CommitmentOf": "Commitment",
                "ClassInfoOf":"ClassId"    
            }
        });
        console.log("[INFO] Connected to Bitgreen Node with genesis: ",api.genesisHash.toHex())
        //make transfer
            api.tx.balances
            .transfer(account, BigInt(100000000000000000000))
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
            let v=read_file("confirmation.html");
            res.send(v);
    });
    // logo.png output
    app.route('/logo').get(function(req,res)
    {
        let s = fs.createReadStream("logo.png");
        s.on('open', function () {
            res.set('Content-Type', 'image/png');
            s.pipe(res);
        });
        s.on('error', function () {
            res.set('Content-Type', 'text/plain');
            res.status(404).end('logo.png not found');
        });
    });
// listening to server port
console.log("[info] - listening for connections on port TCP/3000...");
let server=app.listen(3000,function() {});
// loading certificate/key
const options = {
    key: fs.readFileSync("/etc/letsencrypt/live/testnet.bitgreen.org/privkey.pem"),
    cert: fs.readFileSync("/etc/letsencrypt/live/testnet.bitgreen.org/fullchain.pem")
};
// Https listening on port 8443 -> proxy to 3000
https.createServer(options, app).listen(8443);
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
function isValidAccountAddress(address){
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
