Training vocabulary in any language for kids

# Run locally

WebUI:

```
cd src\WebUI\
pnpm run dev
```

WebUI-Yew:

```
cd src\WebUI-Yew\
trunk serve
```

WebApi:

```
cd src\WebApi\
cargo run
```

Backend + WebUI:

```
run-local.bat
```

This builds the WebUI, starts the Rust backend in production mode on `8101`, and opens the browser automatically.

# Parallel Yew frontend

- the parallel Rust frontend lives in `src/WebUI-Yew`
- it mirrors the current app features: login, exam, score, and logs
- it talks to the same backend API as the Vue app
- during local development it expects the backend on `http://localhost:8101`
- install `trunk` to run it locally: `cargo install trunk`
- build-check it with `cd src\WebUI-Yew && cargo check --target wasm32-unknown-unknown`

It simulates the production setup locally:

- Rust serves both the API and the built frontend
- `NODE_ENV=production` is set
- `VOCABULARRY_HOME` points at the repository root so dictionaries and score files are found
- logs are written to `logs/` under `VOCABULARRY_HOME` unless `VOCABULARRY_LOG_DIR` is set

# Docker build on Windows for Raspberry PI

## Setup a builder

`docker buildx create --name pi-builder --use`

## Build

`docker buildx build --platform linux/arm64 -t vocabu-larry:pi . --load`

# Deploy without registry

Save on development system

`docker save vocabu-larry:pi --output vocabu-larry.tar`

Copy over to PI, log in and run:

`docker stop vocabu-larry` 
`docker rm vocabu-larry` 
`docker load --input vocabu-larry.tar`

# Docker build on Raspberry PI itself

```
sudo docker build -t vocabu-larry .
```

```
sudo docker stop vocabu-larry
sudo docker rm vocabu-larry
```

## Run on PI

`docker run -d --restart=unless-stopped --name vocabu-larry -p 8101:8101 -p 8102:8102 -v /home/me/vocabu-larry:/app/config -e "VOCABULARRY_HOME=/app/config" vocabu-larry:pi`

If you want logs somewhere else, also set `VOCABULARRY_LOG_DIR`, for example `-e "VOCABULARRY_LOG_DIR=/app/config/logs"`.

# SSL certificates

- follow https://github.com/sagardere/set-up-SSL-in-nodejs
- make sure to grant read permissions to "users"

# Backend notes

- the backend now lives in `src/WebApi`
- run it with `cd src\WebApi && cargo run`
- it serves the built WebUI in production from `public/`
- by default it listens on `8101` and `8102`; set `VOCABULARRY_HTTP_PORT` and `VOCABULARRY_HTTPS_PORT` to change that
- local development keeps using `../../` as the repository root for dictionaries and scores
- production uses `NODE_ENV=production` together with `VOCABULARRY_HOME`
- request errors are logged into one daily `.log` file inside `VOCABULARRY_LOG_DIR` or `logs/` under `VOCABULARRY_HOME`
- the main screen includes a `Logs` page that shows backend log contents in the browser
- HTTPS is enabled automatically when `selfsigned.crt` and `selfsigned.key` exist in `VOCABULARRY_HOME`
