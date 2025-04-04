import { trpcServer } from "@hono/trpc-server";
import { Hono } from "hono";
import { appRouter } from "./routes/app";

const app = new Hono();

app.get("/", (c) => {
  return c.text("Hello Hono!");
});

app.use(
  "/trpc/*",
  trpcServer({
    router: appRouter,
  })
);

export default {
  port: 5000,
  fetch: app.fetch,
};
