/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('impact_actions_approval_requests_auditors_votes', {
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
        vote: {
            type: 'varchar(1)',
            notNull: true
        },
        other_info: {
            type: 'varchar(66)',
            notNull: true
        },
        rewards: {
            type: 'timestamp',
            notNull: true
        },
        approval_request_id: {
            type: 'int',
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
    pgm.dropTable('impact_actions_approval_requests_auditors_votes')
};