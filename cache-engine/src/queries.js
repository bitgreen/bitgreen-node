// Bitgreen node infrastructure
// This file contains the block storage functions, the API endpoint functions and 
// the queries used to serve them. 

const Pool = require('pg').Pool
const pool = new Pool()

// *** "VCU" Pallet Section ***
// This section contains functions relating the "VCU" Pallet

// save authorized vcu account to DB
const storeVcuAuthorizedAccount = (request, response) => {
    let { block_number, hash, account, description, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_authorized_accounts ("block_number", "hash", "account", "description", "signer", "date") VALUES ($1, $2, $3, $4, $5, $6)',
        [block_number, hash, account, description, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get authorized accounts by date range and/or by account
const getVcuAuthorizedAccounts = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_authorized_accounts WHERE account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// destroy authorized vcu account to DB
const destroyVcuAuthorizedAccount = (request, response) => {
    let { block_number, hash, account, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_authorized_accounts_destroyed ("block_number", "hash", "account", "signer", "date") VALUES ($1, $2, $3, $4, $5)',
        [block_number, hash, account, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get destroyed authorized accounts by date range and/or by account
const getDestroyedVcuAuthorizedAccounts = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_authorized_accounts_destroyed WHERE account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// save assets generating vcu to DB
const storeVcuAssetsGenerating = (request, response) => {
    let { block_number, hash, agv_account, agv_id, content, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_assets_generating ("block_number", "hash", "agv_account", "agv_id", "content", "signer", "date") VALUES ($1, $2, $3, $4, $5, $6, $7)',
        [block_number, hash, agv_account, agv_id, content, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get assets generating vcu by date range and/or by account
const getVcuAssetsGenerating = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_assets_generating WHERE agv_account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// destroy assets generating vcu to DB
const destroyVcuAssetsGenerating = (request, response) => {
    let { block_number, hash, agv_account, agv_id, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_assets_generating_destroyed ("block_number", "hash", "agv_account", "agv_id", "signer", "date") VALUES ($1, $2, $3, $4, $5, $6)',
        [block_number, hash, agv_account, agv_id, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get destroyed assets generating vcu by date range and/or by account
const getDestroyedVcuAssetsGenerating = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_assets_generating_destroyed WHERE agv_account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// save assets generating schedule vcu to DB
const storeVcuAssetsGeneratingSchedule = (request, response) => {
    let { block_number, hash, agv_account, agv_id, period_days, amount_vcu, token_id, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_assets_generating_schedule ("block_number", "hash", "agv_account", "agv_id", "period_days", "amount_vcu", "token_id", "signer", "date") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)',
        [block_number, hash, agv_account, agv_id, period_days, amount_vcu, token_id, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get assets generating vcu schedule by date range and/or by account
const getVcuAssetsGeneratingSchedule = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_assets_generating_schedule WHERE agv_account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// destroy assets generating schedule vcu to DB
const destroyVcuAssetsGeneratingSchedule = (request, response) => {
    let { block_number, hash, agv_account, agv_id, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_assets_generating_schedule_destroyed ("block_number", "hash", "agv_account", "agv_id", "signer", "date") VALUES ($1, $2, $3, $4, $5, $6)',
        [block_number, hash, agv_account, agv_id, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get destroyed assets generating vcu schedule by date range and/or by account
const getDestroyedVcuAssetsGeneratingSchedule = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_assets_generating_schedule_destroyed WHERE agv_account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// save oracle account minting vcu to DB
const storeVcuOracleAccountMinting = (request, response) => {
    let { block_number, hash, agv_account, agv_id, oracle_account, token_id, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_oracle_account_minting ("block_number", "hash", "agv_account", "agv_id", "oracle_account", "token_id", "signer", "date") VALUES ($1, $2, $3, $4, $5, $6, $7, $8)',
        [block_number, hash, agv_account, agv_id, oracle_account, token_id, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get oracle account minting vcu from DB
const getVcuOracleAccountMinting = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_oracle_account_minting WHERE agv_account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// destroy oracle account minting vcu to DB
const destroyVcuOracleAccountMinting = (request, response) => {
    let { block_number, hash, agv_account, agv_id, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_oracle_account_minting_destroyed ("block_number", "hash", "agv_account", "agv_id", "signer", "date") VALUES ($1, $2, $3, $4, $5, $6)',
        [block_number, hash, agv_account, agv_id, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get oracle account minting vcu from DB
const getDestroyedVcuOracleAccountMinting = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_oracle_account_minting_destroyed WHERE agv_account = $1 AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}


// save proxy settings vcu to DB
const storeVcuProxySettings = (request, response) => {
    let { block_number, hash, accounts, signer, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO vcu_proxy_settings ("block_number", "hash", "accounts", "signer", "date") VALUES ($1, $2, $3, $4, $5)',
        [block_number, hash, accounts, signer, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}


// get assets generating vcu by date range and/or by account
const getVcuProxySettings = (request, response) => {
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM vcu_proxy_settings WHERE date >= $1 AND date <= $2 ORDER BY date,id DESC',
        [date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                accounts: results.rows
            })
        })
}

// *** "Bonds" Pallet Section ***
// This section contains functions relating the "Bonds" Pallet


// *** "Impact Actions" Pallet Section ***
// This section contains functions relating the "Impact Actions" Pallet

// get assets
const getAssets = (request, response) => {
    pool.query('SELECT * FROM ft_assets ORDER BY date,id DESC',
        [], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                assets: results.rows
            })
        })
}

// get trasnactions for a particular asset by date range
const getAssetsTransactions = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }
    let asset_id = request.query.asset_id;

    pool.query('SELECT * FROM ft_transactions WHERE (sender = $1 OR recipient = $1) AND date >= $2 AND date <= $3 AND asset_id = $4 ORDER BY date,id DESC',
        [account, date_start, date_end, asset_id], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                transactions: results.rows
            })
        })
}

// get asset transaction by has
const getAssetsTransaction = (request, response) => {
    let hash = request.query.hash;

    pool.query('SELECT * FROM ft_transactions WHERE hash = $1',
        [hash], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            if(results.rows.length === 0) {
                return response.json({
                    error: 'Transaction not found.'
                }).status(404)
            }
            response.json({
                transaction: results.rows
            })
        })
}

// get impact actions
getImpactActions = (request, response) => {
    pool.query('SELECT * FROM impact_actions ORDER BY date,id DESC',
        [], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions: results.rows
            })
        })
}

// get impact actions approval requests
getImpactActionsApprovalRequests = (request, response) => {
    pool.query('SELECT * FROM impact_actions_approval_requests ORDER BY date,id DESC',
        [], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions_approval_requests: results.rows
            })
        })
}

// get a single impact action request
getImpactActionsApprovalRequest = (request, response) => {
    let approval_request_id = request.query.approval_request_id;

    pool.query('SELECT * FROM impact_actions_approval_requests WHERE id = $1',
        [approval_request_id], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            if(results.rows.length === 0) {
                return response.json({
                    error: 'Approval request not found.'
                }).status(404)
            }
            response.json({
                approval_requests: results.rows
            })
        })
}

// get Impact Actions Approval Requests Auditors
getImpactActionsApprovalRequestsAuditors = (request, response) => {
    let approval_request_id = request.query.approval_request_id;

    pool.query('SELECT * FROM impact_actions_approval_requests_auditors WHERE approval_request_id = $1 ORDER BY date,id DESC',
        [approval_request_id], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions_approval_requests_auditors: results.rows
            })
        })
}

// get Impact Actions Approval Requests Auditors Votes
getImpactActionsApprovalRequestsAuditorsVotes = (request, response) => {
    let approval_request_id = request.query.approval_request_id;

    pool.query('SELECT * FROM impact_actions_approval_requests_auditors_votes WHERE approval_request_id = $1 ORDER BY date,id DESC',
        [approval_request_id], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions_approval_requests_auditors_votes: results.rows
            })
        })
}

