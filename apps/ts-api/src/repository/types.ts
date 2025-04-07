import { z } from 'zod'

export const PostSchema = z.object({
	chat_id: z.string().transform(val => BigInt(val)),
	message_id: z.string().transform(val => Number(val)),
	title: z.string(),
	timestamp: z.string().transform(val => new Date(val)),
	post_timestamp: z.string().transform(val => new Date(val)),
	text: z.string().min(1).max(500)
})
export const createPostSchema = z.object({
	post: PostSchema
})
export const DeletePost = z.object({
	message_id: z.string().transform(val => Number(val))
})
