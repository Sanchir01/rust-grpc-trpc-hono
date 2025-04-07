import { postRepository } from '../repository/post'
import { createPostSchema, DeletePost } from '../repository/types'
import { publicProcedure, router } from './trpc'

export const appRouter = router({
	postList: publicProcedure.query(async () => {
		const users = await postRepository.post.findMany()
		return users
	}),
	postCreate: publicProcedure
		.input(createPostSchema)
		.mutation(async ({ ctx, input }) => {
			const post = await postRepository.post.create(input.post)
			return post
		}),
	deletePost: publicProcedure
		.input(DeletePost)
		.mutation(async ({ ctx, input }) => {
			const post = await postRepository.post.delete(Number(input.message_id))
			return post
		})
})

// Export type router type signature, this is used by the client.
export type AppRouter = typeof appRouter
