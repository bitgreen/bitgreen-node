/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('transactions', {
        id: 'id',
        block_number: {
            type: 'int',
            notNull: true
        },
        hash: {
            type: 'varchar(66)',
            unique: true,
            notNull: true
        },
        sender: {
            type: 'varchar(64)',
            notNull: true
        },
        recipient: {
            type: 'varchar(64)',
            notNull: true
        },
        amount: {
            type: 'numeric(32,0)',
            notNull: true
        },
        gas_fees: {
            type: 'numeric(32,0)',
            notNull: true
        },
        date: {
            type: 'timestamp',
            notNull: true
        },
        created_at: {
            type: 'timestamp',
            notNull: true,
            default: pgm.func('current_timestamp')
        }
    })
};

exports.down = pgm => {
    pgm.dropTable('transactions')
};
