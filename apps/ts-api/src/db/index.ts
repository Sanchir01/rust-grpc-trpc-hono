import { drizzle } from 'drizzle-orm/postgres-js'
//@ts-ignore
import postgres from 'postgres'

const queryClient = postgres(process.env.DATABASE_URL as string)
export const PGDB = drizzle(queryClient)
