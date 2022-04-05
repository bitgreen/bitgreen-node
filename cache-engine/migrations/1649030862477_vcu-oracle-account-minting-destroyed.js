/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('vcu_oracle_account_minting_destroyed', {
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
        agv_account: {
            type: 'varchar(64)',
            notNull: true
        },
        agv_id: {
            type: 'int',
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
    pgm.dropTable('vcu_oracle_account_minting_destroyed')
};