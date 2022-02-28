const Pool = require('pg').Pool
const pool = new Pool()

const getBlockByNumber = (request, response) => {
    let { number } = request

    pool.query('SELECT * FROM blocks WHERE number = $1',
        [number], (error, results) => {
        if (error) {
            return error
        }
        return results.rows;
    })
}

const storeBlock = (request, response) => {
    let { number, hash, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('SELECT * FROM blocks WHERE number = $1',
        [number], (error, results) => {
            if (error) {
                return error
            }
            if(results.rows.length > 0) {
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

const storeTransaction = (request, response) => {
    let { block_number, hash, sender, recipient, amount, gas_fees, date } = request
    date = new Date(parseInt(date)).toISOString()

    pool.query('INSERT INTO transactions ("block_number", "hash", "sender", "recipient", "amount", "gas_fees", "date") VALUES ($1, $2, $3, $4, $5, $6, $7)',
        [block_number, hash, sender, recipient, amount, gas_fees, date], (error, results) => {
        if (error) {
            throw error
        }
        // response.status(201).send(`User added with ID: ${result.insertId}`)
    })
}

module.exports = {
    storeTransaction,
    storeBlock
}