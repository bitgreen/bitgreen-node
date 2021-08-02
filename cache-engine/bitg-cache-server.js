//*********************************************************************************************
// Cache Server that offers API to query blockchain transactions by simple https calls
//*********************************************************************************************

// pulling required libraries
let express = require('express');
const https = require("https");
let fs = require('fs');
let mysql= require('mysql');

//***************************************************************************************************
// customization section - you can change the following constants upon your preferences
const MYSQLIPADDRESS="127.0.0.1";     // ip address of Mysql/Mariadb server (standard port 3306)
const MYSQLUSERNAME="root";           // username to use for Mysql connection
const MYSQLPWD="Aszxqw1234";          // password of the username above
// end customizaton section
//***************************************************************************************************


console.log("[Info] - BitGreen Cache  Server - ver. 1.00 - Starting");
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
    app.get('/transactions',async function(req, res) {
        console.log(req.body);
        account=req.query.account;
        console.log("account:",account);
        get_transactions(res,account);
    });
    // listening to server port
    console.log("[info] - listening for connections on port TCP/3002 and TLS/9443...");
    let server=app.listen(3002,function() {});
/*
    // loading certificate/key
    const options = {
        key: fs.readFileSync("/etc/letsencrypt/live/testnode.bitg.org/privkey.pem"),
        cert: fs.readFileSync("/etc/letsencrypt/live/testnode.bitg.org/fullchain.pem")
    };
    // Https listening on port 9443 -> proxy to 3001
    https.createServer(options, app).listen(9443);
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
// function to send transactions list in json format
async function get_transactions(res,account){
    let connection = mysql.createConnection({
        host     : MYSQLIPADDRESS,
        user     : MYSQLUSERNAME,
        password : MYSQLPWD,
    });
    sqlquery="select * from bitgreen.transactions where sender=? or recipient=? order by dtblockchain,id desc";
    connection.query(
        {
            sql: sqlquery,
            values: [account,account]
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
