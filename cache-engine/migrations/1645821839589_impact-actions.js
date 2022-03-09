/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('impact_actions', {
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
        category: {
            type: 'int',
            notNull: true
        },
        auditors: {
            type: 'int',
            notNull: true
        },
        block_start: {
            type: 'int',
            notNull: true
        },
        block_end: {
            type: 'int',
            notNull: true
        },
        rewards_token: {
            type: 'int',
            notNull: true
        },
        rewards_amount: {
            type: 'int',
            notNull: true
        },
        rewards_oracle: {
            type: 'int',
            notNull: true
        },
        rewards_auditors: {
            type: 'int',
            notNull: true
        },
        slashing_auditors: {
            type: 'int',
            notNull: true
        },
        max_errors_auditor: {
            type: 'int',
            notNull: true
        },
        fields: {
            type: 'varchar(8192)',
            notNull: true
        },
        signer: {
            type: 'varchar(64)',
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
    pgm.dropTable('impact_actions')
};

