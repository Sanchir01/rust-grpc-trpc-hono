import { trpcServer } from '@hono/trpc-server'
import { Hono } from 'hono'
import { appRouter } from './routes/app'
import { logger } from 'hono/logger'
import { config } from 'dotenv'

config()

const app = new Hono()
console.log(process.env.DATABASE_URL)
app.get('/', c => {
	return c.text('Hello Hono!')
})
app.use('*', logger())
app.use(
	'/trpc/*',
	trpcServer({
		router: appRouter
	})
)

export default {
	port: 5000,
	fetch: app.fetch
}
