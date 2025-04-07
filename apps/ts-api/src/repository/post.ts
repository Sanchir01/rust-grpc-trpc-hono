import { posts as postTable } from '../db/schema/post'
import { PGDB } from '../db/index.js'
import { eq } from 'drizzle-orm'
type User = { id: string; title: string }
import postgres = require('postgres')
const users: User[] = [{ id: '1', title: 'Carlo' }]

export const postRepository = {
	post: {
		findMany: async () => {
			try {
				const posts = await PGDB.select().from(postTable)
				if (posts.length === 0) {
					return 'not found posts'
				}
				return posts
			} catch (er) {
				console.error('Error fetching posts:', er)
				return 'error'
			}
		},
		create: async (post: any) => {
			try {
				await PGDB.insert(postTable).values({
					chat_id: post.chat_id,
					message_id: post.message_id,
					timestamp: post.timestamp,
					post_timestamp: post.post_timestamp,
					text: post.text
				})
				return 'success'
			} catch (er) {
				console.error('Error creating post:', er)
				return 'error'
			}
		},
		delete: async (id: number) => {
			try {
				await PGDB.delete(postTable).where(eq(postTable.chat_id, Number(id)))
				return 'success'
			} catch (er) {
				console.error('Error deleting post:', er)
				return 'error'
			}
		}
	}
}
