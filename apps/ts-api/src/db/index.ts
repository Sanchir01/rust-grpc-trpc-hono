import { drizzle } from 'drizzle-orm/postgres-js'
//@ts-ignore
import postgres from 'postgres'
if (!process.env.DATABASE_URL) {
	console.error('DATABASE_URL is not defined in environment variables')
	process.exit(1)
}
const environment = process.env.NODE_ENV || 'development'
const queryClient = postgres(
	process.env.NODE_ENV === 'production'
		? (process.env.DATABASE_URL as string)
		: 'postgresql://postgres:password@localhost:5432/postgres'
)
export const PGDB = drizzle(queryClient)