// get Impact Actions Auditors
getImpactActionsAuditors = (request, response) => {
    pool.query('SELECT * FROM impact_actions_auditors ORDER BY date,id DESC',
        [], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions_auditors: results.rows
            })
        })
}

// get Impact Actions Categories
getImpactActionsCategories = (request, response) => {
    pool.query('SELECT * FROM impact_actions_categories ORDER BY date,id DESC',
        [], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions_categories: results.rows
            })
        })
}

// get Impact Actions Oracles
getImpactActionsOracles = (request, response) => {
    pool.query('SELECT * FROM impact_actions_oracles ORDER BY date,id DESC',
        [], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions_oracles: results.rows
            })
        })
}

// get Impact Action Proxies 
getImpactActionsProxies = (request, response) => {
    pool.query('SELECT * FROM impact_actions_proxies ORDER BY date,id DESC',
        [], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                impact_actions_proxies: results.rows
            })
        })
}



// get block by number
const getBlockByNumber = (request, response) => {
    let {number} = request

    pool.query('SELECT * FROM blocks WHERE number = $1',
        [number], (error, results) => {
            if (error) {
                return error
            }
            return results.rows;
        })
}

// save block to DB
const storeBlock = (request, response) => {
    let {number, hash, date} = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('SELECT * FROM blocks WHERE number = $1',
        [number], (error, results) => {
            if (error) {
                return error
            }
            if (results.rows.length > 0) {
                pool.query('UPDATE blocks SET hash = $1, date = $2 WHERE number = $3',
                    [hash, date, number], (error, results) => {
                        if (error) {
                            return error
                        }
                    })
            } else {
                pool.query('INSERT INTO blocks ("number", "hash", "date") VALUES ($1, $2, $3)',
                    [number, hash, date], (error, results) => {
                        if (error) {
                            return error
                        }
                    })
            }
        })
}

