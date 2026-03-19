ARG NODE_VERSION=22
ARG RUST_VERSION=1.86

FROM node:${NODE_VERSION}-alpine AS frontend-build

WORKDIR /usr/src/app/WebUI

RUN npm install -g pnpm

COPY src/WebUI/package.json ./package.json
COPY src/WebUI/pnpm-lock.yaml ./pnpm-lock.yaml
RUN pnpm install --frozen-lockfile

COPY src/WebUI ./
RUN pnpm run build

FROM rust:${RUST_VERSION}-alpine AS backend-build

WORKDIR /usr/src/app/WebApi

COPY src/WebApi/Cargo.toml ./Cargo.toml
COPY src/WebApi/Cargo.lock ./Cargo.lock
COPY src/WebApi/src ./src
COPY src/WebApi/tests ./tests
COPY src/WebApi/rustfmt.toml ./rustfmt.toml
COPY --from=frontend-build /usr/src/app/WebApi/public ./public

RUN cargo build --release

FROM alpine AS production-stage

ENV NODE_ENV=production

RUN addgroup -S app && adduser -S app -G app
USER app

WORKDIR /app

COPY --from=backend-build /usr/src/app/WebApi/target/release/vocabu-larry-api ./vocabu-larry-api
COPY --from=backend-build /usr/src/app/WebApi/public ./public

EXPOSE 8101 8102

CMD ["/app/vocabu-larry-api"]
