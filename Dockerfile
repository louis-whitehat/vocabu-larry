ARG RUST_VERSION=1.86

FROM rust:${RUST_VERSION}-alpine AS frontend-build

WORKDIR /usr/src/app/WebUI-Yew

RUN apk add --no-cache build-base musl-dev \
	&& cargo install trunk --locked --version 0.21.14 \
	&& rustup target add wasm32-unknown-unknown

COPY src/WebUI-Yew/Cargo.toml ./Cargo.toml
COPY src/WebUI-Yew/Cargo.lock ./Cargo.lock
COPY src/WebUI-Yew/Trunk.toml ./Trunk.toml
COPY src/WebUI-Yew/index.html ./index.html
COPY src/WebUI-Yew/styles.css ./styles.css
COPY src/WebUI-Yew/sw.js ./sw.js
COPY src/WebUI-Yew/src ./src

RUN trunk build --release

FROM rust:${RUST_VERSION}-alpine AS backend-build

WORKDIR /usr/src/app/WebApi

RUN apk add --no-cache build-base musl-dev

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
