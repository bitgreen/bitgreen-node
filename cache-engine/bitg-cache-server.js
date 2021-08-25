//*********************************************************************************************
// Cache Server that offers API to query blockchain transactions by simple https calls
//*********************************************************************************************

// pulling required libraries
let express = require('express');
const https = require("https");
let fs = require('fs');
let mysql= require('mysql');

console.log("[Info] - BitGreen Cache  Server - ver. 1.00 - Starting");
// read the database configuration from environment variables
const DB_HOST=process.env.DB_HOST
const DB_NAME=process.env.DB_NAME
const DB_USER=process.env.DB_USER
const DB_PWD=process.env.DB_PWD
const SSL_CERT=process.env.SSL_CERT
const SSL_KEY=process.env.SSL_KEY
// set default to local host if not set
if (typeof DB_HOST === 'undefined'){
    console.log("[Error] the environment variable DB_HOST is not set.");
    process.exit(1);
}
if (typeof DB_NAME === 'undefined'){
    console.log("[Error] the environment variable DB_NAME is not set.");
    process.exit(1);
}
// DB_USER is mandatory
if (typeof DB_USER  === 'undefined'){
    console.log("[Error] the environment variable DB_USER is not set.");
    process.exit(1);
}
// DB_PWD is mandatory
if (typeof DB_PWD === 'undefined'){
    console.log("[Error] the environment variable DB_PWD is not set.");
    process.exit(1);
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
    app.get('/transactions',async function(req, res) {
        account=req.query.account;
        let dtstart='1990-01-01 00:00:00';
        let dtend='2999-12-31 11:59:59';
        if (typeof req.query.dts!=='undefined'){
            dtstart=req.query.dts;
        }
        if (typeof req.query.dte!=='undefined'){
            dtend=req.query.dte;
        }
        console.log("Get transactions for account:",account," from: ",dtstart," to: ",dtend);
        get_transactions(res,account,dtstart,dtend);
    });
    //get single transaction by txhash
    app.get('/transaction',async function(req, res) {
        account=req.query.account;
        let txhash='';
        if (typeof req.query.txhash!=='undefined'){
            txhash=req.query.txhash;
        }
        console.log("Get single transaction: ",txhash);
        get_transaction(res,txhash);
    });
    //get impact actions configuration
    app.get('/impactactions',async function(req, res) {
        console.log("Get Impact Action ");
        get_impactactions(res);
    });
    //get oracles in impact actions
    app.get('/impactactionsoracles',async function(req, res) {
        console.log("Get Impact Action Oracles");
        get_impactactions_oracles(res);
    });
    //get categories of impact actions
    app.get('/impactactionscategories',async function(req, res) {
        console.log("Get Impact Action Categories");
        get_impactactions_categories(res);
    });
    //get proxy accounts in impact actions
    app.get('/impactactionsproxies',async function(req, res) {
        console.log("Get Impact Action Proxies");
        get_impactactions_proxies(res);
    });
    // listening to server port
    console.log("[Info] - Listening for HTTP connections on port TCP/3002");
    let server=app.listen(3002,function() {});
    if(typeof SSL_CERT!=='undefined' && SSL_KEY!=='undefined'){
        // loading certificate/key
        const options = {
            key: fs.readFileSync(SSL_KEY),
            cert: fs.readFileSync(SSL_CERT)
        };
        console.log("[Info] - Listening for TLS connections on port TCP/9443");
        // Https listening on port 9443 -> proxy to 3002
        https.createServer(options, app).listen(9443);
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
// function to send impact actions configuration in json format
async function get_impactactions(res){
    let connection = mysql.createConnection({
        host     : DB_HOST,
        user     : DB_USER,
        password : DB_PWD,
        database : DB_NAME
    });
    sqlquery="select * from impactactions order by id desc";
    connection.query(
        {
            sql: sqlquery,
        },
        function (error, results, fields) {
            if (error){
                console.log("[Error]"+error);
                throw error;
            }
            if(results.length==0){
                console.log("[Debug] Impact Actions not found");
                res.send('{"proxies":[]}');    
                connection.end();
                return;
            }else{
                let answer='{"impactactions":[';
                let x=0;
                for (r in results) {
                    if(x>0){
                        answer=answer+',';
                    }
                    answer= answer+'{"id":'+results[r].id;
                    answer=answer+',"blocknumber":'+results[r].blocknumber+',"txhash":"'+results[r].txhash+'"';
                    answer=answer+',"sender":"'+results[r].signer+'"';
                    answer=answer+',"description":"'+results[r].description+'"';
                    answer=answer+',"categories":'+results[r].categories;
                    answer=answer+',"auditors":'+results[r].auditors;
                    answer=answer+',"blockstart":'+results[r].blockstart;
                    answer=answer+',"blockend":'+results[r].blockend;
                    answer=answer+',"rewardstoken":'+results[r].rewardstoken;
                    answer=answer+',"rewardsamount":'+results[r].rewardsamount;
                    answer=answer+',"rewardsoracle":'+results[r].rewardsoracle;
                    answer=answer+',"rewardauditors":'+results[r].rewardauditors;
                    answer=answer+',"slashingsauditors":'+results[r].slashingsauditors;
                    answer=answer+',"maxerrorsauditor":'+results[r].maxerrorsauditor;
                    answer=answer+',"slashingsauditors":'+results[r].slashingsauditors;
                    answer=answer+',"fields":'+results[r].fields;
                    answer=answer+',"dtblockchain":"'+results[r].dtblockchain+'"}';
                    x++;
                }
                answer=answer+']}';
                console.log("[Info] Sending Impact Actions: ",answer);
                res.send(answer);
                connection.end();
                return;
            }
        }
    );
}
// function to send impact actions/oracles list in json format
async function get_impactactions_oracles(res){
    let connection = mysql.createConnection({
        host     : DB_HOST,
        user     : DB_USER,
        password : DB_PWD,
        database : DB_NAME
    });
    sqlquery="select * from impactactionsoracles order by id desc";
    connection.query(
        {
            sql: sqlquery,
        },
        function (error, results, fields) {
            if (error){
                console.log("[Error]"+error);
                throw error;
            }
            if(results.length==0){
                console.log("[Debug] Impact Actions Oracles not found");
                res.send('{"oracles":[]}');    
                connection.end();
                return;
            }else{
                let answer='{"categories":[';
                let x=0;
                for (r in results) {
                    if(x>0){
                        answer=answer+',';
                    }
                    answer= answer+'{"id":'+results[r].id;
                    answer=answer+',"blocknumber":'+results[r].blocknumber+',"txhash":"'+results[r].txhash+'"';
                    answer=answer+',"sender":"'+results[r].signer+'"';
                    answer=answer+',"account":"'+results[r].account+'"';
                    answer=answer+',"otherinfo":"'+results[r].otherinfo+'"';
                    answer=answer+',"dtblockchain":"'+results[r].dtblockchain+'"}';
                    x++;
                }
                answer=answer+']}';
                console.log("[Info] Sending Impact Actions Oracles: ",answer);
                res.send(answer);
                connection.end();
                return;
            }
        }
    );
}
// function to send impact actions/proxies list in json format
async function get_impactactions_proxies(res){
    let connection = mysql.createConnection({
        host     : DB_HOST,
        user     : DB_USER,
        password : DB_PWD,
        database : DB_NAME
    });
    sqlquery="select * from impactactionsproxy order by id desc";
    connection.query(
        {
            sql: sqlquery,
        },
        function (error, results, fields) {
            if (error){
                console.log("[Error]"+error);
                throw error;
            }
            if(results.length==0){
                console.log("[Debug] Impact Actions Proxies not found");
                res.send('{"proxies":[]}');    
                connection.end();
                return;
            }else{
                let answer='{"categories":[';
                let x=0;
                for (r in results) {
                    if(x>0){
                        answer=answer+',';
                    }
                    answer= answer+'{"id":'+results[r].id;
                    answer=answer+',"blocknumber":'+results[r].blocknumber+',"txhash":"'+results[r].txhash+'"';
                    answer=answer+',"sender":"'+results[r].signer+'"';
                    answer=answer+',"account":"'+results[r].account+'"';
                    answer=answer+',"dtblockchain":"'+results[r].dtblockchain+'"}';
                    x++;
                }
                answer=answer+']}';
                console.log("[Info] Sending Impact Actions Proxies: ",answer);
                res.send(answer);
                connection.end();
                return;
            }
        }
    );
}
// function to send impact actions/categories list in json format
async function get_impactactions_categories(res){
    let connection = mysql.createConnection({
        host     : DB_HOST,
        user     : DB_USER,
        password : DB_PWD,
        database : DB_NAME
    });
    sqlquery="select * from impactactionscategories order by id";
    connection.query(
        {
            sql: sqlquery,
        },
        function (error, results, fields) {
            if (error){
                console.log("[Error]"+error);
                throw error;
            }
            if(results.length==0){
                console.log("[Debug] Impact Actions Categories not found");
                res.send('{"categories":[]}');    
                connection.end();
                return;
            }else{
                let answer='{"categories":[';
                let x=0;
                for (r in results) {
                    if(x>0){
                        answer=answer+',';
                    }
                    answer= answer+'{"id":'+results[r].id;
                    answer=answer+',"blocknumber":'+results[r].blocknumber+',"txhash":"'+results[r].txhash+'"';
                    answer=answer+',"sender":"'+results[r].signer+'"';
                    answer=answer+',"description":"'+results[r].description+'"';
                    answer=answer+',"dtblockchain":"'+results[r].dtblockchain+'"}';
                    x++;
                }
                answer=answer+']}';
                console.log("[Info] Sending Impact Actions Categories: ",answer);
                res.send(answer);
                connection.end();
                return;
            }
        }
    );
}
// function to send transactions list in json format
async function get_transactions(res,account,dts,dte){
    let connection = mysql.createConnection({
        host     : DB_HOST,
        user     : DB_USER,
        password : DB_PWD,
        database : DB_NAME
    });
    sqlquery="select * from transactions where (sender=? or recipient=?) and dtblockchain>=? and dtblockchain<=? order by dtblockchain,id desc";
    connection.query(
        {
            sql: sqlquery,
            values: [account,account,dts,dte]
        },
        function (error, results, fields) {
            if (error){
                console.log("[Error]"+error);
                throw error;
            }
            if(results.length==0){
                console.log("[Debug] Transactions not found");
                res.send('{"transactions":[]}');    
                connection.end();
                return;
            }else{
                let answer='{"transactions":[';
                let x=0;
                for (r in results) {
                    if(x>0){
                        answer=answer+',';
                    }
                    answer= answer+'{"id":'+results[r].id;
                    answer=answer+',"blocknumber":'+results[r].blocknumber+',"txhash":"'+results[r].txhash+'"';
                    answer=answer+',"sender":"'+results[r].sender+'"';
                    answer=answer+',"recipient":"'+results[r].recipient+'"';
                    answer=answer+',"amount":'+results[r].amount;
                    answer=answer+',"dtblockchain":"'+results[r].dtblockchain+'"}';
                    x++;
                }
                answer=answer+']}';
                console.log("[Info] Sending transactions: ",answer);
                res.send(answer);
                connection.end();
                return;
            }
        }
    );
}
// function to send single transaction  in json format
async function get_transaction(res,txhash){
    let connection = mysql.createConnection({
        host     : DB_HOST,
        user     : DB_USER,
        password : DB_PWD,
        database : DB_NAME
    });
    sqlquery="select * from transactions where txhash=?";
    connection.query(
        {
            sql: sqlquery,
            values: [txhash]
        },
        function (error, results, fields) {
            if (error){
                console.log("[Error]"+error);
                throw error;
            }
            if(results.length==0){
                console.log("[Debug] Transaction not found");
                res.send('{}');    
                connection.end();
                return;
            }else{
                let answer='';
                let x=0;
                for (r in results) {
                    if(x>0){
                        answer=answer+',';
                    }
                    answer= answer+'{"id":'+results[r].id;
                    answer=answer+',"blocknumber":'+results[r].blocknumber+',"txhash":"'+results[r].txhash+'"';
                    answer=answer+',"sender":"'+results[r].sender+'"';
                    answer=answer+',"recipient":"'+results[r].recipient+'"';
                    answer=answer+',"amount":'+results[r].amount;
                    answer=answer+',"dtblockchain":"'+results[r].dtblockchain+'"}';
                    x++;
                }
                console.log("[Info] Sending transaction: ",answer);
                res.send(answer);
                connection.end();
                return;
            }
        }
    );
}
