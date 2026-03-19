Training vocabulary in any language for kids

# Run locally

WebUI:

```
cd src\WebUi\
pnpm run dev
```

WebApi:

```
cd src\WebApi\
pnpm run dev
```

Rust backend + WebUI:

```
run-local.bat
```

This starts the Rust backend on `8101`, the Vite client on `127.0.0.1:5173`, and opens the browser automatically.

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

# SSL certificates

- follow https://github.com/sagardere/set-up-SSL-in-nodejs
- make sure to grant read permissions to "users"

# Rust backend notes

- the Rust backend lives in `src/WebApi-Rust`
- it is currently standalone and not wired into the UI or Docker build yet
- run it with `cd src\WebApi-Rust && cargo run`
- it uses the same API shape as the existing backend
- by default it listens on `8101` and `8102`; set `VOCABULARRY_HTTP_PORT` and `VOCABULARRY_HTTPS_PORT` to run it beside the current backend
- local development keeps using `../../` as the repository root, matching the old backend
- production-style config still uses `NODE_ENV=production` and `VOCABULARRY_HOME`
- HTTPS is enabled automatically when `selfsigned.crt` and `selfsigned.key` exist in `VOCABULARRY_HOME`
