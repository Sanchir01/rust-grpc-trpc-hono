import {
	pgTable,
	bigint,
	integer,
	timestamp,
	varchar
} from 'drizzle-orm/pg-core'

export const posts = pgTable('post', {
	chat_id: bigint('chat_id', { mode: 'number' }).primaryKey().notNull(),
	message_id: integer('message_id'),
	timestamp: timestamp('timestamp', { withTimezone: true }),
	post_timestamp: timestamp('post_timestamp', { withTimezone: true }),
	text: varchar('text', { length: 500 })
})
