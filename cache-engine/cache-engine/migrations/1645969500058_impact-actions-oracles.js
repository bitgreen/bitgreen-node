/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('impact_actions_oracles', {
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
        description: {
            type: 'varchar(128)',
            notNull: true
        },
        signer: {
            type: 'varchar(64)',
            notNull: true
        },
        account: {
            type: 'varchar(48)',
            notNull: true
        },
        other_info: {
            type: 'varchar(66)',
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
    pgm.dropTable('impact_actions_oracles')
};