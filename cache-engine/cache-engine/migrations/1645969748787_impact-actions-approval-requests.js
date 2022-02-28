/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('impact_actions_approval_requests', {
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
        signer: {
            type: 'varchar(64)',
            notNull: true
        },
        info: {
            type: 'varchar(8192)',
            notNull: true
        },
        date: {
            type: 'timestamp',
            notNull: true
        },
        date_approved: {
            type: 'timestamp'
        },
        date_refused: {
            type: 'timestamp'
        },
        created_at: {
            type: 'timestamp',
            notNull: true,
            default: pgm.func('current_timestamp')
        }
    })
};

exports.down = pgm => {
    pgm.dropTable('impact_actions_approval_requests')
};