
import { db } from "../db";
import { publicProcedure, router } from "./trpc";

export const appRouter = router({
  userList: publicProcedure.query(async () => {
    const users = await db.user.findMany();
    return users;
  }),
});

// Export type router type signature, this is used by the client.
export type AppRouter = typeof appRouter;
