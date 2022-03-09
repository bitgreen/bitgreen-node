/* eslint-disable camelcase */

exports.shorthands = undefined;

exports.up = pgm => {
    pgm.createTable('blocks', {
        id: 'id',
        number: {
            type: 'int',
            notNull: true,
            unique: true
        },
        hash: {
            type: 'varchar(66)',
            unique: true,
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
        },
    })
};

exports.down = pgm => {
    pgm.dropTable('blocks')
};
