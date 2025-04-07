FROM oven/bun:latest as builder

COPY apps/ts-api/package.json apps/ts-api/ ./

RUN bun install --frozen-lockfile

RUN bun run build

CMD ["bun", "run", "prod"]