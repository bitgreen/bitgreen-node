/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('analyze_data', {
        id: 'id',
        block_number: {
            type: 'int',
            notNull: true,
        },
        block_hash: {
            type: 'varchar(66)',
            notNull: true
        },
        tx_hash: {
            type: 'varchar(66)',
            notNull: true
        },
        section: {
            type: 'varchar',
            notNull: true
        },
        method: {
            type: 'varchar',
            notNull: true
        },
        created_at: {
            type: 'timestamp',
            notNull: true,
            default: pgm.func('current_timestamp')
        },
    })
};

exports.down = pgm => {
    pgm.dropTable('analyze_data')
};
