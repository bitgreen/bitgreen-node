/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
	    pgm.createTable('bondsmain', {
        id: 'id',
        KycStored: {
            type: 'int',
            notNull: true
        },
        KycSignedforApproval: {
            type: 'varchar(66)',
            unique: true,
            notNull: true
        },
        KycApproved: {
            type: 'varchar(64)',
            notNull: true
        },
        sender: {
            type: 'varchar(48)',
            notNull: true
        },
        category: {
            type: 'varchar(20)',
            notNull: true
        },
        recipient: {
            type: 'varchar(48)',
            notNull: true
        },
        amount: {
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
	    pgm.dropTable('bondsmain')
};