FROM oven/bun:1.4-alpine AS web
WORKDIR /app
COPY package.json bun.lock ./
COPY apps/web/package.json apps/web/
RUN bun install --frozen-lockfile
COPY apps/web apps/web
RUN bun run build

FROM rust:1-alpine AS api
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY apps/api apps/api
COPY --from=web /app/apps/web/dist apps/web/dist
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked && cp target/release/vcc-api /vcc-api

FROM scratch
COPY --from=api /vcc-api /vcc-api
USER 65534
EXPOSE 8080
ENTRYPOINT ["/vcc-api"]
