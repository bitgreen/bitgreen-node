/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('ft_assets', {
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
        owner: {
            type: 'varchar(48)',
            notNull: true
        },
        max_zombies: {
            type: 'int',
            notNull: true
        },
        min_balance: {
            type: 'int',
            notNull: true
        },
        asset_id: {
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
    pgm.dropTable('ft_assets')
};