const storeAnalyzeData = (request, response) => {
    let { block_number, block_hash, tx_hash, section, method } = request

    // skip this section
    if(section === 'timestamp') {
        return
    }

    console.log(`${section}:${method}`);

    pool.query('SELECT * FROM analyze_data WHERE block_number = $1 AND tx_hash = $2',
        [block_number, tx_hash], (error, results) => {
            if (error) {
                return error
            }
            if (results.rows.length === 0) {
                pool.query('INSERT INTO analyze_data ("block_number", "block_hash", "tx_hash", "section", "method") VALUES ($1, $2, $3, $4, $5)',
                    [block_number, block_hash, tx_hash, section, method], (error, results) => {
                        if (error) {
                            console.log(error);
                            return error
                        }
                    })
            }
        })
}

const getAnalyzeData = (request, response) => {
    let section

    if (typeof request.query.section !== 'undefined' && request.query.section !== '') {
        section = request.query.section;
    } else {
        section = 'all'
    }

    if(section === 'all') {
        pool.query('SELECT (array_agg(block_number))[1:5] as block_examples, section, method FROM analyze_data GROUP BY section, method ORDER BY section',
            [], (error, results) => {
                if (error) {
                    // console.log(error.message)
                }
                response.json({
                    data: results.rows
                })
            })
    } else {
        pool.query('SELECT (array_agg(block_number))[1:5] as block_examples, section, method FROM analyze_data WHERE section = $1 GROUP BY section, method ORDER BY section',
            [section], (error, results) => {
                if (error) {
                    // console.log(error.message)
                }
                response.json({
                    data: results.rows
                })
            })
    }
}

// save transaction to DB
const storeTransaction = (request, response) => {
    let { block_number, hash, sender, recipient, amount, gas_fees, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO transactions ("block_number", "hash", "sender", "recipient", "amount", "gas_fees", "date") VALUES ($1, $2, $3, $4, $5, $6, $7)',
        [block_number, hash, sender, recipient, amount, gas_fees, date], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            // response.status(201).send(`User added with ID: ${result.insertId}`)
        })
}

// get transactions by date range
const getTransactions = (request, response) => {
    let account = request.query.account;
    let date_start = '1990-01-01';
    let date_end = '2100-12-31';
    if (typeof request.query.date_start !== 'undefined') {
        date_start = request.query.date_start;
    }
    if (typeof request.query.date_end !== 'undefined') {
        date_end = request.query.date_end;
    }

    pool.query('SELECT * FROM transactions WHERE (sender = $1 OR recipient = $1) AND date >= $2 AND date <= $3 ORDER BY date,id DESC',
        [account, date_start, date_end], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            response.json({
                transactions: results.rows
            })
        })
}

// get transaction by block hash
const getTransaction = (request, response) => {
    let hash = request.query.hash;

    pool.query('SELECT * FROM transactions WHERE hash = $1',
        [hash], (error, results) => {
            if (error) {
                // console.log(error.message)
            }
            if(results.rows.length === 0) {
                return response.json({
                    error: 'Transaction not found.'
                }).status(404)
            }
            response.json({
                transaction: results.rows
            })
        })
}


// export all methods so they can be called externally 
// (from index.js for exmaple)
module.exports = {
    storeBlock,
    storeTransaction,

    storeAnalyzeData,
    getAnalyzeData,

    getTransactions,
    getTransaction,

    getAssets,
    getAssetsTransactions,
    getAssetsTransaction,

    getImpactActions,
    getImpactActionsApprovalRequests,
    getImpactActionsApprovalRequest,
    getImpactActionsApprovalRequestsAuditors,
    getImpactActionsApprovalRequestsAuditorsVotes,
    getImpactActionsAuditors,
    getImpactActionsCategories,
    getImpactActionsOracles,
    getImpactActionsProxies,

    /* vcu */
    storeVcuAuthorizedAccount,
    getVcuAuthorizedAccounts,
    destroyVcuAuthorizedAccount,
    getDestroyedVcuAuthorizedAccounts,

    storeVcuAssetsGenerating,
    getVcuAssetsGenerating,
    destroyVcuAssetsGenerating,
    getDestroyedVcuAssetsGenerating,

    storeVcuAssetsGeneratingSchedule,
    getVcuAssetsGeneratingSchedule,
    destroyVcuAssetsGeneratingSchedule,
    getDestroyedVcuAssetsGeneratingSchedule,

    storeVcuOracleAccountMinting,
    getVcuOracleAccountMinting,
    destroyVcuOracleAccountMinting,
    getDestroyedVcuOracleAccountMinting,

    storeVcuProxySettings,
    getVcuProxySettings
    /* end vcu */
}